use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::models::UiMode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub theme: Theme,
    pub mode: UiMode,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

/// Default for Settings, not Theme
impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::Light,
            mode: UiMode::Main,
        }
    }
}

/// Default for Theme (optional if already derived on enum)
impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl Settings {
    pub fn load() -> (Self, PathBuf) {

        /// Use directories crate to find the path
        let proj_dirs = ProjectDirs::from("com", "remysedlak", "clipvault")
            .expect("Unable to get project dirs");
        let path = proj_dirs.config_dir().join("settings.toml");

        /// Create the config directory if it doesn't exist
        let settings = fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();

        (settings, path)
    }

    /// Save settings to the specified path
    pub fn save(&self, path: &PathBuf) {
        if let Ok(toml) = toml::to_string(self) {
            let _ = fs::create_dir_all(path.parent().unwrap());
            let _ = fs::write(path, toml);
        }
    }
}
