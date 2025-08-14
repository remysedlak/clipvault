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

                let id = ui.make_persistent_id("create_tag_text_input");
                let has_focus_before = ui.memory(|mem| mem.has_focus(id));

                let response = ui.add(
                    egui::TextEdit::singleline(&mut ui_state.user_input)
                        .id(id)
                );

                // Auto-focus first frame
                if !has_focus_before {
                    response.request_focus();
                }

                // Pressing Enter behaves like clicking Submit
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    Self::submit_tag(ui_state, db, tags);
                }

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if ui.button("Cancel").clicked() {
                            ui_state.show_create_popup = false;
                            ui_state.user_input.clear();
                        }
                        if ui.button("Save").clicked() {
                            Self::submit_tag(ui_state, db, tags);
                        }
                    });
                    ui.add_space(8.0);
                });
            });
    }

    fn submit_tag(ui_state: &mut UiState, db: &Connection, tags: &mut Vec<Tag>) {
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
}
