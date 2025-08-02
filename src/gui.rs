use crate::db;
use crate::settings::{Settings, Theme};
use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
use eframe::egui::{self, Color32, Frame as EguiFrame, Label, Layout, RichText, Stroke, TextStyle};
use egui_extras::DatePickerButton;
use rusqlite::Connection;
use std::collections::HashMap;

#[derive(PartialEq)]
enum UiMode {
    Main,
    TagFilter,
}

pub struct ClipVaultApp {
    pub settings: Settings,
    pub settings_path: std::path::PathBuf,
    // id, content, timestamp, pinned
    date: NaiveDate,
    clips: Vec<(i64, String, i64, bool)>,
    db: Connection,
    darkmode: bool,
    show_content: bool,
    ui_mode: UiMode,
    tags: Vec<(i64, String)>,
    user_input: String,
    show_create_popup: bool,
    clip_tags: HashMap<i64, Vec<String>>,
    show_tag_popup_for: Option<i64>,
    selected_tag_id: Option<i64>,
}

impl ClipVaultApp {
    pub fn new(db: Connection) -> Self {
        let clips = db::load_recent_clips(&db, 20).unwrap_or_default();
        let date = Utc::now().date_naive();

        let (settings, settings_path) = Settings::load();
        let darkmode = settings.theme == Theme::Dark;
        let tags = db::load_tags(&db).unwrap_or_default();

        let clip_tags = db::load_clip_tags(&db).unwrap_or_default();

        Self {
            show_tag_popup_for: None,
            selected_tag_id: None,
            date,
            user_input: String::new(),
            clips,
            db,
            darkmode,
            show_content: false,
            settings,
            settings_path,
            tags,
            clip_tags,
            ui_mode: UiMode::Main,
            show_create_popup: false,
        }
    }
}

