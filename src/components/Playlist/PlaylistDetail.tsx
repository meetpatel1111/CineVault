import React, { useEffect, useState } from 'react';
import { playlistService, Playlist, PlaylistMediaItem } from '../../services/playlistService';
import { Button } from '../Button';
import { Spinner } from '../Spinner';
import { MediaGrid } from '../MediaGrid';
import { useToast } from '../Toast';
import { Dropdown } from '../Dropdown';

interface PlaylistDetailProps {
  playlist: Playlist;
  onBack: () => void;
  onPlayMedia: (item: any) => void;
  onDeletePlaylist: () => void;
}

export const PlaylistDetail: React.FC<PlaylistDetailProps> = ({
  playlist,
  onBack,
  onPlayMedia,
  onDeletePlaylist
}) => {
  const [items, setItems] = useState<PlaylistMediaItem[]>([]);
  const [loading, setLoading] = useState(true);
  const { error, success } = useToast();

  const loadItems = async () => {
    setLoading(true);
    try {
      const data = await playlistService.getPlaylistMedia(playlist.id);
      setItems(data);
    } catch (err) {
      console.error(err);
      error('Failed to load playlist items');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadItems();
  }, [playlist.id]);

  const handleRemoveItem = async (mediaId: number) => {
    try {
      await playlistService.removeFromPlaylist(playlist.id, mediaId);
      success('Item removed from playlist');
      loadItems();
    } catch (err) {
      console.error(err);
      error('Failed to remove item');
    }
  };

  const mediaActions = [
    { id: 'play', label: 'Play', icon: '▶️' },
    { id: 'remove', label: 'Remove from Playlist', icon: '🗑️', danger: true }
  ];

  const handleMediaAction = (action: any, item: any) => {
    if (action.id === 'play') {
      onPlayMedia(item);
    } else if (action.id === 'remove') {
      handleRemoveItem(parseInt(item.id));
    }
  };

  // Convert PlaylistMediaItem to format expected by MediaGrid
  const displayItems = items.map(item => ({
    ...item,
    id: item.id.toString(), // MediaGrid expects string ID usually, need to check
    title: item.title || item.file_name,
    type: item.media_type as 'movie' | 'tv' | 'music',
  }));

  const playlistActions = [
    { id: 'rename', label: 'Rename', icon: '✏️' },
    { id: 'delete', label: 'Delete Playlist', icon: '🗑️', danger: true },
  ];

  const handleAction = (item: any) => {
    if (item.id === 'delete') {
      if (confirm(`Are you sure you want to delete "${playlist.name}"?`)) {
        onDeletePlaylist();
      }
    } else if (item.id === 'rename') {
       // TODO: Implement rename modal
       alert('Rename not implemented yet');
    }
  };

  return (
    <div className="playlist-detail">
      <div className="playlist-detail__header" style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        marginBottom: 'var(--space-4)',
        padding: '0 var(--space-4)'
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--space-3)' }}>
          <Button variant="ghost" onClick={onBack} icon={<span>←</span>}>
            Back
          </Button>
          <div>
            <h2 style={{ margin: 0 }}>{playlist.name}</h2>
            {playlist.description && (
              <p style={{ margin: 0, color: 'var(--text-secondary)' }}>{playlist.description}</p>
            )}
          </div>
        </div>

        <Dropdown
          trigger={
            <Button variant="secondary" icon={<span>⋮</span>}>
              Actions
            </Button>
          }
          items={playlistActions}
          onSelect={handleAction}
          align="right"
        />
      </div>

      {loading ? (
        <div style={{ display: 'flex', justifyContent: 'center', padding: 'var(--space-8)' }}>
          <Spinner size="lg" />
        </div>
      ) : (
        <MediaGrid
          items={displayItems}
          onItemClick={onPlayMedia}
          onItemAction={handleMediaAction}
          actionItems={mediaActions}
          emptyMessage="This playlist is empty."
        />
      )}
    </div>
  );
};
