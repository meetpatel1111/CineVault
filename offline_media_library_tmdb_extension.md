# Offline Media Library – TMDB Enrichment Extension

## 1. Purpose
This document defines an **optional, additive TMDB integration layer** for the Offline Media Library & Smart Player. All features described here are strictly optional and do not alter or replace core offline functionality.

---

## 2. Design Principles
- Offline-first remains default
- TMDB is used only for metadata enrichment
- All fetched data is cached locally
- User-supplied TMDB API key
- TMDB can be disabled at any time

---

## 3. Supported TMDB Entities
- Movies
- TV Shows
- Seasons
- Episodes
- Cast & Crew

---

## 4. Intelligent Media Matching

### Features
- Filename parsing (title, year, season, episode)
- TMDB search and fuzzy matching
- Match confidence scoring
- Manual rematch and override

### Stored Locally
- tmdb_id
- match_confidence
- matched_at

---

## 5. Rich Metadata Enrichment

### Movies
- Title and overview
- Release date
- Genres
- Runtime
- Certification
- Language
- IMDb reference ID

### TV Shows
- Show overview
- Season and episode metadata
- Episode titles and summaries

---

## 6. Artwork & Image Management
- Movie posters
- TV show and season posters
- Episode stills
- Backdrops

### Offline Handling
- Images cached locally
- Configurable resolution
- Cache size management

---

## 7. Cast & Crew Explorer
- Actor profiles and images
- Role and character names
- Browse local media by actor or director

---

## 8. Auto Collections & Classification
- Genre-based collections
- Studio-based collections
- Movie series / franchise grouping

---

## 9. Ratings & Popularity (Read-Only)
- TMDB rating
- Vote count
- Popularity score
- Sorting and filtering support

---

## 10. Trailer Management
- Trailer metadata fetch
- Optional local trailer download
- Offline trailer playback

---

## 11. Localization Support
- Metadata language selection
- Poster language preference
- Multi-language overview storage

---

## 12. Smart Recommendations (Local Logic)
- Genre similarity
- Cast and director overlap
- Watch history
- TMDB popularity as a local signal

---

## 13. Settings – TMDB
- Enable / disable TMDB
- API key management
- Preferred language
- Image resolution
- Auto-fetch control
- Cache management

---

## 14. Database Additions

### tmdb_media
- media_id
- tmdb_id
- media_type
- match_confidence
- last_synced_at

### tmdb_metadata
- tmdb_id
- title
- overview
- genres_json
- release_date
- runtime
- rating
- popularity

### tmdb_cast
- tmdb_id
- person_id
- name
- role
- profile_image_path

### tmdb_images
- tmdb_id
- image_type
- local_path
- language
- resolution

---

## 15. Failure & Offline Behavior
| Scenario | Behavior |
|-------|---------|
| No internet | Core app works normally |
| TMDB unavailable | Cached metadata used |
| API key removed | TMDB features disabled |
| Quota exceeded | Graceful pause |

---

## 16. Monetization Alignment (Optional)
TMDB-powered features can be positioned as Pro capabilities while core playback remains free.

---

## 17. Implementation Phases
- Phase 1: Search, matching, posters
- Phase 2: TV metadata, cast explorer
- Phase 3: Recommendations, trailers, localization

