use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, BorderType, Clear, Paragraph},
};
use crate::theme::Theme;

pub fn draw_popup(f: &mut Frame, area: Rect, title: &str, input: &str, theme: &Theme) {
    // popup size
    let popup_area = centered_rect(50, 25, area);

    // الخلفية الشفافة
    f.render_widget(Clear, popup_area);

    // input box text
    let paragraph = Paragraph::new(input)
        .style(Style::default().fg(theme.text_color))
        .alignment(Alignment::Left)
        .block(
            theme.block_style(title, Some("")) 

        );

    f.render_widget(paragraph, popup_area);
}

// helper لتوسيط البوكس
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    horizontal[1]
}
