use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    layout::Alignment,
};
use crate::config::Config;

pub struct ThemeColor {
    pub bg: Color,
    pub fg: Color,
}

pub struct Theme {
    pub border_color: Color,
    pub highlight: ThemeColor,
    pub text_color: Color,
    pub border_type: BorderType,
}

impl Theme {
    pub fn from_config(cfg: &Config) -> Self {
        let tc = &cfg.theme_config;

        Self {
            border_color: parse_hex_color(&tc.border_color),
            highlight: ThemeColor {
                bg: parse_hex_color(&tc.highlight.bg),
                fg: parse_hex_color(&tc.highlight.fg),
            },
            text_color: parse_hex_color(&tc.text_color),
            border_type: match tc.border_type.as_deref() {
                Some("Plain") => BorderType::Plain,
                Some("Double") => BorderType::Double,
                Some("Thick") => BorderType::Thick,
                _ => BorderType::Rounded,
            },
        }
    }

    /// General block style
    pub fn block_style<'a>(&self, title: &'a str, title_bottom: Option<&'a str>) -> Block<'a> {
        let mut block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color))
            .border_type(self.border_type);

        if let Some(bottom) = title_bottom {
            block = block.title_bottom(bottom);
        }

        block
    }

    /// Popup style (thick border + highlight color)
    pub fn popup_block_style<'a>(&self, title: &'a str) -> Block<'a> {
       let mut block = Block::default()
            .title(title)
            // .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.highlight.fg))
            .border_type(self.border_type);
        block
    }
}

/// helper to parse hex like "#RRGGBB"
fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            return Color::Rgb(r, g, b);
        }
    }
    Color::Gray
}

