import { invoke } from '@tauri-apps/api/tauri';
import { save, open } from '@tauri-apps/api/dialog';

export const backupService = {
  async exportDatabase(): Promise<void> {
    const savePath = await save({
      filters: [{ name: 'SQLite Database', extensions: ['db', 'sqlite'] }],
      defaultPath: 'cinevault-backup.db',
    });

    if (savePath) {
      await invoke('export_database', { outputPath: savePath });
    }
  },

  async importDatabase(): Promise<boolean> {
    const selected = await open({
      filters: [{ name: 'SQLite Database', extensions: ['db', 'sqlite'] }],
      multiple: false,
    });

    if (selected && typeof selected === 'string') {
      await invoke('import_database', { inputPath: selected });
      return true;
    }
    return false;
  }
};
