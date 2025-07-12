use std::sync::mpsc::{self, Sender, Receiver};
use std::{thread::{self, JoinHandle}, time::Duration};
use copypasta::{ClipboardContext, ClipboardError, ClipboardProvider};

pub fn monitor_clipboard<F>(on_change: F) -> Result<(Sender<()>, JoinHandle<()>), ClipboardError>
where
    F: Fn(String) + Send + 'static,
{
    let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let mut ctx = ClipboardContext::new()?;
    let handle = thread::spawn(move || {
        let mut last_clip = String::new();
        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }
            match ctx.get_contents() {
                Ok(current) => {
                    if current != last_clip && !current.trim().is_empty() {
                        on_change(current.clone());
                        last_clip = current;
                    }
                }
                Err(err) => {
                    eprintln!("clipboard error: {:?}", err);
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
    Ok((stop_tx, handle))
}
