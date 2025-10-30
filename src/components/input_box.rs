use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    style::Style,
    Frame, backend::Backend,
};
use crate::theme::Theme;

/// Renders an input box (for user text input)
pub fn draw_input_box<B: Backend>(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    theme: &Theme,
) {
    let text = format!("{}: {}", label, value);
    let input = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(theme.text_color));
    f.render_widget(input, area);
}
