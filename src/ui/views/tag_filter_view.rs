use crate::db;
use crate::models::{Clip, Tag, UiState, UiMode};
use crate::ui::popups::create_tag::CreateTagPopup;
use eframe::egui::{self, Layout};
use rusqlite::Connection;

pub struct TagFilterView;

impl TagFilterView {
    pub fn show(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        clips: &mut Vec<Clip>,
        db: &Connection,
        ui_state: &mut UiState,
        tags: &mut Vec<Tag>,
    ) {
        let button_width = 150.0;
        
        ui.horizontal(|ui| {
            ui.heading("Recorded Tags");

            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_sized([60.0, 30.0], egui::Button::new("🔙 Back"))
                    .clicked()
                {
                    ui_state.ui_mode = UiMode::Main;
                }
            });
        });

        ui.vertical_centered(|ui| {
            for tag in tags.iter() {
                if ui
                    .add_sized(
                        [button_width, 30.0],
                        egui::Button::new(format!("🏷 {}", tag.name)),
                    )
                    .clicked()
                {
                    *clips = db::load_clips_for_tag(db, &tag.id)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                    ui_state.ui_mode = UiMode::Main;
                }
            }
            
            if ui
                .add_sized([button_width, 30.0], egui::Button::new("Create new tag"))
                .clicked()
            {
                ui_state.show_create_popup = true;
            }
        });

        if ui_state.show_create_popup {
            CreateTagPopup::show(ctx, ui_state, db, tags);
        }
    }
}