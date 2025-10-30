use ratatui::widgets::ListState;

use crate::{list_tmux_sessions, list_tmuxifier_templates};
use crate::input::EditorMode;

pub enum AppMode {
    MainMenu,
    CreateSession,
    ListSessions,
    SessionActionMenu(String),
}

pub struct App {
    pub name: String,
    pub mode: AppMode,
    pub editor_mode: EditorMode, 

    pub main_menu_items: Vec<&'static str>,
    pub main_menu_selected: usize,
    pub main_list_state: ListState,

    pub sessions: Vec<String>,
    pub session_selected: usize,
    pub session_list_state: ListState,
    pub search_query: String,

    pub templates: Vec<String>,
    pub template_selected: usize,
    pub template_list_state: ListState,

    pub session_actions: Vec<&'static str>,
    pub session_action_selected: usize,
    pub session_action_list_state: ListState,


    pub input_buffer: String,
}

impl App {
    pub fn new() -> Self {
        App {
            name: "tmux-sessioniber".to_string(),
            mode: AppMode::MainMenu,
            editor_mode: EditorMode::Normal,

            main_menu_items: vec!["Create Session", "List Sessions", "Quit"],
            main_menu_selected: 0,
            main_list_state: ListState::default(),

            sessions: list_tmux_sessions(),
            session_selected: 0,
            session_list_state: ListState::default(),
            search_query: String::new(),

            templates: list_tmuxifier_templates(),
            template_selected: 0,
            template_list_state: ListState::default(),

            session_actions: vec![" Attach", " Delete", " Rename"], 
            session_action_selected: 0,
            session_action_list_state: ListState::default(),

            input_buffer: String::new(),
        }
    }

    pub fn filtered_sessions(&self) -> Vec<String> {
        self.sessions
            .iter()
            .filter(|s| s.to_lowercase().contains(&self.search_query.to_lowercase()))
            .cloned()
            .collect()
    }

    pub fn filtered_templates(&self) -> Vec<String> {
        self.templates
            .iter()
            .filter(|s| s.to_lowercase().contains(&self.search_query.to_lowercase()))
            .cloned()
            .collect()
    }
}
