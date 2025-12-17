use rusqlite::{Connection, Result, params};
use crate::db::models::{Playlist, PlaylistType};
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

/// Reorder a playlist item
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
}
