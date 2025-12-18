import React, { useState, useEffect } from 'react';
import { Modal } from '../Modal';
import { Button } from '../Button';
import { Spinner } from '../Spinner';
import { subtitleService, SubtitleTrack } from '../../services/subtitleService';
import { useToast } from '../Toast';
import { open } from '@tauri-apps/api/dialog';
import './SubtitleManagerModal.css';

interface SubtitleManagerModalProps {
  isOpen: boolean;
  onClose: () => void;
  mediaId: number;
  mediaPath: string;
}

export const SubtitleManagerModal: React.FC<SubtitleManagerModalProps> = ({
  isOpen,
  onClose,
  mediaId,
  mediaPath
}) => {
  const [subtitles, setSubtitles] = useState<SubtitleTrack[]>([]);
  const [loading, setLoading] = useState(true);
  const [scanning, setScanning] = useState(false);
  const { success, error, info } = useToast();

  useEffect(() => {
    if (isOpen) {
      loadSubtitles();
    }
  }, [isOpen, mediaId]);

  const loadSubtitles = async () => {
    setLoading(true);
    try {
      const data = await subtitleService.getSubtitleTracks(mediaId);
      setSubtitles(data);
    } catch (err) {
      console.error(err);
      error('Failed to load subtitles');
    } finally {
      setLoading(false);
    }
  };

  const handleScan = async () => {
    setScanning(true);
    try {
      const addedIds = await subtitleService.scanSubtitles(mediaId, mediaPath);
      if (addedIds.length > 0) {
        success(`Found ${addedIds.length} new subtitle(s)`);
        loadSubtitles();
      } else {
        info('No new subtitles found');
      }
    } catch (err) {
      console.error(err);
      error('Failed to scan subtitles');
    } finally {
      setScanning(false);
    }
  };

  const handleAddManual = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Subtitle', extensions: ['srt', 'vtt', 'ass', 'sub'] }]
      });

      if (selected && typeof selected === 'string') {
        await subtitleService.addSubtitleTrack(mediaId, selected, undefined, 'Manual Upload');
        success('Subtitle added');
        loadSubtitles();
      }
    } catch (err) {
      console.error(err);
      error('Failed to add subtitle');
    }
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure you want to remove this subtitle?')) return;

    try {
      await subtitleService.removeSubtitleTrack(id);
      success('Subtitle removed');
      loadSubtitles();
    } catch (err) {
      console.error(err);
      error('Failed to remove subtitle');
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Manage Subtitles"
      footer={
        <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
          <Button variant="ghost" onClick={onClose}>
            Close
          </Button>
        </div>
      }
    >
      <div className="subtitle-manager">
        <div className="subtitle-manager__actions">
          <Button
            onClick={handleScan}
            disabled={scanning}
            icon={scanning ? <Spinner size="sm" /> : <span>🔍</span>}
          >
            {scanning ? 'Scanning...' : 'Auto Scan'}
          </Button>
          <Button
            variant="secondary"
            onClick={handleAddManual}
            icon={<span>📂</span>}
          >
            Add File...
          </Button>
        </div>

        {loading ? (
          <div className="subtitle-manager__loading">
            <Spinner size="md" />
          </div>
        ) : subtitles.length === 0 ? (
          <div className="subtitle-manager__empty">
            <p>No subtitles found.</p>
          </div>
        ) : (
          <div className="subtitle-manager__list">
            {subtitles.map(sub => (
              <div key={sub.id} className="subtitle-manager__item">
                <div className="subtitle-manager__info">
                  <span className="subtitle-manager__lang">{sub.language || 'Unknown'}</span>
                  <span className="subtitle-manager__label">
                    {sub.label || sub.file_path.split(/[/\\]/).pop()}
                  </span>
                  {sub.is_embedded && <span className="subtitle-manager__badge">Embedded</span>}
                </div>
                <button
                  className="subtitle-manager__delete"
                  onClick={() => sub.id && handleDelete(sub.id)}
                  title="Remove"
                >
                  🗑️
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </Modal>
  );
};
