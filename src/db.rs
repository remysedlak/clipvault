use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::params;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("clips.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}

use rusqlite::ToSql;

pub fn save_clip(conn: &Connection, clip: &str, timestamp: &str) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO clips (content, timestamp) VALUES (?1, ?2)",
        params![clip, timestamp],
    )?;
    Ok(())
}


pub fn load_recent_clips(conn: &Connection, limit: usize) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT content, timestamp FROM clips ORDER BY timestamp DESC LIMIT ?")?;
    let rows = stmt.query_map([limit as i64], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut clips = Vec::new();
    for clip in rows {
        clips.push(clip?);
    }
    Ok(clips)
}