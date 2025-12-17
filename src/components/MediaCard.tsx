import React from 'react';
import './MediaCard.css';

interface MediaCardProps {
  title: string;
  year?: number;
  duration?: number;
  posterUrl?: string;
  type: 'movie' | 'tv' | 'music';
  watched?: boolean;
  progress?: number; // 0-100
  rating?: number; // 0-10
  onClick?: () => void;
}

export const MediaCard: React.FC<MediaCardProps> = ({
  title,
  year,
  duration,
  posterUrl,
  type,
  watched = false,
  progress = 0,
  rating,
  onClick,
}) => {
  const formatDuration = (minutes?: number): string => {
    if (!minutes) return '';
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    return hours > 0 ? `${hours}h ${mins}m` : `${mins}m`;
  };

  const getTypeIcon = (mediaType: string): string => {
    switch (mediaType) {
      case 'movie': return '🎬';
      case 'tv': return '📺';
      case 'music': return '🎵';
      default: return '📁';
    }
  };

  return (
    <div className="media-card" onClick={onClick}>
      <div className="media-card__poster">
        {posterUrl ? (
          <img 
            src={posterUrl} 
            alt={title}
            className="media-card__image"
            loading="lazy"
          />
        ) : (
          <div className="media-card__placeholder">
            <span className="media-card__placeholder-icon">
              {getTypeIcon(type)}
            </span>
          </div>
        )}
        
        {watched && (
          <div className="media-card__badge media-card__badge--watched">
            <span>✓</span>
          </div>
        )}
        
        {progress > 0 && progress < 100 && (
          <div className="media-card__progress">
            <div 
              className="media-card__progress-bar"
              style={{ width: `${progress}%` }}
            />
          </div>
        )}
        
        <div className="media-card__overlay">
          <button className="media-card__play-button">
            <PlayIcon />
          </button>
        </div>
      </div>

      <div className="media-card__info">
        <h3 className="media-card__title" title={title}>
          {title}
        </h3>
        
        <div className="media-card__meta">
          {year && <span className="media-card__year">{year}</span>}
          {duration && (
            <>
              {year && <span className="media-card__separator">•</span>}
              <span className="media-card__duration">
                {formatDuration(duration)}
              </span>
            </>
          )}
        </div>
        
        {rating && (
          <div className="media-card__rating">
            <span className="media-card__rating-icon">⭐</span>
            <span className="media-card__rating-value">
              {rating.toFixed(1)}
            </span>
          </div>
        )}
      </div>
    </div>
  );
};

const PlayIcon: React.FC = () => (
  <svg
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="currentColor"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path d="M8 5v14l11-7z" />
  </svg>
);
