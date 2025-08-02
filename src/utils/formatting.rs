use chrono::{DateTime, Datelike, Local};

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