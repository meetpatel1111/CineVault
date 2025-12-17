/// Database schema definitions for CineVault
/// 
/// This module contains all SQL schema definitions for both core
/// functionality and optional TMDB integration.
/// Core schema version
#[allow(dead_code)]
pub const SCHEMA_VERSION: i32 = 1;

/// Core tables for media management
pub const CORE_SCHEMA: &str = r#"
-- Schema version tracking
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Media files table: Core metadata for all media files
CREATE TABLE IF NOT EXISTS media_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    file_hash TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'tv_episode', 'music', 'video', 'audio')),
    
    -- Technical metadata
    duration INTEGER,  -- Duration in seconds
    codec TEXT,
    resolution TEXT,   -- e.g., "1920x1080"
    bitrate INTEGER,   -- Bitrate in kbps
    framerate REAL,
    audio_codec TEXT,
    audio_channels INTEGER,
    
    -- Organization
    title TEXT,
    year INTEGER,
    season_number INTEGER,  -- For TV episodes
    episode_number INTEGER, -- For TV episodes
    
    -- Status
    indexed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TEXT NOT NULL,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    
    -- Metadata
    metadata_json TEXT  -- Additional flexible metadata as JSON
);

CREATE INDEX IF NOT EXISTS idx_media_files_type ON media_files(media_type);
CREATE INDEX IF NOT EXISTS idx_media_files_hash ON media_files(file_hash);
CREATE INDEX IF NOT EXISTS idx_media_files_deleted ON media_files(is_deleted);
CREATE INDEX IF NOT EXISTS idx_media_files_title ON media_files(title);

-- Playback state: Track resume position and watch history
CREATE TABLE IF NOT EXISTS playback_state (
    media_id INTEGER PRIMARY KEY,
    last_position INTEGER NOT NULL DEFAULT 0,  -- Position in seconds
    duration INTEGER,  -- Cached duration
    completed INTEGER NOT NULL DEFAULT 0,  -- 1 if watched to completion
    watch_count INTEGER NOT NULL DEFAULT 0,
    last_played_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_playback_last_played ON playback_state(last_played_at);
CREATE INDEX IF NOT EXISTS idx_playback_completed ON playback_state(completed);

-- Playback history: Full history log of all playback events
CREATE TABLE IF NOT EXISTS playback_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id INTEGER NOT NULL,
    started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ended_at TEXT,
    duration_watched INTEGER,  -- Seconds watched in this session
    completed INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_history_media ON playback_history(media_id);
CREATE INDEX IF NOT EXISTS idx_history_started ON playback_history(started_at);

-- Playlists: User-created and auto-generated playlists
CREATE TABLE IF NOT EXISTS playlists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    playlist_type TEXT NOT NULL CHECK(playlist_type IN ('manual', 'smart', 'auto')),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    thumbnail_path TEXT,
    metadata_json TEXT  -- Additional metadata
);

CREATE INDEX IF NOT EXISTS idx_playlists_type ON playlists(playlist_type);

-- Playlist items: Media files in playlists
CREATE TABLE IF NOT EXISTS playlist_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    playlist_id INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    position INTEGER NOT NULL,  -- Order in playlist
    added_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE,
    UNIQUE(playlist_id, media_id)
);

CREATE INDEX IF NOT EXISTS idx_playlist_items_playlist ON playlist_items(playlist_id);
CREATE INDEX IF NOT EXISTS idx_playlist_items_position ON playlist_items(playlist_id, position);

-- Smart playlist rules: Rules for auto-generated playlists
CREATE TABLE IF NOT EXISTS playlist_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    playlist_id INTEGER NOT NULL,
    rule_type TEXT NOT NULL,  -- e.g., 'media_type', 'genre', 'duration', 'watched', 'rating'
    operator TEXT NOT NULL,   -- e.g., 'equals', 'contains', 'greater_than', 'less_than'
    value TEXT NOT NULL,
    
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_playlist_rules_playlist ON playlist_rules(playlist_id);

-- Collections: User-defined collections (franchises, series, etc.)
CREATE TABLE IF NOT EXISTS collections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT,  -- e.g., 'franchise', 'series', 'custom'
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    poster_path TEXT,
    metadata_json TEXT
);

-- Collection items: Media in collections
CREATE TABLE IF NOT EXISTS collection_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    collection_id INTEGER NOT NULL,
    media_id INTEGER NOT NULL,
    position INTEGER,
    added_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE,
    UNIQUE(collection_id, media_id)
);

CREATE INDEX IF NOT EXISTS idx_collection_items_collection ON collection_items(collection_id);

-- Subtitles: Track external subtitle files
CREATE TABLE IF NOT EXISTS subtitle_tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    language TEXT,
    label TEXT,
    codec TEXT,
    is_embedded INTEGER NOT NULL DEFAULT 0,
    track_index INTEGER,
    added_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_subtitle_tracks_media ON subtitle_tracks(media_id);

-- Audio tracks: Track external audio files
CREATE TABLE IF NOT EXISTS audio_tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    language TEXT,
    codec TEXT,
    channels INTEGER,
    is_default INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_audio_tracks_media ON audio_tracks(media_id);

-- Settings: Application settings
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
"#;

