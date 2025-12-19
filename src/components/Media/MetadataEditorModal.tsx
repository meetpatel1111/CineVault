import React, { useState, useEffect } from 'react';
import { Button, Modal, Input } from '..';
import { MediaFile } from '../../services/mediaService';

interface MetadataEditorModalProps {
  isOpen: boolean;
  onClose: () => void;
  media: MediaFile;
  onSave: (metadata: {
    title?: string;
    year?: number;
    season?: number;
    episode?: number;
    description?: string;
    posterUrl?: string;
  }) => Promise<void>;
}

export const MetadataEditorModal: React.FC<MetadataEditorModalProps> = ({
  isOpen,
  onClose,
  media,
  onSave,
}) => {
  const [title, setTitle] = useState('');
  const [year, setYear] = useState('');
  const [season, setSeason] = useState('');
  const [episode, setEpisode] = useState('');
  const [description, setDescription] = useState('');
  const [posterUrl, setPosterUrl] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (media) {
      setTitle(media.title || media.file_name);
      setYear(media.year?.toString() || '');
      setSeason(media.season_number?.toString() || '');
      setEpisode(media.episode_number?.toString() || '');

      try {
        if (media.metadata_json) {
          const json = JSON.parse(media.metadata_json);
          setDescription(json.overview || '');
          setPosterUrl(json.poster_path || '');
        }
      } catch (e) {
        // Ignore JSON parse errors
      }
    }
  }, [media, isOpen]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      await onSave({
        title,
        year: year ? parseInt(year) : undefined,
        season: season ? parseInt(season) : undefined,
        episode: episode ? parseInt(episode) : undefined,
        description,
        posterUrl,
      });
      onClose();
    } catch (err) {
      console.error('Failed to save metadata', err);
    } finally {
      setLoading(false);
    }
  };

  const isTvShow = media.media_type === 'tv_episode';

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Edit Metadata"
    >
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">Title</label>
          <Input
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Movie or Episode Title"
            fullWidth
          />
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">Year</label>
            <Input
              type="number"
              value={year}
              onChange={(e) => setYear(e.target.value)}
              placeholder="YYYY"
              fullWidth
            />
          </div>

          {/* Placeholder for alignment if needed, or maybe Media Type selector? For now static. */}
          <div className="flex items-end pb-2">
             <span className="text-sm text-gray-400 uppercase tracking-wider">{media.media_type}</span>
          </div>
        </div>

        {isTvShow && (
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">Season</label>
              <Input
                type="number"
                value={season}
                onChange={(e) => setSeason(e.target.value)}
                placeholder="1"
                fullWidth
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">Episode</label>
              <Input
                type="number"
                value={episode}
                onChange={(e) => setEpisode(e.target.value)}
                placeholder="1"
                fullWidth
              />
            </div>
          </div>
        )}

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">Description (Overview)</label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 h-24"
            placeholder="Plot summary..."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">Poster URL</label>
          <Input
            value={posterUrl}
            onChange={(e) => setPosterUrl(e.target.value)}
            placeholder="https://..."
            fullWidth
          />
        </div>

        <div className="pt-2">
           <p className="text-xs text-yellow-500">
             Note: Editing metadata will lock this file from future automatic updates during scans.
           </p>
        </div>

        <div className="flex justify-end space-x-3 mt-6">
          <Button
            type="button"
            variant="ghost"
            onClick={onClose}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            loading={loading}
          >
            Save Changes
          </Button>
        </div>
      </form>
    </Modal>
  );
};
