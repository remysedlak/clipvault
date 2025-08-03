use chrono::NaiveDate;

#[derive(PartialEq)]
pub enum UiMode {
    Main,
    TagFilter,
    Settings,
}

pub struct UiState {
    pub ui_mode: UiMode,
    pub show_content: bool,
    pub delete_db: bool,
    pub date: NaiveDate,
    pub user_input: String,
    pub show_create_popup: bool,
    pub show_tag_popup_for: Option<i64>,
    pub selected_tag_id: Option<i64>,
    pub show_delete_confirmation: bool,
    pub reset_settings: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            ui_mode: UiMode::Main,
            show_content: false,
            date: chrono::Utc::now().date_naive(),
            user_input: String::new(),
            show_create_popup: false,
            show_tag_popup_for: None,
            selected_tag_id: None,
            delete_db: false,
            show_delete_confirmation: false,
            reset_settings: false,
        }
    }
}