/// TMDB integration schema (optional)
pub const TMDB_SCHEMA: &str = r#"
-- TMDB Media Mapping: Links local media to TMDB IDs
CREATE TABLE IF NOT EXISTS tmdb_media (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    media_id INTEGER NOT NULL UNIQUE,
    tmdb_id INTEGER NOT NULL,
    media_type TEXT NOT NULL CHECK(media_type IN ('movie', 'tv', 'season', 'episode')),
    match_confidence REAL,  -- 0.0 to 1.0
    matched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_synced_at TEXT,
    is_manual_match INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (media_id) REFERENCES media_files(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tmdb_media_tmdb ON tmdb_media(tmdb_id);
CREATE INDEX IF NOT EXISTS idx_tmdb_media_type ON tmdb_media(media_type);

-- TMDB Metadata: Cached TMDB metadata
CREATE TABLE IF NOT EXISTS tmdb_metadata (
    tmdb_id INTEGER NOT NULL,
    media_type TEXT NOT NULL,
    title TEXT,
    original_title TEXT,
    overview TEXT,
    release_date TEXT,
    runtime INTEGER,
    status TEXT,
    tagline TEXT,
    
    -- Ratings
    vote_average REAL,
    vote_count INTEGER,
    popularity REAL,
    
    -- External IDs
    imdb_id TEXT,
    
    -- Additional data
    genres_json TEXT,
    keywords_json TEXT,
    languages_json TEXT,
    production_countries_json TEXT,
    
    -- TV specific
    number_of_seasons INTEGER,
    number_of_episodes INTEGER,
    episode_runtime_json TEXT,
    
    -- Timestamps
    fetched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (tmdb_id, media_type)
);

CREATE INDEX IF NOT EXISTS idx_tmdb_metadata_title ON tmdb_metadata(title);

-- TMDB Cast & Crew
CREATE TABLE IF NOT EXISTS tmdb_cast (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tmdb_media_id INTEGER NOT NULL,
    tmdb_person_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    character TEXT,
    role TEXT NOT NULL CHECK(role IN ('cast', 'director', 'writer', 'producer')),
    order_position INTEGER,
    profile_path TEXT,
    
    FOREIGN KEY (tmdb_media_id) REFERENCES tmdb_metadata(tmdb_id)
);

CREATE INDEX IF NOT EXISTS idx_tmdb_cast_media ON tmdb_cast(tmdb_media_id);
CREATE INDEX IF NOT EXISTS idx_tmdb_cast_person ON tmdb_cast(tmdb_person_id);
CREATE INDEX IF NOT EXISTS idx_tmdb_cast_name ON tmdb_cast(name);

-- TMDB Images: Cached image metadata
CREATE TABLE IF NOT EXISTS tmdb_images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tmdb_media_id INTEGER NOT NULL,
    image_type TEXT NOT NULL CHECK(image_type IN ('poster', 'backdrop', 'still', 'profile')),
    file_path TEXT,  -- Remote TMDB path
    local_path TEXT, -- Local cached path
    language TEXT,
    width INTEGER,
    height INTEGER,
    vote_average REAL,
    is_primary INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (tmdb_media_id) REFERENCES tmdb_metadata(tmdb_id)
);

CREATE INDEX IF NOT EXISTS idx_tmdb_images_media ON tmdb_images(tmdb_media_id);
CREATE INDEX IF NOT EXISTS idx_tmdb_images_type ON tmdb_images(image_type);

-- TMDB TV Shows: Additional TV show data
CREATE TABLE IF NOT EXISTS tmdb_tv_shows (
    tmdb_id INTEGER PRIMARY KEY,
    show_name TEXT NOT NULL,
    first_air_date TEXT,
    last_air_date TEXT,
    in_production INTEGER NOT NULL DEFAULT 0,
    next_episode_to_air_json TEXT,
    networks_json TEXT,
    
    FOREIGN KEY (tmdb_id) REFERENCES tmdb_metadata(tmdb_id)
);

-- TMDB Seasons: TV season data
CREATE TABLE IF NOT EXISTS tmdb_seasons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tmdb_show_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    tmdb_id INTEGER,
    name TEXT,
    overview TEXT,
    air_date TEXT,
    episode_count INTEGER,
    poster_path TEXT,
    
    FOREIGN KEY (tmdb_show_id) REFERENCES tmdb_tv_shows(tmdb_id),
    UNIQUE(tmdb_show_id, season_number)
);

CREATE INDEX IF NOT EXISTS idx_tmdb_seasons_show ON tmdb_seasons(tmdb_show_id);

-- TMDB Episodes: TV episode data
CREATE TABLE IF NOT EXISTS tmdb_episodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tmdb_show_id INTEGER NOT NULL,
    season_number INTEGER NOT NULL,
    episode_number INTEGER NOT NULL,
    tmdb_id INTEGER,
    name TEXT,
    overview TEXT,
    air_date TEXT,
    runtime INTEGER,
    still_path TEXT,
    vote_average REAL,
    
    FOREIGN KEY (tmdb_show_id) REFERENCES tmdb_tv_shows(tmdb_id),
    UNIQUE(tmdb_show_id, season_number, episode_number)
);

CREATE INDEX IF NOT EXISTS idx_tmdb_episodes_show ON tmdb_episodes(tmdb_show_id);
CREATE INDEX IF NOT EXISTS idx_tmdb_episodes_season ON tmdb_episodes(tmdb_show_id, season_number);

-- TMDB Collections: Movie collections/franchises
CREATE TABLE IF NOT EXISTS tmdb_collections (
    tmdb_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    overview TEXT,
    poster_path TEXT,
    backdrop_path TEXT
);

-- TMDB Collection Items: Movies in collections
CREATE TABLE IF NOT EXISTS tmdb_collection_items (
    collection_id INTEGER NOT NULL,
    tmdb_movie_id INTEGER NOT NULL,
    
    PRIMARY KEY (collection_id, tmdb_movie_id),
    FOREIGN KEY (collection_id) REFERENCES tmdb_collections(tmdb_id)
);

-- TMDB Settings: TMDB-specific settings
CREATE TABLE IF NOT EXISTS tmdb_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
"#;
