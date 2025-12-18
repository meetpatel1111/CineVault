import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { 
  MainLayout, 
  MediaGrid, 
  Button, 
  SettingsPanel,
  ToastContainer,
  useToast,
  Badge,
  Spinner,
  VideoPlayer,
  AudioPlayer,
  PlaylistList,
  PlaylistDetail,
  AddToPlaylistModal,
  CollectionList,
  CollectionDetail,
  AddToCollectionModal,
  type DropdownItem
} from "./components";
import { mediaService, type ScanProgress } from "./services/mediaService";
import { playbackService } from "./services/playbackService";
import { playlistService, type Playlist } from "./services/playlistService";
import { collectionService, type Collection } from "./services/collectionService";
import "./App.css";

// Mock data for demonstration
const mockMediaItems = [
  {
    id: '1',
    title: 'The Shawshank Redemption',
    year: 1994,
    duration: 142,
    type: 'movie' as const,
    rating: 9.3,
    watched: true,
  },
  {
    id: '2',
    title: 'Inception',
    year: 2010,
    duration: 148,
    type: 'movie' as const,
    rating: 8.8,
    progress: 65,
  },
  {
    id: '3',
    title: 'Breaking Bad',
    year: 2008,
    duration: 49,
    type: 'tv' as const,
    rating: 9.5,
    progress: 45,
  },
  {
    id: '4',
    title: 'The Dark Knight',
    year: 2008,
    duration: 152,
    type: 'movie' as const,
    rating: 9.0,
  },
  {
    id: '5',
    title: 'Interstellar',
    year: 2014,
    duration: 169,
    type: 'movie' as const,
    rating: 8.6,
  },
  {
    id: '6',
    title: 'Stranger Things',
    year: 2016,
    duration: 51,
    type: 'tv' as const,
    rating: 8.7,
    watched: true,
  },
];

