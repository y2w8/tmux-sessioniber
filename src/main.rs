mod app;
mod input;
mod theme;
mod config;
mod components;


use app::{App, AppMode};
use input::handle_input;
use config::Config;
use color_eyre::{Result};
use theme::Theme;
use components::{list_widget::styled_list, help_bar::{self, draw_help_bar}, popup::draw_popup};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
    widgets::{ ListState },
};

use std::process::{Command, Output};
use std::default::Default;
use std::io::{self, Write};


// Helper functions for tmux/tmuxifier
fn list_tmux_sessions() -> Vec<String> {
    

let output = Command::new("tmux")
    .arg("ls")
    .output()
    .unwrap();

    if !output.status.success() {
        return vec![];
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().map(|l| l.split(':').next().unwrap_or("").to_string()).collect()
}

fn list_tmuxifier_templates() -> Vec<String> {
    let output = Command::new("tmuxifier").arg("ls").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut v: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
    v.insert(0, "No Template".to_string()); // Option to create without template
    v
}

fn create_tmux_session(name: &str, template: Option<&str>) {
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    match template {
        Some(t) if t == "No Template" => {
            Command::new("tmux")
                .arg("new-session")
                .arg("-d")
                .arg("-s")
                .arg(name)
                .status()
                .unwrap();
        }
        Some(t) => {
            Command::new("tmuxifier")
                .arg("load-session")
                .arg(t)
                .arg(name)
                .status()
                .unwrap();
        }
        None => {
            // fallback: just make a new tmux session
            Command::new("tmux")
                .arg("new-session")
                .arg("-d")
                .arg("-s")
                .arg(name)
                .status()
                .unwrap();
        }    
      }
execute!(io::stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}

fn delete_tmux_session(name: &str) {
    Command::new("tmux").arg("kill-session").arg("-t").arg(name).status().unwrap();
}

fn rename_tmux_session(old: &str, new: &str) {
    Command::new("tmux").arg("rename-session").arg("-t").arg(old).arg(new).status().unwrap();
}

fn attach_tmux_session(name: &str) {
    // Detach from TUI temporarily
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    Command::new("tmux").arg("attach-session").arg("-t").arg(name).status().unwrap();

    // Re-enter TUI
    execute!(io::stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
}

// ------------------ MAIN ------------------
fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let config = Config::load();
    let theme = config.theme();
    let mut app = App::new();


    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
                .split(size);

            match &app.mode {
                AppMode::MainMenu => {
                    // let items: Vec<ListItem> = app
                    //     .main_menu_items
                    //     .iter()
                    //     .map(|i| ListItem::new(format!(" {}", i)))
                    //     .collect();
                    // let list = List::new(items)
                    //     .block(Block::default().title("Main Menu").borders(Borders::ALL))
                    //     .highlight_style(Style::default().bg(Color::Blue));
                    let menu_items: Vec<String> = app.main_menu_items.iter().map(|s| s.to_string()).collect();
                    styled_list(f, chunks[0], menu_items, &mut app.main_list_state, "Main Menu", &theme, app.main_menu_selected);
                    // f.render_stateful_widget(list, chunks[0], &mut main_list_state);
                    // let help = Paragraph::new("Use Up/Down to move | Enter to select");
                    // f.render_widget(help, chunks[1]);
                }
                AppMode::CreateSession => {
                    let templatess = app.filtered_templates();
                    draw_popup(f, chunks[0], "Enter Session Name", &app.input_buffer, &theme);
                    styled_list(f, chunks[0], templatess, &mut app.template_list_state, &format!("Session Name: {}", app.input_buffer), &theme, app.template_selected);
                    // let template_items: Vec<ListItem> = templatess
                    //     .iter()
                    //     .enumerate()
                    //     .map(|(i, s)| {
                    //         if i == app.template_selected {
                    //             ListItem::new(format!(" {}", s))
                    //         } else {
                    //             ListItem::new(format!("  {}", s))
                    //         }
                    //     })
                    //     .collect();
                    // let list = List::new(template_items)
                    //     .block(Block::default().title(format!("Session Name: {}", app.input_buffer)).borders(Borders::ALL))
                    // .highlight_style(Style::default().bg(Color::Blue));
                    // f.render_stateful_widget(list, chunks[0], &mut template_state);
                    // let help = Paragraph::new("Type name | Up/Down to choose template | Enter to create | Esc to cancel");
                    // f.render_widget(help, chunks[1]);
                }
                AppMode::ListSessions => {
                    let filtered = app.filtered_sessions();
                    styled_list(f, chunks[0], filtered, &mut app.session_list_state, "Sessions", &theme, app.session_selected);
                    draw_help_bar(f, chunks[1], &app.search_query, &theme);
                    // let items: Vec<ListItem> = filtered
                    //     .iter()
                    //     .enumerate()
                    //     .map(|(i, s)| {
                    //         if i == app.session_selected {
                    //             ListItem::new(format!(" {}", s))
                    //         } else {
                    //             ListItem::new(format!("  {}", s))
                    //         }
                    //     })
                    //     .collect();
                    // let list = List::new(items)
                    //     .block(Block::default().title("Sessions").borders(Borders::ALL))
                    //     .highlight_style(Style::default().bg(Color::Blue));
                    // f.render_stateful_widget(list, chunks[0], &mut session_list_state);
                    // let help = Paragraph::new(format!("Search: {}", app.search_query));
                    // f.render_widget(help, chunks[1]);
                }
                AppMode::SessionActionMenu(session) => {
                    let actions: Vec<String> = app.session_actions.iter().map(|s| s.to_string()).collect()
;
                    styled_list(f, chunks[0], actions, &mut app.session_action_list_state, &format!("Session: {}", session), &theme, app.session_action_selected);
                    // let items: Vec<ListItem> = actions.iter().map(|a| ListItem::new(a.to_string())).collect();
                    // let list = List::new(items)
                    //     .block(Block::default().title(format!("Session: {}", session)).borders(Borders::ALL))
                    //     .highlight_style(Style::default().bg(Color::Blue));
                    // f.render_stateful_widget(list, chunks[0], &mut ListState::default());
                    // let help = Paragraph::new("Use Up/Down | Enter to select | Esc to cancel");
                    // f.render_widget(help, chunks[1]);
                }
            }
        })?;
        let event = event::read()?;
        if handle_input(event, &mut app) {
            break;
        }
    }
