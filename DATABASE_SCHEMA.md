# CineVault Database Schema

## Overview

CineVault uses SQLite with WAL (Write-Ahead Logging) mode for better concurrency and performance. The database is split into two main sections:

1. **Core Schema** - Essential offline functionality
2. **TMDB Schema** - Optional metadata enrichment

## Database Configuration

- **Mode**: WAL (Write-Ahead Logging)
- **Foreign Keys**: ENABLED
- **Location**: App data directory (`cinevault.db`)

---

## Core Schema

### `media_files`
Primary table for all indexed media files.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| file_path | TEXT UNIQUE | Absolute file path |
| file_hash | TEXT | SHA256 hash for duplicate detection |
| file_name | TEXT | File name without path |
| file_size | INTEGER | File size in bytes |
| media_type | TEXT | Type: movie, tv_episode, music, video, audio |
| duration | INTEGER | Duration in seconds |
| codec | TEXT | Video codec (e.g., H.264, HEVC) |
| resolution | TEXT | Resolution (e.g., 1920x1080) |
| bitrate | INTEGER | Bitrate in kbps |
| framerate | REAL | Frames per second |
| audio_codec | TEXT | Audio codec |
| audio_channels | INTEGER | Number of audio channels |
| title | TEXT | Parsed/user-defined title |
| year | INTEGER | Release year |
| season_number | INTEGER | For TV episodes |
| episode_number | INTEGER | For TV episodes |
| indexed_at | TEXT | When file was indexed |
| last_modified | TEXT | File modification timestamp |
| is_deleted | INTEGER | Soft delete flag (0/1) |
| metadata_json | TEXT | Additional flexible metadata |

**Indexes:**
- `idx_media_files_type` on `media_type`
- `idx_media_files_hash` on `file_hash`
- `idx_media_files_deleted` on `is_deleted`
- `idx_media_files_title` on `title`

---

### `playback_state`
Tracks resume position and completion status.

| Column | Type | Description |
|--------|------|-------------|
| media_id | INTEGER PRIMARY KEY | References media_files(id) |
| last_position | INTEGER | Last playback position (seconds) |
| duration | INTEGER | Cached duration |
| completed | INTEGER | Watched to completion (0/1) |
| watch_count | INTEGER | Number of times watched |
| last_played_at | TEXT | Last playback timestamp |
| created_at | TEXT | When first played |

**Indexes:**
- `idx_playback_last_played` on `last_played_at`
- `idx_playback_completed` on `completed`

---

### `playback_history`
Full history log of all playback sessions.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| media_id | INTEGER | References media_files(id) |
| started_at | TEXT | Session start time |
| ended_at | TEXT | Session end time |
| duration_watched | INTEGER | Seconds watched in session |
| completed | INTEGER | Session reached completion (0/1) |

**Indexes:**
- `idx_history_media` on `media_id`
- `idx_history_started` on `started_at`

---

### `playlists`
User-created and auto-generated playlists.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| name | TEXT | Playlist name |
| description | TEXT | Optional description |
| type | TEXT | Type: manual, smart, auto |
| created_at | TEXT | Creation timestamp |
| updated_at | TEXT | Last update timestamp |
| thumbnail_path | TEXT | Custom thumbnail |
| metadata_json | TEXT | Additional metadata |

**Indexes:**
- `idx_playlists_type` on `type`

---

### `playlist_items`
Media files in playlists.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| playlist_id | INTEGER | References playlists(id) |
| media_id | INTEGER | References media_files(id) |
| position | INTEGER | Order in playlist |
| added_at | TEXT | When added to playlist |

**Constraints:**
- UNIQUE(playlist_id, media_id)

**Indexes:**
- `idx_playlist_items_playlist` on `playlist_id`
- `idx_playlist_items_position` on `(playlist_id, position)`

---

### `playlist_rules`
Rules for smart/auto playlists.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| playlist_id | INTEGER | References playlists(id) |
| rule_type | TEXT | Type: media_type, genre, duration, watched, rating |
| operator | TEXT | Operator: equals, contains, greater_than, less_than |
| value | TEXT | Rule value |

**Indexes:**
- `idx_playlist_rules_playlist` on `playlist_id`

---

### `collections`
User-defined collections (franchises, series).

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| name | TEXT | Collection name |
| description | TEXT | Description |
| type | TEXT | Type: franchise, series, custom |
| created_at | TEXT | Creation timestamp |
| poster_path | TEXT | Collection poster |
| metadata_json | TEXT | Additional metadata |

---

### `collection_items`
Media in collections.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| collection_id | INTEGER | References collections(id) |
| media_id | INTEGER | References media_files(id) |
| position | INTEGER | Order in collection |

