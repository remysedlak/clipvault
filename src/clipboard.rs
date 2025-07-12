use copypasta::{ClipboardContext, ClipboardProvider};
use std::{thread, time::Duration};

pub fn monitor_clipboard<F: Fn(String) + Send + 'static>(on_change: F) {
    thread::spawn(move || {
        let mut ctx = ClipboardContext::new().unwrap();
        let mut last_clip = String::new();

        loop {
            if let Ok(current) = ctx.get_contents() {
                if current != last_clip && !current.trim().is_empty() {
                    on_change(current.clone());
                    last_clip = current;
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}
