use rusqlite::{Connection, Result, params};
use std::collections::HashMap;
   
pub fn init_db() -> Result<Connection> {
    println!("Initializing database...");
    let conn = Connection::open("clips.db")?;
    println!("Database opened successfully.");
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            pinned INTEGER NOT NULL DEFAULT 0
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS clip_tags (
            clip_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (clip_id, tag_id),
            FOREIGN KEY (clip_id) REFERENCES clips(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )
        ",
        [],
    )?;

    println!("Tables checked/created.");
    Ok(conn)
}

pub fn reset_db(conn: &Connection) -> Result<()> {
    println!("Resetting database: deleting all entries...");

    // Disable foreign key checks temporarily to avoid issues with cascade deletes
    conn.execute_batch("PRAGMA foreign_keys = OFF;")?;

    // Delete all data from each table
    conn.execute_batch("
        DELETE FROM clip_tags;
        DELETE FROM clips;
        DELETE FROM tags;
    ")?;

    // Re-enable foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    // Optional: reclaim free space
    conn.execute_batch("VACUUM;")?;

    println!("Database reset: all tables emptied.");
    init_db();
    Ok(())
}

pub fn count_clips_for_tag(conn: &Connection, tag_id: &i64) -> Result<i64, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM clip_tags WHERE tag_id = ?"
    )?;
    let count: i64 = stmt.query_row([tag_id], |row| row.get(0))?;
    Ok(count)
}

pub fn load_clip_tags(conn: &Connection) -> Result<HashMap<i64, Vec<String>>> {
    let mut stmt = conn.prepare(
        "SELECT clip_tags.clip_id, tags.name
         FROM clip_tags
         INNER JOIN tags ON clip_tags.tag_id = tags.id
         ORDER BY tags.name ASC"
    )?;

    let mut map: HashMap<i64, Vec<String>> = HashMap::new();

    let rows = stmt.query_map([], |row| {
        let clip_id: i64 = row.get(0)?;
        let tag_name: String = row.get(1)?;
        Ok((clip_id, tag_name))
    })?;

    for row in rows {
        let (clip_id, tag_name) = row?;
        map.entry(clip_id).or_default().push(tag_name);
    }

    Ok(map)
}

/// Returns tags associated with a specific clip by clip_id
pub fn load_tags_for_clip(conn: &Connection, clip_id: i64) -> Result<Vec<(i64, String)>> {
    println!("Loading tags for clip_id: {}", clip_id);

    let mut stmt = conn.prepare(
        "
        SELECT tags.id, tags.name
        FROM tags
        INNER JOIN clip_tags ON tags.id = clip_tags.tag_id
        WHERE clip_tags.clip_id = ?1
        ORDER BY tags.name ASC
        "
    )?;

    let tag_iter = stmt.query_map(params![clip_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,     // tag id
            row.get::<_, String>(1)?,  // tag name
        ))
    })?;

    let mut tags = Vec::new();
    for tag_result in tag_iter {
        match &tag_result {
            Ok((id, name)) => println!("Found tag: {} - '{}'", id, name),
            Err(e) => println!("Error reading tag row: {}", e),
        }
        tags.push(tag_result?);
    }

    println!("Total tags found for clip {}: {}", clip_id, tags.len());
    Ok(tags)
}

pub fn load_tags(conn: &Connection) -> Result<Vec<(i64, String)>> {
    println!("Getting all tags...");
    let mut stmt = match conn.prepare("SELECT id, name FROM tags ORDER BY name ASC") {
        Ok(s) => s,
        Err(e) => {
            println!("Error preparing statement: {}", e);
            return Err(e);
        }
    };

    let tags_iter = match stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?))) {
        Ok(iter) => iter,
        Err(e) => {
            println!("Error querying tags: {}", e);
            return Err(e);
        }
    };

    let mut tags = Vec::new();
    for tag_res in tags_iter {
        match tag_res {
            Ok(tag) => tags.push(tag),
            Err(e) => {
                println!("Error reading tag row: {}", e);
                return Err(e);
            }
        }
    }

    println!("Tags received: {}", tags.len());
    Ok(tags)
}