**Constraints:**
- UNIQUE(collection_id, media_id)

**Indexes:**
- `idx_collection_items_collection` on `collection_id`

---

### `subtitles`
External subtitle files.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| media_id | INTEGER | References media_files(id) |
| file_path | TEXT | Subtitle file path |
| language | TEXT | Language code (e.g., en, es) |
| format | TEXT | Format: srt, ass, vtt |
| is_default | INTEGER | Default subtitle (0/1) |

**Indexes:**
- `idx_subtitles_media` on `media_id`

---

### `audio_tracks`
External audio tracks.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| media_id | INTEGER | References media_files(id) |
| file_path | TEXT | Audio file path |
| language | TEXT | Language code |
| codec | TEXT | Audio codec |
| channels | INTEGER | Number of channels |
| is_default | INTEGER | Default track (0/1) |

**Indexes:**
- `idx_audio_tracks_media` on `media_id`

---

### `settings`
Application settings (key-value store).

| Column | Type | Description |
|--------|------|-------------|
| key | TEXT PRIMARY KEY | Setting key |
| value | TEXT | Setting value |
| updated_at | TEXT | Last update timestamp |

**Default Settings:**
- `theme`: "dark"
- `playback_speed`: "1.0"
- `subtitle_enabled`: "true"
- `subtitle_size`: "medium"
- `auto_resume`: "true"
- `completion_threshold`: "0.95"
- `tmdb_enabled`: "false"
- `tmdb_api_key`: ""
- `tmdb_language`: "en-US"
- `tmdb_image_quality`: "original"
- `library_paths`: "[]"
- `auto_scan_interval`: "3600"

---

## TMDB Schema (Optional)

### `tmdb_media`
Links local media to TMDB IDs.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| media_id | INTEGER UNIQUE | References media_files(id) |
| tmdb_id | INTEGER | TMDB identifier |
| media_type | TEXT | Type: movie, tv, season, episode |
| match_confidence | REAL | Confidence score (0.0-1.0) |
| matched_at | TEXT | Match timestamp |
| last_synced_at | TEXT | Last TMDB sync |
| is_manual_match | INTEGER | Manual override (0/1) |

**Indexes:**
- `idx_tmdb_media_tmdb` on `tmdb_id`
- `idx_tmdb_media_type` on `media_type`

---

### `tmdb_metadata`
Cached TMDB metadata.

| Column | Type | Description |
|--------|------|-------------|
| tmdb_id | INTEGER | TMDB identifier |
| media_type | TEXT | Type: movie, tv |
| title | TEXT | Title |
| original_title | TEXT | Original title |
| overview | TEXT | Description |
| release_date | TEXT | Release date |
| runtime | INTEGER | Runtime in minutes |
| status | TEXT | Status (released, etc.) |
| tagline | TEXT | Tagline |
| vote_average | REAL | TMDB rating |
| vote_count | INTEGER | Number of votes |
| popularity | REAL | TMDB popularity score |
| imdb_id | TEXT | IMDb reference ID |
| genres_json | TEXT | Genres (JSON array) |
| keywords_json | TEXT | Keywords (JSON) |
| languages_json | TEXT | Languages (JSON) |
| production_countries_json | TEXT | Countries (JSON) |
| number_of_seasons | INTEGER | For TV shows |
| number_of_episodes | INTEGER | For TV shows |
| episode_runtime_json | TEXT | Episode runtimes (JSON) |
| fetched_at | TEXT | Fetch timestamp |

**Primary Key:** (tmdb_id, media_type)

**Indexes:**
- `idx_tmdb_metadata_title` on `title`

---

### `tmdb_cast`
Cast and crew information.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| tmdb_media_id | INTEGER | References tmdb_metadata(tmdb_id) |
| tmdb_person_id | INTEGER | TMDB person ID |
| name | TEXT | Person name |
| character | TEXT | Character name (for cast) |
| role | TEXT | Role: cast, director, writer, producer |
| order_position | INTEGER | Order in credits |
| profile_path | TEXT | TMDB profile image path |

**Indexes:**
- `idx_tmdb_cast_media` on `tmdb_media_id`
- `idx_tmdb_cast_person` on `tmdb_person_id`
- `idx_tmdb_cast_name` on `name`

---

### `tmdb_images`
Cached image metadata.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| tmdb_media_id | INTEGER | References tmdb_metadata(tmdb_id) |
| image_type | TEXT | Type: poster, backdrop, still, profile |
| file_path | TEXT | Remote TMDB path |
| local_path | TEXT | Local cached path |
| language | TEXT | Image language |
| width | INTEGER | Image width |
| height | INTEGER | Image height |
| vote_average | REAL | TMDB rating |
| is_primary | INTEGER | Primary image (0/1) |

