use crate::db;
use crate::models::{Clip, UiState};
use eframe::egui;
use rusqlite::Connection;

pub struct CreateClip;

impl CreateClip {
    pub fn show(
        ctx: &egui::Context,
        ui_state: &mut UiState,
        db: &Connection,
        clips: &mut Vec<Clip>,
    ) {
        egui::Window::new("Create Clip")
            .collapsible(false)
            .resizable(false)
            .min_width(300.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.vertical(|ui| {

                    let id = ui.make_persistent_id("create_tag_text_input");
                    let has_focus_before = ui.memory(|mem| mem.has_focus(id));
                    let response = ui.add(egui::TextEdit::multiline(&mut ui_state.user_input).id(id));
                    // Auto-focus first frame
                    if !has_focus_before {
                        response.request_focus();
                    }

                    // Pressing Enter behaves like clicking Submit
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        submit_clip(&db, ui_state, clips);
                    }

                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        if ui.button("Submit").clicked() {
                            submit_clip(&db, ui_state, clips);

                        }
                        if ui.button("Cancel").clicked() {
                            ui_state.show_create_clip = false;
                        }
                    });
                })
            });
    }
}
fn submit_clip(db: &Connection, ui_state: &mut UiState, clips: &mut Vec<Clip>) {
    let timestamp = chrono::Utc::now().timestamp();
    if let Err(e) = db::save_clip(&db, &mut ui_state.user_input, timestamp) {
        eprintln!("Failed to save clip: {}", e);
    } else {
        println!("Saved clip: {}, {}", ui_state.user_input, timestamp);
    }
    ui_state.show_create_clip = false;
    ui_state.user_input.clear();
    *clips = db::load_recent_clips(&db, 20)
        .unwrap_or_default()
        .into_iter()
        .map(Clip::from_tuple)
        .collect();
}
