use crate::db;
use eframe::{
    egui::{self, Color32, Frame as EguiFrame, Stroke},
    App as _,
};
use rusqlite::Connection;

pub struct ClipVaultApp {
    // Content, Timestamp
    clips: Vec<(String, String)>,
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
        // Use a light theme for better readability
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(245, 245, 245)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("📋 ClipVault");
                    ui.label("Your recent clipboard history");
                });

                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (content, timestamp) in &self.clips {
                            // Outer card for each clip
                            EguiFrame::none()
                                .fill(Color32::from_rgb(255, 255, 255))
                                .stroke(Stroke::new(1.0, Color32::LIGHT_GRAY))
                                .rounding(8.0)
                                .inner_margin(crate::gui::egui::Margin::symmetric(10.0, 10.0))
                                .show(ui, |ui| {
                                    // 📝 Content Row with copy button and border
                                    EguiFrame::none()
                                        .fill(Color32::from_rgb(250, 250, 250))
                                        .stroke(Stroke::new(1.0, Color32::BLACK))
                                        .rounding(6.0)
                                        .inner_margin(crate::gui::egui::Margin::symmetric(6.0, 6.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("📝");
                                                ui.monospace(content);

                                                // Copy button with tooltip
                                                if ui
                                                    .add(egui::Button::new("📋 Copy").small())
                                                    .on_hover_text("Copy this text to clipboard")
                                                    .clicked()
                                                {
                                                    ctx.output_mut(|o| {
                                                        o.copied_text = content.clone();
                                                    });

                                                    // Small notification
                                                    ui.label(egui::RichText::new("✔ Copied!")
                                                        .color(Color32::DARK_GREEN)
                                                        .small());
                                                }
                                            });
                                        });

                                    ui.add_space(6.0);

                                    // 🕒 Timestamp row
                                    ui.horizontal(|ui| {
                                        ui.label("🕒");
                                        ui.monospace(timestamp);
                                    });
                                });

                            ui.add_space(12.0); // Space between clips
                        }
                    });
            });
    }
}
