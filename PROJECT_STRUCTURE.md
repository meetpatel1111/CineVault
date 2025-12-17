# CineVault Project Structure

```
cinevault/
├── src/                          # React frontend source
│   ├── main.tsx                 # Application entry point
│   ├── App.tsx                  # Main App component
│   ├── App.css                  # App styles
│   └── styles.css               # Global styles
│
├── src-tauri/                    # Rust backend source
│   ├── src/
│   │   └── main.rs              # Tauri application entry
│   ├── icons/                   # Application icons
│   ├── Cargo.toml               # Rust dependencies
│   ├── build.rs                 # Build script
│   └── tauri.conf.json          # Tauri configuration
│
├── dist/                         # Production build output (generated)
├── node_modules/                 # Node dependencies (generated)
├── target/                       # Rust build output (generated)
│
├── package.json                  # Node.js project configuration
├── tsconfig.json                 # TypeScript configuration
├── vite.config.ts               # Vite build configuration
├── index.html                   # HTML entry point
├── .gitignore                   # Git ignore rules
└── README.md                    # Project documentation

## Planned Structure (Future Modules)

### Frontend Structure
```
src/
├── components/                  # React components
│   ├── Library/                # Media library browser
│   ├── Player/                 # Video/audio player
│   ├── Playlists/              # Playlist management
│   └── Settings/               # Settings UI
├── hooks/                      # Custom React hooks
├── services/                   # API service layer
├── types/                      # TypeScript type definitions
└── utils/                      # Utility functions
```

### Backend Structure
```
src-tauri/src/
├── main.rs                     # Application entry
├── db/                         # Database module
│   ├── mod.rs
│   ├── schema.rs              # Database schema
│   └── migrations.rs          # Database migrations
├── indexer/                    # Media indexing module
│   ├── mod.rs
│   ├── scanner.rs             # File system scanner
│   └── metadata.rs            # Metadata extraction
├── player/                     # Playback engine integration
│   ├── mod.rs
│   └── ffmpeg.rs              # FFmpeg wrapper
├── tmdb/                       # TMDB integration (optional)
│   ├── mod.rs
│   ├── client.rs              # API client
│   └── matcher.rs             # Media matching logic
└── utils/                      # Shared utilities
    ├── mod.rs
    └── file_hash.rs           # File hashing utilities
```

## Technology Stack

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 1.5
- **Database**: SQLite (to be added)
- **Playback**: FFmpeg/libVLC (to be integrated)
- **State Management**: React Context/Hooks (or Zustand/Redux later)
- **Styling**: CSS (consider Tailwind or CSS-in-JS later)

## Development Workflow

1. **Frontend Development**: `npm run dev` (runs Vite dev server)
2. **Full App Development**: `npm run tauri dev` (runs Tauri with hot reload)
3. **Production Build**: `npm run tauri build`
4. **Rust Only**: `cd src-tauri && cargo build`

## Next Steps

- [ ] Add SQLite dependencies and database module
- [ ] Create database schema and migrations
- [ ] Implement media indexing module
- [ ] Add FFmpeg/libVLC for playback
- [ ] Build UI components
