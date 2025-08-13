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

        // Load recent clips
        let clips = db
            ::load_recent_clips(&db, 20)
            .unwrap_or_default()
            .into_iter()
            .map(Clip::from_tuple)
            .collect();

        // Load settings and determine dark mode
        let (settings, settings_path) = Settings::load();
        let darkmode = settings.theme == Theme::Dark;

        // Load tags and clip relationships
        let tags = db
            ::load_tags(&db)
            .unwrap_or_default()
            .into_iter()
            .map(Tag::from_tuple)
            .collect();

        let clip_tags = db::load_clip_tags(&db).unwrap_or_default();

        // Initialize the application state
        Self {
            clips,
            db,
            darkmode,
            settings,
            settings_path,
            tags,
            clip_tags,
            window_visible: true,
            ui_state: UiState::default(),
        }
    }
}

impl eframe::App for ClipVaultApp {

    /// save the settings on exit
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.theme = if self.darkmode { Theme::Dark } else { Theme::Light };
        self.settings.mode = self.ui_state.ui_mode;
        let _ = self.settings.save(&self.settings_path);
    }

    /// main update loop
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.window_visible {
            // Do nothing or optionally paint a tray icon or background tasks
            return;
        }
        if self.darkmode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        /// Show the top panel with date picker, content toggle, and settings
        if self.ui_state.ui_mode == UiMode::Main {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                let response = TopPanel::show(
                    ui,
                    &mut self.ui_state.date,
                    &mut self.ui_state.show_content,
                    &mut self.darkmode,
                    &mut self.ui_state.search_query
                );

                if response.search_query_changed {
                    self.clips = db
                        ::search_clips(&self.db, &self.ui_state.search_query)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                    self.ui_state.ui_mode = UiMode::Main;
                }

                if response.show_tags {
                    self.ui_state.ui_mode = UiMode::TagFilter;
                }

                if response.date_changed {
                    self.ui_state.ui_mode = UiMode::Main;
                    if
                        let Ok(clips_for_day) = db::load_clips_for_date(
                            &self.db,
                            self.ui_state.date
                        )
                    {
                        self.clips = clips_for_day.into_iter().map(Clip::from_tuple).collect();
                    }
                }
                if response.refresh_requested {
                    self.clips = db
                        ::load_recent_clips(&self.db, 20)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                    self.ui_state.ui_mode = UiMode::Main;
                }

                if response.settings {
                    self.ui_state.ui_mode = UiMode::Settings;
                }
            });
        }

        // Show the main content area under the top panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ui_state.ui_mode {
                UiMode::Main => {
                    MainView::show(
                        ui,
                        ctx,
                        &mut self.clips,
                        &self.db,
                        &mut self.ui_state,
                        self.darkmode,
                        &mut self.clip_tags,
                        &self.tags
                            .iter()
                            .map(|t| (t.id, t.name.clone(), t.color.clone()))
                            .collect::<Vec<_>>()
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
                        self.clips = db
                            ::load_recent_clips(&self.db, 20)
                            .unwrap_or_default()
                            .into_iter()
                            .map(Clip::from_tuple)
                            .collect();
                        self.tags = db
                            ::load_tags(&self.db)
                            .unwrap_or_default()
                            .into_iter()
                            .map(Tag::from_tuple)
                            .collect();
                        self.ui_state.ui_mode = UiMode::Main;
                    }
                }
            }
        });
    }
}
