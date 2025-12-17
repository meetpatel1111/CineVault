import React, { useRef, useState, useEffect } from 'react';
import './AudioPlayer.css';

interface AudioPlayerProps {
  src: string;
  title?: string;
  artist?: string;
  coverUrl?: string;
  onClose?: () => void;
  onProgress?: (position: number, duration: number) => void;
  initialPosition?: number;
  autoPlay?: boolean;
}

export const AudioPlayer: React.FC<AudioPlayerProps> = ({
  src,
  title,
  artist,
  coverUrl,
  onClose,
  onProgress,
  initialPosition = 0,
  autoPlay = true,
}) => {
  const audioRef = useRef<HTMLAudioElement>(null);
  const [playing, setPlaying] = useState(autoPlay);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(1);
  const [muted, setMuted] = useState(false);

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    if (initialPosition > 0) {
      audio.currentTime = initialPosition;
    }

    const handleLoadedMetadata = () => {
      setDuration(audio.duration);
    };

    const handleTimeUpdate = () => {
      setCurrentTime(audio.currentTime);
      onProgress?.(audio.currentTime, audio.duration);
    };

    const handleEnded = () => {
      setPlaying(false);
    };

    audio.addEventListener('loadedmetadata', handleLoadedMetadata);
    audio.addEventListener('timeupdate', handleTimeUpdate);
    audio.addEventListener('ended', handleEnded);

    return () => {
      audio.removeEventListener('loadedmetadata', handleLoadedMetadata);
      audio.removeEventListener('timeupdate', handleTimeUpdate);
      audio.removeEventListener('ended', handleEnded);
    };
  }, [initialPosition, onProgress]);

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    if (playing) {
      audio.play();
    } else {
      audio.pause();
    }
  }, [playing]);

  useEffect(() => {
    const audio = audioRef.current;
    if (audio) {
      audio.volume = volume;
    }
  }, [volume]);

  useEffect(() => {
    const audio = audioRef.current;
    if (audio) {
      audio.muted = muted;
    }
  }, [muted]);

  const handlePlayPause = () => {
    setPlaying(!playing);
  };

  const handleSeek = (time: number) => {
    const audio = audioRef.current;
    if (audio) {
      audio.currentTime = time;
      setCurrentTime(time);
    }
  };

  const handleProgressClick = (e: React.MouseEvent<HTMLDivElement>) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const pos = (e.clientX - rect.left) / rect.width;
    handleSeek(pos * duration);
  };

  const formatTime = (seconds: number): string => {
    if (isNaN(seconds)) return '0:00';
    
    const minutes = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${minutes}:${secs.toString().padStart(2, '0')}`;
  };

  const progress = duration > 0 ? (currentTime / duration) * 100 : 0;

  return (
    <div className="audio-player">
      <audio ref={audioRef} src={src} autoPlay={autoPlay} />

      <div className="audio-player__header">
        <h2 className="audio-player__title">Now Playing</h2>
        <button className="audio-player__close" onClick={onClose}>
          ✕
        </button>
      </div>

      <div className="audio-player__content">
        <div className="audio-player__artwork">
          {coverUrl ? (
            <img src={coverUrl} alt={title} className="audio-player__cover" />
          ) : (
            <div className="audio-player__placeholder">
              <span className="audio-player__placeholder-icon">🎵</span>
            </div>
          )}
        </div>

        <div className="audio-player__info">
          <h3 className="audio-player__track-title">{title || 'Unknown Track'}</h3>
          {artist && <p className="audio-player__artist">{artist}</p>}
        </div>

        <div className="audio-player__progress" onClick={handleProgressClick}>
          <div className="audio-player__progress-bar" style={{ width: `${progress}%` }} />
          <div className="audio-player__progress-handle" style={{ left: `${progress}%` }} />
        </div>

        <div className="audio-player__time">
          <span>{formatTime(currentTime)}</span>
          <span>{formatTime(duration)}</span>
        </div>

        <div className="audio-player__controls">
          <button className="audio-player__control" onClick={() => handleSeek(Math.max(0, currentTime - 10))}>
            <BackwardIcon />
          </button>

          <button className="audio-player__control audio-player__control--play" onClick={handlePlayPause}>
            {playing ? <PauseIcon /> : <PlayIcon />}
          </button>

          <button className="audio-player__control" onClick={() => handleSeek(Math.min(duration, currentTime + 10))}>
            <ForwardIcon />
          </button>
        </div>

        <div className="audio-player__volume">
          <button className="audio-player__volume-button" onClick={() => setMuted(!muted)}>
            {muted || volume === 0 ? '🔇' : '🔊'}
          </button>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={muted ? 0 : volume}
            onChange={(e) => setVolume(parseFloat(e.target.value))}
            className="audio-player__volume-slider"
          />
        </div>
      </div>
    </div>
  );
};

const PlayIcon = () => (
  <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
    <path d="M8 5v14l11-7z" />
  </svg>
);

const PauseIcon = () => (
  <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
    <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
  </svg>
);

const ForwardIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
    <path d="M4 18l8.5-6L4 6v12zm9-12v12l8.5-6L13 6z" />
  </svg>
);

const BackwardIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
    <path d="M11 18V6l-8.5 6 8.5 6zm.5-6l8.5 6V6l-8.5 6z" />
  </svg>
);
