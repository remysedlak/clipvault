use crate::db;
use crate::models::{Clip, Tag, UiMode, UiState};
use crate::utils::formatting::hex_to_color32;
use eframe::egui::{
    self, Color32, Frame, Margin, RichText, Stroke, TextStyle, Ui,
};
use rusqlite::Connection;

/// Renders a single tag card with button, clip count, and color picker
pub fn show(
    ui: &mut Ui,
    tag: &mut Tag,
    db: &Connection,
    clips: &mut Vec<Clip>,
    ui_state: &mut UiState,
    button_width: f32,
) {
    Frame::none()
        .rounding(8.0)
        .fill(if ui.visuals().dark_mode {
            Color32::from_rgb(40, 40, 40)
        } else {
            Color32::from_rgb(240, 240, 240)
        })
        .stroke(Stroke::new(1.0, Color32::BLACK))
        .inner_margin(Margin::symmetric(15.0, 12.0))
        .show(ui, |ui| {
            ui.set_min_size([button_width, 50.0].into());
            ui.vertical(|ui| {
                // Main tag button
                let button = ui.add_sized(
                    [button_width - 30.0, 35.0],
                    egui::Button::new(
                        RichText::new(&tag.name)
                            .text_style(TextStyle::Button)
                            .strong(),
                    ),
                );

                if button.clicked() {
                    *clips = db::load_clips_for_tag(db, &tag.id)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                    ui_state.ui_mode = UiMode::Main;
                }

                // Clip count display
                if let Ok(count) = db::count_clips_for_tag(db, &tag.id) {
                    ui.label(
                        RichText::new(format!("{} clips", count))
                            .text_style(TextStyle::Small)
                            .color(Color32::GRAY),
                    );
                }

                // Color picker
                color_picker(ui, tag, db);
            });
        });
}

/// Color picker component for the tag
fn color_picker(ui: &mut Ui, tag: &mut Tag, db: &Connection) {
    let mut color = tag
        .color
        .as_ref()
        .and_then(|hex| hex_to_color32(hex))
        .unwrap_or_else(|| {
            if ui.visuals().dark_mode {
                Color32::LIGHT_GRAY
            } else {
                Color32::DARK_GRAY
            }
        });

    ui.horizontal(|ui| {
        if ui.color_edit_button_srgba(&mut color).changed() {
            tag.color = Some(color32_to_hex(color));
            let color_ref = tag.color.as_deref();
            if let Err(e) = db::update_tag_color(db, tag.id, color_ref) {
                eprintln!("Failed to update tag color: {}", e);
            }
        }

        if ui.button("Reset Color").clicked() {
            tag.color = None;
            if let Err(e) = db::update_tag_color(db, tag.id, None) {
                eprintln!("Failed to reset tag color: {}", e);
            }
        }
    });
}

/// Helper: convert Color32 to hex string like "#RRGGBB"
fn color32_to_hex(color: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}