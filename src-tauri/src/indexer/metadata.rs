use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

/// Media metadata extracted from files
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl MediaMetadata {
    /// Extract metadata from a media file using ffprobe
    /// Requires ffprobe to be installed and available in PATH
    #[allow(dead_code)]
    pub fn extract_from_file<P: AsRef<Path>>(path: P) -> Result<Self, MetadataError> {
        let path = path.as_ref();
        
        // Check if file exists
        if !path.exists() {
            return Err(MetadataError::FileNotFound(
                path.display().to_string()
            ));
        }
        
        // Try to extract metadata using ffprobe
        match Self::extract_with_ffprobe(path) {
            Ok(metadata) => Ok(metadata),
            Err(e) => {
                // If ffprobe fails, log the error and return default metadata
                eprintln!("FFprobe extraction failed: {}. Returning default metadata.", e);
                Ok(Self::default())
            }
        }
    }
    
    /// Extract metadata using ffprobe command
    fn extract_with_ffprobe(path: &Path) -> Result<Self, MetadataError> {
        // Run ffprobe with JSON output
        let output = Command::new("ffprobe")
            .args(&[
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                path.to_str().ok_or_else(|| MetadataError::ExtractionFailed(
                    "Invalid path encoding".to_string()
                ))?
            ])
            .output()
            .map_err(|e| MetadataError::ExtractionFailed(
                format!("Failed to run ffprobe: {}. Make sure ffprobe is installed and in PATH.", e)
            ))?;
        
        if !output.status.success() {
            return Err(MetadataError::ExtractionFailed(
                format!("ffprobe failed with status: {}", output.status)
            ));
        }
        
        // Parse JSON output
        let json_str = String::from_utf8(output.stdout)
            .map_err(|e| MetadataError::ExtractionFailed(
                format!("Invalid UTF-8 in ffprobe output: {}", e)
            ))?;
        
        let probe_data: FFProbeOutput = serde_json::from_str(&json_str)
            .map_err(|e| MetadataError::ExtractionFailed(
                format!("Failed to parse ffprobe JSON: {}", e)
            ))?;
        
        // Extract metadata from probe data
        Ok(Self::from_ffprobe(probe_data))
    }
    
    /// Convert FFProbe output to MediaMetadata
    fn from_ffprobe(probe: FFProbeOutput) -> Self {
        let mut metadata = Self::default();
        
        // Get duration from format
        if let Some(format) = probe.format {
            if let Some(duration_str) = format.duration {
                if let Ok(duration_f64) = duration_str.parse::<f64>() {
                    metadata.duration = Some(duration_f64 as u64);
                }
            }
            
            if let Some(bitrate_str) = format.bit_rate {
                if let Ok(bitrate) = bitrate_str.parse::<u64>() {
                    metadata.bitrate = Some(bitrate / 1000); // Convert to kbps
                }
            }
        }
        
        // Find video and audio streams
        for stream in probe.streams {
            match stream.codec_type.as_deref() {
                Some("video") => {
                    metadata.width = stream.width;
                    metadata.height = stream.height;
                    metadata.codec = stream.codec_name;
                    
                    // Parse framerate from avg_frame_rate (e.g., "24000/1001")
                    if let Some(fps_str) = stream.avg_frame_rate {
                        if let Some((num, den)) = fps_str.split_once('/') {
                            if let (Ok(n), Ok(d)) = (num.parse::<f64>(), den.parse::<f64>()) {
                                if d != 0.0 {
                                    metadata.framerate = Some(n / d);
                                }
                            }
                        }
                    }
                }
                Some("audio") => {
                    if metadata.audio_codec.is_none() {
                        metadata.audio_codec = stream.codec_name;
                        metadata.audio_channels = stream.channels;
                        metadata.sample_rate = stream.sample_rate.and_then(|s| s.parse().ok());
                    }
                }
                _ => {}
            }
        }
        
        metadata
    }

    /// Get resolution as a string (e.g., "1920x1080")
    #[allow(dead_code)]
    pub fn resolution_string(&self) -> Option<String> {
        if let (Some(w), Some(h)) = (self.width, self.height) {
            Some(format!("{}x{}", w, h))
        } else {
            None
        }
    }

    /// Get duration as formatted string (e.g., "1h 42m")
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.duration.is_some() && self.codec.is_some()
    }
}

/// Errors that can occur during metadata extraction
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),
}

/// FFProbe JSON output structure
#[derive(Debug, Deserialize)]
struct FFProbeOutput {
    streams: Vec<FFProbeStream>,
    format: Option<FFProbeFormat>,
}

/// FFProbe stream information
#[derive(Debug, Deserialize)]
struct FFProbeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    avg_frame_rate: Option<String>,
    channels: Option<u32>,
    sample_rate: Option<String>,
}

/// FFProbe format information
#[derive(Debug, Deserialize)]
struct FFProbeFormat {
    duration: Option<String>,
    bit_rate: Option<String>,
}

/// Parse title and year from filename
pub fn parse_filename(filename: &str) -> (String, Option<u32>) {
    // Remove file extension
    let name = filename.rsplit_once('.').map(|(n, _)| n).unwrap_or(filename);
    
    // Try to extract year from patterns like (2020) or [2020] or just 2020
    let year_pattern = regex::Regex::new(r"[\(\[]?(\d{4})[\)\]]?").ok();
    
    let year = if let Some(ref re) = year_pattern {
        re.captures(name)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
    } else {
        None
    };
    
    // Clean up the title - remove year and quality info
    let mut title = name.to_string();
    
    // Remove year patterns
    if let Ok(ref re) = regex::Regex::new(r"[\(\[]?\d{4}[\)\]]?") {
        title = re.replace(&title, "").to_string();
    }
    
    // Remove quality/resolution patterns (720p, 1080p, 4K, etc.)
    if let Ok(ref re) = regex::Regex::new(r"\b(720p|1080p|2160p|4k|hd|uhd|bluray|web-?dl|webrip|hdtv)\b") {
        title = re.replace_all(&title, "").to_string();
    }
    
    // Clean up common patterns
    let title = title
        .replace(".", " ")
        .replace("_", " ")
        .replace("-", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    
    (title.trim().to_string(), year)
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