pub fn load_clips_for_tag(conn: &Connection, tag_id: &i64) -> Result<Vec<(i64, String, i64, bool)>> {
    println!("Loading clips for tag_id: {}", tag_id);
    
    let mut stmt = conn.prepare(
        "
        SELECT clips.id, clips.content, clips.timestamp, clips.pinned
        FROM clips
        INNER JOIN clip_tags ON clips.id = clip_tags.clip_id
        WHERE clip_tags.tag_id = ?1
        ORDER BY clips.pinned DESC, clips.timestamp DESC
        "
    )?;

    let rows = stmt.query_map(params![*tag_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,         // clip id
            row.get::<_, String>(1)?,      // clip content
            row.get::<_, i64>(2)?,         // timestamp
            row.get::<_, i64>(3)? != 0     // pinned as bool
        ))
    })?;

    let mut clips = Vec::new();
    for clip_result in rows {
        match &clip_result {
            Ok((id, content, timestamp, pinned)) => {
                println!(
                    "Loaded clip (ID: {}): '{}', timestamp: '{}', pinned: {}",
                    id, content, timestamp, pinned
                );
            }
            Err(e) => println!("Error loading a clip row: {}", e),
        }
        clips.push(clip_result?);
    }

    println!("Total clips loaded for tag {}: {}", tag_id, clips.len());
    Ok(clips)
}

pub fn create_tag(conn: &Connection, name: &str) -> Result<usize> {
    println!("Creating new tag: {}", name);
    let result = conn.execute(
        "INSERT INTO tags (name) VALUES (?1)",
        rusqlite::params![name],
    );
    match result {
        Ok(_) => println!("created tag"), 
        Err(ref e) => println!("Error creating tag: {}", e),
    }
    result
   
}

pub fn assign_tag_to_clip(conn: &Connection, clip_id: i64, tag_id: i64) -> Result<usize> {
    println!("Assigning tag {} to clip {}", tag_id, clip_id);
    let result = conn.execute(
        "INSERT INTO clip_tags (clip_id, tag_id) VALUES (?1, ?2)",
        rusqlite::params![clip_id, tag_id],
    );
    match result {
        Ok(rows) => println!("Created {} row(s).", rows),
        Err(ref e) => println!("Error assigning tag to clip: {}", e),
    }
    result
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
         LIMIT ?",
    )?;
    let rows = stmt.query_map([limit as i64], |row| {
        Ok((
            row.get::<_, i64>(0)?,      // id
            row.get::<_, String>(1)?,   // content
            row.get::<_, i64>(2)?,      // timestamp
            row.get::<_, i64>(3)? != 0, // pinned (as bool)
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
pub fn load_clips_for_date(
    conn: &Connection,
    date: chrono::NaiveDate,
) -> Result<Vec<(i64, String, i64, bool)>> {
    use chrono::{Local, TimeZone};

    // Create start and end of day in LOCAL timezone
    let start_of_day = date.and_hms_opt(0, 0, 0).unwrap();
    let next_day = date + chrono::Duration::days(1);
    let start_of_next_day = next_day.and_hms_opt(0, 0, 0).unwrap();

    // Convert local times to UTC timestamps
    let start_ts = Local
        .from_local_datetime(&start_of_day)
        .unwrap()
        .timestamp();
    let end_ts = Local
        .from_local_datetime(&start_of_next_day)
        .unwrap()
        .timestamp();

    println!(
        "Loading clips between {} (inclusive) and {} (exclusive)",
        start_ts, end_ts
    );

    let mut stmt = conn.prepare(
        "SELECT id, content, timestamp, pinned FROM clips
         WHERE timestamp >= ?1 AND timestamp < ?2
         ORDER BY pinned DESC, timestamp DESC",
    )?;

    let rows = stmt.query_map(params![start_ts, end_ts], |row| {
        Ok((
            row.get::<_, i64>(0)?,      // id
            row.get::<_, String>(1)?,   // content
            row.get::<_, i64>(2)?,      // timestamp
            row.get::<_, i64>(3)? != 0, // pinned (as bool)
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