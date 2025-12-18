import React, { useEffect, useState } from 'react';
import { collectionService, Collection } from '../../services/collectionService';
import { Button } from '../Button';
import { Spinner } from '../Spinner';
import { CreateCollectionModal } from './CreateCollectionModal';
import { useToast } from '../Toast';
import './CollectionList.css';

interface CollectionListProps {
  onSelectCollection: (collection: Collection) => void;
}

export const CollectionList: React.FC<CollectionListProps> = ({ onSelectCollection }) => {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [loading, setLoading] = useState(true);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const { error } = useToast();

  const loadCollections = async () => {
    setLoading(true);
    try {
      const data = await collectionService.getAllCollections();
      setCollections(data);
    } catch (err) {
      console.error(err);
      error('Failed to load collections');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCollections();
  }, []);

  if (loading) {
    return (
      <div className="collection-list__loading">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
    <div className="collection-list">
      <div className="collection-list__header">
        <Button onClick={() => setIsCreateModalOpen(true)} icon={<span>➕</span>}>
          Create Collection
        </Button>
      </div>

      {collections.length === 0 ? (
        <div className="collection-list__empty">
          <span className="collection-list__empty-icon">📚</span>
          <h3>No Collections</h3>
          <p>Create your first collection to organize your media.</p>
        </div>
      ) : (
        <div className="collection-list__grid">
          {collections.map((collection) => (
            <div
              key={collection.id}
              className="collection-card"
              onClick={() => onSelectCollection(collection)}
            >
              <div className="collection-card__icon">
                📚
              </div>
              <div className="collection-card__info">
                <h3 className="collection-card__title">{collection.name}</h3>
                <p className="collection-card__count">{collection.item_count} items</p>
                {collection.description && (
                  <p className="collection-card__description">{collection.description}</p>
                )}
              </div>
            </div>
          ))}
        </div>
      )}

      <CreateCollectionModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onCreated={loadCollections}
      />
    </div>
  );
};
