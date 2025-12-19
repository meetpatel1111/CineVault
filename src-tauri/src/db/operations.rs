use rusqlite::{Connection, Result, params};
use crate::db::models::{MediaFile, MediaType};

/// Insert or update a media file in the database
pub fn upsert_media_file(conn: &Connection, media: &MediaFile) -> Result<i64> {
    let media_type_str = media.media_type.as_str();
    let indexed_at = &media.indexed_at;
    let last_modified = &media.last_modified;
    
    conn.execute(
        "INSERT INTO media_files (
            file_path, file_hash, file_name, file_size, media_type,
            duration, codec, resolution, bitrate, framerate,
            audio_codec, audio_channels,
            title, year, season_number, episode_number,
            indexed_at, last_modified, is_deleted, metadata_json
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5,
            ?6, ?7, ?8, ?9, ?10,
            ?11, ?12,
            ?13, ?14, ?15, ?16,
            ?17, ?18, ?19, ?20
        )
        ON CONFLICT(file_path) DO UPDATE SET
            file_hash = excluded.file_hash,
            file_size = excluded.file_size,
            duration = excluded.duration,
            codec = excluded.codec,
            resolution = excluded.resolution,
            bitrate = excluded.bitrate,
            framerate = excluded.framerate,
            audio_codec = excluded.audio_codec,
            audio_channels = excluded.audio_channels,
            title = excluded.title,
            year = excluded.year,
            season_number = excluded.season_number,
            episode_number = excluded.episode_number,
            last_modified = excluded.last_modified,
            is_deleted = 0,
            metadata_json = excluded.metadata_json",
        params![
            &media.file_path,
            &media.file_hash,
            &media.file_name,
            media.file_size,
            media_type_str,
            media.duration,
            &media.codec,
            &media.resolution,
            media.bitrate,
            media.framerate,
            &media.audio_codec,
            media.audio_channels,
            &media.title,
            media.year,
            media.season_number,
            media.episode_number,
            indexed_at,
            last_modified,
            media.is_deleted as i32,
            &media.metadata_json,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Get all media files
pub fn get_all_media_files(conn: &Connection) -> Result<Vec<MediaFile>> {
    let mut stmt = conn.prepare(
        "SELECT 
            id, file_path, file_hash, file_name, file_size, media_type,
            duration, codec, resolution, bitrate, framerate,
            audio_codec, audio_channels,
            title, year, season_number, episode_number,
            indexed_at, last_modified, is_deleted, metadata_json
        FROM media_files
        WHERE is_deleted = 0
        ORDER BY indexed_at DESC"
    )?;

    let media_iter = stmt.query_map([], |row| {
        let media_type_str: String = row.get(5)?;
        let media_type = MediaType::from_str(&media_type_str)
            .unwrap_or(MediaType::Video);
        
        Ok(MediaFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            file_hash: row.get(2)?,
            file_name: row.get(3)?,
            file_size: row.get(4)?,
            media_type,
            duration: row.get(6)?,
            codec: row.get(7)?,
            resolution: row.get(8)?,
            bitrate: row.get(9)?,
            framerate: row.get(10)?,
            audio_codec: row.get(11)?,
            audio_channels: row.get(12)?,
            title: row.get(13)?,
            year: row.get(14)?,
            season_number: row.get(15)?,
            episode_number: row.get(16)?,
            indexed_at: row.get(17)?,
            last_modified: row.get(18)?,
            is_deleted: row.get::<_, i32>(19)? != 0,
            metadata_json: row.get(20)?,
        })
    })?;

    media_iter.collect()
}

#[cfg(test)]
/// Add a media file to the database (test helper)
pub fn add_media_file(conn: &Connection, media: &MediaFile) -> Result<i64> {
    upsert_media_file(conn, media)
}

/// Get media files by type
pub fn get_media_by_type(conn: &Connection, media_type: MediaType) -> Result<Vec<MediaFile>> {
    let media_type_str = media_type.as_str();
    
    let mut stmt = conn.prepare(
        "SELECT 
            id, file_path, file_hash, file_name, file_size, media_type,
            duration, codec, resolution, bitrate, framerate,
            audio_codec, audio_channels,
            title, year, season_number, episode_number,
            indexed_at, last_modified, is_deleted, metadata_json
        FROM media_files
        WHERE is_deleted = 0 AND media_type = ?1
        ORDER BY indexed_at DESC"
    )?;

    let media_iter = stmt.query_map([media_type_str], |row| {
        Ok(MediaFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            file_hash: row.get(2)?,
            file_name: row.get(3)?,
            file_size: row.get(4)?,
            media_type: media_type.clone(),
            duration: row.get(6)?,
            codec: row.get(7)?,
            resolution: row.get(8)?,
            bitrate: row.get(9)?,
            framerate: row.get(10)?,
            audio_codec: row.get(11)?,
            audio_channels: row.get(12)?,
            title: row.get(13)?,
            year: row.get(14)?,
            season_number: row.get(15)?,
            episode_number: row.get(16)?,
            indexed_at: row.get(17)?,
            last_modified: row.get(18)?,
            is_deleted: row.get::<_, i32>(19)? != 0,
            metadata_json: row.get(20)?,
        })
    })?;

    media_iter.collect()
}

