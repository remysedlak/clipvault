use crate::db;
use crate::models::UiState;
use eframe::egui;
use rusqlite::Connection;
use std::collections::HashMap;

pub struct TagAssignmentPopup;

impl TagAssignmentPopup {
    pub fn show(
        ctx: &egui::Context,
        clip_id: i64,
        ui_state: &mut UiState,
        db: &Connection,
        clip_tags: &mut HashMap<i64, Vec<String>>,
        tags: &[(i64, String, Option<String>)], // Updated to include color
    ) {
        egui::Window::new("Assign Tag")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Select a tag to assign:");
                ui.separator();

                let assigned = clip_tags
                    .get(&clip_id)
                    .cloned()
                    .unwrap_or_default();
                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        ui.add_space(4.0);
                        for (tag_id, tag_name, _) in tags {
                            if !assigned.contains(tag_name) {
                                let is_selected = ui_state.selected_tag_id == Some(*tag_id);
                                if ui
                                    .selectable_label(is_selected, tag_name)

                                    .clicked()
                                {
                                    ui_state.selected_tag_id = Some(*tag_id);
                                }
                            }
                            
                        }
                        ui.add_space(4.0);
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Assign").clicked() {
                        if let Some(tag_id) = ui_state.selected_tag_id {
                            if db::assign_tag_to_clip(db, clip_id, tag_id).is_ok() {
                                *clip_tags = db::load_clip_tags(db).unwrap_or_default();
                            }
                        }
                        ui_state.show_tag_popup_for = None;
                    }

                    if ui.button("Cancel").clicked() {
                        ui_state.show_tag_popup_for = None;
                    }
                });
            });
    }
}