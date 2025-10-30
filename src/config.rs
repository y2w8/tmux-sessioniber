
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::theme::{Theme, ThemeColor};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub theme_config: ThemeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub show_icons: bool,
    pub default_view: String,
    pub template_engine: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub border_color: String,
    pub highlight: ThemeHighlight,
    pub text_color: String,
    pub border_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeHighlight {
    pub bg: String,
    pub fg: String,
}


impl Config {
    pub fn load() -> Self {
        let config_path = Self::path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            let cfg = Self::default();
            // cfg.save();
            cfg
        }
    }

    pub fn save(&self) {
        let config_path = Self::path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let toml_str = toml::to_string_pretty(&self).unwrap();
        fs::write(config_path, toml_str).unwrap();
    }

    fn path() -> PathBuf {
        let dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.join("tmux-sessioniber/config.toml")
    }

    pub fn theme(&self) -> Theme {
        Theme::from_config(self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            settings: Settings {
                show_icons: true,
                default_view: "main".into(),
                template_engine: "tmuxifier".into(),
            },
            theme_config: ThemeConfig {
                border_color: "#6c7086".into(),
                highlight: ThemeHighlight {
                    bg: "#89b4fa".into(),
                    fg: "#1e1e2e".into(),
                },
                text_color: "#cdd6f4".into(),
                border_type: Some("Rounded".into()),
            },
        }
    }
}


// impl Config {
//     pub fn theme(&self) -> Theme {
//         match self.theme.as_str() {
//             "light" => Theme::light(),
//             _ => Theme::dark(),
//         }
//     }
// }
