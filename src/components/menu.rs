
use ratatui::{
    layout::Rect,
    widgets::{List, ListItem, ListState},
    style::{Style, Color},
    Frame,
};

use crate::theme::Theme;

pub fn render_main_menu<'a>(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
    items: &[&'a str],
    selected: usize,
    state: &mut ListState,
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, text)| {
            if i == selected {
                ListItem::new(format!(" {}", text))
            } else {
                ListItem::new(format!("  {}", text))
            }
        })
        .collect();

    let list = List::new(list_items)
        .block(theme.block_style("Main Menu"))
        .highlight_style(Style::default().bg(theme.highlight_color));

    f.render_stateful_widget(list, area, state);
}
