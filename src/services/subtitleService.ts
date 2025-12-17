import { invoke } from '@tauri-apps/api/tauri';

export interface SubtitleTrack {
  id?: number;
  media_id: number;
  file_path: string;
  language?: string;
  label?: string;
  codec?: string;
  is_embedded: boolean;
  track_index?: number;
  added_at: string;
}

export const subtitleService = {
  async addSubtitleTrack(
    mediaId: number,
    filePath: string,
    language?: string,
    label?: string,
    codec?: string,
    isEmbedded: boolean = false,
    trackIndex?: number
  ): Promise<number> {
    return await invoke<number>('add_subtitle_track', {
      mediaId,
      filePath,
      language,
      label,
      codec,
      isEmbedded,
      trackIndex,
    });
  },

  async getSubtitleTracks(mediaId: number): Promise<SubtitleTrack[]> {
    return await invoke<SubtitleTrack[]>('get_subtitle_tracks', { mediaId });
  },

  async removeSubtitleTrack(subtitleId: number): Promise<void> {
    await invoke('remove_subtitle_track', { subtitleId });
  },

  async scanSubtitles(mediaId: number, mediaPath: string): Promise<number[]> {
    return await invoke<number[]>('scan_subtitles', { mediaId, mediaPath });
  },
};
