// Flag for hiding the terminal on Windows
// #![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod clipboard;
mod db;
mod gui;
mod ui;
mod settings;
mod models;
mod utils;
use std::{ error::Error, sync::{ Arc, Mutex, mpsc }, thread, path::PathBuf, env, process::Command };
use tray_icon::{ TrayIconBuilder, menu::{ Menu, MenuEvent, MenuItem } };
use winit::event_loop::{ ControlFlow, EventLoop };

// Include icon bytes from assets folder
const ICON_BYTES: &[u8] = include_bytes!("../assets/clipboard.png");

use std::io::Cursor;
use image::io::Reader as ImageReader;

fn load_icon_embedded() -> tray_icon::Icon {
    // Read image from embedded bytes (ico or png)
    let cursor = Cursor::new(ICON_BYTES);
    let image = ImageReader::new(cursor)
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode embedded icon")
        .to_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    tray_icon::Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}

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
                    fn get_gui_exe_path() -> PathBuf {
                        let mut exe_path = env
                            ::current_exe()
                            .expect("Failed to get current exe path");
                        exe_path.pop(); // Remove clipvault.exe file name
                        exe_path.push("gui.exe"); // Append gui.exe
                        exe_path
                    }

                    // In your tray menu event handler:
                    let gui_path = get_gui_exe_path();

                    match Command::new(gui_path).spawn() {
                        Ok(_) => println!("GUI launched successfully"),
                        Err(e) => println!("Failed to launch GUI: {}", e),
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
    let icon = load_icon_embedded();

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

                    fn get_gui_exe_path() -> PathBuf {
                        let mut exe_path = env
                            ::current_exe()
                            .expect("Failed to get current exe path");
                        exe_path.pop(); // Remove clipvault.exe file name
                        exe_path.push("gui.exe"); // Append gui.exe
                        exe_path
                    }

                    let gui_path = get_gui_exe_path();

                    match Command::new(gui_path).spawn() {
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
