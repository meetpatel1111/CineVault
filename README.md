# 🎬 CineVault

<p align="center">
  <img src="src-tauri/icons/icon.svg" alt="CineVault Icon" width="128" height="128">
</p>

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

# Generate application icons (first time only)
npm run generate-icons

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Icon Generation

CineVault includes a custom-designed icon. To regenerate all icon formats:

```bash
npm run generate-icons
```

For detailed instructions, see [ICON_GENERATION.md](./ICON_GENERATION.md).

You can also preview the icon by opening `tmp_rovodev_icon_preview.html` in your browser.

## Design Documents

- [Core Design](./offline_media_library_core_design.md)
- [TMDB Extension](./offline_media_library_tmdb_extension.md)

## License

TBD
