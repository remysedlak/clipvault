use crate::db;
use eframe::{egui::{self, Color32, Frame as EguiFrame, Stroke}, App as _};
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 ClipVault - Recent Clips");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (content, timestamp) in &self.clips {
                    // This is where we add a styled frame for each entry
                    EguiFrame::none()
                        .fill(Color32::from_rgb(240, 240, 240))
                        .stroke(Stroke::new(1.0, Color32::LIGHT_BLUE))
                        .inner_margin(crate::gui::egui::Margin::symmetric(8.0, 6.0))
                        .rounding(6.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("📝");
                                ui.monospace(content);
                            });
                            ui.horizontal(|ui| {
                                ui.label("🕒");
                                ui.monospace(timestamp);
                            });
                        });

                    ui.add_space(6.0); // spacing between entries
                }
            });
        });
    }
}