function App() {
  const [dbStats, setDbStats] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [scanProgress, setScanProgress] = useState<ScanProgress | null>(null);
  const [mediaItems, setMediaItems] = useState<any[]>(mockMediaItems);
  const [filteredMediaItems, setFilteredMediaItems] = useState<any[]>(mockMediaItems);
  const [playingMedia, setPlayingMedia] = useState<any | null>(null);
  const [currentFilter, setCurrentFilter] = useState<'all' | 'movie' | 'tv' | 'music'>('all');
  const [currentSection, setCurrentSection] = useState<string>('home');
  const [selectedPlaylist, setSelectedPlaylist] = useState<Playlist | null>(null);
  const [selectedCollection, setSelectedCollection] = useState<Collection | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [addToPlaylistMediaId, setAddToPlaylistMediaId] = useState<number | null>(null);
  const [addToCollectionMediaId, setAddToCollectionMediaId] = useState<number | null>(null);
  const [manageSubtitlesMediaId, setManageSubtitlesMediaId] = useState<number | null>(null);
  const [manageSubtitlesMediaPath, setManageSubtitlesMediaPath] = useState<string>('');
  const { toasts, removeToast, success, error, info } = useToast();

  async function loadDbStats() {
    try {
      const stats = await invoke<string>("get_db_stats");
      setDbStats(stats);
    } catch (err) {
      setDbStats(`Database initialized (0 media files)`);
    } finally {
      setLoading(false);
    }
  }

  async function loadMedia() {
    try {
      const files = await mediaService.getAllMedia();
      const displayItems = files.map(f => mediaService.toDisplayFormat(f));
      const items = displayItems.length > 0 ? displayItems : mockMediaItems;
      setMediaItems(items);
      setFilteredMediaItems(items);
    } catch (err) {
      console.error("Failed to load media:", err);
      error("Failed to load media library");
    }
  }

  const handleSearch = (query: string) => {
    setSearchQuery(query);
    applyFilters(query, currentFilter);
  };

  const handleSectionChange = (section: string) => {
    setCurrentSection(section);
    if (section === 'playlists') {
      setSelectedPlaylist(null);
    }
    if (section === 'collections') {
      setSelectedCollection(null);
    }
  };

  const handleFilterChange = (filter: 'all' | 'movie' | 'tv' | 'music') => {
    // If we're not on home/movies/tv/music section, switch to it
    if (currentSection === 'playlists' || currentSection === 'collections') {
      const filterToSection: Record<'all' | 'movie' | 'tv' | 'music', string> = {
        'all': 'home',
        'movie': 'movies',
        'tv': 'tv',
        'music': 'music',
      };
      setCurrentSection(filterToSection[filter]);
    }

    setCurrentFilter(filter);
    applyFilters(searchQuery, filter);
  };

  const applyFilters = (query: string, filter: 'all' | 'movie' | 'tv' | 'music') => {
    let filtered = [...mediaItems];

    // Apply type filter
    if (filter !== 'all') {
      filtered = filtered.filter(item => item.type === filter);
    }

    // Apply search query
    if (query.trim()) {
      const lowerQuery = query.toLowerCase();
      filtered = filtered.filter(item => 
        item.title?.toLowerCase().includes(lowerQuery) ||
        item.year?.toString().includes(query)
      );
    }

    setFilteredMediaItems(filtered);
  };

  useEffect(() => {
    loadDbStats();
    loadMedia();

    // Listen for scan progress events
    const unlisten = listen<ScanProgress>('scan-progress', (event) => {
      setScanProgress(event.payload);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const handleMediaClick = async (item: any) => {
    console.log('Media clicked:', item);
    
    // Get playback state to resume from last position
    try {
      const state = await playbackService.getPlaybackState(parseInt(item.id));
      if (state && state.last_position > 0 && !state.completed) {
        item.resumePosition = state.last_position;
        info(`Resuming from ${Math.floor(state.last_position / 60)}m ${Math.floor(state.last_position % 60)}s`);
      }
    } catch (err) {
      console.error('Failed to get playback state:', err);
    }
    
    setPlayingMedia(item);
    info(`Playing: ${item.title}`);
  };

  const handleClosePlayer = async () => {
    if (playingMedia) {
      // Note: Final position will be saved via handlePlayerProgress
      console.log('Closing player for:', playingMedia.title);
    }
    setPlayingMedia(null);
  };

  const handlePlayerProgress = async (position: number, duration: number) => {
    if (!playingMedia) return;
    
    const mediaId = parseInt(playingMedia.id);
    
    // Save position every 5 seconds
    if (Math.floor(position) % 5 === 0) {
      try {
        await playbackService.updatePosition(mediaId, position, duration);
      } catch (err) {
        console.error('Failed to save playback position:', err);
      }
    }
    
    // Check if should mark as completed (95% threshold)
    if (playbackService.shouldMarkCompleted(position, duration)) {
      try {
        await playbackService.markAsCompleted(mediaId, duration);
        console.log('Marked as completed:', playingMedia.title);
      } catch (err) {
        console.error('Failed to mark as completed:', err);
      }
    }
  };

  const handleScanLibrary = async () => {
    try {
      setScanning(true);
      setScanProgress(null);
      info("Select a folder to scan...");

      const result = await mediaService.selectAndScanDirectory();
      
      if (result) {
        success(
          `Scan complete! Found ${result.total_found} files, ` +
          `${result.added} added, ${result.updated} updated`
        );
        
        // Reload media and stats
        await Promise.all([loadMedia(), loadDbStats()]);
      } else {
        info("Scan cancelled");
      }
    } catch (err) {
      console.error("Scan error:", err);
      error(`Scan failed: ${err}`);
    } finally {
      setScanning(false);
      setScanProgress(null);
    }
  };

  const mediaActions: DropdownItem[] = [
    { id: 'play', label: 'Play', icon: '▶️' },
    { id: 'info', label: 'View Details', icon: 'ℹ️' },
    { id: 'subtitles', label: 'Manage Subtitles', icon: '💬' },
    { id: 'playlist', label: 'Add to Playlist', icon: '➕' },
    { id: 'collection', label: 'Add to Collection', icon: '📚' },
    { id: 'separator', label: '', separator: true },
    { id: 'delete', label: 'Remove from Library', icon: '🗑️', danger: true },
  ];

  const handleMediaAction = (actionItem: DropdownItem, mediaItem: any) => {
    switch (actionItem.id) {
      case 'play':
        handleMediaClick(mediaItem);
        break;
      case 'info':
        info('Opening details... (TODO)');
        break;
      case 'subtitles':
        setManageSubtitlesMediaId(parseInt(mediaItem.id));
        setManageSubtitlesMediaPath(mediaItem.file_path);
        break;
      case 'playlist':
        setAddToPlaylistMediaId(parseInt(mediaItem.id));
        break;
      case 'collection':
        setAddToCollectionMediaId(parseInt(mediaItem.id));
        break;
      case 'delete':
        error('Delete not implemented yet');
        break;
    }
  };

  const renderContent = () => {
    if (currentSection === 'playlists') {
      if (selectedPlaylist) {
        return (
          <PlaylistDetail
            playlist={selectedPlaylist}
            onBack={() => setSelectedPlaylist(null)}
            onPlayMedia={handleMediaClick}
            onDeletePlaylist={async () => {
              try {
                await playlistService.deletePlaylist(selectedPlaylist.id);
                setSelectedPlaylist(null);
                success('Playlist deleted');
              } catch (err) {
                console.error(err);
                error('Failed to delete playlist');
              }
            }}
          />
        );
      }
      return <PlaylistList onSelectPlaylist={setSelectedPlaylist} />;
    }

    if (currentSection === 'collections') {
      if (selectedCollection) {
        return (
          <CollectionDetail
            collection={selectedCollection}
            onBack={() => setSelectedCollection(null)}
            onPlayMedia={handleMediaClick}
            onDeleteCollection={async () => {
              try {
                await collectionService.deleteCollection(selectedCollection.id);
                setSelectedCollection(null);
                success('Collection deleted');
              } catch (err) {
                console.error(err);
                error('Failed to delete collection');
              }
            }}
          />
        );
      }
      return <CollectionList onSelectCollection={setSelectedCollection} />;
    }

    // Default content (Home/Media)
    return (
      <div className="app-content">
        <section className="app-section">
          <div className="app-section-header">
            <div>
              <h2 className="app-section-title">Welcome to CineVault</h2>
              <p className="app-section-subtitle">
                {loading ? 'Loading...' : dbStats}
              </p>
              {scanProgress && (
                <p className="app-section-subtitle" style={{ marginTop: 'var(--space-2)' }}>
                  Scanning: {scanProgress.files_scanned} / {scanProgress.files_found} files - {scanProgress.current_file}
                </p>
              )}
            </div>
            <div style={{ display: 'flex', gap: 'var(--space-3)', alignItems: 'center' }}>
              <Badge variant="info">Beta</Badge>
              <Button
                onClick={handleScanLibrary}
                icon={scanning ? <Spinner size="sm" /> : <span>📁</span>}
                disabled={scanning}
                loading={scanning}
              >
                {scanning ? 'Scanning...' : 'Scan Library'}
              </Button>
              {/* Global actions dropdown removed for now, or could keep for global context */}
              <Button
                variant="ghost"
                icon={<span>⚙️</span>}
                onClick={() => setSettingsOpen(true)}
              >
                Settings
              </Button>
            </div>
          </div>
        </section>

        {!searchQuery && currentFilter === 'all' && (
          <section className="app-section">
            <div className="app-section-header">
              <h2 className="app-section-title">Continue Watching</h2>
              <Badge variant="warning">
                {mediaItems.filter(item => item.progress && item.progress > 0).length} in progress
              </Badge>
            </div>
            <MediaGrid
              items={mediaItems.filter(item => item.progress && item.progress > 0)}
              onItemClick={handleMediaClick}
              onItemAction={handleMediaAction}
              actionItems={mediaActions}
              emptyMessage="No items in progress"
            />
          </section>
        )}

        <section className="app-section">
          <div className="app-section-header">
            <h2 className="app-section-title">
              {searchQuery ? `Search Results: "${searchQuery}"` :
               currentFilter === 'all' ? 'Recently Added' :
               currentFilter === 'movie' ? 'Movies' :
               currentFilter === 'tv' ? 'TV Shows' : 'Music'}
            </h2>
            <Badge variant="success">
              {filteredMediaItems.length} items
            </Badge>
          </div>
          <MediaGrid
            items={filteredMediaItems}
            onItemClick={handleMediaClick}
            onItemAction={handleMediaAction}
            actionItems={mediaActions}
            emptyMessage={searchQuery ? "No results found" : "No media in your library yet. Click 'Scan Library' to get started!"}
          />
        </section>
      </div>
    );
  };

  return (
    <>
      <MainLayout
        onSearch={handleSearch}
        onFilterChange={handleFilterChange}
        onSectionChange={handleSectionChange}
        currentFilter={currentFilter}
        currentSection={currentSection}
      >
        {renderContent()}
      </MainLayout>

      <SettingsPanel 
        isOpen={settingsOpen} 
        onClose={() => setSettingsOpen(false)} 
      />

      <ToastContainer toasts={toasts} onClose={removeToast} />

      <AddToPlaylistModal
        isOpen={addToPlaylistMediaId !== null}
        onClose={() => setAddToPlaylistMediaId(null)}
        mediaId={addToPlaylistMediaId || 0}
      />

      <AddToCollectionModal
        isOpen={addToCollectionMediaId !== null}
        onClose={() => setAddToCollectionMediaId(null)}
        mediaId={addToCollectionMediaId || 0}
      />

      <SubtitleManagerModal
        isOpen={manageSubtitlesMediaId !== null}
        onClose={() => setManageSubtitlesMediaId(null)}
        mediaId={manageSubtitlesMediaId || 0}
        mediaPath={manageSubtitlesMediaPath}
      />

      {playingMedia && (
        playingMedia.type === 'music' ? (
          <AudioPlayer
            src={`asset://localhost/${playingMedia.id}`}
            title={playingMedia.title}
            onClose={handleClosePlayer}
            onProgress={handlePlayerProgress}
            initialPosition={playingMedia.resumePosition || 0}
          />
        ) : (
          <VideoPlayer
            src={`asset://localhost/${playingMedia.id}`}
            title={playingMedia.title}
            onClose={handleClosePlayer}
            onProgress={handlePlayerProgress}
            initialPosition={playingMedia.resumePosition || 0}
            autoPlay
            mediaId={parseInt(playingMedia.id)}
          />
        )
      )}
    </>
  );
}

export default App;
