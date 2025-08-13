use crate::db;
use crate::models::{ Clip, Tag, UiState, UiMode };
use crate::ui::popups::create_tag::CreateTagPopup;
use crate::utils::formatting::hex_to_color32;
use eframe::egui::{
    self,
    Layout,
    TopBottomPanel,
    CentralPanel,
    Color32,
    RichText,
    TextStyle,
};
use egui_extras::{Column, TableBuilder};
use rusqlite::Connection;

pub struct TagFilterView;

impl TagFilterView {
    pub fn show(
        _ui: &mut egui::Ui, // no longer needed since we use panels
        ctx: &egui::Context,
        clips: &mut Vec<Clip>,
        db: &Connection,
        ui_state: &mut UiState,
        tags: &mut Vec<Tag>,
        clip_tags: &mut std::collections::HashMap<i64, Vec<String>>
    ) {
        TopBottomPanel::top("tag_filter_top_panel")
            .min_height(25.0)
            .show(ctx, |ui| {
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.heading("Tag Manager");

                    ui.label(
                        RichText::new(format!("({} tags)", tags.len()))
                            .color(Color32::GRAY)
                            .text_style(TextStyle::Body)
                    );

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if
                            ui
                                .add_sized([60.0, 20.0], egui::Button::new("Back to clips..."))
                                .on_hover_text("Return to main view")
                                .clicked()
                        {
                            ui_state.ui_mode = UiMode::Main;
                        }
                    });
                });
                ui.add_space(2.0);
            });

        TopBottomPanel::bottom("tag_filter_bottom_panel")
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.add_space(2.0);
                ui.vertical_centered(|ui| {
                    if tags.is_empty() {
                        ui.label(
                            RichText::new("No tags yet. Create your first tag!")
                                .color(Color32::GRAY)
                                .italics()
                        );
                    } else {
                        ui.label(
                            RichText::new("Click a tag to filter clips, or manage colors and settings")
                                .color(Color32::GRAY)
                                .text_style(TextStyle::Small)
                        );
                    }

                    ui.add_space(8.0);

                    if
                        ui
                            .add_sized([200.0, 40.0], egui::Button::new("➕ Create New Tag"))
                            .on_hover_text("Add a new tag to organize your clips")
                            .clicked()
                    {
                        ui_state.show_create_popup = true;
                    }
                });
                ui.add_space(10.0);
            });

        CentralPanel::default().show(ctx, |ui| {
            if tags.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("📋").size(48.0).color(Color32::GRAY));
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("No tags created yet")
                                .text_style(TextStyle::Heading)
                                .color(Color32::GRAY)
                        );
                        ui.label(
                            RichText::new("Tags help you organize and find your clipboard history")
                                .text_style(TextStyle::Body)
                                .color(Color32::DARK_GRAY)
                        );
                    });
                });
            } else {
                ui.vertical(|ui| {

                        
                        let mut refresh_needed = false;
                        let mut tags_to_delete = Vec::new();
                        
                        // Create the tags table
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(false)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column:: remainder())   // Tag name
                            .column(Column:: remainder())     // Clip count
                            .column(Column:: remainder())     // Color picker
                            .column(Column::exact(60.0))     // Delete button
                            .header(30.0, |mut header| {

                                header.col(|ui| {
                                    ui.strong("Tag Name");
                                });
                                header.col(|ui| {
                                    ui.strong("Clips");
                                });
                                header.col(|ui| {
                                    ui.strong("Color");
                                });
                                header.col(|ui| {
                                    ui.strong("Delete");
                                });
                            })
                            .body(|mut body| {
                                for (index, tag) in tags.iter_mut().enumerate() {
                                    body.row(35.0, |mut row| {
                                        
                                        
                                        // Tag name column (clickable button)
                                        row.col(|ui| {
                                            let button = ui.add_sized(
                                                [ui.available_width(), 25.0],
                                                egui::Button::new(
                                                    RichText::new(&tag.name)
                                                        .text_style(TextStyle::Button)
                                                )
                                            );
                                            
                                            if button.clicked() {
                                                *clips = db::load_clips_for_tag(db, &tag.id)
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .map(Clip::from_tuple)
                                                    .collect();
                                                ui_state.ui_mode = UiMode::Main;
                                            }
                                        });
                                        
                                        // Clip count column
                                        row.col(|ui| {
                                            if let Ok(count) = db::count_clips_for_tag(db, &tag.id) {
                                                ui.label(
                                                    RichText::new(format!("{}", count))
                                                        .text_style(TextStyle::Body)
                                                        .color(if count > 0 { Color32::WHITE } else { Color32::GRAY })
                                                );
                                            } else {
                                                ui.label(
                                                    RichText::new("?")
                                                        .text_style(TextStyle::Body)
                                                        .color(Color32::RED)
                                                );
                                            }
                                        });
                                        
                                        // Color picker column
                                        row.col(|ui| {
                                            let mut color = tag
                                                .color
                                                .as_ref()
                                                .and_then(|hex| hex_to_color32(hex))
                                                .unwrap_or_else(|| {
                                                    if ui.visuals().dark_mode {
                                                        Color32::LIGHT_GRAY
                                                    } else {
                                                        Color32::DARK_GRAY
                                                    }
                                                });
                                            
                                            if ui.color_edit_button_srgba(&mut color)
                                                .on_hover_text("Set tag color")
                                                .changed() 
                                            {
                                                tag.color = Some(color32_to_hex(color));
                                                let color_ref = tag.color.as_deref();
                                                if let Err(e) = db::update_tag_color(db, tag.id, color_ref) {
                                                    eprintln!("Failed to update tag color: {}", e);
                                                }
                                            }
                                        });
                                        
                                        
                                        // Delete button column
                                        row.col(|ui| {
                                            if ui.small_button("🗑")
                                                .on_hover_text("Delete tag")
                                                .clicked() 
                                            {
                                                if let Err(e) = db::delete_tag(db, tag.id) {
                                                    eprintln!("Failed to delete tag: {}", e);
                                                } else {
                                                    tags_to_delete.push(index);
                                                    refresh_needed = true;
                                                }
                                            }
                                        });
                                    });
                                }
                            });

                        // Handle refresh outside the table to avoid borrowing conflicts
                        if refresh_needed {
                            if let Ok(db_tags) = crate::db::load_tags(db) {
                                *tags = db_tags.into_iter().map(Tag::from_tuple).collect();
                            }
                            *clip_tags = crate::db::load_clip_tags(db).unwrap_or_default();
                        }

                        ui.add_space(2.0);
                    });
            }
        });

        if ui_state.show_create_popup {
            CreateTagPopup::show(ctx, ui_state, db, tags);
        }
    }
}

// Helper: convert Color32 to hex string like "#RRGGBB"
fn color32_to_hex(color: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}