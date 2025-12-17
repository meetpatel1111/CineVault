# CineVault

A privacy-first, offline-only media management and playback application.

## Features

- 🎬 Local media discovery and indexing (Movies, TV Shows, Music)
- ▶️ Advanced playback with hardware acceleration
- 📝 Resume playback and watch history
- 🎵 Subtitle and audio track management
- 📊 Offline analytics and insights
- 💾 Portable backups
- 🎯 Optional TMDB integration for metadata enrichment

## Tech Stack

- **Frontend**: React + TypeScript
- **Backend**: Rust + Tauri
- **Database**: SQLite
- **Playback**: FFmpeg / libVLC

## Development

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- npm or yarn

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Design Documents

- [Core Design](./offline_media_library_core_design.md)
- [TMDB Extension](./offline_media_library_tmdb_extension.md)

## License

TBD
