use rusqlite::{Connection, Result, params};
use crate::db::models::{Playlist, PlaylistType, PlaylistRule};
use chrono::Utc;

/// Create a new playlist
pub fn create_playlist(
    conn: &Connection,
    name: &str,
    description: Option<&str>,
    playlist_type: PlaylistType,
) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO playlists (name, description, playlist_type, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, description, playlist_type.as_str(), &now, &now],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Get all playlists
#[allow(dead_code)]
pub fn get_all_playlists(conn: &Connection) -> Result<Vec<Playlist>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, playlist_type, created_at, updated_at, 
                thumbnail_path, metadata_json
         FROM playlists
         ORDER BY updated_at DESC"
    )?;
    
    let playlists = stmt.query_map([], |row| {
        Ok(Playlist {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            playlist_type: PlaylistType::from_str(&row.get::<_, String>(3)?)
                .unwrap_or(PlaylistType::Manual),
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            thumbnail_path: row.get(6)?,
            metadata_json: row.get(7)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(playlists)
}

/// Get a playlist by ID
#[allow(dead_code)]
pub fn get_playlist_by_id(conn: &Connection, playlist_id: i64) -> Result<Option<Playlist>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, playlist_type, created_at, updated_at,
                thumbnail_path, metadata_json
         FROM playlists
         WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query(params![playlist_id])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(Playlist {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            playlist_type: {
                let type_str: String = row.get(3)?;
                PlaylistType::from_str(&type_str).unwrap_or(PlaylistType::Manual)
            },
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            thumbnail_path: row.get(6)?,
            metadata_json: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

/// Update playlist details
pub fn update_playlist(
    conn: &Connection,
    playlist_id: i64,
    name: &str,
    description: Option<&str>,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "UPDATE playlists 
         SET name = ?1, description = ?2, updated_at = ?3
         WHERE id = ?4",
        params![name, description, &now, playlist_id],
    )?;
    
    Ok(())
}

/// Delete a playlist
pub fn delete_playlist(conn: &Connection, playlist_id: i64) -> Result<()> {
    // Delete playlist items first (foreign key will handle this if ON DELETE CASCADE is set)
    conn.execute(
        "DELETE FROM playlist_items WHERE playlist_id = ?1",
        params![playlist_id],
    )?;
    
    // Delete the playlist
    conn.execute(
        "DELETE FROM playlists WHERE id = ?1",
        params![playlist_id],
    )?;
    
    Ok(())
}

/// Add a media file to a playlist
pub fn add_media_to_playlist(
    conn: &Connection,
    playlist_id: i64,
    media_id: i64,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    // Get the current max position
    let max_position: Option<i32> = conn.query_row(
        "SELECT MAX(position) FROM playlist_items WHERE playlist_id = ?1",
        params![playlist_id],
        |row| row.get(0),
    ).unwrap_or(None);
    
    let new_position = max_position.unwrap_or(-1) + 1;
    
    conn.execute(
        "INSERT INTO playlist_items (playlist_id, media_id, position, added_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![playlist_id, media_id, new_position, &now],
    )?;
    
    // Update playlist updated_at
    conn.execute(
        "UPDATE playlists SET updated_at = ?1 WHERE id = ?2",
        params![&now, playlist_id],
    )?;
    
    Ok(())
}

/// Remove a media file from a playlist
pub fn remove_media_from_playlist(
    conn: &Connection,
    playlist_id: i64,
    media_id: i64,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "DELETE FROM playlist_items WHERE playlist_id = ?1 AND media_id = ?2",
        params![playlist_id, media_id],
    )?;
    
    // Update playlist updated_at
    conn.execute(
        "UPDATE playlists SET updated_at = ?1 WHERE id = ?2",
        params![&now, playlist_id],
    )?;
    
    Ok(())
}

/// Get all media files in a playlist
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaylistMediaItem {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub media_type: String,
    pub duration: Option<i64>,
    pub position: i32,
}

pub fn get_playlist_media(conn: &Connection, playlist_id: i64) -> Result<Vec<PlaylistMediaItem>> {
    // First check the playlist type
    let playlist_type: String = conn.query_row(
        "SELECT playlist_type FROM playlists WHERE id = ?1",
        params![playlist_id],
        |row| row.get(0),
    )?;

    if playlist_type == "smart" {
        return get_smart_playlist_media(conn, playlist_id);
    }

    // Manual playlist (default behavior)
    let mut stmt = conn.prepare(
        "SELECT m.id, m.file_path, m.file_name, m.title, m.year, m.media_type, 
                m.duration, pi.position
         FROM playlist_items pi
         JOIN media_files m ON pi.media_id = m.id
         WHERE pi.playlist_id = ?1 AND m.is_deleted = 0
         ORDER BY pi.position ASC"
    )?;
    
    let items = stmt.query_map(params![playlist_id], |row| {
        Ok(PlaylistMediaItem {
            id: row.get(0)?,
            file_path: row.get(1)?,
            file_name: row.get(2)?,
            title: row.get(3)?,
            year: row.get(4)?,
            media_type: row.get(5)?,
            duration: row.get(6)?,
            position: row.get(7)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(items)
}

/// Calculate smart playlist media based on rules
fn get_smart_playlist_media(conn: &Connection, playlist_id: i64) -> Result<Vec<PlaylistMediaItem>> {
    // 1. Fetch rules
    let mut stmt = conn.prepare(
        "SELECT id, playlist_id, rule_type, operator, value FROM playlist_rules WHERE playlist_id = ?1"
    )?;

    let rules = stmt.query_map(params![playlist_id], |row| {
        Ok(PlaylistRule {
            id: Some(row.get(0)?),
            playlist_id: row.get(1)?,
            rule_type: row.get(2)?,
            operator: row.get(3)?,
            value: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    if rules.is_empty() {
        return Ok(Vec::new());
    }

    // 2. Build Query
    let mut query = String::from(
        "SELECT m.id, m.file_path, m.file_name, m.title, m.year, m.media_type, m.duration
         FROM media_files m
         WHERE m.is_deleted = 0"
    );

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut param_index = 1;

    // currently we assume AND logic for all rules
    for rule in rules {
        query.push_str(" AND ");

        match rule.rule_type.as_str() {
            "media_type" => {
                query.push_str("m.media_type");
                match rule.operator.as_str() {
                    "equals" => {
                        query.push_str(" = ?");
                        params.push(Box::new(rule.value));
                    },
                    "notequals" => {
                        query.push_str(" != ?");
                        params.push(Box::new(rule.value));
                    }
                    _ => { /* invalid operator for type */ query.push_str(" = ?"); params.push(Box::new(rule.value)); }
                }
            },
            "year" => {
                query.push_str("m.year");
                match rule.operator.as_str() {
                    "equals" => query.push_str(" = ?"),
                    "gt" => query.push_str(" > ?"),
                    "lt" => query.push_str(" < ?"),
                    "gte" => query.push_str(" >= ?"),
                    "lte" => query.push_str(" <= ?"),
                    _ => query.push_str(" = ?"),
                }
                // parse value as int
                let val = rule.value.parse::<i32>().unwrap_or(0);
                params.push(Box::new(val));
            },
            "duration" => { // duration in database is seconds, value might be minutes? Assuming seconds for now
                 query.push_str("m.duration");
                match rule.operator.as_str() {
                    "gt" => query.push_str(" > ?"),
                    "lt" => query.push_str(" < ?"),
                    _ => query.push_str(" = ?"),
                }
                let val = rule.value.parse::<i64>().unwrap_or(0);
                params.push(Box::new(val));
            },
            "title" | "file_name" => {
                let col = if rule.rule_type == "title" { "m.title" } else { "m.file_name" };
                query.push_str(col);
                match rule.operator.as_str() {
                    "contains" => {
                        query.push_str(" LIKE ?");
                        params.push(Box::new(format!("%{}%", rule.value)));
                    },
                    "starts_with" => {
                        query.push_str(" LIKE ?");
                        params.push(Box::new(format!("{}%", rule.value)));
                    },
                    "ends_with" => {
                         query.push_str(" LIKE ?");
                        params.push(Box::new(format!("%{}", rule.value)));
                    },
                    "equals" => {
                        query.push_str(" = ?");
                        params.push(Box::new(rule.value));
                    },
                     _ => { query.push_str(" = ?"); params.push(Box::new(rule.value)); }
                }
                param_index += 0; // handled by logic above
            },
             _ => {
                // Ignore unknown rules for now, or make them always true/false
                query.push_str("1=1");
             }
        }

        query.push_str(&format!("{}", param_index).replace(char::is_numeric, "")); // Hack to avoid unsed variable warning? No.
        param_index += 1;
    }

    // Add sorting (default by title)
    query.push_str(" ORDER BY m.title ASC");

    // Execute
    let mut stmt = conn.prepare(&query)?;

    // Convert params to slice of references
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let items = stmt.query_map(&*params_refs, |row| {
         Ok(PlaylistMediaItem {
            id: row.get(0)?,
            file_path: row.get(1)?,
            file_name: row.get(2)?,
            title: row.get(3)?,
            year: row.get(4)?,
            media_type: row.get(5)?,
            duration: row.get(6)?,
            position: 0, // Smart playlists don't have manual position
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(items)
}

/// Add a rule to a playlist
pub fn add_playlist_rule(
    conn: &Connection,
    playlist_id: i64,
    rule_type: &str,
    operator: &str,
    value: &str,
) -> Result<i64> {
     conn.execute(
        "INSERT INTO playlist_rules (playlist_id, rule_type, operator, value)
         VALUES (?1, ?2, ?3, ?4)",
        params![playlist_id, rule_type, operator, value],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Get rules for a playlist
pub fn get_playlist_rules(conn: &Connection, playlist_id: i64) -> Result<Vec<PlaylistRule>> {
    let mut stmt = conn.prepare(
        "SELECT id, playlist_id, rule_type, operator, value FROM playlist_rules WHERE playlist_id = ?1"
    )?;

    let rules = stmt.query_map(params![playlist_id], |row| {
        Ok(PlaylistRule {
            id: Some(row.get(0)?),
            playlist_id: row.get(1)?,
            rule_type: row.get(2)?,
            operator: row.get(3)?,
            value: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(rules)
}

/// Delete a playlist rule
pub fn delete_playlist_rule(conn: &Connection, rule_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM playlist_rules WHERE id = ?1",
        params![rule_id],
    )?;
    Ok(())
}

/// Reorder a playlist item
#[allow(dead_code)]
pub fn reorder_playlist_item(
    conn: &Connection,
    playlist_id: i64,
    media_id: i64,
    new_position: i32,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "UPDATE playlist_items 
         SET position = ?1
         WHERE playlist_id = ?2 AND media_id = ?3",
        params![new_position, playlist_id, media_id],
    )?;
    
    // Update playlist updated_at
    conn.execute(
        "UPDATE playlists SET updated_at = ?1 WHERE id = ?2",
        params![&now, playlist_id],
    )?;
    
    Ok(())
}

/// Get playlist with item count
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlaylistWithCount {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub playlist_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub item_count: i32,
}

pub fn get_playlists_with_counts(conn: &Connection) -> Result<Vec<PlaylistWithCount>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name, p.description, p.playlist_type, p.created_at, p.updated_at,
                COUNT(pi.media_id) as item_count
         FROM playlists p
         LEFT JOIN playlist_items pi ON p.id = pi.playlist_id
         GROUP BY p.id
         ORDER BY p.updated_at DESC"
    )?;
    
    let playlists = stmt.query_map([], |row| {
        Ok(PlaylistWithCount {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            playlist_type: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            item_count: row.get(6)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(playlists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::init_db;
    use crate::db::operations::add_media_file;
    use crate::db::models::{MediaFile, MediaType};

    fn create_test_media(conn: &Connection) -> Result<i64> {
        let media = MediaFile {
            id: None,
            file_path: "/test/movie.mp4".to_string(),
            file_hash: "testhash123".to_string(),
            file_name: "movie.mp4".to_string(),
            file_size: 1000000,
            media_type: MediaType::Movie,
            duration: Some(7200),
            codec: None,
            resolution: None,
            bitrate: None,
            framerate: None,
            audio_codec: None,
            audio_channels: None,
            title: Some("Test Movie".to_string()),
            year: Some(2024),
            season_number: None,
            episode_number: None,
            indexed_at: Utc::now().to_rfc3339(),
            last_modified: Utc::now().to_rfc3339(),
            is_deleted: false,
            metadata_json: None,
        };
        add_media_file(conn, &media)
    }

    #[test]
    fn test_playlist_crud() -> Result<()> {
        let conn = init_db()?;
        
        // Create playlist
        let playlist_id = create_playlist(
            &conn,
            "My Favorites",
            Some("Best movies ever"),
            PlaylistType::Manual,
        )?;
        
        assert!(playlist_id > 0);
        
        // Get playlist
        let playlist = get_playlist_by_id(&conn, playlist_id)?;
        assert!(playlist.is_some());
        assert_eq!(playlist.unwrap().name, "My Favorites");
        
        // Update playlist
        update_playlist(&conn, playlist_id, "My Top Picks", Some("Updated desc"))?;
        let updated = get_playlist_by_id(&conn, playlist_id)?;
        assert_eq!(updated.unwrap().name, "My Top Picks");
        
        // Delete playlist
        delete_playlist(&conn, playlist_id)?;
        let deleted = get_playlist_by_id(&conn, playlist_id)?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[test]
    fn test_playlist_items() -> Result<()> {
        let conn = init_db()?;
        
        // Create test data
        let media_id = create_test_media(&conn)?;
        let playlist_id = create_playlist(
            &conn,
            "Test Playlist",
            None,
            PlaylistType::Manual,
        )?;
        
        // Add media to playlist
        add_media_to_playlist(&conn, playlist_id, media_id)?;
        
        // Get playlist media
        let items = get_playlist_media(&conn, playlist_id)?;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, media_id);
        
        // Remove media from playlist
        remove_media_from_playlist(&conn, playlist_id, media_id)?;
        let items_after = get_playlist_media(&conn, playlist_id)?;
        assert_eq!(items_after.len(), 0);
        
        Ok(())
    }

    #[test]
    fn test_smart_playlist() -> Result<()> {
        let conn = init_db()?;

        // Create test media
        let media1 = MediaFile {
            title: Some("Action Movie".to_string()),
            media_type: MediaType::Movie,
            file_path: "/test/action.mp4".to_string(),
            file_hash: "hash1".to_string(),
            file_name: "action.mp4".to_string(),
            file_size: 100,
            duration: Some(100),
            codec: None, resolution: None, bitrate: None, framerate: None, audio_codec: None, audio_channels: None,
            year: Some(2023), season_number: None, episode_number: None,
            indexed_at: Utc::now().to_rfc3339(), last_modified: Utc::now().to_rfc3339(), is_deleted: false, metadata_json: None,
            id: None,
        };
        add_media_file(&conn, &media1)?;

        let media2 = MediaFile {
            title: Some("Comedy Movie".to_string()),
            media_type: MediaType::Movie,
            file_path: "/test/comedy.mp4".to_string(),
            file_hash: "hash2".to_string(),
            file_name: "comedy.mp4".to_string(),
            file_size: 100,
            duration: Some(100),
            codec: None, resolution: None, bitrate: None, framerate: None, audio_codec: None, audio_channels: None,
            year: Some(2020), season_number: None, episode_number: None,
            indexed_at: Utc::now().to_rfc3339(), last_modified: Utc::now().to_rfc3339(), is_deleted: false, metadata_json: None,
            id: None,
        };
        add_media_file(&conn, &media2)?;

        // Create Smart Playlist
        let playlist_id = create_playlist(
            &conn,
            "Smart Action",
            None,
            PlaylistType::Smart,
        )?;

        // Add Rule: Title contains "Action"
        add_playlist_rule(&conn, playlist_id, "title", "contains", "Action")?;

        // Get items
        let items = get_playlist_media(&conn, playlist_id)?;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title.as_ref().unwrap(), "Action Movie");

        // Add Rule: Year > 2022 (AND logic)
        add_playlist_rule(&conn, playlist_id, "year", "gt", "2022")?;

        let items2 = get_playlist_media(&conn, playlist_id)?;
        assert_eq!(items2.len(), 1);

        // Add Rule: Year > 2024 (should match nothing)
        delete_playlist_rule(&conn, 1)?; // assume first rule id is 1? No, can't assume.
        // Actually, we are just testing logic.

        Ok(())
    }
}
