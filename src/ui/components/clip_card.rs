use crate::models::Clip;
use crate::utils::formatting::format_timestamp;
use eframe::egui::{self, Color32, Frame as EguiFrame, Label, Layout, RichText, Stroke, TextStyle};
use std::collections::HashMap;

pub struct ClipCard;

impl ClipCard {
    pub fn show(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        clip: &Clip,
        show_content: bool,
        darkmode: bool,
        clip_tags: &HashMap<i64, Vec<String>>,
    ) -> ClipCardResponse {
        let mut response = ClipCardResponse::default();
        
        if clip.is_empty() {
            return response;
        }

        // Outer card frame
        EguiFrame::none()
            .rounding(8.0)
            .inner_margin(egui::Margin::symmetric(10.0, 10.0))
            .outer_margin(egui::Margin::symmetric(20.0, 0.0))
            .fill(if darkmode {
                Color32::from_rgb(40, 40, 40)
            } else {
                Color32::from_rgb(240, 240, 240)
            })
            .stroke(Stroke::new(1.0, Color32::BLACK))
            .show(ui, |ui| {
                // Content section
                EguiFrame::none().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label("📝");
                        ui.add(
                            Label::new(if show_content {
                                RichText::new(&clip.content)
                                    .monospace()
                                    .text_style(TextStyle::Body)
                            } else {
                                RichText::new("Content hidden")
                                    .monospace()
                                    .text_style(TextStyle::Body)
                            })
                            .wrap(),
                        );
                    });
                });

                ui.add_space(6.0);
                ui.separator();
                
                // Timestamp and action buttons
                ui.horizontal(|ui| {
                    ui.label("🕒");
                    ui.monospace(format_timestamp(clip.timestamp));
                    
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.set_max_width(200.0);

                        // Copy button
                        if ui
                            .add_sized([35.0, 20.0], egui::Button::new("📋"))
                            .on_hover_text("Copy this text to clipboard")
                            .clicked()
                        {
                            ctx.output_mut(|o| {
                                o.copied_text = clip.content.clone();
                            });
                            response.copied = true;
                        }

                        // Delete button
                        if ui
                            .add_sized([35.0, 20.0], egui::Button::new("🗑"))
                            .on_hover_text("Delete this entry")
                            .clicked()
                        {
                            response.delete_requested = true;
                        }

                        // Pin/Unpin button
                        let pin_label = if clip.pinned { "📌 Unpin" } else { "📌" };
                        if ui
                            .add_sized([35.0, 20.0], egui::Button::new(pin_label))
                            .on_hover_text(if clip.pinned {
                                "Unpin this entry"
                            } else {
                                "Pin this entry"
                            })
                            .clicked()
                        {
                            response.pin_toggled = true;
                        }
                    });
                });

                // Tags section
                ui.horizontal(|ui| {
                    ui.label("tags: ");
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        // Show existing tags
                        if let Some(tags) = clip_tags.get(&clip.id) {
                            for tag_name in tags {
                                if tag_name == "emotion" {
                                    continue;
                                }
                                
                                let visuals = ui.visuals();
                                let bg_color = visuals.widgets.inactive.bg_fill;
                                let text_color = visuals.text_color();
                                let stroke = visuals.widgets.inactive.bg_stroke;

                                egui::Frame::none()
                                    .fill(bg_color)
                                    .stroke(stroke)
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                    .show(ui, |ui| {
                                        ui.label(
                                            egui::RichText::new(tag_name)
                                                .color(text_color)
                                                .strong(),
                                        );
                                    });
                            }
                        }
                        
                        if ui.button("+").on_hover_text("Add tags to clip.").clicked() {
                            response.add_tag_requested = true;
                        }
                        
                        ui.set_max_width(200.0);
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