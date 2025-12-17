import React, { useRef, useState, useEffect } from 'react';
import { PlayerControls } from './PlayerControls';
import './VideoPlayer.css';

interface VideoPlayerProps {
  src: string;
  title?: string;
  onClose?: () => void;
  onProgress?: (position: number, duration: number) => void;
  initialPosition?: number;
  autoPlay?: boolean;
}

export const VideoPlayer: React.FC<VideoPlayerProps> = ({
  src,
  title,
  onClose,
  onProgress,
  initialPosition = 0,
  autoPlay = true,
}) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [playing, setPlaying] = useState(autoPlay);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(1);
  const [muted, setMuted] = useState(false);
  const [playbackRate, setPlaybackRate] = useState(1);
  const [fullscreen, setFullscreen] = useState(false);
  const [showControls, setShowControls] = useState(true);
  const [buffered, setBuffered] = useState(0);
  const controlsTimeoutRef = useRef<number>();

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    // Set initial position
    if (initialPosition > 0) {
      video.currentTime = initialPosition;
    }

    const handleLoadedMetadata = () => {
      setDuration(video.duration);
    };

    const handleTimeUpdate = () => {
      setCurrentTime(video.currentTime);
      onProgress?.(video.currentTime, video.duration);
    };

    const handleProgress = () => {
      if (video.buffered.length > 0) {
        const bufferedEnd = video.buffered.end(video.buffered.length - 1);
        setBuffered((bufferedEnd / video.duration) * 100);
      }
    };

    const handleEnded = () => {
      setPlaying(false);
    };

    video.addEventListener('loadedmetadata', handleLoadedMetadata);
    video.addEventListener('timeupdate', handleTimeUpdate);
    video.addEventListener('progress', handleProgress);
    video.addEventListener('ended', handleEnded);

    return () => {
      video.removeEventListener('loadedmetadata', handleLoadedMetadata);
      video.removeEventListener('timeupdate', handleTimeUpdate);
      video.removeEventListener('progress', handleProgress);
      video.removeEventListener('ended', handleEnded);
    };
  }, [initialPosition, onProgress]);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    if (playing) {
      video.play();
    } else {
      video.pause();
    }
  }, [playing]);

  useEffect(() => {
    const video = videoRef.current;
    if (video) {
      video.volume = volume;
    }
  }, [volume]);

  useEffect(() => {
    const video = videoRef.current;
    if (video) {
      video.muted = muted;
    }
  }, [muted]);

  useEffect(() => {
    const video = videoRef.current;
    if (video) {
      video.playbackRate = playbackRate;
    }
  }, [playbackRate]);

  const handlePlayPause = () => {
    setPlaying(!playing);
  };

  const handleSeek = (time: number) => {
    const video = videoRef.current;
    if (video) {
      video.currentTime = time;
      setCurrentTime(time);
    }
  };

  const handleVolumeChange = (newVolume: number) => {
    setVolume(newVolume);
    if (newVolume > 0 && muted) {
      setMuted(false);
    }
  };

  const handleMuteToggle = () => {
    setMuted(!muted);
  };

  const handlePlaybackRateChange = (rate: number) => {
    setPlaybackRate(rate);
  };

  const handleFullscreenToggle = () => {
    const container = videoRef.current?.parentElement;
    if (!container) return;

    if (!fullscreen) {
      if (container.requestFullscreen) {
        container.requestFullscreen();
      }
    } else {
      if (document.exitFullscreen) {
        document.exitFullscreen();
      }
    }
    setFullscreen(!fullscreen);
  };

  const handleMouseMove = () => {
    setShowControls(true);
    
    if (controlsTimeoutRef.current) {
      clearTimeout(controlsTimeoutRef.current);
    }

    if (playing) {
      controlsTimeoutRef.current = setTimeout(() => {
        setShowControls(false);
      }, 3000);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    switch (e.key) {
      case ' ':
        e.preventDefault();
        handlePlayPause();
        break;
      case 'ArrowLeft':
        handleSeek(Math.max(0, currentTime - 10));
        break;
      case 'ArrowRight':
        handleSeek(Math.min(duration, currentTime + 10));
        break;
      case 'ArrowUp':
        handleVolumeChange(Math.min(1, volume + 0.1));
        break;
      case 'ArrowDown':
        handleVolumeChange(Math.max(0, volume - 0.1));
        break;
      case 'f':
        handleFullscreenToggle();
        break;
      case 'm':
        handleMuteToggle();
        break;
      case 'Escape':
        if (fullscreen) {
          handleFullscreenToggle();
        } else {
          onClose?.();
        }
        break;
    }
  };

  return (
    <div
      className={`video-player ${fullscreen ? 'video-player--fullscreen' : ''}`}
      onMouseMove={handleMouseMove}
      onKeyDown={handleKeyPress}
      tabIndex={0}
    >
      {title && (
        <div className={`video-player__header ${showControls ? 'visible' : ''}`}>
          <h2 className="video-player__title">{title}</h2>
          <button className="video-player__close" onClick={onClose}>
            ✕
          </button>
        </div>
      )}

      <video
        ref={videoRef}
        className="video-player__video"
        src={src}
        autoPlay={autoPlay}
        onClick={handlePlayPause}
      />

      <PlayerControls
        playing={playing}
        currentTime={currentTime}
        duration={duration}
        volume={volume}
        muted={muted}
        playbackRate={playbackRate}
        buffered={buffered}
        visible={showControls}
        onPlayPause={handlePlayPause}
        onSeek={handleSeek}
        onVolumeChange={handleVolumeChange}
        onMuteToggle={handleMuteToggle}
        onPlaybackRateChange={handlePlaybackRateChange}
        onFullscreenToggle={handleFullscreenToggle}
      />
    </div>
  );
};
