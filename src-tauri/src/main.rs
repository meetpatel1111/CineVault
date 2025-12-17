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
        
        // Create media file record
        let media = db::MediaFile {
            id: None,
            file_path: file.path.to_string_lossy().to_string(),
            file_hash,
            file_name: file.file_name.clone(),
            file_size: file.size as i64,
            media_type,
            duration: None,
            codec: None,
            resolution: None,
            bitrate: None,
            framerate: None,
            audio_codec: None,
            audio_channels: None,
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

#[derive(serde::Serialize)]
struct ScanResult {
    total_found: usize,
    added: usize,
    updated: usize,
    errors: usize,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