/// Search media files by title
pub fn search_media(conn: &Connection, query: &str) -> Result<Vec<MediaFile>> {
    let search_query = format!("%{}%", query);
    
    let mut stmt = conn.prepare(
        "SELECT 
            id, file_path, file_hash, file_name, file_size, media_type,
            duration, codec, resolution, bitrate, framerate,
            audio_codec, audio_channels,
            title, year, season_number, episode_number,
            indexed_at, last_modified, is_deleted, metadata_json
        FROM media_files
        WHERE is_deleted = 0 AND (title LIKE ?1 OR file_name LIKE ?1)
        ORDER BY indexed_at DESC"
    )?;

    let media_iter = stmt.query_map([&search_query], |row| {
        let media_type_str: String = row.get(5)?;
        let media_type = MediaType::from_str(&media_type_str)
            .unwrap_or(MediaType::Video);
        
        Ok(MediaFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            file_hash: row.get(2)?,
            file_name: row.get(3)?,
            file_size: row.get(4)?,
            media_type,
            duration: row.get(6)?,
            codec: row.get(7)?,
            resolution: row.get(8)?,
            bitrate: row.get(9)?,
            framerate: row.get(10)?,
            audio_codec: row.get(11)?,
            audio_channels: row.get(12)?,
            title: row.get(13)?,
            year: row.get(14)?,
            season_number: row.get(15)?,
            episode_number: row.get(16)?,
            indexed_at: row.get(17)?,
            last_modified: row.get(18)?,
            is_deleted: row.get::<_, i32>(19)? != 0,
            metadata_json: row.get(20)?,
        })
    })?;

    media_iter.collect()
}

/// Filter media files by criteria
pub fn filter_media(conn: &Connection, criteria: &crate::db::models::FilterCriteria) -> Result<Vec<MediaFile>> {
    let mut query = String::from(
        "SELECT
            id, file_path, file_hash, file_name, file_size, media_type,
            duration, codec, resolution, bitrate, framerate,
            audio_codec, audio_channels,
            title, year, season_number, episode_number,
            indexed_at, last_modified, is_deleted, metadata_json
        FROM media_files
        WHERE is_deleted = 0"
    );

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut param_idx = 1;

    // Year range
    if let Some(min_year) = criteria.min_year {
        query.push_str(&format!(" AND year >= ?{}", param_idx));
        params.push(Box::new(min_year));
        param_idx += 1;
    }
    if let Some(max_year) = criteria.max_year {
        query.push_str(&format!(" AND year <= ?{}", param_idx));
        params.push(Box::new(max_year));
        param_idx += 1;
    }

    // Duration range
    if let Some(min_duration) = criteria.min_duration {
        query.push_str(&format!(" AND duration >= ?{}", param_idx));
        params.push(Box::new(min_duration));
        param_idx += 1;
    }
    if let Some(max_duration) = criteria.max_duration {
        query.push_str(&format!(" AND duration <= ?{}", param_idx));
        params.push(Box::new(max_duration));
        param_idx += 1;
    }

    // Codecs
    if let Some(codecs) = &criteria.codecs {
        if !codecs.is_empty() {
            query.push_str(" AND codec IN (");
            for (i, codec) in codecs.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                query.push_str(&format!("?{}", param_idx));
                params.push(Box::new(codec.clone()));
                param_idx += 1;
            }
            query.push(')');
        }
    }

    // Media Types
    if let Some(types) = &criteria.media_types {
        if !types.is_empty() {
            query.push_str(" AND media_type IN (");
            for (i, mtype) in types.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                // Convert frontend type strings to backend strings if needed,
                // but usually they match or we can map them.
                // Assuming simple mapping or direct match for now.
                // The frontend sends "Movie", "TvEpisode", "Music" which match MediaType enum variants string representation (mostly)
                // Actually enum variants are TitleCase usually, let's ensure we match what's in DB
                // DB stores: 'movie', 'tv_episode', 'music', 'video', 'audio' (lowercase snake_case usually from MediaType::as_str())

                let db_type = match mtype.as_str() {
                    "Movie" => "movie",
                    "TvEpisode" => "tv_episode",
                    "Music" => "music",
                    "Video" => "video",
                    "Audio" => "audio",
                    _ => mtype, // Fallback
                };

                query.push_str(&format!("?{}", param_idx));
                params.push(Box::new(db_type.to_string()));
                param_idx += 1;
            }
            query.push(')');
        }
    }

    // Resolutions (this is trickier as it's text search usually)
    if let Some(resolutions) = &criteria.resolutions {
        if !resolutions.is_empty() {
            query.push_str(" AND (");
            for (i, res) in resolutions.iter().enumerate() {
                if i > 0 {
                    query.push_str(" OR ");
                }
                match res.as_str() {
                    "4k" => {
                        query.push_str(&format!("(resolution LIKE ?{} OR resolution LIKE ?{})", param_idx, param_idx + 1));
                        params.push(Box::new("%2160p%".to_string()));
                        params.push(Box::new("%4K%".to_string())); // Some might say 4K
                        param_idx += 2;
                    },
                    "1080p" => {
                        query.push_str(&format!("resolution LIKE ?{}", param_idx));
                        params.push(Box::new("%1080p%".to_string()));
                        param_idx += 1;
                    },
                    "720p" => {
                        query.push_str(&format!("resolution LIKE ?{}", param_idx));
                        params.push(Box::new("%720p%".to_string()));
                        param_idx += 1;
                    },
                    "sd" => {
                         // Simplify SD as anything not 720p/1080p/2160p? Or explicit 480p/576p
                         query.push_str(&format!("(resolution LIKE ?{} OR resolution LIKE ?{})", param_idx, param_idx + 1));
                         params.push(Box::new("%480p%".to_string()));
                         params.push(Box::new("%576p%".to_string()));
                         param_idx += 2;
                    },
                    _ => {
                        // Generic match
                         query.push_str(&format!("resolution LIKE ?{}", param_idx));
                         params.push(Box::new(format!("%{}%", res)));
                         param_idx += 1;
                    }
                }
            }
            query.push(')');
        }
    }

    query.push_str(" ORDER BY indexed_at DESC");

    let mut stmt = conn.prepare(&query)?;

    // We need to convert our Vec<Box<dyn ToSql>> to a slice of references
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let media_iter = stmt.query_map(params_refs.as_slice(), |row| {
        let media_type_str: String = row.get(5)?;
        let media_type = MediaType::from_str(&media_type_str)
            .unwrap_or(MediaType::Video);

        Ok(MediaFile {
            id: Some(row.get(0)?),
            file_path: row.get(1)?,
            file_hash: row.get(2)?,
            file_name: row.get(3)?,
            file_size: row.get(4)?,
            media_type,
            duration: row.get(6)?,
            codec: row.get(7)?,
            resolution: row.get(8)?,
            bitrate: row.get(9)?,
            framerate: row.get(10)?,
            audio_codec: row.get(11)?,
            audio_channels: row.get(12)?,
            title: row.get(13)?,
            year: row.get(14)?,
            season_number: row.get(15)?,
            episode_number: row.get(16)?,
            indexed_at: row.get(17)?,
            last_modified: row.get(18)?,
            is_deleted: row.get::<_, i32>(19)? != 0,
            metadata_json: row.get(20)?,
        })
    })?;

    media_iter.collect()
}

