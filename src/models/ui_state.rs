use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use crate::settings::Settings;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UiMode {
    Main,
    TagFilter,
    Settings,
}

pub struct UiState {
    pub ui_mode: UiMode,
    pub show_content: bool,
    pub date: NaiveDate,
    pub user_input: String,
    pub show_create_popup: bool,
    pub show_tag_popup_for: Option<i64>,
    pub selected_tag_id: Option<i64>,
    pub show_delete_confirmation: bool,
    pub search_query: String,
}

impl Default for UiState {
    fn default() -> Self {
        let (settings, _) = Settings::load();
        Self {
            ui_mode: settings.mode,
            show_content: false,
            date: chrono::Utc::now().date_naive(),
            user_input: String::new(),
            show_create_popup: false,
            show_tag_popup_for: None,
            selected_tag_id: None,
            show_delete_confirmation: false,
            search_query: String::new(),
        }
    }
}