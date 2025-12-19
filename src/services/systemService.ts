import { invoke } from '@tauri-apps/api/tauri';

export interface DependencyStatus {
  ffmpeg: boolean;
  vlc: boolean;
}

export const systemService = {
  async checkDependencies(): Promise<DependencyStatus> {
    return await invoke<DependencyStatus>('check_dependencies');
  },

  async extractAllMetadata(): Promise<{ total: number, processed: number, updated: number, errors: number }> {
    return await invoke('extract_all_metadata');
  }
};
