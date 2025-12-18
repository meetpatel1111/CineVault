import React, { useEffect, useState } from 'react';
import { collectionService, Collection, CollectionMediaItem } from '../../services/collectionService';
import { Button } from '../Button';
import { Spinner } from '../Spinner';
import { MediaGrid } from '../MediaGrid';
import { useToast } from '../Toast';
import { Dropdown } from '../Dropdown';

interface CollectionDetailProps {
  collection: Collection;
  onBack: () => void;
  onPlayMedia: (item: any) => void;
  onDeleteCollection: () => void;
}

export const CollectionDetail: React.FC<CollectionDetailProps> = ({
  collection,
  onBack,
  onPlayMedia,
  onDeleteCollection
}) => {
  const [items, setItems] = useState<CollectionMediaItem[]>([]);
  const [loading, setLoading] = useState(true);
  const { error, success } = useToast();

  const loadItems = async () => {
    setLoading(true);
    try {
      const data = await collectionService.getCollectionMedia(collection.id);
      setItems(data);
    } catch (err) {
      console.error(err);
      error('Failed to load collection items');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadItems();
  }, [collection.id]);

  const handleRemoveItem = async (mediaId: number) => {
    try {
      await collectionService.removeFromCollection(collection.id, mediaId);
      success('Item removed from collection');
      loadItems();
    } catch (err) {
      console.error(err);
      error('Failed to remove item');
    }
  };

  const mediaActions = [
    { id: 'play', label: 'Play', icon: '▶️' },
    { id: 'remove', label: 'Remove from Collection', icon: '🗑️', danger: true }
  ];

  const handleMediaAction = (action: any, item: any) => {
    if (action.id === 'play') {
      onPlayMedia(item);
    } else if (action.id === 'remove') {
      handleRemoveItem(parseInt(item.id));
    }
  };

  // Convert CollectionMediaItem to format expected by MediaGrid
  const displayItems = items.map(item => ({
    ...item,
    id: item.id.toString(),
    title: item.title || item.file_name,
    type: item.media_type as 'movie' | 'tv' | 'music',
  }));

  const collectionActions = [
    { id: 'rename', label: 'Rename', icon: '✏️' },
    { id: 'delete', label: 'Delete Collection', icon: '🗑️', danger: true },
  ];

  const handleAction = (item: any) => {
    if (item.id === 'delete') {
      if (confirm(`Are you sure you want to delete "${collection.name}"?`)) {
        onDeleteCollection();
      }
    } else if (item.id === 'rename') {
       // TODO: Implement rename modal
       alert('Rename not implemented yet');
    }
  };

  return (
    <div className="collection-detail">
      <div className="collection-detail__header" style={{
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
            <h2 style={{ margin: 0 }}>{collection.name}</h2>
            {collection.description && (
              <p style={{ margin: 0, color: 'var(--text-secondary)' }}>{collection.description}</p>
            )}
          </div>
        </div>

        <Dropdown
          trigger={
            <Button variant="secondary" icon={<span>⋮</span>}>
              Actions
            </Button>
          }
          items={collectionActions}
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
          emptyMessage="This collection is empty."
        />
      )}
    </div>
  );
};
