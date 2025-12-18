import React, { useEffect, useState } from 'react';
import { playlistService, Playlist } from '../../services/playlistService';
import { Button } from '../Button';
import { Spinner } from '../Spinner';
import { CreatePlaylistModal } from './CreatePlaylistModal';
import { useToast } from '../Toast';
import './PlaylistList.css';

interface PlaylistListProps {
  onSelectPlaylist: (playlist: Playlist) => void;
}

export const PlaylistList: React.FC<PlaylistListProps> = ({ onSelectPlaylist }) => {
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  const [loading, setLoading] = useState(true);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const { error } = useToast();

  const loadPlaylists = async () => {
    setLoading(true);
    try {
      const data = await playlistService.getAllPlaylists();
      setPlaylists(data);
    } catch (err) {
      console.error(err);
      error('Failed to load playlists');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadPlaylists();
  }, []);

  if (loading) {
    return (
      <div className="playlist-list__loading">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
    <div className="playlist-list">
      <div className="playlist-list__header">
        <Button onClick={() => setIsCreateModalOpen(true)} icon={<span>➕</span>}>
          Create Playlist
        </Button>
      </div>

      {playlists.length === 0 ? (
        <div className="playlist-list__empty">
          <span className="playlist-list__empty-icon">📋</span>
          <h3>No Playlists</h3>
          <p>Create your first playlist to organize your media.</p>
        </div>
      ) : (
        <div className="playlist-list__grid">
          {playlists.map((playlist) => (
            <div
              key={playlist.id}
              className="playlist-card"
              onClick={() => onSelectPlaylist(playlist)}
            >
              <div className="playlist-card__icon">
                {playlist.playlist_type === 'smart' ? '🧠' : '📋'}
              </div>
              <div className="playlist-card__info">
                <h3 className="playlist-card__title">{playlist.name}</h3>
                <p className="playlist-card__count">{playlist.item_count} items</p>
                {playlist.description && (
                  <p className="playlist-card__description">{playlist.description}</p>
                )}
              </div>
            </div>
          ))}
        </div>
      )}

      <CreatePlaylistModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onCreated={loadPlaylists}
      />
    </div>
  );
};