**Indexes:**
- `idx_tmdb_images_media` on `tmdb_media_id`
- `idx_tmdb_images_type` on `image_type`

---

### `tmdb_tv_shows`
Additional TV show data.

| Column | Type | Description |
|--------|------|-------------|
| tmdb_id | INTEGER PRIMARY KEY | References tmdb_metadata(tmdb_id) |
| show_name | TEXT | Show name |
| first_air_date | TEXT | First air date |
| last_air_date | TEXT | Last air date |
| in_production | INTEGER | Still in production (0/1) |
| next_episode_to_air_json | TEXT | Next episode info (JSON) |
| networks_json | TEXT | Networks (JSON) |

---

### `tmdb_seasons`
TV season data.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| tmdb_show_id | INTEGER | References tmdb_tv_shows(tmdb_id) |
| season_number | INTEGER | Season number |
| tmdb_id | INTEGER | TMDB season ID |
| name | TEXT | Season name |
| overview | TEXT | Description |
| air_date | TEXT | Air date |
| episode_count | INTEGER | Number of episodes |
| poster_path | TEXT | Season poster |

**Constraints:**
- UNIQUE(tmdb_show_id, season_number)

**Indexes:**
- `idx_tmdb_seasons_show` on `tmdb_show_id`

---

### `tmdb_episodes`
TV episode data.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER PRIMARY KEY | Unique identifier |
| tmdb_show_id | INTEGER | References tmdb_tv_shows(tmdb_id) |
| season_number | INTEGER | Season number |
| episode_number | INTEGER | Episode number |
| tmdb_id | INTEGER | TMDB episode ID |
| name | TEXT | Episode name |
| overview | TEXT | Description |
| air_date | TEXT | Air date |
| runtime | INTEGER | Runtime in minutes |
| still_path | TEXT | Episode still image |
| vote_average | REAL | TMDB rating |

**Constraints:**
- UNIQUE(tmdb_show_id, season_number, episode_number)

**Indexes:**
- `idx_tmdb_episodes_show` on `tmdb_show_id`
- `idx_tmdb_episodes_season` on `(tmdb_show_id, season_number)`

---

### `tmdb_collections`
Movie collections/franchises.

| Column | Type | Description |
|--------|------|-------------|
| tmdb_id | INTEGER PRIMARY KEY | TMDB collection ID |
| name | TEXT | Collection name |
| overview | TEXT | Description |
| poster_path | TEXT | Collection poster |
| backdrop_path | TEXT | Collection backdrop |

---

### `tmdb_collection_items`
Movies in collections.

| Column | Type | Description |
|--------|------|-------------|
| collection_id | INTEGER | References tmdb_collections(tmdb_id) |
| tmdb_movie_id | INTEGER | TMDB movie ID |

**Primary Key:** (collection_id, tmdb_movie_id)

---

### `tmdb_settings`
TMDB-specific settings.

| Column | Type | Description |
|--------|------|-------------|
| key | TEXT PRIMARY KEY | Setting key |
| value | TEXT | Setting value |
| updated_at | TEXT | Last update timestamp |

---

## Migration System

The database uses a simple migration system tracked by the `schema_version` table:

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Current Version: 1
- Initial schema with all core and TMDB tables
- Default settings initialized

### Future Migrations
- Version 2+: TBD based on feature requirements

---

## Performance Considerations

1. **Indexes**: All foreign keys and frequently queried columns are indexed
2. **WAL Mode**: Enables better read/write concurrency
3. **Foreign Keys**: Enforced for referential integrity
4. **Soft Deletes**: `is_deleted` flag prevents data loss
5. **JSON Fields**: Flexible metadata storage without schema changes

---

## Usage Examples

### Query recently played media
```sql
SELECT m.*, p.last_played_at
FROM media_files m
JOIN playback_state p ON m.id = p.media_id
WHERE m.is_deleted = 0
ORDER BY p.last_played_at DESC
LIMIT 10;
```

### Find unwatched movies
```sql
SELECT m.*
FROM media_files m
LEFT JOIN playback_state p ON m.id = p.media_id
WHERE m.media_type = 'movie'
  AND m.is_deleted = 0
  AND (p.completed IS NULL OR p.completed = 0)
ORDER BY m.indexed_at DESC;
```

### Get playlist with items
```sql
SELECT p.*, m.title, m.duration
FROM playlists p
JOIN playlist_items pi ON p.id = pi.playlist_id
JOIN media_files m ON pi.media_id = m.id
WHERE p.id = ?
ORDER BY pi.position;
```
