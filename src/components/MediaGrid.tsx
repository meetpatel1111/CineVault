import React from 'react';
import { MediaCard } from './MediaCard';
import { DropdownItem } from './Dropdown';
import './MediaGrid.css';

interface MediaItem {
  id: string;
  title: string;
  year?: number;
  duration?: number;
  posterUrl?: string;
  type: 'movie' | 'tv' | 'music';
  watched?: boolean;
  progress?: number;
  rating?: number;
}

interface MediaGridProps {
  items: MediaItem[];
  onItemClick?: (item: MediaItem) => void;
  onItemAction?: (action: DropdownItem, item: MediaItem) => void;
  actionItems?: DropdownItem[];
  emptyMessage?: string;
}

export const MediaGrid: React.FC<MediaGridProps> = ({
  items,
  onItemClick,
  onItemAction,
  actionItems,
  emptyMessage = 'No media found',
}) => {
  if (items.length === 0) {
    return (
      <div className="media-grid-empty">
        <div className="media-grid-empty__icon">📁</div>
        <p className="media-grid-empty__message">{emptyMessage}</p>
      </div>
    );
  }

  return (
    <div className="media-grid">
      {items.map((item) => (
        <MediaCard
          key={item.id}
          {...item}
          onClick={() => onItemClick?.(item)}
          onAction={onItemAction ? (action) => onItemAction(action, item) : undefined}
          actionItems={actionItems}
        />
      ))}
    </div>
  );
};