//         if let Event::Key(key) = event::read()? {
//             match &mut app.mode {
//                 AppMode::MainMenu => match key.code {
//                     KeyCode::Char('q') => break,
//                     KeyCode::Up => {
//                         if app.main_menu_selected > 0 { app.main_menu_selected -= 1; main_list_state.select(Some(app.main_menu_selected)); }
//                     }
//                     KeyCode::Down => {
//                         if app.main_menu_selected + 1 < app.main_menu_items.len() { app.main_menu_selected += 1; main_list_state.select(Some(app.main_menu_selected)); }
//                     }
//                     KeyCode::Enter => {
//                         match app.main_menu_selected {
//                             0 => app.mode = AppMode::CreateSession,
//                             1 => {
//                                 app.sessions = list_tmux_sessions();
//                                 app.mode = AppMode::ListSessions;
//                             }
//                             2 => break,
//                             _ => {}
//                         }
//                     }
//                     _ => {}
//                 },
//                 AppMode::CreateSession => match key.code {
//                     KeyCode::Esc => app.mode = AppMode::MainMenu,
//                     KeyCode::Char(c) => app.input_buffer.push(c),
//                     KeyCode::Backspace => { app.input_buffer.pop(); }
//                     KeyCode::Up => {
//                         if app.template_selected > 0 { app.template_selected -= 1; }
// template_state.select(Some(app.template_selected));
//                     }
//                     KeyCode::Down => {
//                         if app.template_selected + 1 < app.templates.len() { app.template_selected += 1; }
// template_state.select(Some(app.template_selected));
//                     }
//                     KeyCode::Enter => {
//                         let name = app.input_buffer.clone();
//                         let template = app.templates.get(app.template_selected).map(|s| s.as_str());
//                         create_tmux_session(&name, template);
//                         app.sessions = list_tmux_sessions();
//                         app.input_buffer.clear();
//                         app.template_selected = 0;
//                         app.mode = AppMode::MainMenu;
//                     }
//                     _ => {}
//                 },
//                 AppMode::ListSessions => match key.code {
//                     KeyCode::Esc => app.mode = AppMode::MainMenu,
//                     KeyCode::Up => {
//                         if app.session_selected > 0 { app.session_selected -= 1; session_list_state.select(Some(app.session_selected)); }
//                     }
//                     KeyCode::Down => {
//                         if app.session_selected + 1 < app.filtered_sessions().len() { app.session_selected += 1; session_list_state.select(Some(app.session_selected)); }
//                     }
//                     KeyCode::Char(c) => {
//                         // basic search
//                         app.search_query.push(c);
//                         app.session_selected = 0;
//                         session_list_state.select(Some(app.session_selected));
//                     }
//                     KeyCode::Backspace => {
//                         app.search_query.pop();
//                         app.session_selected = 0;
//                         session_list_state.select(Some(app.session_selected));
//                     }
//                     KeyCode::Enter => {
//                         if !app.filtered_sessions().is_empty() {
//                             let session_name = app.filtered_sessions()[app.session_selected].clone();
//                             app.mode = AppMode::SessionActionMenu(session_name);
//                         }
//                     }
//                     _ => {}
//                 },
//                 AppMode::SessionActionMenu(session) => match key.code {
//                     KeyCode::Esc => app.mode = AppMode::ListSessions,
//                     KeyCode::Enter => {
//                         // for demo, attach session
//                         delete_tmux_session(session);
//                         app.sessions = list_tmux_sessions();
//                         app.mode = AppMode::ListSessions;
//                     }
//                     _ => {}
//                 },
//             }
//         }
    

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
