use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    println!("Initializing database...");
    let conn = Connection::open("clips.db")?;
    println!("Database opened successfully.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            pinned INTEGER NOT NULL DEFAULT 0
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
        "INSERT INTO clips (content, timestamp, pinned) VALUES (?1, ?2, 0)",
        params![clip, timestamp],
    );
    match result {
        Ok(rows) => println!("Inserted {} row(s) into clips.", rows),
        Err(ref e) => println!("Error inserting clip: {}", e),
    }
    result.map(|_| ())
}

pub fn load_recent_clips(conn: &Connection, limit: usize) -> Result<Vec<(i64, String, i64, bool)>> {
    println!("Loading up to {} recent clips...", limit);
    let mut stmt = conn.prepare(
        "SELECT id, content, timestamp, pinned FROM clips 
         ORDER BY pinned DESC, timestamp DESC 
         LIMIT ?"
    )?;
    let rows = stmt.query_map([limit as i64], |row| {
        Ok((
            row.get::<_, i64>(0)?,     // id
            row.get::<_, String>(1)?,  // content
            row.get::<_, i64>(2)?,     // timestamp
            row.get::<_, i64>(3)? != 0 // pinned (as bool)
        ))
    })?;
    let mut clips = Vec::new();
    for clip in rows {
        match &clip {
            Ok((id, content, timestamp, pinned)) => {
                println!(
                    "Loaded clip (ID: {}): '{}', timestamp: '{}', pinned: {}",
                    id, content, timestamp, pinned
                );
            }
            Err(e) => println!("Error loading a clip row: {}", e),
        }
        clips.push(clip?);
    }
    println!("Total clips loaded: {}", clips.len());
    Ok(clips)
}

pub fn toggle_pin_clip(conn: &Connection, id: i64) -> Result<usize> {
    println!("Toggling pin for clip with ID: {}", id);
    conn.execute(
        "UPDATE clips SET pinned = NOT pinned WHERE id = ?1",
        params![id],
    )
}

/// Load clips that fall on a specific UTC date
/// Load clips that fall on a specific LOCAL date (converts to UTC for query)
pub fn load_clips_for_date(conn: &Connection, date: chrono::NaiveDate) -> Result<Vec<(i64, String, i64, bool)>> {
    use chrono::{Local, TimeZone};
    
    // Create start and end of day in LOCAL timezone
    let start_of_day = date.and_hms_opt(0, 0, 0).unwrap();
    let next_day = date + chrono::Duration::days(1);
    let start_of_next_day = next_day.and_hms_opt(0, 0, 0).unwrap();
    
    // Convert local times to UTC timestamps
    let start_ts = Local.from_local_datetime(&start_of_day).unwrap().timestamp();
    let end_ts = Local.from_local_datetime(&start_of_next_day).unwrap().timestamp();
    
    println!("Loading clips between {} (inclusive) and {} (exclusive)", start_ts, end_ts);
    
    let mut stmt = conn.prepare(
        "SELECT id, content, timestamp, pinned FROM clips
         WHERE timestamp >= ?1 AND timestamp < ?2
         ORDER BY pinned DESC, timestamp DESC"
    )?;
    
    let rows = stmt.query_map(params![start_ts, end_ts], |row| {
        Ok((
            row.get::<_, i64>(0)?,     // id
            row.get::<_, String>(1)?,  // content
            row.get::<_, i64>(2)?,     // timestamp
            row.get::<_, i64>(3)? != 0 // pinned (as bool)
        ))
    })?;
    
    let mut clips = Vec::new();
    for clip in rows {
        match &clip {
            Ok((id, content, timestamp, pinned)) => {
                println!(
                    "Loaded clip (ID: {}): '{}', timestamp: '{}', pinned: {}",
                    id, content, timestamp, pinned
                );
            }
            Err(e) => println!("Error loading a clip row: {}", e),
        }
        clips.push(clip?);
    }
    
    println!("Total clips loaded for date: {}", clips.len());
    Ok(clips)
}