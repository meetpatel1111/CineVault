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
- **Local media discovery and indexing**
- **Advanced audio/video playback**
- **Smart offline playlists**
- **Resume playback and watch history**
- **Subtitle and audio track management**
- **Offline analytics and insights**
- **Portable backups**
- **Multi-user profiles (Local)**
- **Metadata editing and overrides**
- **Media casting and transcoding**

---

## 4. Supported Media Formats

### Video
- **Containers**: MP4, MKV, AVI, MOV, WEBM, M4V, TS, FLV
- **Codecs**: H.264, HEVC (H.265), AV1, VP9, MPEG-2, ProRes

### Audio
- **Lossy**: MP3, AAC, OGG, WMA, OPUS
- **Lossless**: FLAC, ALAC, WAV, AIFF, DSD

### Subtitles
- **Text**: SRT, ASS/SSA, VTT, SUB, MPL2
- **Image-based**: PGS (Blu-ray), VobSub (DVD)
- **Features**: Embedded and external file support

---

## 5. High-Level Architecture
```
UI (React / Flutter)
↓
Media Controller (State & Profiles)
↓
Playback Engine (FFmpeg / libVLC / MPV)
↓
Indexer & Transcoder
↓
SQLite Metadata Store
↓
Local File System
```

---

## 6. Functional Modules

### 6.1 Media Discovery & Indexing
- **Recursive Scan**: Deep folder scanning with ignore patterns.
- **Incremental Indexing**: Fast re-scan based on file modification times.
- **Hashing**: SHA256/XXHash for duplicate detection and file integrity.
- **Extraction**: FFmpeg-based extraction of codec, bitrate, resolution, color space, and HDR metadata.

### 6.2 Media Library Management
- **Categorization**: Auto-sort into Movies, TV (Season/Episode), Music (Artist/Album).
- **Collections**: Manual collections and auto-grouped "Box Sets" (e.g., Harry Potter Collection).
- **Metadata Editor**: Manual override for titles, sort titles, dates, and posters.
- **Filtering**: Advanced boolean logic (Codec=HEVC AND Year>2020).

### 6.3 Smart Playlists
- **Rule Engine**: Construct playlists based on metadata rules.
- **Operators**: Equals, Contains, Starts With, Greater Than, Less Than, Date Ranges.
- **Dynamic Updates**: Playlists update automatically as library changes.

### 6.4 Playback Engine
- **Hardware Acceleration**: VAAPI, NVDEC, DXVA2 support.
- **Speed Control**: 0.25x to 4.0x with pitch correction.
- **Sync Adjustments**: Audio and subtitle delay offset (±ms).
- **Post-Processing**: Deinterlacing, color correction, sharpening.
- **A-B Loop**: Repeat specific sections of media.

### 6.5 Resume Playback & History
- **Granular History**: Exact stop time tracking per file.
- **Completion Logic**: Configurable threshold (e.g., 90%) for "Watched" status.
- **Session Logging**: Track *when* a file was watched for analytics.

### 6.6 Subtitle & Audio Management
- **Auto-Select**: Language preference logic (e.g., prefer "English" audio, no subs).
- **External Files**: Auto-load `movie.srt`, `movie.en.srt`.
- **Styling**: Custom font, size, color, and background opacity for text subtitles.
- **Lyrics**: Synchronized (`.lrc`) and unsynchronized lyrics support for music.

### 6.7 Offline Analytics
- **Dashboard**: Visual charts for library growth and watch habits.
- **Insights**: "Most re-watched movies", "Favorite Directors", "Binge Velocity".
- **Heatmaps**: Calendar view of watching activity.

### 6.8 Backup & Portability
- **Database Dump**: Export full SQLite DB to JSON or compressed format.
- **Portable Mode**: Run entirely from a USB stick with relative paths.
- **Path Remapping**: Tool to fix broken paths after moving library.

### 6.9 Multi-User Profiles (Local)
- **Profile Switching**: Password-protected local user profiles.
- **Isolation**: Separate watch history, playlists, and settings per user.
- **Parental Controls**: Restrict libraries or ratings per profile.

### 6.10 Transcoding & Optimization
- **Optimize for Mobile**: Convert 4K HDR to 1080p SDR for mobile syncing.
- **Pre-Transcode**: Background processing for incompatible formats.
- **Container Swapping**: Remux MKV to MP4 without re-encoding streams.

### 6.11 Casting & Remote Play
- **DLNA/UPnP**: Server mode to stream to Smart TVs.
- **Web UI**: Optional local HTTP server for remote browser playback.
- **Remote Control**: Control desktop player via mobile app API.

---

## 7. Database Overview (SQLite)

### media_files
- id, path, hash, type, size, created_at
- metadata_json (dynamic fields)

### playback_state
- media_id, user_id (for profiles), position, status

### profiles
- id, name, avatar_path, is_restricted, pin_hash

### playlists & rules
- Supports definition per profile

### metadata_overrides
- Stores user edits separately from scanned data

---

## 8. UI / UX Principles
- **Theater Mode**: Dim UI, focus on content.
- **Keyboard-First**: Full navigation via arrow keys and shortcuts (J/K/L, Space).
- **Density Control**: Compact vs. Comfortable list views.
- **Responsive**: Adaptive layout for resizing and portrait mode (music).

---

## 9. Recommended Tech Stack
- **Core**: Rust + Tauri
- **Frontend**: React + TypeScript + Recharts
- **Database**: SQLite (WAL mode)
- **Media Engine**: FFmpeg (Probe/Transcode) + LibVLC (Playback)

---

## 10. Roadmap

### Phase 1: Foundation (Completed)
- Core indexing, Database, HTML5 Playback, UI Basics.

### Phase 2: Intelligence (Current)
- Smart Playlists, Analytics, Metadata Extraction, Backup.

### Phase 3: Advanced Media (Planned)
- LibVLC full integration, Audio switching, Subtitle styling.

### Phase 4: Expansion (Planned)
- Profiles, Metadata Editor, Transcoding, Casting.
