use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub theme: Theme,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl Settings {
    pub fn load() -> (Self, PathBuf) {
        let proj_dirs = ProjectDirs::from("com", "remysedlak", "clipvault")
            .expect("Unable to get project dirs");
        let path = proj_dirs.config_dir().join("settings.toml");

        let settings = fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default();

        (settings, path)
    }

    pub fn save(&self, path: &PathBuf) {
        if let Ok(toml) = toml::to_string(self) {
            let _ = fs::create_dir_all(path.parent().unwrap());
            let _ = fs::write(path, toml);
        }
    }
}
