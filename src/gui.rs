use crate::db;
use eframe::egui::{self, Color32, Frame as EguiFrame, Label, Layout, RichText, Stroke, TextStyle};
use rusqlite::Connection;
use chrono::{DateTime, Local, Datelike};

pub struct ClipVaultApp {
    // Content, Timestamp
    clips: Vec<(i64, String, i64, bool)>,
    db: Connection,
}

impl ClipVaultApp {
    pub fn new(db: Connection) -> Self {
        let clips = db::load_recent_clips(&db, 20).unwrap_or_default();
        Self { clips, db }
    }
}

impl eframe::App for ClipVaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Use a light theme for readability
        ctx.set_visuals(egui::Visuals::light());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("📋 ClipVault");
                ui.separator();
                ui.label("Recent clipboard history");

                // Optional: add dark/light toggle or refresh
                if ui.button("🔄 Refresh").clicked() {
                    self.clips = db::load_recent_clips(&self.db, 20).unwrap_or_default();
                }
                ui.button("📅")
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(245, 245, 245)))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        // Track if a clip was deleted this frame
                        let mut deleted_id: Option<i64> = None;
                        let mut pinned_id: Option<i64> = None;
                        if self.clips.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label(
                                    RichText::new("No clips found.")
                                        .color(Color32::DARK_GRAY)
                                        .italics()
                                        .text_style(TextStyle::Heading),           
                                );
                                
                            });
                        }

                        for (id, content, timestamp, bool) in &self.clips {
                            // Skip empty content clips to avoid layout issues
                            if content.trim().is_empty() {
                                continue;
                            }

                            // State variable to show copy confirmation
                            let mut just_copied = false;

                            // Outer card frame
                            EguiFrame::none()
                                .fill(Color32::from_rgb(255, 255, 255))
                                .stroke(Stroke::new(1.0, Color32::LIGHT_GRAY))
                                .rounding(8.0)
                                .inner_margin(crate::gui::egui::Margin::symmetric(10.0, 10.0))
                                .outer_margin(crate::gui::egui::Margin::symmetric(20.0, 0.0))
                                .show(ui, |ui| {
                                    // Content row with black border, wrapping text, and copy button
                                    EguiFrame::none()
                                        .fill(Color32::from_rgb(250, 250, 250))
                                        .stroke(Stroke::new(1.0, Color32::BLACK))
                                        .rounding(6.0)
                                        .inner_margin(crate::gui::egui::Margin::symmetric(6.0, 6.0))
                                        .show(ui, |ui| {
                                            ui.vertical(|ui| {
                                                let max_width = ui.available_width();
                                                ui.set_max_width(max_width);

                                                ui.label("📝");

                                                ui.add(
                                                    Label::new(
                                                        RichText::new(content)
                                                            .monospace()
                                                            .text_style(TextStyle::Body)
                                                            .color(Color32::BLACK),
                                                    )
                                                    .wrap(),
                                                );

                                                ui.with_layout(
                                                    Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        // Copy button
                                                        if ui
                                                            .add(
                                                                egui::Button::new("📋 Copy")
                                                                    .small(),
                                                            )
                                                            .on_hover_text(
                                                                "Copy this text to clipboard",
                                                            )
                                                            .clicked()
                                                        {
                                                            ctx.output_mut(|o| {
                                                                o.copied_text = content.clone();
                                                            });
                                                            just_copied = true;
                                                        }

                                                        // Delete button
                                                        if ui
                                                            .add(
                                                                egui::Button::new("🗑 Delete")
                                                                    .small()
                                                                    .fill(Color32::from_rgb(255, 230, 230)),
                                                            )
                                                            .on_hover_text("Delete this entry")
                                                            .clicked()
                                                        {
                                                            deleted_id = Some(*id);
                                                        }

                                                        // Pin/Unpin button
                                                        let is_pinned = *bool;
                                                        let pin_label = if is_pinned { "📌 Unpin" } else { "📌 Pin" };
                                                        let pin_color = if is_pinned {
                                                            Color32::from_rgb(255, 255, 120)
                                                        } else {
                                                            Color32::from_rgb(255, 255, 197)
                                                        };
                                                        if ui
                                                            .add(
                                                                egui::Button::new(pin_label)
                                                                    .small()
                                                                    .fill(pin_color),
                                                            )
                                                            .on_hover_text(if is_pinned { "Unpin this entry" } else { "Pin this entry" })
                                                            .clicked()
                                                        {
                                                            let _ = db::toggle_pin_clip(&self.db, *id);
                                                            pinned_id = Some(*id);
                                                        }
                                                    },
                                                );
                                            });
                                        });

                                    if just_copied {
                                        ui.label(
                                            RichText::new("✔ Copied!")
                                                .color(Color32::DARK_GREEN)
                                                .small(),
                                        );
                                    }

                                    ui.add_space(6.0);

                                    // Timestamp row
                                    ui.horizontal(|ui| {
                                        ui.label("🕒");
                                        
                                        ui.monospace(format_timestamp(*timestamp));
                                    });
                                });

                            ui.add_space(12.0); // spacing between clips

                            // If deleted, break to avoid double-borrow
                            if deleted_id.is_some() {
                                break;
                            }
                        }

                        // Actually delete and refresh after the loop
                        if let Some(id) = deleted_id {
                            let _ = db::delete_clip(&self.db, id);
                            self.clips = db::load_recent_clips(&self.db, 20).unwrap_or_default();
                        }
                    });
            });
    }
}

fn format_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| {
            let dt = dt.with_timezone(&Local);
            let day = dt.day();
            let suffix = match day {
                11 | 12 | 13 => "th",
                _ => match day % 10 {
                    1 => "st",
                    2 => "nd",
                    3 => "rd",
                    _ => "th",
                },
            };
            format!(
                "{} {}{}, {} {}:{} {}",
                dt.format("%B"),
                day,
                suffix,
                dt.format("%Y"),
                dt.format("%-I"),
                dt.format("%M"),
                dt.format("%p")
            )
        })
        .unwrap_or_else(|| "Invalid timestamp".to_string())
}