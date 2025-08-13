use crate::models::Clip;
use crate::utils::formatting::format_timestamp;
use eframe::egui::{ self, Color32, Frame as EguiFrame, Label, Layout, RichText, Stroke, TextStyle };
use std::{collections::HashMap};

pub struct ClipCard;

impl ClipCard {
    // Added `tag_colors` mapping tag name -> Color32 for showing colors
    pub fn show(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        clip: &Clip,
        show_content: bool,
        darkmode: bool,
        clip_tags: &HashMap<i64, Vec<String>>,
        tag_colors: &HashMap<String, Color32> // NEW param
    ) -> ClipCardResponse {
        let mut response = ClipCardResponse::default();

        if clip.is_empty() {
            return response;
        }

        const BUTTON_SIZE: [f32; 2] = [25.0, 20.0];

        // Outer card frame
        EguiFrame::none()
            .rounding(8.0)
            .inner_margin(egui::Margin::symmetric(10.0, 10.0))
            .outer_margin(egui::Margin::symmetric(20.0, 0.0))
            .fill(
                if darkmode {
                    Color32::from_rgb(40, 40, 40)
                } else {
                    Color32::from_rgb(240, 240, 240)
                }
            )
            .stroke(Stroke::new(1.0, Color32::BLACK))
            .show(ui, |ui| {
                // Content section
                EguiFrame::none().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.add(
                            Label::new(
                                if show_content {
                                    RichText::new(&clip.content)
                                        .monospace()
                                        .text_style(TextStyle::Body)
                                } else {
                                    RichText::new("Content hidden")
                                        .monospace()
                                        .text_style(TextStyle::Body)
                                }
                            ).wrap()
                        );
                    });
                });

                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);
                
                // Tags section
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        if let Some(tags) = clip_tags.get(&clip.id) {
                            for tag_name in tags {
                                // Lookup the color or fallback
                                let tag_color = tag_colors
                                    .get(tag_name)
                                    .copied()
                                    .unwrap_or_else(|| {
                                        if darkmode {
                                            Color32::LIGHT_GRAY
                                        } else {
                                            Color32::DARK_GRAY
                                        }
                                    });

                                egui::Frame
                                    ::none()
                                    .fill(tag_color)
                                    .stroke(Stroke::new(1.0, contrast_color(tag_color)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(4.0, 4.0))
                                    .show(ui, |ui| {
                                        ui.label(
                                            egui::RichText
                                                ::new(tag_name)
                                                .color(contrast_color(tag_color))
                                                .strong()
                                        );
                                    });
                            }
                        }

                        if ui.add_sized(BUTTON_SIZE, egui::Button::new("+")).on_hover_text("Add tags to clip.").clicked() {
                            response.add_tag_requested = true;
                        }

                        ui.set_max_width(200.0);
                    });
                });

                ui.add_space(6.0);

                // Timestamp and action buttons
                ui.horizontal(|ui| {
                    ui.label("🕒");
                    ui.monospace(format_timestamp(clip.timestamp));

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.set_max_width(200.0);
                        if
                            ui
                                .add_sized(BUTTON_SIZE, egui::Button::new("📋"))
                                .on_hover_text("Copy this text to clipboard")
                                .clicked()
                        {
                            ctx.output_mut(|o| {
                                o.copied_text = clip.content.clone();
                            });
                            response.copied = true;
                        }

                        if
                            ui
                                .add_sized(BUTTON_SIZE, egui::Button::new("🗑"))
                                .on_hover_text("Delete this entry")
                                .clicked()
                        {
                            response.delete_requested = true;
                        }

                        let pin_label = if clip.pinned { "📌 Unpin" } else { "📌" };
                        if
                            ui
                                .add_sized(BUTTON_SIZE, egui::Button::new(pin_label))
                                .on_hover_text(
                                    if clip.pinned {
                                        "Unpin this entry"
                                    } else {
                                        "Pin this entry"
                                    }
                                )
                                .clicked()
                        {
                            response.pin_toggled = true;
                        }
                    });
                });
            });

        response
    }
}

#[derive(Default)]
pub struct ClipCardResponse {
    pub copied: bool,
    pub delete_requested: bool,
    pub pin_toggled: bool,
    pub add_tag_requested: bool,
}

// Helper for contrasting text color on a colored background
fn contrast_color(bg: Color32) -> Color32 {
    let brightness = 0.299 * (bg.r() as f32) + 0.587 * (bg.g() as f32) + 0.114 * (bg.b() as f32);
    if brightness > 186.0 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}
