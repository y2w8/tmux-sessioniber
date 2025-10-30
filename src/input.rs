use crossterm::event::{Event, KeyCode};
use crate::{
    App, AppMode,
    create_tmux_session, delete_tmux_session, list_tmux_sessions,
};
use ratatui::widgets::{ListState};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
    Visual,
}


pub fn handle_input(event: Event, app: &mut App) -> bool {
    if let Event::Key(key) = event {
        return match app.editor_mode {
            EditorMode::Normal => handle_normal_mode(key.code, app),
            EditorMode::Insert => handle_insert_mode(key.code, app),
            EditorMode::Visual => handle_visual_mode(key.code, app),
        };
    }
    false
}



fn handle_normal_mode(code: KeyCode, app: &mut App) -> bool {
    match &mut app.mode {
        AppMode::MainMenu => match code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('i') => app.editor_mode = EditorMode::Insert, // الدخول للوضع الكتابي
            KeyCode::Char('v') => app.editor_mode = EditorMode::Visual,
            KeyCode::Char('j') | KeyCode::Down => move_down(&mut app.main_menu_selected, app.main_menu_items.len(), &mut app.main_list_state),
            KeyCode::Char('k') | KeyCode::Up => move_up(&mut app.main_menu_selected, app.main_menu_items.len(), &mut app.main_list_state),
            KeyCode::Enter | KeyCode::Char('l') => match app.main_menu_selected {
                0 => app.mode = AppMode::CreateSession,
                1 => { app.sessions = list_tmux_sessions(); app.mode = AppMode::ListSessions; }
                2 => return true,
                _ => {}
            },
            _ => {}
        },

        AppMode::ListSessions => match code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::MainMenu,
            KeyCode::Char('i') => app.editor_mode = EditorMode::Insert,
            KeyCode::Char('v') => app.editor_mode = EditorMode::Visual,
            KeyCode::Char('j') | KeyCode::Down => {
                // compute filtered first (immutable), then mutate selection safely
                let filtered = app.filtered_sessions();
                let len = filtered.len();
                move_down(&mut app.session_selected, len, &mut app.session_list_state);

                // optionally toggle selection when in visual mode (example)
                // let idx = app.session_selected;
                // app.visual_select_toggle(idx);
            }

            KeyCode::Char('k') | KeyCode::Up => {
                let filtered = app.filtered_sessions();
                let len = filtered.len();
                move_up(&mut app.session_selected, len, &mut app.session_list_state);
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if let Some(s) = app.filtered_sessions().get(app.session_selected) {
                    app.mode = AppMode::SessionActionMenu(s.clone());
                }
            }
            _ => {}
        },

        AppMode::CreateSession => match code {
            KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::MainMenu,
            KeyCode::Char('i') => app.editor_mode = EditorMode::Insert,
            KeyCode::Char('j') | KeyCode::Down => move_down(&mut app.template_selected, app.templates.len(), &mut app.template_list_state),
            KeyCode::Char('k') | KeyCode::Up => move_up(&mut app.template_selected, app.templates.len(), &mut app.template_list_state),
            KeyCode::Enter | KeyCode::Char('l') => {
                let name = app.input_buffer.clone();
                let template = app.templates.get(app.template_selected).map(|s| s.as_str());
                create_tmux_session(&name, template);
                app.sessions = list_tmux_sessions();
                app.input_buffer.clear();
                app.mode = AppMode::MainMenu;
            }
            _ => {}
        },

        AppMode::SessionActionMenu(session) => match code {
            KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::ListSessions,
            KeyCode::Char('j') | KeyCode::Down => move_down(&mut app.session_action_selected, app.session_actions.len(), &mut app.session_list_state),
            KeyCode::Char('k') | KeyCode::Up => move_up(&mut app.session_action_selected, app.session_actions.len(), &mut app.session_list_state),
            // KeyCode::Enter => {
            //     match app.session_action_selected {
            //         0 => { Command::new("tmux").arg("attach").arg("-t").arg(session).spawn().ok(); }
            //         1 => delete_tmux_session(session),
            //         2 => { /* rename flow later */ }
            //         _ => {}
            //     }
            //     app.sessions = list_tmux_sessions();
            //     app.mode = AppMode::ListSessions;
            // }
            _ => {}
        },
    }
    false
}



fn handle_insert_mode(code: KeyCode, app: &mut App) -> bool {
    match app.mode {
        AppMode::ListSessions => match code {
            KeyCode::Esc => app.editor_mode = EditorMode::Normal,
            KeyCode::Backspace => { app.search_query.pop(); }
            KeyCode::Char(c) => app.search_query.push(c),
            _ => {}
        },
        AppMode::CreateSession => match code {
            KeyCode::Esc => app.editor_mode = EditorMode::Normal,
            KeyCode::Backspace => { app.input_buffer.pop(); }
            KeyCode::Char(c) => app.input_buffer.push(c),
            _ => {}
        },
        _ => {}
    }
    false
}




