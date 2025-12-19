use rusqlite::{Connection, Result, params};
use chrono::Utc;

/// Collection model
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Collection {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub poster_path: Option<String>,
    pub metadata_json: Option<String>,
}

/// Create a new collection
pub fn create_collection(
    conn: &Connection,
    name: &str,
    description: Option<&str>,
) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO collections (name, description, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![name, description, &now, &now],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Get all collections
#[allow(dead_code)]
pub fn get_all_collections(conn: &Connection) -> Result<Vec<Collection>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at, poster_path, metadata_json
         FROM collections
         ORDER BY updated_at DESC"
    )?;
    
    let collections = stmt.query_map([], |row| {
        Ok(Collection {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            poster_path: row.get(5)?,
            metadata_json: row.get(6)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(collections)
}

/// Get a collection by ID
#[allow(dead_code)]
pub fn get_collection_by_id(conn: &Connection, collection_id: i64) -> Result<Option<Collection>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at, poster_path, metadata_json
         FROM collections
         WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query(params![collection_id])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(Collection {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            poster_path: row.get(5)?,
            metadata_json: row.get(6)?,
        }))
    } else {
        Ok(None)
    }
}

/// Update collection details
pub fn update_collection(
    conn: &Connection,
    collection_id: i64,
    name: &str,
    description: Option<&str>,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "UPDATE collections 
         SET name = ?1, description = ?2, updated_at = ?3
         WHERE id = ?4",
        params![name, description, &now, collection_id],
    )?;
    
    Ok(())
}

/// Delete a collection
pub fn delete_collection(conn: &Connection, collection_id: i64) -> Result<()> {
    // Delete collection items first
    conn.execute(
        "DELETE FROM collection_items WHERE collection_id = ?1",
        params![collection_id],
    )?;
    
    // Delete the collection
    conn.execute(
        "DELETE FROM collections WHERE id = ?1",
        params![collection_id],
    )?;
    
    Ok(())
}

/// Add a media file to a collection
pub fn add_media_to_collection(
    conn: &Connection,
    collection_id: i64,
    media_id: i64,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT OR IGNORE INTO collection_items (collection_id, media_id, added_at)
         VALUES (?1, ?2, ?3)",
        params![collection_id, media_id, &now],
    )?;
    
    // Update collection updated_at
    conn.execute(
        "UPDATE collections SET updated_at = ?1 WHERE id = ?2",
        params![&now, collection_id],
    )?;
    
    Ok(())
}

/// Remove a media file from a collection
pub fn remove_media_from_collection(
    conn: &Connection,
    collection_id: i64,
    media_id: i64,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "DELETE FROM collection_items WHERE collection_id = ?1 AND media_id = ?2",
        params![collection_id, media_id],
    )?;
    
    // Update collection updated_at
    conn.execute(
        "UPDATE collections SET updated_at = ?1 WHERE id = ?2",
        params![&now, collection_id],
    )?;
    
    Ok(())
}

/// Get all media files in a collection
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollectionMediaItem {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub media_type: String,
    pub duration: Option<i64>,
    pub added_at: String,
}

pub fn get_collection_media(conn: &Connection, collection_id: i64) -> Result<Vec<CollectionMediaItem>> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.file_path, m.file_name, m.title, m.year, m.media_type, 
                m.duration, ci.added_at
         FROM collection_items ci
         JOIN media_files m ON ci.media_id = m.id
         WHERE ci.collection_id = ?1 AND m.is_deleted = 0
         ORDER BY ci.added_at DESC"
    )?;
    
    let items = stmt.query_map(params![collection_id], |row| {
        Ok(CollectionMediaItem {
            id: row.get(0)?,
            file_path: row.get(1)?,
            file_name: row.get(2)?,
            title: row.get(3)?,
            year: row.get(4)?,
            media_type: row.get(5)?,
            duration: row.get(6)?,
            added_at: row.get(7)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(items)
}

/// Get collections with item count
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollectionWithCount {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub item_count: i32,
}

pub fn get_collections_with_counts(conn: &Connection) -> Result<Vec<CollectionWithCount>> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.name, c.description, c.created_at, c.updated_at,
                COUNT(ci.media_id) as item_count
         FROM collections c
         LEFT JOIN collection_items ci ON c.id = ci.collection_id
         GROUP BY c.id
         ORDER BY c.updated_at DESC"
    )?;
    
    let collections = stmt.query_map([], |row| {
        Ok(CollectionWithCount {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            item_count: row.get(5)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(collections)
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
            file_hash: "testhash456".to_string(),
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
            is_locked: false,
        };
        add_media_file(conn, &media)
    }

    #[test]
    fn test_collection_crud() -> Result<()> {
        let conn = init_db()?;
        
        // Create collection
        let collection_id = create_collection(
            &conn,
            "Marvel Movies",
            Some("All Marvel Cinematic Universe films"),
        )?;
        
        assert!(collection_id > 0);
        
        // Get collection
        let collection = get_collection_by_id(&conn, collection_id)?;
        assert!(collection.is_some());
        assert_eq!(collection.unwrap().name, "Marvel Movies");
        
        // Update collection
        update_collection(&conn, collection_id, "MCU Collection", Some("Updated desc"))?;
        let updated = get_collection_by_id(&conn, collection_id)?;
        assert_eq!(updated.unwrap().name, "MCU Collection");
        
        // Delete collection
        delete_collection(&conn, collection_id)?;
        let deleted = get_collection_by_id(&conn, collection_id)?;
        assert!(deleted.is_none());
        
        Ok(())
    }

    #[test]
    fn test_collection_items() -> Result<()> {
        let conn = init_db()?;
        
        // Create test data
        let media_id = create_test_media(&conn)?;
        let collection_id = create_collection(
            &conn,
            "Test Collection",
            None,
        )?;
        
        // Add media to collection
        add_media_to_collection(&conn, collection_id, media_id)?;
        
        // Get collection media
        let items = get_collection_media(&conn, collection_id)?;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, media_id);
        
        // Remove media from collection
        remove_media_from_collection(&conn, collection_id, media_id)?;
        let items_after = get_collection_media(&conn, collection_id)?;
        assert_eq!(items_after.len(), 0);
        
        Ok(())
    }
}
