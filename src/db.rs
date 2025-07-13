use rusqlite::{Connection, Result};
use rusqlite::params;

pub fn init_db() -> Result<Connection> {
    println!("Initializing database...");
    let conn = Connection::open("clips.db")?;
    println!("Database opened successfully.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
    id INTEGER PRIMARY KEY,
    content TEXT NOT NULL,
    timestamp INTEGER NOT NULL
)",
        [],
    )?;
    println!("Table checked/created.");
    Ok(conn)
}

pub fn save_clip(conn: &Connection, clip: &str, timestamp: i64) -> Result<(), rusqlite::Error> {
    println!("Saving clip: '{}', timestamp: '{}'", clip, timestamp);
    let result = conn.execute(
        "INSERT INTO clips (content, timestamp) VALUES (?1, ?2)",
        params![clip, timestamp],
    );
    match result {
        Ok(rows) => println!("Inserted {} row(s) into clips.", rows),
        Err(ref e) => println!("Error inserting clip: {}", e),
    }
    result.map(|_| ())
}


pub fn load_recent_clips(conn: &Connection, limit: usize) -> Result<Vec<(String, i64)>> {
    println!("Loading up to {} recent clips...", limit);
    let mut stmt = conn.prepare("SELECT content, timestamp FROM clips ORDER BY timestamp DESC LIMIT ?")?;
    let rows = stmt.query_map([limit as i64], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut clips = Vec::new();
    for clip in rows {
        match &clip {
            Ok((content, timestamp)) => println!("Loaded clip: '{}', timestamp: '{}'", content, timestamp),
            Err(e) => println!("Error loading a clip row: {}", e),
        }
        clips.push(clip?);
    }
    println!("Total clips loaded: {}", clips.len());
    Ok(clips)
}
