use clipvault::{db, gui};
use eframe::{NativeOptions, egui, icon_data::from_png_bytes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ClipVault GUI...");

    // Load PNG icon
    let icon_bytes = include_bytes!("../../assets/clipboard.png");
    let icon = from_png_bytes(icon_bytes).expect("Invalid PNG");

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(icon),
        ..Default::default()
    };

    let gui_db = db::init_db()?;

    eframe::run_native(
        "ClipVault",
        native_options,
        Box::new(|_cc| Ok(Box::new(gui::ClipVaultApp::new(gui_db)))),
    )?;

    Ok(())
}
