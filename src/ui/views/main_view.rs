use crate::db;
use crate::models::{Clip, UiState};
use crate::ui::components::clip_card::{ClipCard};
use crate::ui::popups::tag_assignment::TagAssignmentPopup;
use eframe::egui::{self, Color32, RichText, TextStyle};
use rusqlite::Connection;
use std::collections::HashMap;

pub struct MainView;

impl MainView {
    pub fn show(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        clips: &mut Vec<Clip>,
        db: &Connection,
        ui_state: &mut UiState,
        darkmode: bool,
        clip_tags: &mut HashMap<i64, Vec<String>>,
        tags: &[(i64, String)],
    ) {
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let mut deleted_id: Option<i64> = None;
                let mut pinned_id: Option<i64> = None;

                if clips.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No clips found.")
                                .color(Color32::DARK_GRAY)
                                .italics()
                                .text_style(TextStyle::Heading),
                        );
                    });
                }

                for clip in clips.iter() {
                    if clip.is_empty() {
                        continue;
                    }

                    let response = ClipCard::show(
                        ui, ctx, clip, ui_state.show_content, darkmode, clip_tags
                    );

                    if response.delete_requested {
                        deleted_id = Some(clip.id);
                    }
                    if response.pin_toggled {
                        pinned_id = Some(clip.id);
                    }
                    if response.add_tag_requested {
                        ui_state.show_tag_popup_for = Some(clip.id);
                        ui_state.selected_tag_id = None;
                    }

                    ui.add_space(12.0);

                    if deleted_id.is_some() || pinned_id.is_some() {
                        break;
                    }
                }

                // Handle tag assignment popup
                if let Some(clip_id) = ui_state.show_tag_popup_for {
                    TagAssignmentPopup::show(
                        ctx, 
                        clip_id, 
                        ui_state, 
                        db, 
                        clip_tags, 
                        tags
                    );
                }

                // Handle deletions and pins
                if let Some(id) = deleted_id {
                    let _ = db::delete_clip(db, id);
                    *clips = db::load_recent_clips(db, 20)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                }

                if let Some(id) = pinned_id {
                    let _ = db::toggle_pin_clip(db, id);
                    *clips = db::load_recent_clips(db, 20)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                }
            });
    }
}