use crate::db;

use eframe::{egui, Frame};
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 ClipVault - Recent Clips");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for clip in &self.clips {
                    ui.separator();
                    ui.label(format!("{} - {}", clip.0, clip.1));
                    
                }
            });
        });
    }
}