fn handle_visual_mode(code: KeyCode, app: &mut App) -> bool {
    match &mut app.mode {
        AppMode::ListSessions => match code {
            KeyCode::Esc => {
                app.editor_mode = EditorMode::Normal;
            }

            KeyCode::Char('j') | KeyCode::Down => {
                // compute filtered first (immutable), then mutate selection safely
                let filtered = app.filtered_sessions();
                let len = filtered.len();
                move_down(&mut app.session_selected, len, &mut app.session_list_state);

                // optionally toggle selection when in visual mode (example)
                // let idx = app.session_selected;
                // app.visual_select_toggle(idx);
            }

            KeyCode::Char('k') | KeyCode::Up => {
                let filtered = app.filtered_sessions();
                let len = filtered.len();
                move_up(&mut app.session_selected, len, &mut app.session_list_state);
            }

            KeyCode::Char('x') => {
                // delete currently selected session in visual mode (example)
                let filtered = app.filtered_sessions();
                if !filtered.is_empty() {
                    if let Some(name) = filtered.get(app.session_selected) {
                        delete_tmux_session(name);
                        app.sessions = list_tmux_sessions();
                        // clamp selection
                        let new_len = app.filtered_sessions().len();
                        if app.session_selected >= new_len && new_len > 0 {
                            app.session_selected = new_len - 1;
                        }
                        app.session_list_state.select(Some(app.session_selected));
                    }
                }
            }

            // you can add space to toggle selection, etc.
            KeyCode::Char(' ') => {
                // toggle selection in a visual-set (see suggestion below)
                // let idx = app.session_selected;
                // app.toggle_visual_selection(idx);
            }

            _ => {}
        },

        _ => {}
    }

    false
}

// pub fn handle_input(event: Event, app: &mut App) -> bool {
//     if let Event::Key(key) = event {
//         match &mut app.mode {
//             AppMode::MainMenu => match key.code {
//                 KeyCode::Char('q') => return true, // quit
//                 KeyCode::Char('k') | KeyCode::Up => move_up(&mut app.main_menu_selected, app.main_menu_items.len(), &mut app.main_list_state),
//                 KeyCode::Char('j') | KeyCode::Down => move_down(&mut app.main_menu_selected, app.main_menu_items.len(), &mut app.main_list_state),
//                 KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => match app.main_menu_selected {
//                     0 => app.mode = AppMode::CreateSession,
//                     1 => { app.sessions = list_tmux_sessions(); app.mode = AppMode::ListSessions; }
//                     2 => return true,
//                     _ => {}
//                 },
//                 _ => {}
//             },
//
//             AppMode::CreateSession => match key.code {
//                 KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::MainMenu,
//                 KeyCode::Char(c) => app.input_buffer.push(c),
//                 KeyCode::Backspace => { app.input_buffer.pop(); }
//                 KeyCode::Char('k') | KeyCode::Up => move_up(&mut app.template_selected, app.templates.len(), &mut app.template_list_state),
//                 KeyCode::Char('j') | KeyCode::Down => move_down(&mut app.template_selected, app.templates.len(), &mut app.template_list_state),
//                 KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
//                     let name = app.input_buffer.clone();
//                     let template = app.templates.get(app.template_selected).map(|s| s.as_str());
//                     create_tmux_session(&name, template);
//                     app.sessions = list_tmux_sessions();
//                     app.input_buffer.clear();
//                     app.template_selected = 0;
//                     app.mode = AppMode::MainMenu;
//                 }
//                 _ => {}
//             },
//
//             AppMode::ListSessions => match key.code {
//                 KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::MainMenu,
//     KeyCode::Char('k') | KeyCode::Up => {
//         let filtered = app.filtered_sessions();
//         let len = filtered.len();
//         move_up(&mut app.session_selected, len, &mut app.session_list_state);
//     }
//
//     KeyCode::Char('j') | KeyCode::Down => {
//         let filtered = app.filtered_sessions();
//         let len = filtered.len();
//         move_down(&mut app.session_selected, len, &mut app.session_list_state);
//     }
//                 KeyCode::Char(c) if c.is_ascii_alphanumeric() => {
//                     app.search_query.push(c);
//                     app.session_selected = 0;
//                     app.session_list_state.select(Some(app.session_selected));
//                 }
//                 KeyCode::Backspace => {
//                     app.search_query.pop();
//                     app.session_selected = 0;
//                     app.session_list_state.select(Some(app.session_selected));
//                 }
//                 KeyCode::Enter => {
//                     if !app.filtered_sessions().is_empty() {
//                         let session_name = app.filtered_sessions()[app.session_selected].clone();
//                         app.mode = AppMode::SessionActionMenu(session_name);
//                     }
//                 }
//                 _ => {}
//             },
//
//             AppMode::SessionActionMenu(session) => match key.code {
//                 KeyCode::Esc | KeyCode::Char('h') => app.mode = AppMode::ListSessions,
//                 KeyCode::Enter => {
//                     delete_tmux_session(session);
//                     app.sessions = list_tmux_sessions();
//                     app.mode = AppMode::ListSessions;
//                 }
//                 _ => {}
//             },
//         }
//     }
//     false
// }

pub fn move_up(selected: &mut usize, len: usize, state: &mut ListState) {
    if *selected > 0 {
        *selected -= 1;
    }
    state.select(Some(*selected));
}

/// Move selection down safely
pub fn move_down(selected: &mut usize, len: usize, state: &mut ListState) {
    if len == 0 { 
        *selected = 0;
        state.select(None);
        return;
    }
    if *selected + 1 < len {
        *selected += 1;
    } else {
        // optional: wrap-around
        // *selected = 0;
    }
    state.select(Some(*selected));
}
