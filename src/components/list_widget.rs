
use ratatui::{
    prelude::Frame, layout::Rect, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, ListState}
};

use crate::theme::Theme;

pub fn styled_list<'a>(
    f: &mut Frame,
    area: Rect,
    items: Vec<String>,
    state: &mut ListState,
    title: &'a str,
    theme: &Theme,
    selected_index: usize,
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let prefix = if i == selected_index { "\u{2009}" } else { "\u{2009} " };
            ListItem::new(format!("{} {}", prefix, s))
        })
        .collect();

    let list = List::new(list_items)
        .block(theme.block_style("tmux-sessioniber", Some(title)))
        .highlight_style(Style::default().bg(theme.highlight.bg).fg(theme.highlight.fg));

    // let mut state = ratatui::widgets::ListState::default();
    state.select(Some(selected_index));

    f.render_stateful_widget(list, area, state);
}
