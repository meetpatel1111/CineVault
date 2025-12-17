use serde::{Deserialize, Serialize};
use std::path::Path;

/// Media metadata extracted from files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub duration: Option<u64>,       // Duration in seconds
    pub width: Option<u32>,          // Video width
    pub height: Option<u32>,         // Video height
    pub codec: Option<String>,       // Video codec
    pub audio_codec: Option<String>, // Audio codec
    pub bitrate: Option<u64>,        // Bitrate in kbps
    pub framerate: Option<f64>,      // Frames per second
    pub audio_channels: Option<u32>, // Number of audio channels
    pub sample_rate: Option<u32>,    // Audio sample rate
}

impl Default for MediaMetadata {
    fn default() -> Self {
        Self {
            duration: None,
            width: None,
            height: None,
            codec: None,
            audio_codec: None,
            bitrate: None,
            framerate: None,
            audio_channels: None,
            sample_rate: None,
        }
    }
}

impl MediaMetadata {
    /// Extract metadata from a media file
    /// Note: This is a placeholder. In a real implementation, you would use
    /// FFmpeg, libVLC, or similar to extract actual metadata.
    pub fn extract_from_file<P: AsRef<Path>>(_path: P) -> Result<Self, MetadataError> {
        // TODO: Implement actual metadata extraction using FFmpeg or similar
        // For now, return default metadata
        Ok(Self::default())
    }

    /// Get resolution as a string (e.g., "1920x1080")
    pub fn resolution_string(&self) -> Option<String> {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            Some(format!("{}x{}", w, h))
        } else {
            None
        }
    }

    /// Get duration as formatted string (e.g., "1h 42m")
    pub fn duration_string(&self) -> Option<String> {
        self.duration.map(|seconds| {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            if hours > 0 {
                format!("{}h {}m", hours, minutes)
            } else {
                format!("{}m", minutes)
            }
        })
    }

    /// Check if metadata is complete
    pub fn is_complete(&self) -> bool {
        self.duration.is_some() && self.codec.is_some()
    }
}

/// Errors that can occur during metadata extraction
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),
}

/// Parse title and year from filename
pub fn parse_filename(filename: &str) -> (String, Option<u32>) {
    // Remove file extension
    let name = filename.rsplit_once('.').map(|(n, _)| n).unwrap_or(filename);
    
    // Try to extract year from patterns like (2020) or [2020]
    let year_pattern = regex::Regex::new(r"[\(\[](\d{4})[\)\]]").ok();
    
    let year = if let Some(re) = year_pattern {
        re.captures(name)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    } else {
        None
    };
    
    // Clean up the title
    let title = if let Some(re) = year_pattern {
        re.replace(name, "").to_string()
    } else {
        name.to_string()
    };
    
    // Clean up common patterns
    let title = title
        .replace(".", " ")
        .replace("_", " ")
        .replace("-", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    
    (title, year)
}

/// Parse TV show episode information from filename
/// Format: S01E05 or 1x05
pub fn parse_episode_info(filename: &str) -> Option<(u32, u32)> {
    let filename = filename.to_lowercase();
    
    // Try S01E05 format
    if let Some(caps) = regex::Regex::new(r"s(\d+)e(\d+)")
        .ok()
        .and_then(|re| re.captures(&filename))
    {
        let season = caps.get(1)?.as_str().parse().ok()?;
        let episode = caps.get(2)?.as_str().parse().ok()?;
        return Some((season, episode));
    }
    
    // Try 1x05 format
    if let Some(caps) = regex::Regex::new(r"(\d+)x(\d+)")
        .ok()
        .and_then(|re| re.captures(&filename))
    {
        let season = caps.get(1)?.as_str().parse().ok()?;
        let episode = caps.get(2)?.as_str().parse().ok()?;
        return Some((season, episode));
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_filename() {
        let (title, year) = parse_filename("The.Matrix.1999.1080p.mp4");
        assert_eq!(title, "The Matrix");
        assert_eq!(year, Some(1999));

        let (title, year) = parse_filename("Inception (2010).mkv");
        assert_eq!(title, "Inception");
        assert_eq!(year, Some(2010));

        let (title, year) = parse_filename("Movie.Title.mp4");
        assert_eq!(title, "Movie Title");
        assert_eq!(year, None);
    }

    #[test]
    fn test_parse_episode_info() {
        assert_eq!(parse_episode_info("Show.S01E05.mp4"), Some((1, 5)));
        assert_eq!(parse_episode_info("Show.1x05.mkv"), Some((1, 5)));
        assert_eq!(parse_episode_info("Show.s02e12.avi"), Some((2, 12)));
        assert_eq!(parse_episode_info("Movie.2020.mp4"), None);
    }

    #[test]
    fn test_resolution_string() {
        let mut metadata = MediaMetadata::default();
        metadata.width = Some(1920);
        metadata.height = Some(1080);
        assert_eq!(metadata.resolution_string(), Some("1920x1080".to_string()));
    }

    #[test]
    fn test_duration_string() {
        let mut metadata = MediaMetadata::default();
        metadata.duration = Some(6300); // 1h 45m
        assert_eq!(metadata.duration_string(), Some("1h 45m".to_string()));

        metadata.duration = Some(900); // 15m
        assert_eq!(metadata.duration_string(), Some("15m".to_string()));
    }
}
