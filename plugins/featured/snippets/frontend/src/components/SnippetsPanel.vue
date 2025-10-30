<template>
  <div class="snippets-panel">
    <header class="header">
      <div>
        <h2>Snippets</h2>
        <div class="sub">Persisted in the local database</div>
      </div>
      <button class="btn" @click="toggleCreateMode">
        {{ createMode ? 'Cancel' : 'New Snippet' }}
      </button>
    </header>

    <div v-if="error" class="error">{{ error }}</div>

    <div class="layout">
      <!-- Left Sidebar: Saved Searches -->
      <aside class="sidebar">
        <div class="sidebar-header">
          <h3>Saved Searches</h3>
        </div>

        <div class="search-input-container">
          <input
            class="search"
            v-model="q"
            placeholder="Search title / tag / language"
            @keydown.enter="saveCurrentSearch"
          />
          <button
            class="btn-save-search"
            @click="saveCurrentSearch"
            :disabled="!q.trim()"
            title="Save this search"
          >
            💾
          </button>
        </div>

        <div class="saved-searches-list">
          <div v-if="savedSearches.length === 0" class="muted">No saved searches</div>
          <div
            v-for="search in savedSearches"
            :key="searchIdKey(search)"
            class="saved-search-item"
            :class="{ active: activeSearchId === searchIdKey(search) }"
          >
            <button
              class="search-text"
              @click="applySavedSearch(search)"
            >
              {{ search.data.query }}
            </button>
            <button
              class="btn-delete-search"
              @click="removeSavedSearch(search)"
              title="Delete search"
            >
              ✕
            </button>
          </div>
        </div>
      </aside>

      <!-- Main Area: Snippet Cards -->
      <main class="main">
        <!-- Create New Snippet Form -->
        <div v-if="createMode" class="snippet-card create-card">
          <div class="card-header">
            <h3>New Snippet</h3>
          </div>
          <div class="form">
            <label>
              Title
              <input v-model="form.title" placeholder="Snippet title" />
            </label>

            <label>
              Language
              <input v-model="form.language" placeholder="e.g. typescript, json, bash" />
            </label>

            <label>
              Tags (comma separated)
              <input v-model="tagsText" placeholder="tag1, tag2, tag3" />
            </label>

            <label>
              Code
              <textarea v-model="form.code" rows="12" spellcheck="false" placeholder="Paste your code here..."></textarea>
            </label>

            <div class="actions">
              <button class="btn btn-primary" :disabled="!form.title || !form.code" @click="save">Save Snippet</button>
              <button class="btn" @click="toggleCreateMode">Cancel</button>
            </div>
          </div>
        </div>

        <!-- Snippet Cards Grid -->
        <div v-if="loading && snippets.length === 0" class="muted">Loading…</div>
        <div v-else-if="filtered.length === 0" class="muted">
          {{ q ? 'No snippets match your search' : 'No snippets yet. Create your first one!' }}
        </div>

        <div v-else class="snippets-grid">
          <div
            v-for="snippet in filtered"
            :key="idKey(snippet)"
            class="snippet-card"
          >
            <div class="card-header">
              <div class="card-title-row">
                <h3>{{ snippet.data.title }}</h3>
                <div class="card-actions">
                  <button class="btn-icon" @click="copy(snippet.data.code)" title="Copy code">📋</button>
                  <button class="btn-icon" @click="startEdit(snippet)" title="Edit">✏️</button>
                  <button class="btn-icon btn-danger" @click="remove(snippet)" title="Delete">🗑️</button>
                </div>
              </div>
              <div class="card-meta">
                <span class="language-tag">{{ snippet.data.language }}</span>
                <span v-for="tag in snippet.data.tags" :key="tag" class="tag">{{ tag }}</span>
                <span class="date">{{ formatDate(snippet.data.updatedAt) }}</span>
              </div>
            </div>

            <!-- Edit Mode -->
            <div v-if="editingId === idKey(snippet)" class="form">
              <label>
                Title
                <input v-model="form.title" />
              </label>

              <label>
                Language
                <input v-model="form.language" />
              </label>

              <label>
                Tags (comma separated)
                <input v-model="tagsText" />
              </label>

              <label>
                Code
                <textarea v-model="form.code" rows="12" spellcheck="false"></textarea>
              </label>

              <div class="actions">
                <button class="btn btn-primary" :disabled="!form.title || !form.code" @click="saveEdit(snippet)">Save</button>
                <button class="btn" @click="cancelEdit">Cancel</button>
              </div>
            </div>

            <!-- View Mode -->
            <pre v-else class="code"><code v-html="highlightCode(snippet.data.code, snippet.data.language)"></code></pre>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import hljs from 'highlight.js/lib/common';