/// Mark files not in the provided list as deleted (for cleanup after scan)
#[allow(dead_code)]
pub fn mark_missing_files(conn: &Connection, existing_paths: &[String]) -> Result<usize> {
    // Build placeholders for SQL IN clause
    if existing_paths.is_empty() {
        // Mark all as deleted
        return conn.execute(
            "UPDATE media_files SET is_deleted = 1 WHERE is_deleted = 0",
            [],
        );
    }

    let placeholders = existing_paths.iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");
    
    let query = format!(
        "UPDATE media_files SET is_deleted = 1 
         WHERE is_deleted = 0 AND file_path NOT IN ({})",
        placeholders
    );

    let params: Vec<&dyn rusqlite::ToSql> = existing_paths
        .iter()
        .map(|s| s as &dyn rusqlite::ToSql)
        .collect();

    conn.execute(&query, params.as_slice())
}

/// Get library statistics
pub fn get_library_stats(conn: &Connection) -> Result<LibraryStats> {
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM media_files WHERE is_deleted = 0",
        [],
        |row| row.get(0),
    )?;

    let movies: i64 = conn.query_row(
        "SELECT COUNT(*) FROM media_files WHERE is_deleted = 0 AND media_type = 'movie'",
        [],
        |row| row.get(0),
    )?;

    let tv_episodes: i64 = conn.query_row(
        "SELECT COUNT(*) FROM media_files WHERE is_deleted = 0 AND media_type = 'tv_episode'",
        [],
        |row| row.get(0),
    )?;

    let music: i64 = conn.query_row(
        "SELECT COUNT(*) FROM media_files WHERE is_deleted = 0 AND media_type = 'music'",
        [],
        |row| row.get(0),
    )?;

    let total_size: i64 = conn.query_row(
        "SELECT COALESCE(SUM(file_size), 0) FROM media_files WHERE is_deleted = 0",
        [],
        |row| row.get(0),
    )?;

    Ok(LibraryStats {
        total: total as usize,
        movies: movies as usize,
        tv_episodes: tv_episodes as usize,
        music: music as usize,
        total_size: total_size as u64,
    })
}

#[derive(Debug, serde::Serialize)]
pub struct LibraryStats {
    pub total: usize,
    pub movies: usize,
    pub tv_episodes: usize,
    pub music: usize,
    pub total_size: u64,
}
