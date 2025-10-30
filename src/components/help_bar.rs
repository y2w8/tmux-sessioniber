
use ratatui::{ widgets::Paragraph, layout::Rect, Frame, backend::Backend, style::Style };
use crate::theme::Theme;

/// Draws a help bar (bottom hint area)
pub fn draw_help_bar(
    f: &mut Frame,
    area: Rect,
    text: &str,
    theme: &Theme,
) {
    let paragraph = Paragraph::new(text)
        .block(theme.block_style("Help", Some("help")))
        .style(Style::default().fg(theme.text_color));
    f.render_widget(paragraph, area);
}
