import React, { useState, useEffect } from 'react';
import { Modal } from '../Modal';
import { Spinner } from '../Spinner';
import { playlistService, Playlist } from '../../services/playlistService';
import { useToast } from '../Toast';
import './AddToPlaylistModal.css';

interface AddToPlaylistModalProps {
  isOpen: boolean;
  onClose: () => void;
  mediaId: number;
}

export const AddToPlaylistModal: React.FC<AddToPlaylistModalProps> = ({
  isOpen,
  onClose,
  mediaId
}) => {
  const [playlists, setPlaylists] = useState<Playlist[]>([]);
  const [loading, setLoading] = useState(true);
  const [addingTo, setAddingTo] = useState<number | null>(null);
  const { success, error } = useToast();

  useEffect(() => {
    if (isOpen) {
      loadPlaylists();
    }
  }, [isOpen]);

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

  const handleAddToPlaylist = async (playlist: Playlist) => {
    setAddingTo(playlist.id);
    try {
      await playlistService.addToPlaylist(playlist.id, mediaId);
      success(`Added to playlist "${playlist.name}"`);
      onClose();
    } catch (err) {
      console.error(err);
      error('Failed to add to playlist');
    } finally {
      setAddingTo(null);
    }
  };

  const manualPlaylists = playlists.filter(p => p.playlist_type === 'manual');

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Add to Playlist"
    >
      <div className="add-to-playlist">
        {loading ? (
          <div className="add-to-playlist__loading">
            <Spinner size="md" />
          </div>
        ) : manualPlaylists.length === 0 ? (
          <div className="add-to-playlist__empty">
            <p>No playlists found.</p>
            {/* Could add a create button here in future */}
          </div>
        ) : (
          <div className="add-to-playlist__list">
            {manualPlaylists.map(playlist => (
              <button
                key={playlist.id}
                className="add-to-playlist__item"
                onClick={() => handleAddToPlaylist(playlist)}
                disabled={addingTo !== null}
              >
                <span className="add-to-playlist__icon">📋</span>
                <span className="add-to-playlist__name">{playlist.name}</span>
                <span className="add-to-playlist__count">{playlist.item_count} items</span>
                {addingTo === playlist.id && <Spinner size="sm" />}
              </button>
            ))}
          </div>
        )}
      </div>
    </Modal>
  );
};
