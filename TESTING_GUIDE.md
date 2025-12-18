# CineVault Testing Guide

## Quick Start

### 1. Start the Application
```bash
npm run tauri dev
```

### 2. Initial Setup
- The app will launch with the beautiful dark-themed UI
- You'll see a welcome screen with "Scan Library" button
- Database is automatically initialized

---

## Test Scenarios

### ✅ Test 1: Library Scanning

**Steps:**
1. Click the **"Scan Library"** button in the top right
2. Select a folder containing media files (movies, TV shows, or music)
3. Watch the real-time progress: "Scanning: X / Y files - filename.mp4"
4. Wait for success notification: "Scan complete! Found X files, Y added..."

**Expected Results:**
- Progress updates in real-time
- Toast notification on completion
- Media cards appear in "Recently Added" section
- Database stats update (shown in header subtitle)
- Titles parsed from filenames (e.g., "The Matrix 1999" → "The Matrix" (1999))
- **Thumbnails** generated for videos (if FFmpeg is installed)

**Test Files:**
- Movies: `The.Matrix.1999.1080p.mp4`
- TV Shows: `Breaking.Bad.S01E05.mkv`
- Music: `Song.Title.mp3`

---

### ✅ Test 2: Media Playback (Video)

**Steps:**
1. Click any movie/video card in the grid
2. Video player opens fullscreen
3. Video starts playing automatically

**Expected Results:**
- Fullscreen video player
- Title shown in top bar
- Controls visible (play/pause, seek, volume, speed, fullscreen)
- Progress bar shows playback position
- Time display shows current/total duration

**Player Controls to Test:**
- Click video to pause/play
- Drag progress bar to seek
- Adjust volume slider
- Change playback speed (dropdown: 0.5x-2x)
- Click fullscreen button

**Advanced Playback (Optional):**
- If built with `vlc` feature and `libvlc` installed, test playing MKV/AVI files.
- Otherwise, check that MP4 plays via HTML5.

**Keyboard Shortcuts:**
- `Space`: Play/pause
- `Arrow Left/Right`: Seek ±10 seconds
- `Arrow Up/Down`: Volume ±10%
- `F`: Toggle fullscreen
- `M`: Mute/unmute
- `Escape`: Close player

---

### ✅ Test 3: Resume Playback

**Steps:**
1. Play a video for 30-60 seconds
2. Close the player (Escape or X button)
3. Click the same video again

**Expected Results:**
- Toast notification: "Resuming from Xm Ys"
- Video starts from where you left off
- Position saved every 5 seconds

---

### ✅ Test 4: Smart Playlists

**Steps:**
1. Navigate to "Playlists" section
2. Create a new playlist with type "Smart" (backend API only currently, or mock via dev tools)
3. Add a rule (e.g., "year > 2000")
4. Verify the playlist automatically populates with matching media

**Verification via Console:**
```javascript
// Example code to run in dev console to test smart playlist
await window.__TAURI__.invoke('create_playlist', { name: 'Smart Movies', playlistType: 'smart' });
// Add rule: media_type = movie
await window.__TAURI__.invoke('add_playlist_rule', { playlistId: 1, ruleType: 'media_type', operator: 'equals', value: 'movie' });
// Check results
await window.__TAURI__.invoke('get_playlist_media', { playlistId: 1 });
```

---

### ✅ Test 5: Subtitle Management

**Steps:**
1. Hover over a media card
2. Click the "Actions" dropdown
3. Select "Manage Subtitles"
4. Add a local `.srt` file

**Expected Results:**
- Subtitle file is added to database
- Toast notification appears

---

### ✅ Test 6: Collections & Playlists

**Steps:**
1. Create a "Manual Playlist"
2. Add items to it via the Media Card dropdown
3. Verify items appear in the Playlist Detail view
4. Create a "Collection"
5. Add items and verify grouping

---

## Database Verification

### Check Saved Data
The database is stored at: `%APPDATA%/com.cinevault.app/cinevault.db`

You can use SQLite tools to inspect:
```bash
sqlite3 "%APPDATA%/com.cinevault.app/cinevault.db"
```

**Check tables:**
```sql
-- View all media files
SELECT title, year, media_type FROM media_files LIMIT 10;

-- View playlist rules
SELECT * FROM playlist_rules;

-- View subtitle tracks
SELECT * FROM subtitle_tracks;
```

---

## Known Limitations

### HTML5 Player Limitations
✅ **Supported Formats:**
- Video: MP4 (H.264), WebM, OGG
- Audio: MP3, WAV, FLAC, M4A, OGG

⚠️ **Requires LibVLC (Backend Feature):**
- MKV files
- AVI files
- Advanced codecs (HEVC, AV1)

### Features Not Yet Implemented
- ❌ TMDB metadata fetching
- ❌ Advanced filtering/sorting in UI (Backend ready)
- ❌ Export/backup functionality

---

## Performance Testing

### Large Libraries
Test with different library sizes:
- Small: < 100 files
- Medium: 100-1,000 files
- Large: 1,000-10,000 files

**What to Check:**
- Scan speed (should process ~50-100 files/second)
- UI responsiveness
- Search speed
- Grid scroll performance

---

## Have Fun Testing! 🎬

CineVault is your privacy-first, offline media library. Enjoy exploring your media collection!
