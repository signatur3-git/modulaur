import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { SavedSearch } from '../types';

function bareId(id: any): string {
  if (typeof id === 'string') {
    const full = id;
    if (full.includes(':')) return full.split(':').slice(1).join(':');
    return full;
  }
  if (!id) return '';
  const rawId = id && typeof id === 'object' ? JSON.parse(JSON.stringify(id)) : id;
  if (rawId && typeof rawId === 'object' && rawId.tb && rawId.id) {
    if (typeof rawId.id === 'object' && rawId.id.String) return rawId.id.String;
    if (typeof rawId.id === 'string') return rawId.id;
  }
  return String(rawId);
}

export function useSavedSearches(pageId?: string) {
  const savedSearches = ref<SavedSearch[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const source = pageId ? `snippets-plugin-page-${pageId}` : 'snippets-plugin';

  async function loadSavedSearches() {
    loading.value = true;
    error.value = null;
    try {
      const records = await invoke<SavedSearch[]>('get_records_by_type', { recordType: 'saved_search' });

      const filteredRecords = pageId
        ? records.filter(r => r.source === source || r.source === 'snippets-plugin')
        : records.filter(r => r.source === 'snippets-plugin');

      savedSearches.value = filteredRecords.sort((a, b) =>
        new Date(b.data.createdAt).getTime() - new Date(a.data.createdAt).getTime()
      );
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      savedSearches.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function createSavedSearch(query: string) {
    if (!query.trim()) return;

    loading.value = true;
    error.value = null;
    try {
      const now = new Date().toISOString();
      const record = {
        record_type: 'saved_search',
        source: source,
        timestamp: now,
        data: {
          query: query.trim(),
          createdAt: now
        },
        metadata: {
          tags: ['snippets', 'search'],
          title: query.trim()
        }
      };
      await invoke('upsert_record', { record });
      await loadSavedSearches();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteSavedSearch(search: SavedSearch) {
    loading.value = true;
    error.value = null;
    try {
      const id = bareId(search.id);
      await invoke('delete_record', { id });
      await loadSavedSearches();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return { savedSearches, loading, error, loadSavedSearches, createSavedSearch, deleteSavedSearch };
}

