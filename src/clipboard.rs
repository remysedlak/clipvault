use copypasta::{ClipboardContext, ClipboardProvider};
use std::{thread, time::Duration};
use chrono::Utc;

pub fn monitor_clipboard<F>(on_change: F)
where
    F: Fn(String, String) + Send + 'static,
{
    thread::spawn(move || {
        let mut ctx = ClipboardContext::new().unwrap();
        let mut last_clip = String::new();

        loop {
            if let Ok(current) = ctx.get_contents() {
                if current != last_clip && !current.trim().is_empty() {
                    let timestamp = Utc::now().to_rfc3339();
                    on_change(current.clone(), timestamp.clone());
                    last_clip = current;
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}