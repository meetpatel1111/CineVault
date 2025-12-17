import { invoke } from '@tauri-apps/api/tauri';

export interface PlaybackState {
  media_id: number;
  last_position: number;
  duration?: number;
  completed: boolean;
  watch_count: number;
  last_played_at: string;
  created_at: string;
}

export interface RecentlyPlayed {
  media_id: number;
  file_path: string;
  file_name: string;
  title?: string;
  year?: number;
  media_type: string;
  last_position: number;
  duration?: number;
  completed: boolean;
  last_played_at: string;
}

export interface WatchStats {
  total_watched: number;
  total_in_progress: number;
  total_watch_time: number;
  total_sessions: number;
}

export const playbackService = {
  /**
   * Update playback position for a media file
   */
  async updatePosition(
    mediaId: number,
    position: number,
    duration?: number
  ): Promise<void> {
    return invoke('update_playback_position', {
      mediaId,
      position: Math.floor(position),
      duration: duration ? Math.floor(duration) : null,
    });
  },

  /**
   * Mark media as completed (watched to end)
   */
  async markAsCompleted(mediaId: number, duration: number): Promise<void> {
    return invoke('mark_as_completed', {
      mediaId,
      duration: Math.floor(duration),
    });
  },

  /**
   * Get playback state for a media file
   */
  async getPlaybackState(mediaId: number): Promise<PlaybackState | null> {
    return invoke<PlaybackState | null>('get_playback_state', { mediaId });
  },

  /**
   * Get recently played media
   */
  async getRecentlyPlayed(limit: number = 20): Promise<RecentlyPlayed[]> {
    return invoke<RecentlyPlayed[]>('get_recently_played', { limit });
  },

  /**
   * Get media in progress (not completed)
   */
  async getInProgress(limit: number = 20): Promise<RecentlyPlayed[]> {
    return invoke<RecentlyPlayed[]>('get_in_progress', { limit });
  },

  /**
   * Get watch statistics
   */
  async getWatchStats(): Promise<WatchStats> {
    return invoke<WatchStats>('get_watch_stats');
  },

  /**
   * Calculate progress percentage
   */
  getProgressPercentage(position: number, duration: number): number {
    if (duration === 0) return 0;
    return Math.min(100, Math.max(0, (position / duration) * 100));
  },

  /**
   * Check if media should be marked as completed (95% threshold)
   */
  shouldMarkCompleted(position: number, duration: number): boolean {
    return this.getProgressPercentage(position, duration) >= 95;
  },
};
