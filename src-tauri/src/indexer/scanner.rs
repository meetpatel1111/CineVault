use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Supported media file extensions
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v"];
const AUDIO_EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "aac", "ogg", "m4a", "wma", "opus"];
const SUBTITLE_EXTENSIONS: &[&str] = &["srt", "ass", "vtt", "sub"];

/// Progress information for scanning operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current_file: String,
    pub files_scanned: usize,
    pub files_found: usize,
    pub current_dir: String,
}

/// Media file scanner
pub struct MediaScanner {
    video_extensions: Vec<String>,
    audio_extensions: Vec<String>,
    subtitle_extensions: Vec<String>,
}

impl Default for MediaScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaScanner {
    /// Create a new media scanner with default extensions
    pub fn new() -> Self {
        MediaScanner {
            video_extensions: VIDEO_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            audio_extensions: AUDIO_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
            subtitle_extensions: SUBTITLE_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Scan a directory recursively for media files
    pub fn scan_directory<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<ScannedFile>, ScanError> {
        let mut files = Vec::new();
        self.scan_recursive(path.as_ref(), &mut files)?;
        Ok(files)
    }

    /// Recursive directory scanning
    fn scan_recursive(&self, path: &Path, files: &mut Vec<ScannedFile>) -> Result<(), ScanError> {
        if !path.exists() {
            return Err(ScanError::PathNotFound(path.to_string_lossy().to_string()));
        }

        if !path.is_dir() {
            return Err(ScanError::NotADirectory(path.to_string_lossy().to_string()));
        }

        let entries = fs::read_dir(path)
            .map_err(|e| ScanError::ReadError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| ScanError::ReadError(e.to_string()))?;
            let entry_path = entry.path();

            // Skip hidden files and directories
            if let Some(name) = entry_path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }

            if entry_path.is_dir() {
                // Recursively scan subdirectories
                self.scan_recursive(&entry_path, files)?;
            } else if entry_path.is_file() {
                // Check if file is a supported media type
                if let Some(scanned) = self.process_file(&entry_path)? {
                    files.push(scanned);
                }
            }
        }

        Ok(())
    }

    /// Process a single file and determine its type
    fn process_file(&self, path: &Path) -> Result<Option<ScannedFile>, ScanError> {
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        let Some(ext) = extension else {
            return Ok(None);
        };

        let file_type = if self.video_extensions.contains(&ext) {
            FileType::Video
        } else if self.audio_extensions.contains(&ext) {
            FileType::Audio
        } else if self.subtitle_extensions.contains(&ext) {
            FileType::Subtitle
        } else {
            return Ok(None);
        };

        let metadata = fs::metadata(path)
            .map_err(|e| ScanError::ReadError(e.to_string()))?;

        let modified = metadata.modified()
            .map_err(|e| ScanError::ReadError(e.to_string()))?;

        Ok(Some(ScannedFile {
            path: path.to_path_buf(),
            file_name: path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_type,
            size: metadata.len(),
            modified,
        }))
    }

    /// Check if a file is a supported media file
    #[allow(dead_code)]
    pub fn is_media_file<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        if let Some(ext) = extension {
            self.video_extensions.contains(&ext) 
                || self.audio_extensions.contains(&ext)
        } else {
            false
        }
    }

    /// Get the media type from a file path
    #[allow(dead_code)]
    pub fn get_file_type<P: AsRef<Path>>(&self, path: P) -> Option<FileType> {
        let path = path.as_ref();
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())?;

        if self.video_extensions.contains(&extension) {
            Some(FileType::Video)
        } else if self.audio_extensions.contains(&extension) {
            Some(FileType::Audio)
        } else if self.subtitle_extensions.contains(&extension) {
            Some(FileType::Subtitle)
        } else {
            None
        }
    }
}

/// Represents a scanned file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub file_name: String,
    pub file_type: FileType,
    pub size: u64,
    pub modified: SystemTime,
}

/// Type of media file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Video,
    Audio,
    Subtitle,
}

impl FileType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            FileType::Video => "video",
            FileType::Audio => "audio",
            FileType::Subtitle => "subtitle",
        }
    }
}

/// Errors that can occur during scanning
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("Path not found: {0}")]
    PathNotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("Read error: {0}")]
    ReadError(String),

    #[error("Permission denied: {0}")]
    #[allow(dead_code)]
    PermissionDenied(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_detection() {
        let scanner = MediaScanner::new();
        
        assert!(scanner.is_media_file("movie.mp4"));
        assert!(scanner.is_media_file("video.mkv"));
        assert!(scanner.is_media_file("song.mp3"));
        assert!(scanner.is_media_file("audio.flac"));
        
        assert!(!scanner.is_media_file("document.txt"));
        assert!(!scanner.is_media_file("image.jpg"));
    }

    #[test]
    fn test_file_type_detection() {
        let scanner = MediaScanner::new();
        
        assert_eq!(scanner.get_file_type("movie.mp4"), Some(FileType::Video));
        assert_eq!(scanner.get_file_type("song.mp3"), Some(FileType::Audio));
        assert_eq!(scanner.get_file_type("subtitle.srt"), Some(FileType::Subtitle));
        assert_eq!(scanner.get_file_type("document.txt"), None);
    }
}
