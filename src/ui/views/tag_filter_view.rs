use crate::models::{ Clip, Tag, UiState, UiMode };
use crate::ui::components::tag_card;
use crate::ui::popups::create_tag::CreateTagPopup;
use eframe::egui::{
    self,
    Layout,
    TopBottomPanel,
    CentralPanel,
    ScrollArea,
    Color32,
    RichText,
    TextStyle,
};
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
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if tags.is_empty() {
                        ui.label(
                            RichText::new("No tags yet. Create your first tag!")
                                .color(Color32::GRAY)
                                .italics()
                        );
                    } else {
                        ui.label(
                            RichText::new("Click a tag to filter clips, or create a new one")
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
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let available_width = ui.available_width();
                        let button_width = 150.0;
                        let buttons_per_row = (
                            (available_width - 40.0) /
                            (button_width + 10.0)
                        ).floor() as usize;
                        let buttons_per_row = buttons_per_row.max(1);

                        let mut refresh_needed = false;

                        for chunk in tags.chunks_mut(buttons_per_row) {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.y = 10.0;

                                // Calculate centering
                                // let total_buttons = chunk.len();
                                // let total_button_width = (total_buttons as f32) * button_width;
                                // let total_spacing =
                                //     ((total_buttons - 1) as f32) * ui.spacing().item_spacing.x;
                                // let used_width = total_button_width + total_spacing;
                                // let available_width = ui.available_width();
                                // let leftover_space = available_width - used_width;
                                // let left_margin = (leftover_space / 2.0).max(0.0);
                                // ui.add_space(left_margin);

                                for tag in chunk {
                                    if tag_card::show(ui, tag, db, clips, ui_state, button_width) {
                                        refresh_needed = true;
                                    }
                                }
                            });
                            ui.add_space(15.0);
                        }

                        // Handle refresh outside the loop to avoid borrowing conflicts
                        if refresh_needed {
                            if let Ok(db_tags) = crate::db::load_tags(db) {
                                *tags = db_tags.into_iter().map(Tag::from_tuple).collect();
                            }
                            *clip_tags = crate::db::load_clip_tags(db).unwrap_or_default();
                        }

                        ui.add_space(20.0);
                    });
            }
        });

        if ui_state.show_create_popup {
            CreateTagPopup::show(ctx, ui_state, db, tags);
        }
    }
}
