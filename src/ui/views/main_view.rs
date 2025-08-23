use crate::db;
use crate::models::{ Clip, UiState };
use crate::ui::components::clip_card::{ ClipCard };
use crate::ui::popups::tag_assignment::TagAssignmentPopup;
use crate::ui::popups::create_clip::CreateClip;
use eframe::egui::{ self, Color32, RichText, TextStyle };
use rusqlite::Connection;
use std::collections::HashMap;
use crate::utils::formatting::hex_to_color32;

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
        tags: &[(i64, String, Option<String>)] // Updated to include color
    ) {
        egui::ScrollArea
            ::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.add_space(6.0);
                let mut deleted_id: Option<i64> = None;
                let mut pinned_id: Option<i64> = None;

                if ui_state.show_create_clip_popup {
                    CreateClip::show(ctx, ui_state, db, clips);
                }

                if clips.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No clips found.")
                                .color(Color32::DARK_GRAY)
                                .italics()
                                .text_style(TextStyle::Heading)
                        );
                    });
                }

                for clip in clips.iter() {
                    if clip.is_empty() {
                        continue;
                    }

                    // Build the tag_colors map once before rendering cards:
                    let tag_colors: HashMap<String, Color32> = tags
                        .iter()
                        .filter_map(|tag| {
                            tag.2.as_ref().and_then(|hex| {
                                hex_to_color32(hex).map(|color| (tag.1.clone(), color)) // Use tag.1.clone() here for key
                            })
                        })
                        .collect();

                    let response = ClipCard::show(
                        ui,
                        ctx,
                        clip,
                        ui_state.show_content,
                        darkmode,
                        clip_tags,
                        &tag_colors,
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

                    ui.add_space(6.0);

                    if deleted_id.is_some() || pinned_id.is_some() {
                        break;
                    }
                }

                // Handle tag assignment popup
                if let Some(clip_id) = ui_state.show_tag_popup_for {
                    TagAssignmentPopup::show(ctx, clip_id, ui_state, db, clip_tags, tags);
                }

                // Handle deletions and pins
                if let Some(id) = deleted_id {
                    let _ = db::delete_clip(db, id);
                    *clips = db
                        ::load_recent_clips(db, 20)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                }

                if let Some(id) = pinned_id {
                    let _ = db::toggle_pin_clip(db, id);
                    *clips = db
                        ::load_recent_clips(db, 20)
                        .unwrap_or_default()
                        .into_iter()
                        .map(Clip::from_tuple)
                        .collect();
                }
            });
    }
}
