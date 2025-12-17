use serde::{Deserialize, Serialize};

/// Playlist type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlaylistType {
    Manual,
    Smart,
    Auto,
}

impl PlaylistType {
    pub fn as_str(&self) -> &str {
        match self {
            PlaylistType::Manual => "manual",
            PlaylistType::Smart => "smart",
            PlaylistType::Auto => "auto",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "manual" => Some(PlaylistType::Manual),
            "smart" => Some(PlaylistType::Smart),
            "auto" => Some(PlaylistType::Auto),
            _ => None,
        }
    }
}

/// Represents a playlist in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub playlist_type: PlaylistType,
    pub created_at: String,
    pub updated_at: String,
    pub thumbnail_path: Option<String>,
    pub metadata_json: Option<String>,
}

/// Media file record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: Option<i64>,
    pub file_path: String,
    pub file_hash: String,
    pub file_name: String,
    pub file_size: i64,
    pub media_type: MediaType,
    
    // Technical metadata
    pub duration: Option<i64>,
    pub codec: Option<String>,
    pub resolution: Option<String>,
    pub bitrate: Option<i64>,
    pub framerate: Option<f64>,
    pub audio_codec: Option<String>,
    pub audio_channels: Option<i32>,
    
    // Organization
    pub title: Option<String>,
    pub year: Option<i32>,
    pub season_number: Option<i32>,
    pub episode_number: Option<i32>,
    
    // Status
    pub indexed_at: String,
    pub last_modified: String,
    pub is_deleted: bool,
    
    // Metadata
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    TvEpisode,
    Music,
    Video,
    Audio,
}

impl MediaType {
    pub fn as_str(&self) -> &str {
        match self {
            MediaType::Movie => "movie",
            MediaType::TvEpisode => "tv_episode",
            MediaType::Music => "music",
            MediaType::Video => "video",
            MediaType::Audio => "audio",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "movie" => Some(MediaType::Movie),
            "tv_episode" => Some(MediaType::TvEpisode),
            "music" => Some(MediaType::Music),
            "video" => Some(MediaType::Video),
            "audio" => Some(MediaType::Audio),
            _ => None,
        }
    }
}

/// Playback state record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    pub media_id: i64,
    pub last_position: i64,
    pub duration: Option<i64>,
    pub completed: bool,
    pub watch_count: i32,
    pub last_played_at: String,
    pub created_at: String,
}


/// Collection record
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub collection_type: Option<String>,
    pub created_at: String,
    pub poster_path: Option<String>,
    pub metadata_json: Option<String>,
}

/// TMDB media mapping
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMedia {
    pub id: Option<i64>,
    pub media_id: i64,
    pub tmdb_id: i64,
    pub media_type: String,
    pub match_confidence: Option<f64>,
    pub matched_at: String,
    pub last_synced_at: Option<String>,
    pub is_manual_match: bool,
}

/// TMDB metadata
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbMetadata {
    pub tmdb_id: i64,
    pub media_type: String,
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub release_date: Option<String>,
    pub runtime: Option<i32>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    
    // Ratings
    pub vote_average: Option<f64>,
    pub vote_count: Option<i32>,
    pub popularity: Option<f64>,
    
    // External IDs
    pub imdb_id: Option<String>,
    
    // Additional data (JSON)
    pub genres_json: Option<String>,
    pub keywords_json: Option<String>,
    pub languages_json: Option<String>,
    pub production_countries_json: Option<String>,
    
    // TV specific
    pub number_of_seasons: Option<i32>,
    pub number_of_episodes: Option<i32>,
    pub episode_runtime_json: Option<String>,
    
    pub fetched_at: String,
}

/// Settings record
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}
