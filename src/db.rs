use rusqlite::{Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("clips.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL DEFAULT (strftime('%s','now'))
        )",
        [],
    )?;
    Ok(conn)
}

use rusqlite::ToSql;

pub fn save_clip(conn: &Connection, clip: &str) -> Result<()> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    conn.execute(
        "INSERT INTO clips (content, timestamp) VALUES (?, ?)",
        &[&clip as &dyn ToSql, &now],
    )?;
    Ok(())
}


pub fn load_recent_clips(conn: &Connection, limit: usize) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT content FROM clips ORDER BY timestamp DESC LIMIT ?")?;
    let rows = stmt.query_map([limit as i64], |row| row.get(0))?;
    let mut clips = Vec::new();
    for clip in rows {
        clips.push(clip?);
    }
    Ok(clips)
}
