use copypasta::{ClipboardContext, ClipboardProvider};
use std::{
    error::Error,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};
use chrono::Utc;

pub fn monitor_clipboard<F>(
    on_change: F,
) -> Result<(Sender<()>, JoinHandle<()>), Box<dyn Error>>
where
    F: Fn(String, String) + Send + 'static,
{
    let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let mut ctx = ClipboardContext::new()?;
    let handle = thread::spawn(move || {
        let mut last_clip = String::new();
        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }
            if let Ok(current) = ctx.get_contents() {
                if !current.trim().is_empty() && current != last_clip {
                    let timestamp = Utc::now().to_rfc3339();
                    on_change(current.clone(), timestamp);
                    last_clip = current;
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
    Ok((stop_tx, handle))
}