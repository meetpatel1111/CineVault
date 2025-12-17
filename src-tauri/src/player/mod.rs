/// Player module for media playback
/// 
/// Currently uses HTML5 video/audio through the frontend.
/// Future: Integrate FFmpeg or libVLC for advanced codec support.
use std::path::{Path, PathBuf};

/// Player state
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerState {
    pub file_path: String,
    pub position: f64,
    pub duration: f64,
    pub playing: bool,
}

/// Convert file path to playback URL
/// For now, we use the convertFileSrc API from Tauri
#[allow(dead_code)]
pub fn get_playback_url(file_path: &str) -> String {
    // Tauri will handle this on the frontend with convertFileSrc
    file_path.to_string()
}

/// Check if file format is supported by HTML5
#[allow(dead_code)]
pub fn is_html5_supported(extension: &str) -> bool {
    matches!(
        extension.to_lowercase().as_str(),
        "mp4" | "webm" | "ogg" | "mp3" | "wav" | "flac" | "m4a"
    )
}

/// Get recommended player for a file
#[allow(dead_code)]
pub fn get_recommended_player(file_path: &Path) -> PlayerType {
    let extension = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    if is_html5_supported(extension) {
        PlayerType::Html5
    } else {
        PlayerType::External
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum PlayerType {
    Html5,
    External,
    // Future: FFmpeg, LibVLC
}
