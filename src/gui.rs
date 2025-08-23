use crate::db::{ self };
use crate::models::{ Clip, Tag, UiMode, UiState };
use crate::settings::{ Settings, Theme };
use crate::ui::components::main_top_panel::{ TopPanel };
use crate::ui::views::main_view::MainView;
use crate::ui::views::tag_filter_view::TagFilterView;
use crate::ui::views::settings_view::SettingsView;
use eframe::egui;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct ClipVaultApp {
    pub settings: Settings,
    pub settings_path: std::path::PathBuf,
    clips: Vec<Clip>,
    db: Connection,
    darkmode: bool,
    window_visible: bool,
    ui_state: UiState,
    tags: Vec<Tag>,
    clip_tags: HashMap<i64, Vec<String>>,
}

impl ClipVaultApp {
    pub fn new(db: Connection) -> Self {
        // Load settings and determine dark mode
        let (settings, settings_path) = Settings::load();
        let darkmode = settings.theme == Theme::Dark;

        let mut app = Self {
            clips: Vec::new(),
            db,
            darkmode,
            settings,
            settings_path,
            tags: Vec::new(),
            clip_tags: HashMap::new(),
            window_visible: true,
            ui_state: UiState::default(),
        };

        // Initialize data
        app.load_clips_based_on_state();
        app.reload_tags();
        app.clip_tags = db::load_clip_tags(&app.db).unwrap_or_default();

        app
    }

    // Helper methods to reduce duplication
    fn load_clips_based_on_state(&mut self) {
        self.clips = if self.ui_state.search_query.is_empty() {
            db::load_recent_clips(&self.db, self.ui_state.clip_limit)
        } else {
            db::search_clips(&self.db, &self.ui_state.search_query, Some(self.ui_state.clip_limit))
        }
            .unwrap_or_default()
            .into_iter()
            .map(Clip::from_tuple)
            .collect();
    }

    fn reload_tags(&mut self) {
        self.tags = db::load_tags(&self.db)
            .unwrap_or_default()
            .into_iter()
            .map(Tag::from_tuple)
            .collect();
    }

    fn reset_to_main_state(&mut self) {
        self.ui_state.ui_mode = UiMode::Main;
        self.ui_state.search_query = "".to_string();
        self.ui_state.date_filter = chrono::Utc::now().date_naive();
        self.load_clips_based_on_state();
    }

    fn get_tag_data(&self) -> Vec<(i64, String, Option<String>)> {
        self.tags
            .iter()
            .map(|t| (t.id, t.name.clone(), t.color.clone()))
            .collect()
    }
}

impl eframe::App for ClipVaultApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.theme = if self.darkmode { Theme::Dark } else { Theme::Light };
        self.settings.mode = self.ui_state.ui_mode;
        self.settings.auto_hide_clips = self.ui_state.auto_hide_clips;
        let _ = self.settings.save(&self.settings_path);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.window_visible {
            return;
        }

        ctx.set_visuals(if self.darkmode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        // Show the top panel
        if self.ui_state.ui_mode == UiMode::Main {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                let response = TopPanel::show(
                    ui,
                    &mut self.ui_state.date_filter,
                    &mut self.ui_state.show_content,
                    &mut self.darkmode,
                    &mut self.ui_state.search_query,
                    &mut self.ui_state.clip_limit
                );

                // Handle clip loading changes (combines search_query_changed and clip_limit_changed)
                if response.clip_limit_changed || response.search_query_changed {
                    self.load_clips_based_on_state();
                    self.ui_state.ui_mode = UiMode::Main;
                }

                if response.show_tags {
                    self.ui_state.ui_mode = UiMode::TagFilter;
                }

                if response.date_changed {
                    self.ui_state.ui_mode = UiMode::Main;
                    if let Ok(clips_for_day) = db::load_clips_for_date(&self.db, self.ui_state.date_filter) {
                        self.clips = clips_for_day.into_iter().map(Clip::from_tuple).collect();
                    }
                }

                if response.refresh_requested {
                    self.reset_to_main_state();
                }

                if response.settings {
                    self.ui_state.ui_mode = UiMode::Settings;
                }

                if response.add_clip {
                    self.ui_state.show_create_clip_popup = true;
                }
            });
        }

        // Show the main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ui_state.ui_mode {
                UiMode::Main => {
                    let tag_data = self.get_tag_data();
                    MainView::show(
                        ui,
                        ctx,
                        &mut self.clips,
                        &self.db,
                        &mut self.ui_state,
                        self.darkmode,
                        &mut self.clip_tags,
                        &tag_data
                    );
                }
                UiMode::TagFilter => {
                    TagFilterView::show(
                        ui,
                        ctx,
                        &mut self.clips,
                        &self.db,
                        &mut self.ui_state,
                        &mut self.tags,
                        &mut self.clip_tags
                    );
                }
                UiMode::Settings => {
                    let response = SettingsView::show(ui, ctx, &mut self.ui_state);

                    if response.reset_settings {
                        self.darkmode = false;
                        self.ui_state.show_content = false;
                    }

                    if response.delete_db {
                        let _ = db::reset_db(&self.db);
                        self.reset_to_main_state();
                        self.reload_tags();
                    }
                }
            }
        });
    }
}