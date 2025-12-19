use rusqlite::{Connection, Result, params};
use chrono::Utc;

/// Subtitle track model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubtitleTrack {
    pub id: Option<i64>,
    pub media_id: i64,
    pub file_path: String,
    pub language: Option<String>,
    pub label: Option<String>,
    pub codec: Option<String>,
    pub is_embedded: bool,
    pub track_index: Option<i32>,
    pub added_at: String,
}

/// Add a subtitle track
#[allow(clippy::too_many_arguments)]
pub fn add_subtitle_track(
    conn: &Connection,
    media_id: i64,
    file_path: &str,
    language: Option<&str>,
    label: Option<&str>,
    codec: Option<&str>,
    is_embedded: bool,
    track_index: Option<i32>,
) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO subtitle_tracks 
         (media_id, file_path, language, label, codec, is_embedded, track_index, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![media_id, file_path, language, label, codec, is_embedded, track_index, &now],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Get all subtitle tracks for a media file
pub fn get_subtitle_tracks(conn: &Connection, media_id: i64) -> Result<Vec<SubtitleTrack>> {
    let mut stmt = conn.prepare(
        "SELECT id, media_id, file_path, language, label, codec, is_embedded, track_index, added_at
         FROM subtitle_tracks
         WHERE media_id = ?1
         ORDER BY track_index ASC, added_at ASC"
    )?;
    
    let tracks = stmt.query_map(params![media_id], |row| {
        Ok(SubtitleTrack {
            id: Some(row.get(0)?),
            media_id: row.get(1)?,
            file_path: row.get(2)?,
            language: row.get(3)?,
            label: row.get(4)?,
            codec: row.get(5)?,
            is_embedded: row.get(6)?,
            track_index: row.get(7)?,
            added_at: row.get(8)?,
        })
    })?
    .collect::<Result<Vec<_>>>()?;
    
    Ok(tracks)
}

/// Remove a subtitle track
pub fn remove_subtitle_track(conn: &Connection, subtitle_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM subtitle_tracks WHERE id = ?1",
        params![subtitle_id],
    )?;
    
    Ok(())
}

/// Get subtitle track by ID
#[allow(dead_code)]
pub fn get_subtitle_track_by_id(conn: &Connection, subtitle_id: i64) -> Result<Option<SubtitleTrack>> {
    let mut stmt = conn.prepare(
        "SELECT id, media_id, file_path, language, label, codec, is_embedded, track_index, added_at
         FROM subtitle_tracks
         WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query(params![subtitle_id])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(SubtitleTrack {
            id: Some(row.get(0)?),
            media_id: row.get(1)?,
            file_path: row.get(2)?,
            language: row.get(3)?,
            label: row.get(4)?,
            codec: row.get(5)?,
            is_embedded: row.get(6)?,
            track_index: row.get(7)?,
            added_at: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

/// Auto-discover subtitle files in the same directory as media file
pub fn discover_subtitle_files(media_path: &str) -> Result<Vec<String>> {
    use std::path::Path;
    
    let media_path = Path::new(media_path);
    let parent_dir = match media_path.parent() {
        Some(dir) => dir,
        None => return Ok(vec![]),
    };
    
    let file_stem = match media_path.file_stem() {
        Some(stem) => stem.to_string_lossy().to_string(),
        None => return Ok(vec![]),
    };
    
    let subtitle_extensions = ["srt", "vtt", "ass", "ssa", "sub", "idx"];
    let mut subtitle_files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy().to_string();
                    
                    // Check if filename starts with the same stem
                    if name_str.starts_with(&file_stem) {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            if subtitle_extensions.contains(&ext_str.as_str()) {
                                subtitle_files.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(subtitle_files)
}

/// Parse language from subtitle filename (e.g., "movie.en.srt" -> "en")
pub fn parse_language_from_filename(file_path: &str) -> Option<String> {
    use std::path::Path;
    
    let path = Path::new(file_path);
    let file_stem = path.file_stem()?.to_string_lossy();
    
    // Common language codes
    let language_codes = vec![
        "en", "eng", "english",
        "es", "spa", "spanish",
        "fr", "fre", "french",
        "de", "ger", "german",
        "it", "ita", "italian",
        "pt", "por", "portuguese",
        "ja", "jpn", "japanese",
        "ko", "kor", "korean",
        "zh", "chi", "chinese",
        "ru", "rus", "russian",
        "ar", "ara", "arabic",
        "hi", "hin", "hindi",
    ];
    
    // Split by dots and check each part
    for part in file_stem.split('.') {
        let lower_part = part.to_lowercase();
        if language_codes.contains(&lower_part.as_str()) {
            return Some(lower_part);
        }
    }
    
    None
}

/// Auto-scan and add external subtitle files for a media file
pub fn scan_and_add_subtitles(conn: &Connection, media_id: i64, media_path: &str) -> Result<Vec<i64>> {
    let subtitle_files = discover_subtitle_files(media_path)?;
    let mut added_ids = Vec::new();
    
    for subtitle_path in subtitle_files {
        let language = parse_language_from_filename(&subtitle_path);
        let label = language.clone().map(|lang| format!("{} (External)", lang.to_uppercase()));
        
        let subtitle_id = add_subtitle_track(
            conn,
            media_id,
            &subtitle_path,
            language.as_deref(),
            label.as_deref(),
            None, // codec will be determined by player
            false, // external file
            None, // no track index for external
        )?;
        
        added_ids.push(subtitle_id);
    }
    
    Ok(added_ids)
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
            file_hash: "testhash789".to_string(),
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
    fn test_subtitle_crud() -> Result<()> {
        let conn = init_db()?;
        let media_id = create_test_media(&conn)?;
        
        // Add subtitle
        let subtitle_id = add_subtitle_track(
            &conn,
            media_id,
            "/test/movie.en.srt",
            Some("en"),
            Some("English"),
            Some("srt"),
            false,
            None,
        )?;
        
        assert!(subtitle_id > 0);
        
        // Get subtitles
        let tracks = get_subtitle_tracks(&conn, media_id)?;
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].language, Some("en".to_string()));
        
        // Get by ID
        let track = get_subtitle_track_by_id(&conn, subtitle_id)?;
        assert!(track.is_some());
        
        // Remove subtitle
        remove_subtitle_track(&conn, subtitle_id)?;
        let tracks_after = get_subtitle_tracks(&conn, media_id)?;
        assert_eq!(tracks_after.len(), 0);
        
        Ok(())
    }

    #[test]
    fn test_parse_language() {
        assert_eq!(parse_language_from_filename("movie.en.srt"), Some("en".to_string()));
        assert_eq!(parse_language_from_filename("movie.english.srt"), Some("english".to_string()));
        assert_eq!(parse_language_from_filename("movie.es.vtt"), Some("es".to_string()));
        assert_eq!(parse_language_from_filename("movie.srt"), None);
    }
}
