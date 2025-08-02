mod clipboard;
mod db;
mod gui;
mod ui;
mod settings;
mod models;
mod utils;

use eframe::NativeOptions;
use std::{
    error::Error,
    sync::{Arc, Mutex, mpsc},
    thread,
};
use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::event_loop::{ControlFlow, EventLoop};

#[allow(dead_code)]
#[derive(Debug)]
enum AppEvent {
    OpenGui,
    Quit,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize DB connection (thread safe with Mutex)
    let db = Arc::new(Mutex::new(db::init_db()?));

    // Start clipboard monitoring thread
    {
        let db = db.clone();
        thread::spawn(move || {
            let _ = clipboard::monitor_clipboard(move |clip, _old_timestamp| {
                // Generate current timestamp as i64 (seconds since epoch)
                let timestamp = chrono::Utc::now().timestamp();

                let db = db.lock().unwrap();
                if let Err(e) = db::save_clip(&db, &clip, timestamp) {
                    eprintln!("Failed to save clip: {}", e);
                } else {
                    println!("Saved clip: {}, {}", clip, timestamp);
                }
            });
        });
    }

    // Create channel for GUI communication
    let (__gui_tx, gui_rx) = mpsc::channel::<AppEvent>();

    // Start GUI handler thread
    thread::spawn(move || {
        while let Ok(event) = gui_rx.recv() {
            match event {
                AppEvent::OpenGui => {
                    println!("Opening GUI...");
                    let native_options = NativeOptions::default();
                    match db::init_db() {
                        Ok(gui_db) => {
                            println!("Database initialized for GUI");
                            match eframe::run_native(
                                "ClipVault GUI",
                                native_options,
                                Box::new(|_cc| {
                                    println!("Creating GUI app...");
                                    Ok(Box::new(gui::ClipVaultApp::new(gui_db)))
                                }),
                            ) {
                                Ok(_) => println!("GUI closed normally"),
                                Err(e) => println!("GUI error: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to open DB for GUI: {}", e),
                    }
                }
                AppEvent::Quit => {
                    println!("Quitting application...");
                    std::process::exit(0);
                }
            }
        }
    });

    // Create event loop
    let event_loop = EventLoop::new()?;

    // Create tray menu
    let tray_menu = Menu::new();
    let open_item = MenuItem::new("Open", true, None);
    let quit_item = MenuItem::new("Quit", true, None);

    tray_menu.append(&open_item)?;
    tray_menu.append(&quit_item)?;

    // Load icon (try different approaches)
    let icon = load_icon_with_fallback();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("ClipVault")
        .with_icon(icon)
        .build()?;

    println!("Tray running. Right-click icon to open GUI or quit.");

    // Get menu event receiver
    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |_event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        // Handle menu events
        if let Ok(event) = menu_channel.try_recv() {
            match event.id {
                id if id == open_item.id() => {
                    println!("Open button clicked - launching GUI as separate process...");
                    use std::process::Command;

                    match Command::new("cargo").args(&["run", "--bin", "gui"]).spawn() {
                        Ok(_) => println!("GUI launched successfully"),
                        Err(e) => println!("Failed to launch GUI: {}", e),
                    }
                }
                id if id == quit_item.id() => {
                    println!("Quit button clicked");
                    elwt.exit();
                }
                _ => {}
            }
        }

        // Sleep to prevent busy waiting
        thread::sleep(std::time::Duration::from_millis(16));
    })?;

    Ok(())
}

fn load_icon_with_fallback() -> tray_icon::Icon {
    // Try to load from assets folder
    let icon_path = "assets/icon.ico";

    if let Ok(icon) = load_icon_from_path(icon_path) {
        println!("Loaded icon from: {}", icon_path);
        return icon;
    } else {
        println!("Failed to load icon from: {}", icon_path);
    }

    // Fallback: create a dummy 1x1 transparent pixel
    let rgba = vec![0, 0, 0, 0]; // Transparent pixel
    tray_icon::Icon::from_rgba(rgba, 1, 1).expect("Failed to create fallback icon")
}

fn load_icon_from_path(path: &str) -> Result<tray_icon::Icon, Box<dyn Error>> {
    let image = image::open(path)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Ok(tray_icon::Icon::from_rgba(rgba, width, height)?)
}
