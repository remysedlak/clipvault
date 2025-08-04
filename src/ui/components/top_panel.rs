use eframe::egui::{self, Layout};
use egui_extras::DatePickerButton;
use chrono::NaiveDate;

pub struct TopPanel;

impl TopPanel {
    pub fn show(
        ui: &mut egui::Ui,
        date: &mut NaiveDate,
        show_content: &mut bool,
        darkmode: &mut bool,
    ) -> TopPanelResponse {
        let mut response = TopPanelResponse::default();
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.heading("📋 ClipVault");
            ui.separator();
            ui.label("Recent clipboard history");

            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                // Show/Hide content toggle
                let show_content_label = if *show_content { "🙈 Hide" } else { "👁 Show" };
                if ui.button("Settings").on_hover_text("Settings").clicked() {
                    response.settings = true;
                }
                if ui
                    .button(show_content_label)
                    .on_hover_text("Show or hide the content of all clips")
                    .clicked()
                {
                    *show_content = !*show_content;
                }
                
                

                // Tags button
                if ui.button("🗁").on_hover_text("View tags").clicked() {
                    response.show_tags = true;
                }

                

                
                
                // Date picker
                if ui
                    .add(
                        DatePickerButton::new(date)
                            .show_icon(true)
                            .highlight_weekends(false)
                            .format(""),
                    )
                    .on_hover_text("Select a date to filter clips")
                    .changed()
                {
                    response.date_changed = true;
                }
                
                // Dark/Light mode toggle
                let mode_label = if *darkmode { "🌙" } else { "🔆" };
                if ui
                    .button(mode_label)
                    .on_hover_text("Toggle dark/light mode")
                    .clicked()
                {
                    *darkmode = !*darkmode;
                }

                // Refresh button
                if ui
                    .button("🔄")
                    .on_hover_text("Refresh clipboard entries.")
                    .clicked()
                {
                    response.refresh_requested = true;
                }
            });
        });
        
        ui.add_space(4.0);
        response
        
    }
}

#[derive(Default)]
pub struct TopPanelResponse {
    pub show_tags: bool,
    pub date_changed: bool,
    pub refresh_requested: bool,
    pub settings: bool,
}