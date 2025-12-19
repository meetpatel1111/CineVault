# 🎬 CineVault

<p align="center">
  <img src="src-tauri/icons/icon.svg" alt="CineVault Icon" width="128" height="128">
</p>

A privacy-first, offline-only media management and playback application.

## Features

- 🎬 **Local Media Discovery**: Recursively scans and indexes Movies, TV Shows, and Music.
- ▶️ **Advanced Playback**:
  - HTML5 Player for standard formats.
  - Optional LibVLC integration for MKV/AVI support.
  - Hardware acceleration.
- 🧠 **Smart Playlists**: Dynamic playlists based on rules (Genre, Year, Duration, etc.).
- 📝 **Watch Tracking**: Resume playback, watch history, and completion status.
- 🎵 **Track Management**: Support for multiple subtitles and audio tracks.
- 📊 **Offline Analytics**: Visual dashboard for watch habits and library stats.
- 💾 **Backup & Restore**: Portable database export for data safety.
- 🖼️ **Metadata Extraction**: Automatic thumbnail generation and file analysis.

## Tech Stack

- **Frontend**: React + TypeScript + Recharts
- **Backend**: Rust + Tauri + SQLite
- **Playback**: FFmpeg / libVLC (Optional)

## Development

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- npm or yarn
- **Linux only:** GTK development libraries

### Linux System Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf ffmpeg libvlc-dev
```

### Setup

```bash
# Install dependencies
npm install

# Generate icons
npm run generate-icons

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Icon Generation

To regenerate icons from `src-tauri/icons/icon.svg`:
```bash
npm run generate-icons
```

## Build Status

- ✅ **Core Features**: Complete
- ✅ **Extensions**: Analytics, Backup, Smart Playlists Implemented
- ✅ **Status**: Release Candidate Ready

See [CURRENT_IMPLEMENTATION_STATUS.md](./CURRENT_IMPLEMENTATION_STATUS.md) for details.

## Design Documents

- [Core Design](./offline_media_library_core_design.md)
- [TMDB Extension](./offline_media_library_tmdb_extension.md)

## License

TBD
