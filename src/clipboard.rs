use std::sync::mpsc::{self, Sender, Receiver};
use std::{thread::{self, JoinHandle}, time::Duration, fmt};
use copypasta::{ClipboardContext, ClipboardProvider};

// Option 2: Define your own ClipboardError type
#[derive(Debug)]
pub struct ClipboardError(pub Box<dyn std::error::Error>);

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clipboard error: {}", self.0)
    }
}

impl std::error::Error for ClipboardError {}

impl From<Box<dyn std::error::Error>> for ClipboardError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ClipboardError(error)
    }
}

pub fn monitor_clipboard<F>(on_change: F) -> Result<(Sender<()>, JoinHandle<()>), ClipboardError>
where
    F: Fn(String) + Send + 'static,
{
    let (stop_tx, stop_rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let mut ctx = ClipboardContext::new().map_err(ClipboardError::from)?;
    
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
<<<<<<< HEAD
=======
    
    Ok((stop_tx, handle))
}

// Usage example:
fn main() -> Result<(), ClipboardError> {
    let (stop_tx, handle) = monitor_clipboard(|content| {
        println!("Clipboard changed: {}", content);
    })?;
    
    // Let it run for 10 seconds
    thread::sleep(Duration::from_secs(10));
    
    // Stop monitoring  
    stop_tx.send(()).map_err(|e| ClipboardError(Box::new(e)))?;
    handle.join().unwrap();
    
    Ok(())
>>>>>>> f7297d4b2246f673033595c7dc9f1f6a416f1b7b
}