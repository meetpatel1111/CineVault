import React, { useState } from 'react';
import { Modal } from '../Modal';
import { Input } from '../Input';
import { Button } from '../Button';
import { RuleEditor, type EditableRule } from './RuleEditor';
import { playlistService } from '../../services/playlistService';
import { useToast } from '../Toast';

interface CreatePlaylistModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated?: () => void;
}

export const CreatePlaylistModal: React.FC<CreatePlaylistModalProps> = ({
  isOpen,
  onClose,
  onCreated
}) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [playlistType, setPlaylistType] = useState<'manual' | 'smart'>('manual');
  const [rules, setRules] = useState<EditableRule[]>([]);
  const [loading, setLoading] = useState(false);
  const { success, error } = useToast();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    setLoading(true);
    try {
      // 1. Create Playlist
      const id = await playlistService.createPlaylist(name, description, playlistType);

      // 2. Add Rules if Smart
      if (playlistType === 'smart' && rules.length > 0) {
        for (const rule of rules) {
          if (rule.value) { // Skip empty rules
             await playlistService.addRule(id, rule.rule_type, rule.operator, rule.value);
          }
        }
      }

      success(`Playlist "${name}" created`);

      // Reset State
      setName('');
      setDescription('');
      setPlaylistType('manual');
      setRules([]);

      if (onCreated) onCreated();
      onClose();
    } catch (err) {
      console.error(err);
      error('Failed to create playlist');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Create New Playlist"
      footer={
        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: 'var(--space-2)' }}>
          <Button variant="ghost" onClick={onClose} disabled={loading}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={loading || !name.trim()}>
            {loading ? 'Creating...' : 'Create Playlist'}
          </Button>
        </div>
      }
    >
      <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space-4)' }}>
        <Input
          label="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="My Playlist"
          autoFocus
        />
        <Input
          label="Description (Optional)"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="A collection of my favorite movies..."
        />

        <div className="form-group">
          <label style={{ display: 'block', marginBottom: 'var(--space-2)', fontSize: 'var(--text-sm)', color: 'var(--color-text-secondary)' }}>
            Playlist Type
          </label>
          <div style={{ display: 'flex', gap: 'var(--space-2)' }}>
            <Button
              variant={playlistType === 'manual' ? 'primary' : 'secondary'}
              size="sm"
              onClick={() => setPlaylistType('manual')}
            >
              Manual
            </Button>
            <Button
              variant={playlistType === 'smart' ? 'primary' : 'secondary'}
              size="sm"
              onClick={() => setPlaylistType('smart')}
            >
              Smart
            </Button>
          </div>
          <p style={{ marginTop: 'var(--space-1)', fontSize: 'var(--text-xs)', color: 'var(--color-text-tertiary)' }}>
            {playlistType === 'manual'
              ? "Manually add items to this playlist."
              : "Automatically populate based on rules (e.g. Genre = Action)."}
          </p>
        </div>

        {playlistType === 'smart' && (
          <div className="form-group">
            <label style={{ display: 'block', marginBottom: 'var(--space-2)', fontSize: 'var(--text-sm)', color: 'var(--color-text-secondary)' }}>
              Rules
            </label>
            <RuleEditor rules={rules} onChange={setRules} />
          </div>
        )}
      </div>
    </Modal>
  );
};
