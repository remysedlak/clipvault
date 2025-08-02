use crate::db;
use crate::models::{Clip, Tag, UiState, UiMode};
use crate::ui::popups::create_tag::CreateTagPopup;
use eframe::egui::{self, Layout, TopBottomPanel, CentralPanel, ScrollArea, Color32, RichText, TextStyle, Frame, Stroke, Margin};
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
    ) {
        // Top Panel: Title + Back Button + Tag Count
        TopBottomPanel::top("tag_filter_top_panel")
            .min_height(60.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.heading("Tag Manager");
                    
                    // Show tag count
                    ui.label(RichText::new(format!("({} tags)", tags.len()))
                        .color(Color32::GRAY)
                        .text_style(TextStyle::Body));
                    
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add_sized([80.0, 35.0], egui::Button::new("🔙 Back"))
                            .on_hover_text("Return to main view")
                            .clicked()
                        {
                            ui_state.ui_mode = UiMode::Main;
                        }
                    });
                });
                ui.add_space(8.0);
            });

        // Bottom Panel: Create new tag button + instructions
        TopBottomPanel::bottom("tag_filter_bottom_panel")
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if tags.is_empty() {
                        ui.label(RichText::new("No tags yet. Create your first tag!")
                            .color(Color32::GRAY)
                            .italics());
                    } else {
                        ui.label(RichText::new("Click a tag to filter clips, or create a new one")
                            .color(Color32::GRAY)
                            .text_style(TextStyle::Small));
                    }
                    
                    ui.add_space(8.0);
                    
                    if ui
                        .add_sized([200.0, 40.0], egui::Button::new("➕ Create New Tag"))
                        .on_hover_text("Add a new tag to organize your clips")
                        .clicked()
                    {
                        ui_state.show_create_popup = true;
                    }
                });
                ui.add_space(10.0);
            });

        // Central Panel: Scrollable tag list with improved styling
        CentralPanel::default().show(ctx, |ui| {
            if tags.is_empty() {
                // Empty state
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("📋")
                            .size(48.0)
                            .color(Color32::GRAY));
                        ui.add_space(10.0);
                        ui.label(RichText::new("No tags created yet")
                            .text_style(TextStyle::Heading)
                            .color(Color32::GRAY));
                        ui.label(RichText::new("Tags help you organize and find your clipboard history")
                            .text_style(TextStyle::Body)
                            .color(Color32::DARK_GRAY));
                    });
                });
            } else {
                // Tag list with full-width scrolling
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(20.0);
                        
                        // Use a grid layout for better organization
                        let available_width = ui.available_width();
                        let button_width = 280.0;
                        let buttons_per_row = ((available_width - 40.0) / (button_width + 10.0)).floor() as usize;
                        let buttons_per_row = buttons_per_row.max(1); // At least 1 button per row
                        
                        for chunk in tags.chunks(buttons_per_row) {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 10.0;
                                
                                for tag in chunk {
                                    // Enhanced tag button with card styling
                                    let button_response = Frame::none()
                                        .fill(if ui.visuals().dark_mode {
                                            Color32::from_rgb(50, 50, 60)
                                        } else {
                                            Color32::from_rgb(248, 249, 250)
                                        })
                                        .stroke(Stroke::new(1.0, if ui.visuals().dark_mode {
                                            Color32::from_rgb(70, 70, 80)
                                        } else {
                                            Color32::from_rgb(220, 220, 220)
                                        }))
                                        .rounding(8.0)
                                        .inner_margin(Margin::symmetric(15.0, 12.0))
                                        .show(ui, |ui| {
                                            ui.set_min_size([button_width, 50.0].into());
                                            
                                            let button = ui.add_sized(
                                                [button_width - 30.0, 35.0],
                                                egui::Button::new(
                                                    RichText::new(format!("🏷️ {}", tag.name))
                                                        .text_style(TextStyle::Button)
                                                        .strong()
                                                )
                                                .fill(Color32::TRANSPARENT)
                                                .stroke(Stroke::NONE)
                                            );
                                            
                                            if button.clicked() {
                                                *clips = db::load_clips_for_tag(db, &tag.id)
                                                    .unwrap_or_default()
                                                    .into_iter()
                                                    .map(Clip::from_tuple)
                                                    .collect();
                                                ui_state.ui_mode = UiMode::Main;
                                            }
                                            
                                            // Show clip count for this tag if available
                                            if let Ok(count) = db::count_clips_for_tag(db, &tag.id) {
                                                ui.label(RichText::new(format!("{} clips", count))
                                                    .text_style(TextStyle::Small)
                                                    .color(Color32::GRAY));
                                            }
                                            
                                            button.on_hover_text(format!("Show all clips tagged with '{}'", tag.name))
                                        });
                                }
                            });
                            ui.add_space(15.0);
                        }
                        
                        ui.add_space(20.0);
                    });
            }
        });

        // Popup handler
        if ui_state.show_create_popup {
            CreateTagPopup::show(ctx, ui_state, db, tags);
        }
    }
}