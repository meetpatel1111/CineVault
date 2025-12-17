import { invoke } from '@tauri-apps/api/tauri';

export interface Playlist {
  id: number;
  name: string;
  description?: string;
  playlist_type: string;
  created_at: string;
  updated_at: string;
  item_count: number;
}

export interface PlaylistMediaItem {
  id: number;
  file_path: string;
  file_name: string;
  title?: string;
  year?: number;
  media_type: string;
  duration?: number;
  position: number;
}

export const playlistService = {
  async createPlaylist(name: string, description?: string, playlistType: string = 'manual'): Promise<number> {
    return await invoke<number>('create_playlist', { 
      name, 
      description, 
      playlistType 
    });
  },

  async getAllPlaylists(): Promise<Playlist[]> {
    return await invoke<Playlist[]>('get_all_playlists');
  },

  async getPlaylistMedia(playlistId: number): Promise<PlaylistMediaItem[]> {
    return await invoke<PlaylistMediaItem[]>('get_playlist_media', { playlistId });
  },

  async addToPlaylist(playlistId: number, mediaId: number): Promise<void> {
    await invoke('add_to_playlist', { playlistId, mediaId });
  },

  async removeFromPlaylist(playlistId: number, mediaId: number): Promise<void> {
    await invoke('remove_from_playlist', { playlistId, mediaId });
  },

  async updatePlaylist(playlistId: number, name: string, description?: string): Promise<void> {
    await invoke('update_playlist', { playlistId, name, description });
  },

  async deletePlaylist(playlistId: number): Promise<void> {
    await invoke('delete_playlist', { playlistId });
  },
};