impl eframe::App for ClipVaultApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Example: update the theme setting before saving
        self.settings.theme = if self.darkmode {
            Theme::Dark
        } else {
            Theme::Light
        };

        let _ = self.settings.save(&self.settings_path);
    }
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
                    let show_content_label = if self.show_content { "🫣" } else { "👁" };
                    if ui
                        .button(show_content_label)
                        .on_hover_text("Show or hide the content of all clips")
                        .clicked()
                    {
                        self.show_content = !self.show_content;
                    }
                    if ui.button("🏷").on_hover_text("View tags").clicked() {
                        self.ui_mode = UiMode::TagFilter;
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
                    let mode_label = if self.darkmode { "🌙" } else { "🔆" };
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
                        self.ui_mode = UiMode::Main;
                    }
                });
            });
        });

        egui::CentralPanel::default()
            // .frame(egui::Frame::none().fill(Color32::from_rgb(245, 245, 245)))
            .show(ctx, |ui| {
                match self.ui_mode {
                    UiMode::Main => {
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
                                                            Label::new(if self.show_content {
                                                                RichText::new(content)
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
                                                                [35.0, 20.0],
                                                                egui::Button::new("📋"),
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

                                                        if ui.button("+").clicked() {
                                                            self.show_tag_popup_for = Some(*id);
                                                            self.selected_tag_id = None;
                                                        }

                                                        // Delete button with fixed size
                                                        if ui
                                                            .add_sized(
                                                                [35.0, 20.0],
                                                                egui::Button::new("🗑"), // .fill(Color32::from_rgb(255, 230, 230)),
                                                            )
                                                            .on_hover_text("Delete this entry")
                                                            .clicked()
                                                        {
                                                            deleted_id = Some(*id);
                                                        }

                                                        // Pin/Unpin button with fixed size
                                                        let is_pinned = *bool;
                                                        let pin_label = if is_pinned {
                                                            "📌 Unpin"
                                                        } else {
                                                            "📌"
                                                        };
                                                        if ui
                                                            .add_sized(
                                                                [35.0, 20.0],
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
                                            ui.horizontal(|ui| {
                                                ui.label("tags: ");

                                                ui.with_layout(
                                                    Layout::left_to_right(egui::Align::Center),
                                                    |ui| {
                                                        // show all existing tags:
                                                        if let Some(tags) = self.clip_tags.get(id) {
                                                            for tag_name in tags {
                                                                if tag_name == "emotion" {
                                                                    continue;
                                                                }

                                                                let visuals = ui.visuals();
                                                                let bg_color = visuals
                                                                    .widgets
                                                                    .inactive
                                                                    .bg_fill;
                                                                let text_color =
                                                                    visuals.text_color();
                                                                let stroke = visuals
                                                                    .widgets
                                                                    .inactive
                                                                    .bg_stroke;

                                                                egui::Frame::none()
                                                                    .fill(bg_color)
                                                                    .stroke(stroke)
                                                                    .rounding(egui::Rounding::same(
                                                                        6.0,
                                                                    ))
                                                                    .inner_margin(
                                                                        egui::Margin::symmetric(
                                                                            6.0, 4.0,
                                                                        ),
                                                                    )
                                                                    .show(ui, |ui| {
                                                                        ui.label(
                                                                            egui::RichText::new(
                                                                                format!(
                                                                                    "{}",
                                                                                    tag_name
                                                                                ),
                                                                            )
                                                                            .color(text_color)
                                                                            .strong(),
                                                                        );
                                                                    });
                                                            }
                                                        }

                                                        // Set a fixed width for the button area
                                                        ui.set_max_width(200.0); // Adjust as needed
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

                                if let Some(clip_id) = self.show_tag_popup_for {
                                    egui::Window::new("Assign Tag")
                                        .collapsible(false)
                                        .resizable(false)
                                        .show(ctx, |ui| {
                                            ui.label("Select a tag to assign:");

                                            // Show all tags that aren't already on the clip
                                            let assigned = self
                                                .clip_tags
                                                .get(&clip_id)
                                                .cloned()
                                                .unwrap_or_default();

                                            for (tag_id, tag_name) in &self.tags {
                                                if !assigned.contains(tag_name) {
                                                    let is_selected =
                                                        self.selected_tag_id == Some(*tag_id);
                                                    if ui
                                                        .selectable_label(
                                                            is_selected,
                                                            format!("{}", tag_name),
                                                        )
                                                        .clicked()
                                                    {
                                                        self.selected_tag_id = Some(*tag_id);
                                                    }
                                                }
                                            }

                                            ui.horizontal(|ui| {
                                                if ui.button("Assign").clicked() {
                                                    if let Some(tag_id) = self.selected_tag_id {
                                                        if db::assign_tag_to_clip(
                                                            &self.db, clip_id, tag_id,
                                                        )
                                                        .is_ok()
                                                        {
                                                            // Reload mapping
                                                            self.clip_tags =
                                                                db::load_clip_tags(&self.db)
                                                                    .unwrap_or_default();
                                                        }
                                                    }
                                                    self.show_tag_popup_for = None;
                                                }

                                                if ui.button("Cancel").clicked() {
                                                    self.show_tag_popup_for = None;
                                                }
                                            });
                                        });
                                }

                                // Actually delete and refresh after the loop
                                if let Some(id) = deleted_id {
                                    let _ = db::delete_clip(&self.db, id);
                                    self.clips =
                                        db::load_recent_clips(&self.db, 20).unwrap_or_default();
                                }

                                // Actually pin/unpin and refresh after the loop
                                if let Some(id) = pinned_id {
                                    let _ = db::toggle_pin_clip(&self.db, id);
                                    self.clips =
                                        db::load_recent_clips(&self.db, 20).unwrap_or_default();
                                }
                            });
                    }
                    UiMode::TagFilter => {
                        let button_width = 150.0;
                        ui.horizontal(|ui| {
                            ui.heading("Recorded Tags");

                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add_sized([60.0, 30.0], egui::Button::new("🔙 Back"))
                                    .clicked()
                                {
                                    self.ui_mode = UiMode::Main;
                                }
                            });
                        });

                        ui.vertical_centered(|ui|  {
                            for (tag_id, tag_name) in &self.tags {
                                if ui
                                    .add_sized(
                                        [button_width, 30.0],
                                        egui::Button::new(format!("🏷 {}", tag_name)),
                                    )
                                    .clicked()
                                {
                                    self.clips =
                                        db::load_clips_for_tag(&self.db, tag_id).unwrap_or_default();
                                    self.ui_mode = UiMode::Main;
                                }
                            }
                            if ui
                                .add_sized([button_width, 30.0], egui::Button::new("Create new tag"))
                                .clicked()
                            {
                                self.show_create_popup = true;
                            }
                        });
                        

                        if self.show_create_popup {
                            egui::Window::new("Create Tag")
                                .collapsible(false)
                                .resizable(false)
                                .min_width(300.0) // 👈 Wider popup
                                .min_height(200.0) // 👈 Taller popup
                                .show(ctx, |ui| {
                                    ui.label("Type your new tag, then click submit.");
                                    ui.text_edit_singleline(&mut self.user_input);
                                    if ui.button("Close").clicked() {
                                        self.show_create_popup = false; // close the popup
                                    }
                                    // Submit button
                                    if ui.button("Submit").clicked() {
                                        // Save the input somewhere else, e.g., input_result
                                        self.user_input = self.user_input.clone();
                                        self.show_create_popup = false; // close popup
                                        let _ = db::create_tag(&self.db, &self.user_input);
                                        self.tags = db::load_tags(&self.db).unwrap_or_default();
                                    }
                                });
                        }
                    }
                }
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
