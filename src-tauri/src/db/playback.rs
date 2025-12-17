use rusqlite::{Connection, Result, params};
use chrono::Utc;

/// Update or create playback state for a media file
pub fn update_playback_position(
    conn: &Connection,
    media_id: i64,
    position: i64,
    duration: Option<i64>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO playback_state (media_id, last_position, duration, last_played_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(media_id) DO UPDATE SET
            last_position = excluded.last_position,
            duration = COALESCE(excluded.duration, duration),
            last_played_at = excluded.last_played_at",
        params![media_id, position, duration, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

/// Mark media as completed (watched to end)
pub fn mark_as_completed(conn: &Connection, media_id: i64, duration: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO playback_state (media_id, last_position, duration, completed, watch_count, last_played_at)
         VALUES (?1, ?2, ?3, 1, 1, ?4)
         ON CONFLICT(media_id) DO UPDATE SET
            last_position = ?2,
            duration = ?3,
            completed = 1,
            watch_count = watch_count + 1,
            last_played_at = ?4",
        params![media_id, duration, duration, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

/// Get playback state for a media file
pub fn get_playback_state(conn: &Connection, media_id: i64) -> Result<Option<PlaybackState>> {
    let mut stmt = conn.prepare(
        "SELECT media_id, last_position, duration, completed, watch_count, last_played_at, created_at
         FROM playback_state
         WHERE media_id = ?1"
    )?;

    let result = stmt.query_row([media_id], |row| {
        Ok(PlaybackState {
            media_id: row.get(0)?,
            last_position: row.get(1)?,
            duration: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            watch_count: row.get(4)?,
            last_played_at: row.get(5)?,
            created_at: row.get(6)?,
        })
    });

    match result {
        Ok(state) => Ok(Some(state)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Log a playback session to history
pub fn log_playback_session(
    conn: &Connection,
    media_id: i64,
    duration_watched: i64,
    completed: bool,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO playback_history (media_id, started_at, ended_at, duration_watched, completed)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            media_id,
            Utc::now().to_rfc3339(),
            Utc::now().to_rfc3339(),
            duration_watched,
            completed as i32,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Get recently played media
pub fn get_recently_played(conn: &Connection, limit: usize) -> Result<Vec<RecentlyPlayed>> {
    let mut stmt = conn.prepare(
        "SELECT 
            m.id, m.file_path, m.file_name, m.title, m.year, m.media_type,
            p.last_position, p.duration, p.completed, p.last_played_at
         FROM playback_state p
         JOIN media_files m ON p.media_id = m.id
         WHERE m.is_deleted = 0
         ORDER BY p.last_played_at DESC
         LIMIT ?1"
    )?;

    let items = stmt.query_map([limit], |row| {
        Ok(RecentlyPlayed {
            media_id: row.get(0)?,
            file_path: row.get(1)?,
            file_name: row.get(2)?,
            title: row.get(3)?,
            year: row.get(4)?,
            media_type: row.get(5)?,
            last_position: row.get(6)?,
            duration: row.get(7)?,
            completed: row.get::<_, i32>(8)? != 0,
            last_played_at: row.get(9)?,
        })
    })?;

    items.collect()
}

/// Get items in progress (not completed, with position > 0)
pub fn get_in_progress(conn: &Connection, limit: usize) -> Result<Vec<RecentlyPlayed>> {
    let mut stmt = conn.prepare(
        "SELECT 
            m.id, m.file_path, m.file_name, m.title, m.year, m.media_type,
            p.last_position, p.duration, p.completed, p.last_played_at
         FROM playback_state p
         JOIN media_files m ON p.media_id = m.id
         WHERE m.is_deleted = 0 
           AND p.completed = 0 
           AND p.last_position > 0
         ORDER BY p.last_played_at DESC
         LIMIT ?1"
    )?;

    let items = stmt.query_map([limit], |row| {
        Ok(RecentlyPlayed {
            media_id: row.get(0)?,
            file_path: row.get(1)?,
            file_name: row.get(2)?,
            title: row.get(3)?,
            year: row.get(4)?,
            media_type: row.get(5)?,
            last_position: row.get(6)?,
            duration: row.get(7)?,
            completed: row.get::<_, i32>(8)? != 0,
            last_played_at: row.get(9)?,
        })
    })?;

    items.collect()
}

/// Get watch statistics
pub fn get_watch_stats(conn: &Connection) -> Result<WatchStats> {
    let total_watched: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT media_id) FROM playback_state WHERE completed = 1",
        [],
        |row| row.get(0),
    )?;

    let total_in_progress: i64 = conn.query_row(
        "SELECT COUNT(*) FROM playback_state WHERE completed = 0 AND last_position > 0",
        [],
        |row| row.get(0),
    )?;

    let total_watch_time: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_watched), 0) FROM playback_history",
        [],
        |row| row.get(0),
    )?;

    let total_sessions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM playback_history",
        [],
        |row| row.get(0),
    )?;

    Ok(WatchStats {
        total_watched: total_watched as usize,
        total_in_progress: total_in_progress as usize,
        total_watch_time: total_watch_time as u64,
        total_sessions: total_sessions as usize,
    })
}

// PlaybackState is defined in models.rs and re-exported via mod.rs

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RecentlyPlayed {
    pub media_id: i64,
    pub file_path: String,
    pub file_name: String,
    pub title: Option<String>,
    pub year: Option<i32>,
    pub media_type: String,
    pub last_position: i64,
    pub duration: Option<i64>,
    pub completed: bool,
    pub last_played_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WatchStats {
    pub total_watched: usize,
    pub total_in_progress: usize,
    pub total_watch_time: u64,
    pub total_sessions: usize,
}
