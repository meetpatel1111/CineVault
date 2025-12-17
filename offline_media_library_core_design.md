# Offline Media Library & Smart Player – Core Design

## 1. Product Vision
A privacy-first, offline-only media management and playback application that indexes local audio and video libraries, delivers high-performance playback, and provides intelligent organization without any dependency on cloud services or user accounts.

---

## 2. Target Users
- Users with large local media collections
- Privacy-conscious individuals
- Offline home media servers
- Professionals and travelers

---

## 3. Core Capabilities
- Local media discovery and indexing
- Advanced audio/video playback
- Smart offline playlists
- Resume playback and watch history
- Subtitle and audio track management
- Offline analytics and insights
- Portable backups

---

## 4. Supported Media Formats

### Video
MP4, MKV, AVI, MOV, WEBM (H.264, HEVC, AV1)

### Audio
MP3, FLAC, WAV, AAC, OGG

### Subtitles
SRT, ASS, VTT (embedded and external)

---

## 5. High-Level Architecture
```
UI (React / Flutter)
↓
Media Controller
↓
Playback Engine (FFmpeg / libVLC)
↓
Indexer
↓
SQLite Metadata Store
↓
Local File System
```

---

## 6. Functional Modules

### 6.1 Media Discovery & Indexing
- Recursive folder scanning
- Incremental re-indexing
- File hash based duplicate detection
- Local metadata extraction (codec, duration, bitrate, resolution)

### 6.2 Media Library Management
- Movies / TV / Music separation
- Custom collections
- Filters (resolution, codec, duration, watched state)
- Sorting (recent, alphabetical, size, duration)

### 6.3 Smart Playlists
- Rule-based playlists
- Manual playlists
- Auto-generated collections

### 6.4 Playback Engine
- Hardware-accelerated decoding
- Speed control
- Subtitle and audio sync
- Aspect ratio and zoom control

### 6.5 Resume Playback & History
- Per-file resume
- Watch completion tracking
- Playback history

### 6.6 Subtitle Management
- Embedded and external subtitle detection
- Subtitle styling and language preference

### 6.7 Offline Analytics
- Most watched media
- Playback duration statistics
- Usage trends

### 6.8 Backup & Portability
- Metadata export/import
- Playlist export
- Library relinking on restore

---

## 7. Database Overview (SQLite)

### media_files
- id
- file_path
- file_hash
- media_type
- duration
- size
- codec
- resolution

### playback_state
- media_id
- last_position
- completed
- last_played_at

### playlists
- id
- name
- type

### playlist_rules
- playlist_id
- rule_json

---

## 8. UI / UX Principles
- Dark mode first
- Keyboard-friendly
- Grid and list views
- Minimal, distraction-free design

---

## 9. Recommended Tech Stack
- Desktop: Tauri + Rust + React
- Database: SQLite (WAL mode)
- Playback: FFmpeg / libVLC

---

## 10. Roadmap
- Phase 1: Core playback and indexing
- Phase 2: Playlists and analytics
- Phase 3: Backup, performance tuning, packaging

