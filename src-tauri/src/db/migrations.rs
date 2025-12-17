use rusqlite::{Connection, Result};
use super::schema::{CORE_SCHEMA, TMDB_SCHEMA, SCHEMA_VERSION};

/// Run all database migrations
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Check current schema version
    let current_version = get_schema_version(conn)?;
    
    if current_version < 1 {
        println!("Running migration: v1 - Initial schema");
        migrate_v1(conn)?;
    }
    
    // Future migrations go here
    // if current_version < 2 {
    //     migrate_v2(conn)?;
    // }
    
    Ok(())
}

/// Get the current schema version
fn get_schema_version(conn: &Connection) -> Result<i32> {
    // First check if the schema_version table exists
    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='schema_version'",
        [],
        |row| row.get::<_, i32>(0).map(|count| count > 0),
    )?;
    
    if !table_exists {
        return Ok(0);
    }
    
    // Get the latest version
    let version: Result<i32> = conn.query_row(
        "SELECT MAX(version) FROM schema_version",
        [],
        |row| row.get(0),
    );
    
    match version {
        Ok(v) => Ok(v),
        Err(_) => Ok(0),
    }
}

/// Set the schema version
fn set_schema_version(conn: &Connection, version: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO schema_version (version) VALUES (?1)",
        [version],
    )?;
    Ok(())
}

/// Migration v1: Initial schema
fn migrate_v1(conn: &Connection) -> Result<()> {
    // Execute core schema
    conn.execute_batch(CORE_SCHEMA)?;
    
    // Execute TMDB schema
    conn.execute_batch(TMDB_SCHEMA)?;
    
    // Set schema version
    set_schema_version(conn, 1)?;
    
    // Insert default settings
    insert_default_settings(conn)?;
    
    println!("Migration v1 completed successfully");
    Ok(())
}

/// Insert default application settings
fn insert_default_settings(conn: &Connection) -> Result<()> {
    let default_settings = vec![
        ("theme", "dark"),
        ("playback_speed", "1.0"),
        ("subtitle_enabled", "true"),
        ("subtitle_size", "medium"),
        ("auto_resume", "true"),
        ("completion_threshold", "0.95"),
        ("tmdb_enabled", "false"),
        ("tmdb_api_key", ""),
        ("tmdb_language", "en-US"),
        ("tmdb_image_quality", "original"),
        ("library_paths", "[]"),
        ("auto_scan_interval", "3600"),
    ];
    
    for (key, value) in default_settings {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    #[test]
    fn test_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        
        // Verify schema version
        let version: i32 = conn.query_row(
            "SELECT MAX(version) FROM schema_version",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(version, 1);
        
        // Verify some tables exist
        let tables = vec![
            "media_files",
            "playback_state",
            "playlists",
            "tmdb_media",
            "tmdb_metadata",
        ];
        
        for table in tables {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get::<_, i32>(0).map(|count| count > 0),
            ).unwrap();
            
            assert!(exists, "Table {} should exist", table);
        }
    }
    
    #[test]
    fn test_default_settings() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        
        // Check a default setting
        let theme: String = conn.query_row(
            "SELECT value FROM settings WHERE key='theme'",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(theme, "dark");
    }
}
