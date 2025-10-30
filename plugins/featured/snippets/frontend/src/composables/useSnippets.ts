import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Snippet, SnippetInput } from '../types';

function extractId(id: any): string {
  if (typeof id === 'string') return id;
  if (!id) return '';
  const rawId = id && typeof id === 'object' ? JSON.parse(JSON.stringify(id)) : id;
  if (rawId && typeof rawId === 'object' && rawId.tb && rawId.id) {
    if (typeof rawId.id === 'object' && rawId.id.String) return `${rawId.tb}:${rawId.id.String}`;
    if (typeof rawId.id === 'string') return `${rawId.tb}:${rawId.id}`;
  }
  return String(rawId);
}

function bareId(id: any): string {
  const full = extractId(id);
  if (full.includes(':')) return full.split(':').slice(1).join(':');
  return full;
}

export function useSnippets(pageId?: string) {
  const snippets = ref<Snippet[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Create a page-specific source identifier
  const source = pageId ? `snippets-plugin-page-${pageId}` : 'snippets-plugin';

  async function loadSnippets() {
    loading.value = true;
    error.value = null;
    try {
      // Load all snippet records
      const records = await invoke<Snippet[]>('get_records_by_type', { recordType: 'snippet' });

      // Filter by source - include both page-specific and generic sources for backward compatibility
      // After migration, snippets may have either the old page-specific source or the generic source
      const filteredRecords = pageId
        ? records.filter(r => r.source === source || r.source === 'snippets-plugin')
        : records.filter(r => r.source === 'snippets-plugin');


      snippets.value = filteredRecords.sort((a, b) => new Date(b.data.updatedAt).getTime() - new Date(a.data.updatedAt).getTime());
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      snippets.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function createSnippet(input: SnippetInput) {
    loading.value = true;
    error.value = null;
    try {
      const now = new Date().toISOString();
      const record = {
        record_type: 'snippet',
        source: source, // Use page-specific source
        timestamp: now,
        data: {
          ...input,
          createdAt: now,
          updatedAt: now
        },
        metadata: {
          tags: ['snippets', ...input.tags],
          status: null,
          title: input.title,
          description: input.language
        }
      };
      await invoke('upsert_record', { record });
      await loadSnippets();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateSnippet(snippet: Snippet, input: SnippetInput) {
    loading.value = true;
    error.value = null;
    try {
      const id = bareId(snippet.id);
      const now = new Date().toISOString();
      const record = {
        record_type: 'snippet',
        source: 'snippets-plugin',
        timestamp: snippet.timestamp,
        data: {
          ...snippet.data,
          ...input,
          updatedAt: now
        },
        metadata: {
          ...(snippet.metadata || {}),
          tags: ['snippets', ...input.tags],
          title: input.title,
          description: input.language
        }
      };
      await invoke('update_record', { id, record });
      await loadSnippets();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteSnippet(snippet: Snippet) {
    loading.value = true;
    error.value = null;
    try {
      const id = bareId(snippet.id);
      await invoke('delete_record', { id });
      await loadSnippets();
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return { snippets, loading, error, loadSnippets, createSnippet, updateSnippet, deleteSnippet };
}

