use crate::db;
use crate::models::{Clip, Tag, UiState, UiMode};
use crate::ui::popups::create_tag::CreateTagPopup;
use eframe::egui::{
    self, Layout, TopBottomPanel, CentralPanel, ScrollArea, Color32, RichText, TextStyle, Frame, Stroke,
    Margin,
};
use rusqlite::Connection;
use crate::utils::formatting::hex_to_color32;

pub struct TagFilterView;

impl TagFilterView {
    pub fn show(
        _ui: &mut egui::Ui, // no longer needed since we use panels
        ctx: &egui::Context,
        clips: &mut Vec<Clip>,
        db: &Connection,
        ui_state: &mut UiState,
        tags: &mut Vec<Tag>,
    ) {
        TopBottomPanel::top("tag_filter_top_panel")
            .min_height(25.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.heading("Tag Manager");

                    ui.label(
                        RichText::new(format!("({} tags)", tags.len()))
                            .color(Color32::GRAY)
                            .text_style(TextStyle::Body),
                    );

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add_sized([60.0, 20.0], egui::Button::new("Back to clips..."))
                            .on_hover_text("Return to main view")
                            .clicked()
                        {
                            ui_state.ui_mode = UiMode::Main;
                        }
                    });
                });
                ui.add_space(8.0);
            });

        TopBottomPanel::bottom("tag_filter_bottom_panel")
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if tags.is_empty() {
                        ui.label(
                            RichText::new("No tags yet. Create your first tag!")
                                .color(Color32::GRAY)
                                .italics(),
                        );
                    } else {
                        ui.label(
                            RichText::new("Click a tag to filter clips, or create a new one")
                                .color(Color32::GRAY)
                                .text_style(TextStyle::Small),
                        );
                    }

                    ui.add_space(8.0);

                    if ui
                        .add_sized([200.0, 40.0], egui::Button::new("➕ Create New Tag"))
                        .on_hover_text("Add a new tag to organize your clips")
                        .clicked()
                    {
                        ui_state.show_create_popup = true;
                    }
                });
                ui.add_space(10.0);
            });

        CentralPanel::default().show(ctx, |ui| {
            if tags.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("📋").size(48.0).color(Color32::GRAY));
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("No tags created yet")
                                .text_style(TextStyle::Heading)
                                .color(Color32::GRAY),
                        );
                        ui.label(
                            RichText::new("Tags help you organize and find your clipboard history")
                                .text_style(TextStyle::Body)
                                .color(Color32::DARK_GRAY),
                        );
                    });
                });
            } else {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(20.0);

                        let available_width = ui.available_width();
                        let button_width = 150.0;
                        let buttons_per_row =
                            ((available_width - 40.0) / (button_width + 10.0)).floor() as usize;
                        let buttons_per_row = buttons_per_row.max(1);

                        for chunk in tags.chunks_mut(buttons_per_row) {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 10.0;

                                for tag in chunk {
                                    Frame::none()
                                        .rounding(8.0)
                                        .fill(
                                        if ui.visuals().dark_mode {
                                            Color32::from_rgb(40, 40, 40)
                                        } else {
                                            Color32::from_rgb(240, 240, 240)
                                        }
                                        )
                                        .stroke(Stroke::new(1.0, Color32::BLACK))
                                        .inner_margin(Margin::symmetric(15.0, 12.0))
                                        .show(ui, |ui| {
                                            ui.set_min_size([button_width, 50.0].into());

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

                                            if let Ok(count) = db::count_clips_for_tag(db, &tag.id) {
                                                ui.label(
                                                    RichText::new(format!("{} clips", count))
                                                        .text_style(TextStyle::Small)
                                                        .color(Color32::GRAY),
                                                );
                                            }

                                            // Pass `db` to allow DB updates inside the picker
                                            tag_color_picker(ui, tag, db);
                                        });
                                }
                            });
                            ui.add_space(15.0);
                        }

                        ui.add_space(20.0);
                    });
            }
        });

        if ui_state.show_create_popup {
            CreateTagPopup::show(ctx, ui_state, db, tags);
        }
    }
}

// Helper: convert Color32 to hex string like "#RRGGBB"
fn color32_to_hex(color: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

// UI for picking tag color with DB update
fn tag_color_picker(ui: &mut egui::Ui, tag: &mut Tag, db: &Connection) {
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
