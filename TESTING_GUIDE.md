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

**To Verify:**
- Console logs: "Progress: X/Y" every 5 seconds
- Position persists across app restarts

---

### ✅ Test 4: Watch Completion

**Steps:**
1. Play a video
2. Seek to ~95% of duration
3. Let it play to the end

**Expected Results:**
- Automatically marked as completed
- Console log: "Marked as completed: [title]"
- Watch count incremented in database

---

### ✅ Test 5: Audio Playback

**Steps:**
1. Scan a folder with music files
2. Click a music track

**Expected Results:**
- Audio player appears at bottom of screen
- Shows album artwork placeholder (🎵)
- Track title displayed
- Controls: Play/pause, skip ±10s, volume
- Progress bar updates
- Can continue browsing library while music plays

---

### ✅ Test 6: Search Functionality

**Steps:**
1. Click the search bar in top navigation
2. Type part of a movie/show title
3. Press Enter

**Expected Results:**
- Search results appear (not yet implemented in UI, but backend is ready)
- Searches both titles and filenames

---

### ✅ Test 7: Settings Panel

**Steps:**
1. Click the **Settings** button (top right) or sidebar
2. Navigate through tabs: General, Library, Playback, TMDB

**Expected Results:**
- Modal opens with 4 tabs
- Settings displayed:
  - General: Theme selection, startup options
  - Library: Folder paths, scan interval
  - Playback: Speed, auto-resume, completion threshold
  - TMDB: API key, language, image quality

---

### ✅ Test 8: UI Components

**Test Dropdowns:**
1. Click "Actions" dropdown button
2. Try menu items: Play, View Details, Add to Playlist, Remove

**Test Toasts:**
- Automatic toasts appear on actions
- 4 types: Success (green), Error (red), Warning (orange), Info (blue)
- Auto-dismiss after 5 seconds
- Closeable with X button

**Test Badges:**
- "Beta" badge (blue)
- Item count badges (green/orange)

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

-- View playback states
SELECT m.title, p.last_position, p.duration, p.completed 
FROM playback_state p 
JOIN media_files m ON p.media_id = m.id;

-- View watch history
SELECT COUNT(*) as total_sessions FROM playback_history;
```

---

## Known Limitations (Current Version)

### HTML5 Player Limitations
✅ **Supported Formats:**
- Video: MP4 (H.264), WebM, OGG
- Audio: MP3, WAV, FLAC, M4A, OGG

⚠️ **Not Yet Supported:**
- MKV files (need FFmpeg/libVLC integration)
- AVI files
- Advanced codecs (HEVC, AV1)
- Embedded subtitles

### Features Not Yet Implemented
- ❌ Subtitle loading/display
- ❌ Multiple audio track selection
- ❌ Smart playlists
- ❌ Collections/grouping
- ❌ TMDB metadata fetching
- ❌ Advanced filtering/sorting in UI
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

### Memory Usage
- Check Task Manager during playback
- Memory should stay reasonable (< 500MB for typical usage)

---

## Troubleshooting

### Video Won't Play
**Possible causes:**
1. Unsupported codec (MKV, AVI not supported by HTML5)
2. File path issues
3. File permissions

**Solution:**
- Check browser console for errors
- Try MP4 files first
- Check file exists at scanned path

### Playback Position Not Saving
**Possible causes:**
1. Media ID not parsed correctly
2. Database write error

**Solution:**
- Check console logs for "Failed to save playback position"
- Verify database file exists and is writable

### Scan Not Finding Files
**Possible causes:**
1. Wrong folder selected
2. Hidden files/folders (starting with .)
3. No supported media files

**Solution:**
- Select folder with visible media files
- Check file extensions are supported

---

## Success Criteria

✅ **Minimum Viable Product:**
- [x] Scan directory and index files
- [x] Display media in grid
- [x] Play video files (MP4)
- [x] Play audio files (MP3)
- [x] Resume playback from last position
- [x] Beautiful dark UI
- [x] Responsive design

🎉 **All core features are working!**

---

## Next Steps After Testing

Based on test results:
1. **If Everything Works:** Celebrate! 🎉 Consider adding polish features
2. **If Issues Found:** Document bugs and prioritize fixes
3. **Performance Issues:** Run optimization (Task 13)
4. **Missing Features:** Prioritize from remaining tasks

---

## Reporting Issues

When reporting bugs, please include:
- Steps to reproduce
- Expected vs actual behavior
- Console logs (F12 → Console tab)
- Media file format/codec
- Operating system

---

## Have Fun Testing! 🎬

CineVault is your privacy-first, offline media library. Enjoy exploring your media collection!
