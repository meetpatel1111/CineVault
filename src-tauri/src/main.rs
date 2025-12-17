// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod indexer;
mod player;

use std::sync::Mutex;
use tauri::State;
use chrono::Utc;

// Application state
struct AppState {
    db: Mutex<db::Database>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to CineVault.", name)
}

#[tauri::command]
fn get_db_stats(state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    let stats = db::get_library_stats(&conn).map_err(|e| e.to_string())?;
    
    Ok(format!(
        "Library: {} files ({} movies, {} TV episodes, {} music tracks)",
        stats.total, stats.movies, stats.tv_episodes, stats.music
    ))
}

#[tauri::command]
async fn scan_directory(
    path: String,
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<ScanResult, String> {
    println!("Scanning directory: {}", path);
    
    // Create scanner
    let scanner = indexer::MediaScanner::new();
    
    // Scan directory
    let files = scanner.scan_directory(&path)
        .map_err(|e| format!("Scan error: {}", e))?;
    
    println!("Found {} media files", files.len());
    
    // Get database connection
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    let mut added = 0;
    let mut updated = 0;
    let mut errors = 0;
    
    for (idx, file) in files.iter().enumerate() {
        // Emit progress event
        let progress = indexer::ScanProgress {
            current_file: file.file_name.clone(),
            files_scanned: idx + 1,
            files_found: files.len(),
            current_dir: path.clone(),
        };
        
        let _ = window.emit("scan-progress", &progress);
        
        // Calculate file hash
        let file_hash = indexer::hash::quick_hash(&file.path)
            .unwrap_or_else(|_| "unknown".to_string());
        
        // Parse filename for title and year
        let (title, year) = indexer::metadata::parse_filename(&file.file_name);
        
        // Check for TV episode info
        let (season_number, episode_number) = indexer::metadata::parse_episode_info(&file.file_name)
            .map(|(s, e)| (Some(s as i32), Some(e as i32)))
            .unwrap_or((None, None));
        
        // Determine media type
        let media_type = if season_number.is_some() {
            db::MediaType::TvEpisode
        } else if file.file_type == indexer::scanner::FileType::Audio {
            db::MediaType::Music
        } else {
            db::MediaType::Movie
        };
        
        // Extract metadata from file (if ffprobe is available)
        let metadata = indexer::metadata::MediaMetadata::extract_from_file(&file.path)
            .ok();
        
        // Create media file record
        let media = db::MediaFile {
            id: None,
            file_path: file.path.to_string_lossy().to_string(),
            file_hash,
            file_name: file.file_name.clone(),
            file_size: file.size as i64,
            media_type,
            duration: metadata.as_ref().and_then(|m| m.duration).map(|d| d as i64),
            codec: metadata.as_ref().and_then(|m| m.codec.clone()),
            resolution: metadata.as_ref().and_then(|m| m.resolution_string()),
            bitrate: metadata.as_ref().and_then(|m| m.bitrate).map(|b| b as i64),
            framerate: metadata.as_ref().and_then(|m| m.framerate),
            audio_codec: metadata.as_ref().and_then(|m| m.audio_codec.clone()),
            audio_channels: metadata.as_ref().and_then(|m| m.audio_channels).map(|c| c as i32),
            title: Some(title),
            year: year.map(|y| y as i32),
            season_number,
            episode_number,
            indexed_at: Utc::now().to_rfc3339(),
            last_modified: file.modified
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .and_then(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| Utc::now().to_rfc3339()),
            is_deleted: false,
            metadata_json: None,
        };
        
        // Insert or update in database
        match db::upsert_media_file(&conn, &media) {
            Ok(_) => {
                if media.id.is_some() {
                    updated += 1;
                } else {
                    added += 1;
                }
            }
            Err(e) => {
                eprintln!("Error inserting media file: {}", e);
                errors += 1;
            }
        }
    }
    
    println!("Scan complete: {} added, {} updated, {} errors", added, updated, errors);
    
    Ok(ScanResult {
        total_found: files.len(),
        added,
        updated,
        errors,
    })
}

#[tauri::command]
fn get_all_media(state: State<AppState>) -> Result<Vec<db::MediaFile>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_all_media_files(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_media_by_type(
    media_type: String,
    state: State<AppState>,
) -> Result<Vec<db::MediaFile>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    let media_type = match media_type.as_str() {
        "movie" => db::MediaType::Movie,
        "tv_episode" => db::MediaType::TvEpisode,
        "music" => db::MediaType::Music,
        _ => return Err("Invalid media type".to_string()),
    };
    
    db::get_media_by_type(&conn, media_type).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_media(query: String, state: State<AppState>) -> Result<Vec<db::MediaFile>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::search_media(&conn, &query).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_playback_position(
    media_id: i64,
    position: i64,
    duration: Option<i64>,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::update_playback_position(&conn, media_id, position, duration)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn mark_as_completed(
    media_id: i64,
    duration: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::mark_as_completed(&conn, media_id, duration)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_playback_state(
    media_id: i64,
    state: State<AppState>,
) -> Result<Option<db::PlaybackState>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_playback_state(&conn, media_id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[tauri::command]
fn get_recently_played(
    limit: usize,
    state: State<AppState>,
) -> Result<Vec<db::RecentlyPlayed>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_recently_played(&conn, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_in_progress(
    limit: usize,
    state: State<AppState>,
) -> Result<Vec<db::RecentlyPlayed>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_in_progress(&conn, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_watch_stats(state: State<AppState>) -> Result<db::WatchStats, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_watch_stats(&conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn extract_metadata(
    media_id: i64,
    state: State<'_, AppState>,
) -> Result<MetadataResult, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    // Get media file from database
    let media = db::get_all_media_files(&conn)
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|m| m.id == Some(media_id))
        .ok_or_else(|| "Media file not found".to_string())?;
    
    // Extract metadata
    let metadata = indexer::metadata::MediaMetadata::extract_from_file(&media.file_path)
        .map_err(|e| e.to_string())?;
    
    // Update database with extracted metadata
    let updated_media = db::MediaFile {
        duration: metadata.duration.map(|d| d as i64),
        codec: metadata.codec.clone(),
        resolution: metadata.resolution_string(),
        bitrate: metadata.bitrate.map(|b| b as i64),
        framerate: metadata.framerate,
        audio_codec: metadata.audio_codec.clone(),
        audio_channels: metadata.audio_channels.map(|c| c as i32),
        ..media
    };
    
    db::upsert_media_file(&conn, &updated_media)
        .map_err(|e| e.to_string())?;
    
    Ok(MetadataResult {
        duration: metadata.duration,
        resolution: metadata.resolution_string(),
        codec: metadata.codec,
        audio_codec: metadata.audio_codec,
        bitrate: metadata.bitrate,
        framerate: metadata.framerate,
    })
}

// Subtitle commands
#[tauri::command]
fn add_subtitle_track(
    media_id: i64,
    file_path: String,
    language: Option<String>,
    label: Option<String>,
    codec: Option<String>,
    is_embedded: bool,
    track_index: Option<i32>,
    state: State<AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::add_subtitle_track(
        &conn,
        media_id,
        &file_path,
        language.as_deref(),
        label.as_deref(),
        codec.as_deref(),
        is_embedded,
        track_index,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_subtitle_tracks(
    media_id: i64,
    state: State<AppState>,
) -> Result<Vec<db::SubtitleTrack>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_subtitle_tracks(&conn, media_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_subtitle_track(
    subtitle_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::remove_subtitle_track(&conn, subtitle_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_subtitles(
    media_id: i64,
    media_path: String,
    state: State<AppState>,
) -> Result<Vec<i64>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::scan_and_add_subtitles(&conn, media_id, &media_path)
        .map_err(|e| e.to_string())
}

// Collection commands
#[tauri::command]
fn create_collection(
    name: String,
    description: Option<String>,
    state: State<AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::create_collection(&conn, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_collections(state: State<AppState>) -> Result<Vec<db::CollectionWithCount>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_collections_with_counts(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_collection_media(
    collection_id: i64,
    state: State<AppState>,
) -> Result<Vec<db::CollectionMediaItem>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_collection_media(&conn, collection_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_to_collection(
    collection_id: i64,
    media_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::add_media_to_collection(&conn, collection_id, media_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_from_collection(
    collection_id: i64,
    media_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::remove_media_from_collection(&conn, collection_id, media_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_collection(
    collection_id: i64,
    name: String,
    description: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::update_collection(&conn, collection_id, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_collection(
    collection_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::delete_collection(&conn, collection_id)
        .map_err(|e| e.to_string())
}

// Playlist commands
#[tauri::command]
fn create_playlist(
    name: String,
    description: Option<String>,
    playlist_type: String,
    state: State<AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    let ptype = match playlist_type.as_str() {
        "manual" => db::PlaylistType::Manual,
        "smart" => db::PlaylistType::Smart,
        "auto" => db::PlaylistType::Auto,
        _ => return Err("Invalid playlist type".to_string()),
    };
    
    db::create_playlist(&conn, &name, description.as_deref(), ptype)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_playlists(state: State<AppState>) -> Result<Vec<db::PlaylistWithCount>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_playlists_with_counts(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_playlist_media(
    playlist_id: i64,
    state: State<AppState>,
) -> Result<Vec<db::PlaylistMediaItem>, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::get_playlist_media(&conn, playlist_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_to_playlist(
    playlist_id: i64,
    media_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::add_media_to_playlist(&conn, playlist_id, media_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_from_playlist(
    playlist_id: i64,
    media_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::remove_media_from_playlist(&conn, playlist_id, media_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_playlist(
    playlist_id: i64,
    name: String,
    description: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::update_playlist(&conn, playlist_id, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_playlist(
    playlist_id: i64,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    db::delete_playlist(&conn, playlist_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn extract_all_metadata(
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<BatchMetadataResult, String> {
    let db = state.db.lock().unwrap();
    let conn = db.connection();
    let conn = conn.lock().unwrap();
    
    // Get all media files
    let media_files = db::get_all_media_files(&conn)
        .map_err(|e| e.to_string())?;
    
    let total = media_files.len();
    let mut processed = 0;
    let mut updated = 0;
    let mut errors = 0;
    
    for media in media_files {
        processed += 1;
        
        // Emit progress
        let _ = window.emit("metadata-extraction-progress", MetadataProgress {
            current: processed,
            total,
            current_file: media.file_name.clone(),
        });
        
        // Skip if metadata already exists
        if media.duration.is_some() && media.codec.is_some() {
            continue;
        }
        
        // Extract metadata
        match indexer::metadata::MediaMetadata::extract_from_file(&media.file_path) {
            Ok(metadata) => {
                // Update database
                let updated_media = db::MediaFile {
                    duration: metadata.duration.map(|d| d as i64),
                    codec: metadata.codec.clone(),
                    resolution: metadata.resolution_string(),
                    bitrate: metadata.bitrate.map(|b| b as i64),
                    framerate: metadata.framerate,
                    audio_codec: metadata.audio_codec.clone(),
                    audio_channels: metadata.audio_channels.map(|c| c as i32),
                    ..media
                };
                
                match db::upsert_media_file(&conn, &updated_media) {
                    Ok(_) => updated += 1,
                    Err(e) => {
                        eprintln!("Error updating metadata: {}", e);
                        errors += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error extracting metadata: {}", e);
                errors += 1;
            }
        }
    }
    
    Ok(BatchMetadataResult {
        total,
        processed,
        updated,
        errors,
    })
}

#[derive(serde::Serialize)]
struct ScanResult {
    total_found: usize,
    added: usize,
    updated: usize,
    errors: usize,
}

#[derive(serde::Serialize)]
struct MetadataResult {
    duration: Option<u64>,
    resolution: Option<String>,
    codec: Option<String>,
    audio_codec: Option<String>,
    bitrate: Option<u64>,
    framerate: Option<f64>,
}

#[derive(serde::Serialize)]
struct BatchMetadataResult {
    total: usize,
    processed: usize,
    updated: usize,
    errors: usize,
}

#[derive(serde::Serialize, Clone)]
struct MetadataProgress {
    current: usize,
    total: usize,
    current_file: String,
}

fn main() {
    // Initialize database
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
        .expect("Failed to get app data directory");
    
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    
    let db_path = app_data_dir.join("cinevault.db");
    println!("Database path: {:?}", db_path);
    
    let database = db::Database::new(db_path).expect("Failed to initialize database");
    database.migrate().expect("Failed to run migrations");
    
    println!("Database initialized successfully");
    
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(database),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_db_stats,
            scan_directory,
            get_all_media,
            get_media_by_type,
            search_media,
            update_playback_position,
            mark_as_completed,
            get_playback_state,
            get_recently_played,
            get_in_progress,
            get_watch_stats,
            extract_metadata,
            extract_all_metadata,
            add_subtitle_track,
            get_subtitle_tracks,
            remove_subtitle_track,
            scan_subtitles,
            create_collection,
            get_all_collections,
            get_collection_media,
            add_to_collection,
            remove_from_collection,
            update_collection,
            delete_collection,
            create_playlist,
            get_all_playlists,
            get_playlist_media,
            add_to_playlist,
            remove_from_playlist,
            update_playlist,
            delete_playlist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
