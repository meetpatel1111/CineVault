import React, { useState, useEffect } from 'react';
import { Modal } from '../Modal';
import { Spinner } from '../Spinner';
import { collectionService, Collection } from '../../services/collectionService';
import { useToast } from '../Toast';
import './AddToCollectionModal.css';

interface AddToCollectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  mediaId: number;
}

export const AddToCollectionModal: React.FC<AddToCollectionModalProps> = ({
  isOpen,
  onClose,
  mediaId
}) => {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [loading, setLoading] = useState(true);
  const [addingTo, setAddingTo] = useState<number | null>(null);
  const { success, error } = useToast();

  useEffect(() => {
    if (isOpen) {
      loadCollections();
    }
  }, [isOpen]);

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

  const handleAddToCollection = async (collection: Collection) => {
    setAddingTo(collection.id);
    try {
      await collectionService.addToCollection(collection.id, mediaId);
      success(`Added to collection "${collection.name}"`);
      onClose();
    } catch (err) {
      console.error(err);
      error('Failed to add to collection');
    } finally {
      setAddingTo(null);
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title="Add to Collection"
    >
      <div className="add-to-collection">
        {loading ? (
          <div className="add-to-collection__loading">
            <Spinner size="md" />
          </div>
        ) : collections.length === 0 ? (
          <div className="add-to-collection__empty">
            <p>No collections found.</p>
          </div>
        ) : (
          <div className="add-to-collection__list">
            {collections.map(collection => (
              <button
                key={collection.id}
                className="add-to-collection__item"
                onClick={() => handleAddToCollection(collection)}
                disabled={addingTo !== null}
              >
                <span className="add-to-collection__icon">📚</span>
                <span className="add-to-collection__name">{collection.name}</span>
                <span className="add-to-collection__count">{collection.item_count} items</span>
                {addingTo === collection.id && <Spinner size="sm" />}
              </button>
            ))}
          </div>
        )}
      </div>
    </Modal>
  );
};
