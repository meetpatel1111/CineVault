#[cfg(test)]
mod tests {
    use super::super::*;
    use std::path::PathBuf;

    #[test]
    fn test_database_creation() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        // Verify schema version
        let version: i32 = conn.query_row(
            "SELECT MAX(version) FROM schema_version",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(version, 2);
    }

    #[test]
    fn test_core_tables_exist() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        let tables = vec![
            "media_files",
            "playback_state",
            "playback_history",
            "playlists",
            "playlist_items",
            "playlist_rules",
            "collections",
            "collection_items",
            "subtitle_tracks",
            "audio_tracks",
            "settings",
        ];
        
        for table in tables {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get::<_, i32>(0).map(|count| count > 0),
            ).unwrap();
            
            assert!(exists, "Core table '{}' should exist", table);
        }
    }

    #[test]
    fn test_tmdb_tables_exist() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        let tables = vec![
            "tmdb_media",
            "tmdb_metadata",
            "tmdb_cast",
            "tmdb_images",
            "tmdb_tv_shows",
            "tmdb_seasons",
            "tmdb_episodes",
            "tmdb_collections",
            "tmdb_collection_items",
            "tmdb_settings",
        ];
        
        for table in tables {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get::<_, i32>(0).map(|count| count > 0),
            ).unwrap();
            
            assert!(exists, "TMDB table '{}' should exist", table);
        }
    }

    #[test]
    fn test_default_settings() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        // Check default settings
        let settings = vec![
            ("theme", "dark"),
            ("playback_speed", "1.0"),
            ("tmdb_enabled", "false"),
            ("auto_resume", "true"),
        ];
        
        for (key, expected_value) in settings {
            let value: String = conn.query_row(
                "SELECT value FROM settings WHERE key=?1",
                [key],
                |row| row.get(0),
            ).unwrap();
            
            assert_eq!(value, expected_value, "Setting '{}' should be '{}'", key, expected_value);
        }
    }

    #[test]
    fn test_insert_media_file() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        conn.execute(
            "INSERT INTO media_files (file_path, file_hash, file_name, file_size, media_type, last_modified)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
            [
                "/path/to/movie.mp4",
                "abc123hash",
                "movie.mp4",
                "1024000000",
                "movie",
            ],
        ).unwrap();
        
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM media_files",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(count, 1);
    }

    #[test]
    fn test_foreign_key_cascade() {
        let db = Database::new(PathBuf::from(":memory:")).unwrap();
        db.migrate().unwrap();
        
        let conn = db.connection();
        let conn = conn.lock().unwrap();
        
        // Insert a media file
        conn.execute(
            "INSERT INTO media_files (file_path, file_hash, file_name, file_size, media_type, last_modified)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
            ["/path/to/movie.mp4", "abc123", "movie.mp4", "1024000", "movie"],
        ).unwrap();
        
        let media_id: i64 = conn.last_insert_rowid();
        
        // Insert playback state
        conn.execute(
            "INSERT INTO playback_state (media_id, last_position) VALUES (?1, ?2)",
            [media_id.to_string(), "300".to_string()],
        ).unwrap();
        
        // Verify playback state exists
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM playback_state WHERE media_id=?1",
            [media_id],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);
        
        // Delete media file
        conn.execute("DELETE FROM media_files WHERE id=?1", [media_id]).unwrap();
        
        // Verify playback state was cascaded
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM playback_state WHERE media_id=?1",
            [media_id],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 0, "Playback state should be deleted via CASCADE");
    }
}
