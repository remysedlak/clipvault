use crate::db::{self, load_tags};
use crate::models::{Clip, Tag, UiMode, UiState};
use crate::settings::{Settings, Theme};
use crate::ui::components::top_panel::{TopPanel, TopPanelResponse};
use crate::ui::views::main_view::MainView;
use crate::ui::views::tag_filter_view::TagFilterView;
use eframe::egui;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct ClipVaultApp {
    pub settings: Settings,
    pub settings_path: std::path::PathBuf,
    clips: Vec<Clip>,
    db: Connection,
    darkmode: bool,
    ui_state: UiState,
    tags: Vec<Tag>,
    clip_tags: HashMap<i64, Vec<String>>,
}

impl ClipVaultApp {
    pub fn new(db: Connection) -> Self {
        let clips = db::load_recent_clips(&db, 20)
            .unwrap_or_default()
            .into_iter()
            .map(Clip::from_tuple)
            .collect();

        let (settings, settings_path) = Settings::load();
        let darkmode = settings.theme == Theme::Dark;

        let tags = db::load_tags(&db)
            .unwrap_or_default()
            .into_iter()
            .map(Tag::from_tuple)
            .collect();

        let clip_tags = db::load_clip_tags(&db).unwrap_or_default();

        Self {
            clips,
            db,
            darkmode,
            settings,
            settings_path,
            tags,
            clip_tags,
            ui_state: UiState::default(),
        }
    }
}

impl eframe::App for ClipVaultApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.theme = if self.darkmode {
            Theme::Dark
        } else {
            Theme::Light
        };
        let _ = self.settings.save(&self.settings_path);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.darkmode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let response = TopPanel::show(
                ui,
                &mut self.ui_state.date,
                &mut self.ui_state.show_content,
                &mut self.darkmode,
            );

            if response.show_tags {
                self.ui_state.ui_mode = UiMode::TagFilter;
            }
            if response.delete_db {
                let _ = db::reset_db(&self.db);
                self.clips = db::load_recent_clips(&self.db, 20)
                    .unwrap_or_default()
                    .into_iter()
                    .map(Clip::from_tuple)
                    .collect();
                self.tags = db::load_tags(&self.db)
                    .unwrap_or_default()
                    .into_iter()
                    .map(Tag::from_tuple)
                    .collect();
                self.ui_state.ui_mode = UiMode::Main;
            }

            if response.date_changed {
                if let Ok(clips_for_day) = db::load_clips_for_date(&self.db, self.ui_state.date) {
                    self.clips = clips_for_day.into_iter().map(Clip::from_tuple).collect();
                }
            }
            if response.refresh_requested {
                self.clips = db::load_recent_clips(&self.db, 20)
                    .unwrap_or_default()
                    .into_iter()
                    .map(Clip::from_tuple)
                    .collect();
                self.ui_state.ui_mode = UiMode::Main;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state.ui_mode {
            UiMode::Main => {
                MainView::show(
                    ui,
                    ctx,
                    &mut self.clips,
                    &self.db,
                    &mut self.ui_state,
                    self.darkmode,
                    &mut self.clip_tags,
                    &self
                        .tags
                        .iter()
                        .map(|t| (t.id, t.name.clone()))
                        .collect::<Vec<_>>(),
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
                );
            }
        });
    }
}
