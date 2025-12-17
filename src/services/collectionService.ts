import { invoke } from '@tauri-apps/api/tauri';

export interface Collection {
  id: number;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
  item_count: number;
}

export interface CollectionMediaItem {
  id: number;
  file_path: string;
  file_name: string;
  title?: string;
  year?: number;
  media_type: string;
  duration?: number;
  added_at: string;
}

export const collectionService = {
  async createCollection(name: string, description?: string): Promise<number> {
    return await invoke<number>('create_collection', { name, description });
  },

  async getAllCollections(): Promise<Collection[]> {
    return await invoke<Collection[]>('get_all_collections');
  },

  async getCollectionMedia(collectionId: number): Promise<CollectionMediaItem[]> {
    return await invoke<CollectionMediaItem[]>('get_collection_media', { collectionId });
  },

  async addToCollection(collectionId: number, mediaId: number): Promise<void> {
    await invoke('add_to_collection', { collectionId, mediaId });
  },

  async removeFromCollection(collectionId: number, mediaId: number): Promise<void> {
    await invoke('remove_from_collection', { collectionId, mediaId });
  },

  async updateCollection(collectionId: number, name: string, description?: string): Promise<void> {
    await invoke('update_collection', { collectionId, name, description });
  },

  async deleteCollection(collectionId: number): Promise<void> {
    await invoke('delete_collection', { collectionId });
  },
};
