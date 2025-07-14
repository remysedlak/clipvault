use crate::db;
use chrono::{DateTime, Datelike, Local, Utc, NaiveDate};
use eframe::egui::{self, Color32, Frame as EguiFrame, Label, Layout, RichText, Stroke, TextStyle};
use egui_extras::DatePickerButton;
use rusqlite::Connection;

pub struct ClipVaultApp {
    // id, content, timestamp, pinned
    date: NaiveDate,
    clips: Vec<(i64, String, i64, bool)>,
    db: Connection,
    darkmode: bool,
    show_content: bool,
}

impl ClipVaultApp {
    pub fn new(db: Connection) -> Self {
        let clips = db::load_recent_clips(&db, 20).unwrap_or_default();
        let date = Utc::now().date_naive();

        Self {
            date,
            clips: clips.clone(),
            db,
            darkmode: true,
            show_content: false,
        }
    }
}

impl eframe::App for ClipVaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set visuals based on darkmode
        if self.darkmode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📋 ClipVault");
                ui.separator();
                ui.label("Recent clipboard history");

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    // Show/Hide content toggle button
                    let show_content_label = if self.show_content { "🙈 Hide" } else { "👁 Show" };
                    if ui
                        .button(show_content_label)
                        .on_hover_text("Show or hide the content of all clips")
                        .clicked()
                    {
                        self.show_content = !self.show_content;
                    }
                    if ui
                        .add(
                            DatePickerButton::new(&mut self.date)
                                .show_icon(true)
                                .highlight_weekends(false)
                                .format(""),
                        )
                        .on_hover_text("Select a date to filter clips")
                        .changed()
                    {

                        // Load only the clips for the selected date
                        if let Ok(clips_for_day) = db::load_clips_for_date(&self.db, self.date) {
                            self.clips = clips_for_day;
                        }
                    }
                    // Dark/Light mode toggle button
                    let mode_label = if self.darkmode {
                        "🌙 Dark"
                    } else {
                        "🔆 Light"
                    };
                    if ui
                        .button(mode_label)
                        .on_hover_text("Toggle dark/light mode")
                        .clicked()
                    {
                        self.darkmode = !self.darkmode;
                    }

                    // Refresh button
                    if ui
                        .button("🔄 Refresh")
                        .on_hover_text("Refresh clipboard entries.")
                        .clicked()
                    {
                        self.clips = db::load_recent_clips(&self.db, 20).unwrap_or_default();
                    }
                });
            });
        });

        egui::CentralPanel::default()
            // .frame(egui::Frame::none().fill(Color32::from_rgb(245, 245, 245)))
            .show(ctx, |ui| {
                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        // Track if a clip was deleted this frame
                        let mut deleted_id: Option<i64> = None;
                        // Track if a clip was pinned/unpinned this frame
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
                                // .fill(Color32::from_rgb(255, 255, 255))
                                // .stroke(Stroke::new(1.0, Color32::LIGHT_GRAY))
                                .rounding(8.0)
                                .inner_margin(egui::Margin::symmetric(10.0, 10.0))
                                .outer_margin(egui::Margin::symmetric(20.0, 0.0))
                                .fill(if self.darkmode {
                                    Color32::from_rgb(40, 40, 40)
                                } else {
                                    Color32::from_rgb(240, 240, 240)
                                })
                                .stroke(Stroke::new(1.0, Color32::BLACK))
                                .show(ui, |ui| {
                                    // Content row with black border, wrapping text, and copy button
                                    EguiFrame::none()
                                        // .fill(Color32::from_rgb(250, 250, 250))
                                        .show(ui, |ui| {
                                            ui.vertical(|ui| {
                                                // let max_width = ui.available_width();
                                                // ui.set_max_width(max_width);

                                                ui.label("📝");
                                                    
                                                ui.add(
                                                    Label::new(
                                                        if self.show_content {
                                                            RichText::new(content)
                                                                .monospace()
                                                                .text_style(TextStyle::Body)
                                                        }
                                                        else {
                                                            RichText::new("Content hidden")
                                                                .monospace()
                                                                .text_style(TextStyle::Body)
                                                        }
                                                    )
                                                    .wrap(),
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
                                    ui.separator();
                                    // Timestamp row
                                    ui.horizontal(|ui| {
                                        ui.label("🕒");

                                        ui.monospace(format_timestamp(*timestamp));
                                        ui.with_layout(
                                            Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                // Set a fixed width for the button area
                                                ui.set_max_width(200.0); // Adjust as needed

                                                // Copy button with fixed size
                                                if ui
                                                    .add_sized(
                                                        [50.0, 20.0],
                                                        egui::Button::new("📋 Copy"),
                                                    )
                                                    .on_hover_text("Copy this text to clipboard")
                                                    .clicked()
                                                {
                                                    ctx.output_mut(|o| {
                                                        o.copied_text = content.clone();
                                                    });
                                                    just_copied = true;
                                                }

                                                // Delete button with fixed size
                                                if ui
                                                    .add_sized(
                                                        [50.0, 20.0],
                                                        egui::Button::new("🗑 Delete"), // .fill(Color32::from_rgb(255, 230, 230)),
                                                    )
                                                    .on_hover_text("Delete this entry")
                                                    .clicked()
                                                {
                                                    deleted_id = Some(*id);
                                                }

                                                // Pin/Unpin button with fixed size
                                                let is_pinned = *bool;
                                                let pin_label =
                                                    if is_pinned { "📌 Unpin" } else { "📌" };
                                                if ui
                                                    .add_sized(
                                                        [50.0, 20.0],
                                                        egui::Button::new(pin_label),
                                                    )
                                                    .on_hover_text(if is_pinned {
                                                        "Unpin this entry"
                                                    } else {
                                                        "Pin this entry"
                                                    })
                                                    .clicked()
                                                {
                                                    pinned_id = Some(*id);
                                                }
                                            },
                                        );
                                    });
                                });

                            ui.add_space(12.0); // spacing between clips

                            // If deleted or pinned, break to avoid double-borrow
                            if deleted_id.is_some() || pinned_id.is_some() {
                                break;
                            }
                        }

                        // Actually delete and refresh after the loop
                        if let Some(id) = deleted_id {
                            let _ = db::delete_clip(&self.db, id);
                            self.clips = db::load_recent_clips(&self.db, 20).unwrap_or_default();
                        }

                        // Actually pin/unpin and refresh after the loop
                        if let Some(id) = pinned_id {
                            let _ = db::toggle_pin_clip(&self.db, id);
                            self.clips = db::load_recent_clips(&self.db, 20).unwrap_or_default();
                        }
                    });
            });
    }
}

//  turns UTC integer into a human-readable format
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
