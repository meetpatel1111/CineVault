import { invoke } from '@tauri-apps/api/tauri';

export interface DailyWatchStat {
  date: string;
  minutes: number;
  sessions: number;
}

export interface MediaTypeStat {
  media_type: string;
  count: number;
}

export interface WatchStats {
  total_watched: number;
  total_in_progress: number;
  total_watch_time: number;
  total_sessions: number;
}

export const analyticsService = {
  async getWatchStats(): Promise<WatchStats> {
    return await invoke<WatchStats>('get_watch_stats');
  },

  async getWatchHistoryChart(days: number = 30): Promise<DailyWatchStat[]> {
    return await invoke<DailyWatchStat[]>('get_watch_history_chart', { days });
  },

  async getMediaTypeDistribution(): Promise<MediaTypeStat[]> {
    return await invoke<MediaTypeStat[]>('get_media_type_distribution');
  }
};
