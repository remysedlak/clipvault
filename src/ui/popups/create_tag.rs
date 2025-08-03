use crate::db;
use crate::models::{Tag, UiState};
use eframe::egui;
use rusqlite::Connection;

pub struct CreateTagPopup;

impl CreateTagPopup {
    pub fn show(ctx: &egui::Context, ui_state: &mut UiState, db: &Connection, tags: &mut Vec<Tag>) {
        egui::Window::new("Create Tag")
            .collapsible(false)
            .resizable(false)
            .min_width(300.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.label("Type your new tag, then click submit.");
                ui.add_space(16.0);
                ui.text_edit_singleline(&mut ui_state.user_input);
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    // Use a layout that fills the space
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if ui.button("Submit").clicked() {
                            if !ui_state.user_input.trim().is_empty() {
                                if let Ok(_) = db::create_tag(db, &ui_state.user_input) {
                                    *tags = db::load_tags(db)
                                        .unwrap_or_default()
                                        .into_iter()
                                        .map(Tag::from_tuple)
                                        .collect();
                                    ui_state.user_input.clear();
                                }
                            }
                            ui_state.show_create_popup = false;
                        }

                        if ui.button("Cancel").clicked() {
                            ui_state.show_create_popup = false;
                            ui_state.user_input.clear();
                        }
                    });
                    ui.add_space(8.0);
                });
            });
    }
}