import type { Snippet, SnippetInput, SavedSearch } from '../types';
import { useSnippets } from '../composables/useSnippets';
import { useSavedSearches } from '../composables/useSavedSearches';

defineProps<{ panel: { i: string; [key: string]: any } }>();

const pageRoute = computed(() => {
  const hash = window.location.hash;
  const pageMatch = hash.match(/#\/page\/([^\/]+)/);
  return pageMatch ? pageMatch[1] : undefined;
});

const { snippets, loading, error, loadSnippets, createSnippet, updateSnippet, deleteSnippet } = useSnippets(pageRoute.value);
const { savedSearches, loadSavedSearches, createSavedSearch, deleteSavedSearch } = useSavedSearches(pageRoute.value);

const createMode = ref(false);
const editingId = ref<string | null>(null);
const q = ref('');
const activeSearchId = ref<string | null>(null);

const form = ref<SnippetInput>({
  title: '',
  language: 'typescript',
  code: '',
  tags: []
});

const tagsText = computed({
  get() {
    return (form.value.tags || []).join(', ');
  },
  set(v: string) {
    form.value.tags = v
      .split(',')
      .map(t => t.trim())
      .filter(Boolean);
  }
});

const filtered = computed(() => {
  const query = q.value.trim().toLowerCase();
  if (!query) return snippets.value;
  return snippets.value.filter(s => {
    const hay = `${s.data.title} ${s.data.language} ${(s.data.tags || []).join(' ')}`.toLowerCase();
    return hay.includes(query);
  });
});

function idKey(s: Snippet) {
  return typeof s.id === 'string' ? s.id : JSON.stringify(s.id);
}

function searchIdKey(s: SavedSearch) {
  return typeof s.id === 'string' ? s.id : JSON.stringify(s.id);
}

function highlightCode(code: string, lang: string) {
  try {
    if (lang && hljs.getLanguage(lang)) {
      return hljs.highlight(code, { language: lang }).value;
    }
  } catch {
    // ignore
  }
  return hljs.highlightAuto(code).value;
}

async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    const ta = document.createElement('textarea');
    ta.value = text;
    document.body.appendChild(ta);
    ta.select();
    document.execCommand('copy');
    document.body.removeChild(ta);
  }
}

function toggleCreateMode() {
  createMode.value = !createMode.value;
  if (createMode.value) {
    editingId.value = null;
    form.value = { title: '', language: 'typescript', code: '', tags: [] };
  }
}

function startEdit(s: Snippet) {
  createMode.value = false;
  editingId.value = idKey(s);
  form.value = {
    title: s.data.title,
    language: s.data.language,
    code: s.data.code,
    tags: s.data.tags || []
  };
}

function cancelEdit() {
  editingId.value = null;
}

async function save() {
  const input: SnippetInput = {
    title: form.value.title.trim(),
    language: (form.value.language || '').trim() || 'text',
    code: form.value.code,
    tags: form.value.tags || []
  };

  await createSnippet(input);
  createMode.value = false;
  form.value = { title: '', language: 'typescript', code: '', tags: [] };
}

async function saveEdit(snippet: Snippet) {
  const input: SnippetInput = {
    title: form.value.title.trim(),
    language: (form.value.language || '').trim() || 'text',
    code: form.value.code,
    tags: form.value.tags || []
  };

  await updateSnippet(snippet, input);
  editingId.value = null;
}

async function remove(s: Snippet) {
  if (!confirm(`Delete snippet "${s.data.title}"?`)) return;
  await deleteSnippet(s);
}

async function saveCurrentSearch() {
  if (!q.value.trim()) return;
  await createSavedSearch(q.value.trim());
}

function applySavedSearch(search: SavedSearch) {
  q.value = search.data.query;
  activeSearchId.value = searchIdKey(search);
}

async function removeSavedSearch(search: SavedSearch) {
  await deleteSavedSearch(search);
  if (activeSearchId.value === searchIdKey(search)) {
    activeSearchId.value = null;
  }
}

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return 'just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 7) return `${diffDays}d ago`;
  return date.toLocaleDateString();
}

onMounted(async () => {
  console.log('[SnippetsPanel] onMounted - pageRoute:', pageRoute.value);
  await Promise.all([loadSnippets(), loadSavedSearches()]);
  console.log('[SnippetsPanel] onMounted - snippets loaded:', snippets.value.length);
  console.log('[SnippetsPanel] onMounted - saved searches loaded:', savedSearches.value.length);
});
</script>

<style scoped>
.snippets-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--panel-gap);
  padding: var(--panel-padding);
  background: var(--bg-panel);
  color: var(--text-primary);
  border-radius: var(--panel-radius);
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-lg);
}

.header h2 {
  color: var(--text-heading);
  margin: 0;
}

.sub {
  color: var(--text-secondary);
  font-size: 12px;
}

