use clipvault::{db, gui};
use eframe::NativeOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ClipVault GUI...");
    
    let gui_db = db::init_db()?;
    let native_options = NativeOptions::default();
    
    eframe::run_native(
        "ClipVault GUI",
        native_options,
        Box::new(|_cc| Ok(Box::new(gui::ClipVaultApp::new(gui_db)))),
    )?;
    
    Ok(())
}