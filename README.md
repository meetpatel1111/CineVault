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
- **Linux only:** GTK development libraries (see below)

### Linux System Dependencies

If you're developing on Linux, install these system packages:

```bash
# Ubuntu/Debian
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

# Fedora
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

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

### Build Status

All compilation errors have been resolved:
- ✅ Rust code compiles on all platforms
- ✅ Professional icon system implemented
- ✅ CI/CD workflows configured with proper dependencies

See [ALL_FIXES_SUMMARY.md](./ALL_FIXES_SUMMARY.md) for complete details.

## Design Documents

- [Core Design](./offline_media_library_core_design.md)
- [TMDB Extension](./offline_media_library_tmdb_extension.md)

## License

TBD
