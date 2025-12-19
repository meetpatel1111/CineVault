use rusqlite::{Connection, Result, params};
use crate::indexer::metadata::AudioTrackMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub id: i64,
    pub media_id: i64,
    pub file_path: String,
    pub language: Option<String>,
    pub codec: Option<String>,
    pub channels: Option<i32>,
    pub is_default: bool,
}

/// Save extracted audio tracks to the database
pub fn save_audio_tracks(
    conn: &Connection,
    media_id: i64,
    file_path: &str,
    tracks: &[AudioTrackMetadata],
) -> Result<()> {
    // First, clear existing tracks for this media to avoid duplicates on re-scan
    conn.execute(
        "DELETE FROM audio_tracks WHERE media_id = ?1",
        params![media_id],
    )?;

    // Insert new tracks
    for track in tracks {
        conn.execute(
            "INSERT INTO audio_tracks (media_id, file_path, language, codec, channels, is_default)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                media_id,
                file_path,
                track.language,
                track.codec,
                track.channels,
                if track.is_default { 1 } else { 0 }
            ],
        )?;
    }

    Ok(())
}

/// Get audio tracks for a media file
pub fn get_audio_tracks(conn: &Connection, media_id: i64) -> Result<Vec<AudioTrack>> {
    let mut stmt = conn.prepare(
        "SELECT id, media_id, file_path, language, codec, channels, is_default
         FROM audio_tracks
         WHERE media_id = ?1
         ORDER BY id ASC"
    )?;

    let tracks = stmt.query_map(params![media_id], |row| {
        Ok(AudioTrack {
            id: row.get(0)?,
            media_id: row.get(1)?,
            file_path: row.get(2)?,
            language: row.get(3)?,
            codec: row.get(4)?,
            channels: row.get(5)?,
            is_default: row.get::<_, i32>(6)? == 1,
        })
    })?
    .collect::<Result<Vec<_>>>()?;

    Ok(tracks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::init_db;
    use crate::db::operations::add_media_file;
    use crate::db::models::{MediaFile, MediaType};
    use chrono::Utc;

    #[test]
    fn test_audio_tracks_crud() -> Result<()> {
        let conn = init_db()?;

        // Create dummy media
        let media = MediaFile {
            id: None,
            file_path: "/test/movie.mkv".to_string(),
            file_hash: "hash".to_string(),
            file_name: "movie.mkv".to_string(),
            file_size: 100,
            media_type: MediaType::Movie,
            duration: None, codec: None, resolution: None, bitrate: None, framerate: None, audio_codec: None, audio_channels: None,
            title: None, year: None, season_number: None, episode_number: None,
            indexed_at: Utc::now().to_rfc3339(), last_modified: Utc::now().to_rfc3339(), is_deleted: false, metadata_json: None,
        };
        let media_id = add_media_file(&conn, &media)?;

        // Save tracks
        let tracks = vec![
            AudioTrackMetadata {
                index: 0,
                codec: "aac".to_string(),
                language: Some("eng".to_string()),
                channels: Some(2),
                is_default: true,
            },
            AudioTrackMetadata {
                index: 1,
                codec: "ac3".to_string(),
                language: Some("spa".to_string()),
                channels: Some(6),
                is_default: false,
            }
        ];

        save_audio_tracks(&conn, media_id, "/test/movie.mkv", &tracks)?;

        // Fetch back
        let saved = get_audio_tracks(&conn, media_id)?;
        assert_eq!(saved.len(), 2);
        assert_eq!(saved[0].language.as_deref(), Some("eng"));
        assert!(saved[0].is_default);
        assert_eq!(saved[1].language.as_deref(), Some("spa"));
        assert!(!saved[1].is_default);

        Ok(())
    }
}
