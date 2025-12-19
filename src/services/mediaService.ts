import { invoke } from '@tauri-apps/api/tauri';
import { dialog } from '@tauri-apps/api';

export interface MediaFile {
  id?: number;
  file_path: string;
  file_hash: string;
  file_name: string;
  file_size: number;
  media_type: 'movie' | 'tv_episode' | 'music' | 'video' | 'audio';
  duration?: number;
  codec?: string;
  resolution?: string;
  bitrate?: number;
  framerate?: number;
  audio_codec?: string;
  audio_channels?: number;
  title?: string;
  year?: number;
  season_number?: number;
  episode_number?: number;
  indexed_at: string;
  last_modified: string;
  is_deleted: boolean;
  metadata_json?: string;
}

export interface ScanProgress {
  current_file: string;
  files_scanned: number;
  files_found: number;
  current_dir: string;
}

export interface ScanResult {
  total_found: number;
  added: number;
  updated: number;
  errors: number;
}

export interface FilterCriteria {
  min_year?: number;
  max_year?: number;
  min_duration?: number;
  max_duration?: number;
  resolutions?: string[];
  codecs?: string[];
  media_types?: string[];
}

export const mediaService = {
  /**
   * Scan a directory for media files
   */
  async scanDirectory(path: string): Promise<ScanResult> {
    return invoke<ScanResult>('scan_directory', { path });
  },

  /**
   * Open directory picker and scan selected directory
   */
  async selectAndScanDirectory(): Promise<ScanResult | null> {
    const selected = await dialog.open({
      directory: true,
      multiple: false,
      title: 'Select Media Library Folder',
    });

    if (selected && typeof selected === 'string') {
      return this.scanDirectory(selected);
    }

    return null;
  },

  /**
   * Get all media files
   */
  async getAllMedia(): Promise<MediaFile[]> {
    return invoke<MediaFile[]>('get_all_media');
  },

  /**
   * Get media files by type
   */
  async getMediaByType(type: 'movie' | 'tv_episode' | 'music'): Promise<MediaFile[]> {
    return invoke<MediaFile[]>('get_media_by_type', { mediaType: type });
  },

  /**
   * Search media files
   */
  async searchMedia(query: string): Promise<MediaFile[]> {
    return invoke<MediaFile[]>('search_media', { query });
  },

  /**
   * Filter media files
   */
  async filterMedia(criteria: FilterCriteria): Promise<MediaFile[]> {
    return invoke<MediaFile[]>('filter_media', { criteria });
  },

  /**
   * Convert MediaFile to display format
   */
  toDisplayFormat(media: MediaFile) {
    return {
      id: media.id?.toString() || media.file_path,
      title: media.title || media.file_name,
      year: media.year,
      duration: media.duration ? Math.floor(media.duration / 60) : undefined,
      type: media.media_type === 'tv_episode' ? 'tv' : media.media_type,
      rating: undefined,
      watched: false,
      progress: 0,
      posterUrl: undefined,
    };
  },
};
