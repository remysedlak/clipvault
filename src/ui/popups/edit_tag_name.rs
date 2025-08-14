use crate::db;
use crate::models::{Tag, UiState};
use eframe::egui::{self, Color32, RichText, TextEdit, TextStyle};
use rusqlite::Connection;

pub struct EditTagPopup;

impl EditTagPopup {
    pub fn show(ctx: &egui::Context, ui_state: &mut UiState, db: &Connection, tags: &mut Vec<Tag>) {
        egui::Window::new("Edit Tag")
            .collapsible(false)
            .resizable(false)
            .min_width(320.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name:").text_style(TextStyle::Body).strong());

                    let id = ui.make_persistent_id("edit_tag_text_input");
                    let has_focus_before = ui.memory(|mem| mem.has_focus(id));

                    let response = ui.add(
                        TextEdit::singleline(ui_state.edit_tag_name.get_or_insert_with(String::new))
                            .id(id)
                            .desired_width(200.0)
                    );

                    if !has_focus_before {
                        response.request_focus();
                    }

                    // Pressing Enter triggers save
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        Self::submit_edit(ui_state, db, tags);
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Color:").text_style(TextStyle::Body).strong());

                    let rgb = ui_state
                        .edit_tag_color
                        .unwrap_or(Color32::from_rgb(200, 200, 200))
                        .to_array();
                    let mut rgb3 = [rgb[0], rgb[1], rgb[2]];

                    if ui.color_edit_button_srgb(&mut rgb3).changed() {
                        ui_state.edit_tag_color = Some(Color32::from_rgb(rgb3[0], rgb3[1], rgb3[2]));
                    }
                });

                ui.add_space(16.0);
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        ui_state.show_tag_popup_for = None;
                        ui_state.edit_tag_name = None;
                        ui_state.edit_tag_color = None;
                    }
                    if ui.button("Save").clicked() {
                        Self::submit_edit(ui_state, db, tags);
                    }
                });
            });
    }

    fn submit_edit(ui_state: &mut UiState, db: &Connection, tags: &mut Vec<Tag>) {
        if let (Some(tag_id), Some(name)) = (ui_state.show_tag_popup_for, &ui_state.edit_tag_name) {
            if !name.trim().is_empty() {
                let color_hex = ui_state
                    .edit_tag_color
                    .map(|c| format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b()))
                    .unwrap_or_else(|| "#CCCCCC".to_string());

                if let Ok(_) = db::update_tag(db, tag_id, name, &color_hex) {
                    *tags = db::load_tags(db)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Tag::from_tuple)
                        .collect();
                }
            }
        }
        ui_state.show_tag_popup_for = None;
        ui_state.edit_tag_name = None;
        ui_state.edit_tag_color = None;
    }
}