.layout {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: var(--panel-gap);
}

/* Sidebar - Saved Searches */
.sidebar {
  min-height: 0;
  overflow: auto;
  border: 1px solid var(--border-color);
  border-radius: var(--panel-radius);
  padding: var(--space-md);
  background: var(--bg-panel-header);
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.sidebar-header h3 {
  margin: 0;
  font-size: 14px;
  color: var(--text-heading);
  font-weight: 600;
}

.search-input-container {
  display: flex;
  gap: var(--space-xs);
}

.search {
  flex: 1;
  padding: var(--space-sm);
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: var(--bg-panel);
  color: var(--text-primary);
  font-size: 13px;
}

.search::placeholder {
  color: var(--text-muted);
}

.btn-save-search {
  padding: var(--space-sm);
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: var(--bg-button-secondary);
  cursor: pointer;
  font-size: 16px;
}

.btn-save-search:hover:not(:disabled) {
  background: var(--bg-button-secondary-hover);
}

.btn-save-search:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.saved-searches-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.saved-search-item {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  padding: var(--space-xs);
  border-radius: 6px;
  border: 1px solid var(--border-subtle);
  background: var(--bg-panel);
}

.saved-search-item.active {
  border-color: var(--accent-primary);
  background: color-mix(in srgb, var(--accent-primary) 10%, var(--bg-panel));
}

.search-text {
  flex: 1;
  text-align: left;
  padding: var(--space-xs);
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
  word-break: break-word;
}

.search-text:hover {
  color: var(--accent-primary);
}

.btn-delete-search {
  padding: 2px 6px;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 14px;
  border-radius: 4px;
}

.btn-delete-search:hover {
  background: color-mix(in srgb, var(--accent-danger) 18%, transparent);
  color: var(--accent-danger);
}

/* Main Area */
.main {
  min-height: 0;
  overflow: auto;
  border: 1px solid var(--border-color);
  border-radius: var(--panel-radius);
  background: var(--bg-panel);
  padding: var(--space-md);
}

.snippets-grid {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

/* Snippet Cards */
.snippet-card {
  border: 1px solid var(--border-color);
  border-radius: var(--panel-radius);
  background: var(--bg-panel-header);
  padding: var(--space-md);
}

.create-card {
  border-color: var(--accent-primary);
  background: color-mix(in srgb, var(--accent-primary) 5%, var(--bg-panel-header));
}

.card-header {
  margin-bottom: var(--space-md);
}

.card-title-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--space-md);
  margin-bottom: var(--space-xs);
}

.card-title-row h3 {
  margin: 0;
  color: var(--text-heading);
  font-size: 16px;
  flex: 1;
}

.card-actions {
  display: flex;
  gap: var(--space-xs);
}

.btn-icon {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-panel);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.btn-icon:hover {
  background: var(--bg-button-secondary-hover);
}

.btn-icon.btn-danger:hover {
  background: color-mix(in srgb, var(--accent-danger) 18%, transparent);
  border-color: var(--accent-danger);
}

.card-meta {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-xs);
  font-size: 12px;
}

.language-tag {
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--accent-primary);
  color: white;
  font-weight: 500;
}

.tag {
  padding: 2px 8px;
  border-radius: 4px;
  background: color-mix(in srgb, var(--accent-primary) 20%, transparent);
  color: var(--text-primary);
}

.date {
  color: var(--text-muted);
  margin-left: auto;
}

.code {
  margin-top: var(--space-md);
  padding: var(--space-md);
  border-radius: 6px;
  background: #0f172a;
  color: #e2e8f0;
  overflow: auto;
  max-height: 400px;
}

.code :deep(code) {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 13px;
  line-height: 1.5;
}

/* Forms */
.form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

label {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

input,
textarea {
  font-size: 14px;
  padding: var(--space-sm);
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: var(--bg-panel);
  color: var(--text-primary);
}

textarea {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  resize: vertical;
}

.actions {
  display: flex;
  gap: var(--space-sm);
  flex-wrap: wrap;
}

.btn {
  padding: var(--space-sm) var(--space-lg);
  border: 1px solid var(--border-color);
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.btn:hover {
  background: var(--bg-button-secondary-hover);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--bg-button);
  border-color: transparent;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--bg-button-hover);
}

.header .btn {
  background: var(--bg-button);
  border-color: transparent;
  color: white;
}

.header .btn:hover {
  background: var(--bg-button-hover);
}

.error {
  padding: var(--space-md);
  border-radius: var(--panel-radius);
  background: color-mix(in srgb, var(--accent-danger) 16%, transparent);
  border: 1px solid color-mix(in srgb, var(--accent-danger) 28%, transparent);
  color: var(--text-primary);
}

.muted {
  color: var(--text-muted);
  padding: var(--space-md);
  text-align: center;
}
</style>
