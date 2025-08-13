use crate::models::{UiState, UiMode};
use eframe::egui::{self, Color32, RichText, Layout, TopBottomPanel, CentralPanel, Stroke, Rounding, Vec2};

pub struct SettingsView;

impl SettingsView {
    pub fn show(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        ui_state: &mut UiState,
    ) -> SettingsResponse {
        let mut response = SettingsResponse::default();
       
       let visuals = ui.visuals();
        let bg_color = visuals.widgets.inactive.bg_fill;
        let text_color = visuals.text_color();
        let stroke = visuals.widgets.inactive.bg_stroke;
        ui.add_space(10.0);
        
        // Top Panel: Title + Back Button
        TopBottomPanel::top("settings_top_panel")
            .min_height(25.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Settings").size(24.0));
                   
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        let back_button = ui.add_sized(
                            [120.0, 32.0], 
                            egui::Button::new(RichText::new("Back to Clips").size(14.0))
                                .fill(bg_color)
                                .stroke(stroke)
                                .rounding(Rounding::same(6.0))
                        );
                        
                        if back_button.on_hover_text("Return to main view").clicked() {
                            ui_state.ui_mode = UiMode::Main;
                        }
                    });
                });
                ui.add_space(8.0);
            });
            
        // Central Panel: Settings options with improved styling
        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    
                    // Settings section with card-like appearance
                    egui::Frame::none()
                        .stroke(stroke)
                        .rounding(Rounding::same(12.0))
                        .inner_margin(egui::Margin::same(32.0))
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                // Section title
                                ui.label(RichText::new("Application Settings")
                                    .size(18.0)
                                    .color(text_color));
                                
                                ui.add_space(24.0);
                                
                                // Reset Settings Button
                                let reset_button = ui.add_sized(
                                    [280.0, 48.0],
                                    egui::Button::new(RichText::new("🔄 Reset User Settings").size(16.0))
                                        .fill(Color32::from_rgb(59, 130, 246))
                                        .stroke(Stroke::new(1.0, Color32::from_rgb(37, 99, 235)))
                                        .rounding(Rounding::same(8.0))
                                );
                                
                                if reset_button
                                    .on_hover_text("Reset all user preferences to default values")
                                    .clicked() 
                                {
                                   response.reset_settings = true
                                }
                                
                                ui.add_space(16.0);
                                
                                // Separator line
                                ui.separator();
                                ui.add_space(16.0);
                                
                                // Delete All Button
                                let delete_button = ui.add_sized(
                                    [280.0, 48.0],
                                    egui::Button::new(RichText::new("🚮 Delete All Entries").size(16.0).color(Color32::WHITE))
                                        .fill(Color32::from_rgb(220, 38, 38))
                                        .stroke(Stroke::new(1.0, Color32::from_rgb(185, 28, 28)))
                                        .rounding(Rounding::same(8.0))
                                );
                                
                                if delete_button
                                    .on_hover_text("⚠️ This will permanently delete all clipboard entries")
                                    .clicked() 
                                {
                                    ui_state.show_delete_confirmation = true;
                                }
                                
                                ui.add_space(8.0);
                                
                                // Warning text
                                ui.label(RichText::new("This action cannot be undone")
                                    .size(12.0)
                                    .color(Color32::from_rgb(156, 163, 175))
                                    .italics());
                            });
                        });
                });
            });
        });
       
        // Keep your existing popup - it's perfect!
        if ui_state.show_delete_confirmation {
            egui::Window::new("Confirm Deletion")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.set_min_width(300.0);
                    
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        
                        // Warning icon and text
                        ui.label(RichText::new("⚠️").size(32.0).color(Color32::from_rgb(239, 68, 68)));
                        ui.add_space(8.0);
                        
                        ui.label(RichText::new("Are you sure you want to delete all records?")
                            .size(14.0)
                            .color(Color32::WHITE));
                        
                        ui.label(RichText::new("This action cannot be undone.")
                            .size(12.0)
                            .color(Color32::GRAY)
                            .italics());
                        
                        ui.add_space(16.0);
                        
                        ui.horizontal(|ui| {
                            // Delete button
                            let delete_btn = ui.add_sized(
                                [100.0, 32.0],
                                egui::Button::new(RichText::new("Yes, Delete").color(Color32::WHITE))
                                    .fill(Color32::from_rgb(220, 38, 38))
                                    .rounding(Rounding::same(6.0))
                            );
                            
                            if delete_btn.clicked() {
                                response.delete_db = true;
                                ui_state.show_delete_confirmation = false;
                            }
                            
                            ui.add_space(8.0);
                            
                            // Cancel button
                            let cancel_btn = ui.add_sized(
                                [100.0, 32.0],
                                egui::Button::new("Cancel")
                                    .fill(Color32::from_rgb(75, 85, 99))
                                    .rounding(Rounding::same(6.0))
                            );
                            
                            if cancel_btn.clicked() {
                                ui_state.show_delete_confirmation = false;
                            }
                        });
                        
                        ui.add_space(8.0);
                    });
                });
        }
       
        response
    }
}

#[derive(Default)]
pub struct SettingsResponse {
    pub delete_db: bool,
    pub reset_settings: bool,
}