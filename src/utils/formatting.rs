use chrono::{DateTime, Datelike, Local};
use eframe::egui::Color32;

/// Convert a hex color string (e.g., "#RRGGBB") to a Color32
pub fn hex_to_color32(hex: &str) -> Option<Color32> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
    Some(Color32::from_rgb(r, g, b))
}

/// Format a UTC timestamp (e.g., 1755048970) to a human readible date
pub fn format_timestamp(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| {
            let dt = dt.with_timezone(&Local);
            let day = dt.day();
            let suffix = match day {
                11 | 12 | 13 => "th",
                _ => match day % 10 {
                    1 => "st",
                    2 => "nd", 
                    3 => "rd",
                    _ => "th",
                },
            };
            format!(
                "{} {}{}, {} {}:{} {}",
                dt.format("%B"),
                day,
                suffix,
                dt.format("%Y"),
                dt.format("%-I"),
                dt.format("%M"),
                dt.format("%p")
            )
        })
        .unwrap_or_else(|| "Invalid timestamp".to_string())
}