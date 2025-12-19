import { invoke } from '@tauri-apps/api/tauri';

export interface AudioTrack {
  id: number;
  media_id: number;
  file_path: string;
  language?: string;
  codec?: string;
  channels?: number;
  is_default: boolean;
}

export const audioTrackService = {
  async getAudioTracks(mediaId: number): Promise<AudioTrack[]> {
    return await invoke<AudioTrack[]>('get_audio_tracks', { mediaId });
  },

  async setAudioTrack(trackIndex: number): Promise<void> {
    return await invoke('set_audio_track', { trackIndex });
  },
};
