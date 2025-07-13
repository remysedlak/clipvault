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

pub fn delete_clip(conn: &Connection, id: i64) -> Result<usize> {
    println!("Deleting clip with ID: {}", id);
    let result = conn.execute("DELETE FROM clips WHERE id = ?1", params![id]);
    match result {
        Ok(rows) => println!("Deleted {} row(s).", rows),
        Err(ref e) => println!("Error deleting clip: {}", e),
    }
    result
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


pub fn load_recent_clips(conn: &Connection, limit: usize) -> Result<Vec<(i64, String, i64)>> {
    println!("Loading up to {} recent clips...", limit);
    let mut stmt = conn.prepare("SELECT id, content, timestamp FROM clips ORDER BY timestamp DESC LIMIT ?")?;
    let rows = stmt.query_map([limit as i64], |row| {
        Ok((
            row.get::<_, i64>(0)?, // id
            row.get::<_, String>(1)?, // content
            row.get::<_, i64>(2)?, // timestamp
        ))
    })?;

    let mut clips = Vec::new();
    for clip in rows {
        match &clip {
            Ok((id, content, timestamp)) => println!("Loaded clip (ID: {}): '{}', timestamp: '{}'", id, content, timestamp),
            Err(e) => println!("Error loading a clip row: {}", e),
        }
        clips.push(clip?);
    }
    println!("Total clips loaded: {}", clips.len());
    Ok(clips)
}
