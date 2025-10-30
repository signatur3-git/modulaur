<template>
  <div class="markdown-notepad">
    <div class="notepad-header" v-if="!singleDocumentMode">
      <h3>📝 Markdown Notepad</h3>
      <div class="mode-toggle">
        <button @click="mode = 'edit'" :class="{active: mode === 'edit'}" title="Edit Mode">
          ✏️ Edit
        </button>
        <button @click="mode = 'preview'" :class="{active: mode === 'preview'}" title="Preview Mode">
          👁️ Preview
        </button>
        <button @click="mode = 'split'" :class="{active: mode === 'split'}" title="Split View">
          ⚡ Split
        </button>
      </div>
    </div>

    <!-- Tab Bar -->
    <div class="tab-bar" v-if="!singleDocumentMode">
      <VueDraggable
        v-model="notes"
        tag="div"
        :animation="200"
        handle=".tab-drag-handle"
        :force-fallback="true"
        :fallback-tolerance="3"
        ghost-class="tab-ghost"
        @end="handleTabReorder"
        class="tabs-container"
        @mousedown.stop
        @touchstart.stop
      >
          <div
            v-for="note in notes"
            :key="note.id"
            class="tab"
            :class="{active: note.id === activeNoteId}"
            @click="selectNote(note.id)"
            @dblclick="startRename(note.id)"
          >
            <span class="tab-drag-handle" title="Drag to reorder">⋮⋮</span>
            <input
              v-if="renamingId === note.id"
              v-model="note.title"
              @blur="stopRename"
              @keyup.enter="stopRename"
              @keyup.escape="cancelRename"
              @click.stop
              ref="renameInput"
              class="rename-input"
            />
            <span v-else class="tab-title">{{ note.title }}</span>
            <button @click.stop="closeNote(note.id)" class="close-btn" title="Close">×</button>
          </div>
        </VueDraggable>
      <button class="new-tab" @click="createNote" title="New Note">+ New</button>
    </div>

    <!-- Content Area -->
    <div class="content-area" :class="`mode-${effectiveMode}`">
      <!-- Edit Mode -->
      <div v-if="effectiveMode === 'edit' || effectiveMode === 'split'" class="editor">
        <div class="toolbar" v-if="!singleDocumentMode">
          <button @click="insertMarkdown('**', '**')" title="Bold" class="toolbar-btn">
            <strong>B</strong>
          </button>
          <button @click="insertMarkdown('*', '*')" title="Italic" class="toolbar-btn">
            <em>I</em>
          </button>
          <button @click="insertMarkdown('# ', '')" title="Heading 1" class="toolbar-btn">H1</button>
          <button @click="insertMarkdown('## ', '')" title="Heading 2" class="toolbar-btn">H2</button>
          <button @click="insertMarkdown('### ', '')" title="Heading 3" class="toolbar-btn">H3</button>
          <button @click="insertMarkdown('- ', '')" title="Bullet List" class="toolbar-btn">• List</button>
          <button @click="insertMarkdown('```\n', '\n```')" title="Code Block" class="toolbar-btn">&lt;/&gt;</button>
          <button @click="insertMarkdown('`', '`')" title="Inline Code" class="toolbar-btn">`code`</button>
          <button @click="insertMarkdown('[', '](url)')" title="Link" class="toolbar-btn">🔗</button>
        </div>
        <textarea
          v-model="currentNote.content"
          @input="autoSave"
          placeholder="Write your markdown here..."
          ref="editorTextarea"
        ></textarea>
      </div>

      <!-- Preview Mode -->
      <div v-if="effectiveMode === 'preview' || effectiveMode === 'split'" class="preview">
        <div v-html="renderedMarkdown" class="markdown-body"></div>
      </div>
    </div>

    <!-- Footer -->
    <div class="notepad-footer" v-if="!singleDocumentMode">
      <div class="footer-info">
        {{ notes.length }} note{{ notes.length !== 1 ? 's' : '' }}
        <span v-if="currentNote.content"> • {{ wordCount }} words</span>
        <span v-if="lastSaved"> • Saved {{ lastSaved }}</span>
      </div>
      <div class="footer-actions">
        <button @click="exportNote" title="Export current note" class="footer-btn">📥 Export</button>
        <button @click="importNote" title="Import note" class="footer-btn">📤 Import</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue';
import { marked } from 'marked';
import hljs from 'highlight.js';
import { VueDraggable } from 'vue-draggable-plus';

// Props - panel object allows per-panel isolation
const props = defineProps<{
  isEditing?: boolean;
  panel: {
    i: string;
    [key: string]: any;
  };
}>();

// Configure marked with syntax highlighting
marked.setOptions({
  highlight: (code: string, lang: string) => {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(code, { language: lang }).value;
      } catch (e) {
        console.error('Highlight error:', e);
      }
    }
    return code;
  },
  breaks: true,
  gfm: true
} as any);

interface Note {
  id: string;
  title: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

// State
const notes = ref<Note[]>([]);
const activeNoteId = ref<string>('');
const mode = ref<'edit' | 'preview' | 'split'>('split');
const renamingId = ref<string>('');
const oldTitle = ref<string>('');
const lastSaved = ref<string>('');
const editorTextarea = ref<HTMLTextAreaElement>();
const renameInput = ref<HTMLInputElement>();

// Storage key unique to this panel instance
const storageKey = computed(() => `markdown-notes-${props.panel.i}`);

// Current note
const currentNote = computed(() => {
  return notes.value.find(n => n.id === activeNoteId.value) || {
    id: '',
    title: 'No Note',
    content: '',
    createdAt: '',
    updatedAt: ''
  };
});

// With this:
const singleDocumentMode = ref(props.panel.config?.singleDocumentMode || false);

// Effective mode - in single document mode, always split
const effectiveMode = computed(() => {
  if (singleDocumentMode.value) return (props.isEditing ? 'edit' : 'preview');
  return mode.value;
});

// Rendered markdown
const renderedMarkdown = computed(() => {
  try {
    return marked(currentNote.value.content || '') as string;
  } catch (e) {
    console.error('Markdown parse error:', e);
    return '<p>Error rendering markdown</p>';
  }
});

// Word count
const wordCount = computed(() => {
  const text = currentNote.value.content.trim();
  if (!text) return 0;
  return text.split(/\s+/).length;
});

// Functions
function createNote() {
  const note: Note = {
    id: Date.now().toString(),
    title: 'Untitled Note',
    content: '# New Note\n\nStart writing your markdown here...\n\n## Features\n\n- **Bold** and *italic* text\n- Lists and checkboxes\n- Code blocks with syntax highlighting\n- Links and images\n\n```javascript\nconst example = "Hello World";\nconsole.log(example);\n```',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
  notes.value.push(note);
  activeNoteId.value = note.id;
  saveToStorage();
}

function selectNote(id: string) {
  activeNoteId.value = id;
}

function closeNote(id: string) {
  const index = notes.value.findIndex(n => n.id === id);
  if (index === -1) return;

  notes.value.splice(index, 1);

  if (activeNoteId.value === id) {
    if (notes.value.length > 0) {
      // Select previous note or first note
      const newIndex = Math.max(0, index - 1);
      activeNoteId.value = notes.value[newIndex].id;
    } else {
      activeNoteId.value = '';
    }
  }

  saveToStorage();
}

function handleTabReorder() {
  // Save the new order to storage after drag and drop
  saveToStorage();
}

function startRename(id: string) {
  const note = notes.value.find(n => n.id === id);
  if (!note) return;

  renamingId.value = id;
  oldTitle.value = note.title;

  nextTick(() => {
    if (renameInput.value && renameInput.value[0]) {
      renameInput.value[0].focus();
      renameInput.value[0].select();
    }
  });
}

function stopRename() {
  if (renamingId.value) {
    const note = notes.value.find(n => n.id === renamingId.value);
    if (note && !note.title.trim()) {
      note.title = 'Untitled';
    }
    renamingId.value = '';
    saveToStorage();
  }
}

function cancelRename() {
  if (renamingId.value) {
    const note = notes.value.find(n => n.id === renamingId.value);
    if (note) {
      note.title = oldTitle.value;
    }
    renamingId.value = '';
  }
}

function insertMarkdown(before: string, after: string) {
  const textarea = editorTextarea.value;
  if (!textarea) return;

  const start = textarea.selectionStart;
  const end = textarea.selectionEnd;
  const selectedText = currentNote.value.content.substring(start, end);
  const text = currentNote.value.content;

  currentNote.value.content =
    text.substring(0, start) +
    before + selectedText + after +
    text.substring(end);

  // Restore cursor position
  nextTick(() => {
    if (textarea) {
      const newPosition = start + before.length + selectedText.length;
      textarea.focus();
      textarea.setSelectionRange(newPosition, newPosition);
    }
  });

  autoSave();
}

let saveTimer: number;
function autoSave() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    if (currentNote.value.id) {
      const note = notes.value.find(n => n.id === currentNote.value.id);
      if (note) {
        note.updatedAt = new Date().toISOString();
      }
    }
    saveToStorage();
    lastSaved.value = 'just now';

    // Clear "just now" after 3 seconds
    setTimeout(() => {
      lastSaved.value = '';
    }, 3000);
  }, 500);
}

function saveToStorage() {
  try {
    localStorage.setItem(storageKey.value, JSON.stringify(notes.value));
    if (activeNoteId.value) {
      localStorage.setItem(`${storageKey.value}-active`, activeNoteId.value);
    }
    localStorage.setItem(`${storageKey.value}-mode`, mode.value);
    localStorage.setItem(`${storageKey.value}-singleDocumentMode`, singleDocumentMode.value.toString());
  } catch (e) {
    console.error('Failed to save notes:', e);
  }
}

function loadFromStorage() {
  try {
    console.log(`[Markdown Panel] Loading storage for key: ${storageKey.value}`)
    const stored = localStorage.getItem(storageKey.value);
    if (stored) {
      console.log(`[Markdown Panel] Found stored data, length: ${stored.length}`)
      notes.value = JSON.parse(stored);
      console.log(`[Markdown Panel] Loaded ${notes.value.length} notes`)

      // Load active note
      const activeStored = localStorage.getItem(`${storageKey.value}-active`);
      console.log(`[Markdown Panel] Active note ID from storage: ${activeStored}`)
      if (activeStored && notes.value.find(n => n.id === activeStored)) {
        activeNoteId.value = activeStored;
        console.log(`[Markdown Panel] Set active note to: ${activeStored}`)
      } else if (notes.value.length > 0) {
        activeNoteId.value = notes.value[0].id;
        console.log(`[Markdown Panel] No active note found, using first note: ${notes.value[0].id}`)
      }

      // Load mode
      const modeStored = localStorage.getItem(`${storageKey.value}-mode`);
      if (modeStored && ['edit', 'preview', 'split'].includes(modeStored)) {
        mode.value = modeStored as 'edit' | 'preview' | 'split';
        console.log(`[Markdown Panel] Loaded mode: ${mode.value}`)
      }

      // Load single document mode
      const singleDocStored = localStorage.getItem(`${storageKey.value}-singleDocumentMode`);
      if (singleDocStored !== undefined) {
        const parsed = singleDocStored === 'true';
        if (props.panel.config?.singleDocumentMode !== undefined) {
          // Respect panel config if defined
          singleDocumentMode.value = props.panel.config.singleDocumentMode;
        } else {
          singleDocumentMode.value = parsed;
        }
        console.log(`[Markdown Panel] Loaded singleDocumentMode: ${singleDocumentMode.value}`)
      }
    } else {
      console.log(`[Markdown Panel] No stored data found for key: ${storageKey.value}`)
    }

    // Create initial note if none exist
    if (notes.value.length === 0) {
      createNote();
    }
  } catch (e) {
    console.error('Failed to load notes:', e);
    createNote();
  }
}

function exportNote() {
  if (!currentNote.value.id) return;

  const blob = new Blob([currentNote.value.content], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${currentNote.value.title.replace(/[^a-z0-9]/gi, '_').toLowerCase()}.md`;
  a.click();
  URL.revokeObjectURL(url);
}

function importNote() {
  const input = document.createElement('input');
  input.type = 'file';
  input.accept = '.md,.markdown,.txt';
  input.onchange = (e: Event) => {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const content = event.target?.result as string;
      const note: Note = {
        id: Date.now().toString(),
        title: file.name.replace(/\.(md|markdown|txt)$/, ''),
        content,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      };
      notes.value.push(note);
      activeNoteId.value = note.id;
      saveToStorage();
    };
    reader.readAsText(file);
  };
  input.click();
}

// Watch mode changes
watch(mode, () => {
  saveToStorage();
});

// Watch single document mode config
watch(() => props.panel.config?.singleDocumentMode, (newVal) => {
  if (newVal !== undefined) {
    singleDocumentMode.value = newVal;
  }
});

// Load on mount
onMounted(() => {
  loadFromStorage();
});
</script>

<style>
.markdown-notepad {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  overflow: hidden;
  box-shadow: var(--panel-shadow);
}

.notepad-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--panel-header-padding);
  background: var(--bg-panel-header);
  color: var(--text-primary);
  border-bottom: 1px solid var(--border-color);
}

.notepad-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-heading);
}

.mode-toggle {
  display: flex;
  gap: var(--space-sm);
}

.mode-toggle button {
  padding: var(--space-sm) var(--space-md);
  border: 2px solid var(--border-subtle);
  background: var(--bg-button-secondary);
  color: var(--text-on-accent);
  cursor: pointer;
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.9rem;
  transition: all 0.2s;
}

.mode-toggle button:hover {
  background: var(--bg-button-secondary-hover);
  border-color: var(--border-color);
}

.mode-toggle button.active {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-color: var(--accent-primary);
  font-weight: bold;
}

.tab-bar {
  display: flex;
  gap: var(--space-xs);
  padding: var(--space-sm) var(--space-sm) 0;
  background: var(--bg-panel-header);
  overflow-x: auto;
  border-bottom: 2px solid var(--border-color);
}

.tabs-container {
  display: flex;
  gap: var(--space-xs);
  flex: 1;
}

.tab {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background: var(--bg-button-secondary);
  border-radius: calc(var(--panel-radius) / 2) calc(var(--panel-radius) / 2) 0 0;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 120px;
  max-width: 200px;
}

.tab-drag-handle {
  cursor: move;
  color: var(--text-muted);
  font-size: 0.9rem;
  line-height: 1;
  padding: 0 2px;
  user-select: none;
  opacity: 0.5;
  transition: opacity 0.2s;
}

.tab:hover .tab-drag-handle {
  opacity: 1;
}

.tab-ghost {
  opacity: 0.5;
  background: var(--accent-primary) !important;
}

.tab:hover {
  background: var(--bg-button-secondary-hover);
}

.tab.active {
  background: var(--bg-panel);
  font-weight: 600;
  box-shadow: 0 -2px 4px rgba(0,0,0,0.1);
  border: 1px solid var(--border-color);
  border-bottom: none;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary);
}

.rename-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: inherit;
  font-family: inherit;
  outline: none;
  padding: 0;
  color: var(--text-primary);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  line-height: 1;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
}

.close-btn:hover {
  background: var(--bg-button-secondary-hover);
  color: var(--accent-danger);
}

.new-tab {
  padding: var(--space-sm) var(--space-md);
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: calc(var(--panel-radius) / 2) calc(var(--panel-radius) / 2) 0 0;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.new-tab:hover {
  background: var(--accent-hover);
}

.content-area {
  flex: 1;
  display: flex;
  gap: var(--panel-gap);
  padding: var(--panel-padding);
  min-height: 0;
  background: var(--bg-panel);
}

.content-area.mode-edit {
  padding: var(--panel-padding) 0 var(--panel-padding) 0;
}

.content-area.mode-edit .editor {
  flex: 1;
}

.content-area.mode-preview .preview {
  flex: 1;
}

.content-area.mode-split .editor,
.content-area.mode-split .preview {
  flex: 1;
}

.editor, .preview {
  border: 1px solid var(--border-color);
  border-radius: var(--panel-radius);
  overflow: hidden;
  background: var(--bg-panel);
  display: flex;
  flex-direction: column;
}

.toolbar {
  display: flex;
  gap: var(--space-xs);
  padding: var(--space-sm);
  background: var(--bg-panel-header);
  border-bottom: 1px solid var(--border-color);
  flex-wrap: wrap;
}

.toolbar-btn {
  padding: calc(var(--space-xs) * 1.6) calc(var(--space-sm) * 1.2);
  border: 1px solid var(--border-subtle);
  background: var(--bg-panel);
  cursor: pointer;
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.85rem;
  transition: all 0.2s;
  color: var(--text-primary);
}

.toolbar-btn:hover {
  background: var(--bg-panel-header);
  border-color: var(--border-color);
}

.toolbar-btn:active {
  background: var(--border-subtle);
}

textarea {
  flex: 1;
  width: 100%;
  border: none;
  outline: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 0.95rem;
  line-height: 1.6;
  resize: none;
  padding: var(--space-lg);
  background: var(--bg-panel);
  color: var(--text-primary);
}

.preview {
  padding: var(--space-lg);
  overflow: auto;
  color: var(--text-primary);
}

.markdown-body {
  max-width: 100%;
  word-wrap: break-word;
}

/* Markdown styling */
.markdown-body h1 {
  border-bottom: 2px solid var(--border-color);
  padding-bottom: calc(var(--space-xs) * 3);
  margin-top: var(--space-xl);
  margin-bottom: var(--space-lg);
  color: var(--text-heading);
}

.markdown-body h2 {
  border-bottom: 1px solid var(--border-color);
  padding-bottom: calc(var(--space-xs) * 3);
  margin-top: var(--space-xl);
  margin-bottom: calc(var(--space-lg) * 0.8);
  color: var(--text-heading);
}

.markdown-body h3 {
  margin-top: var(--space-lg);
  margin-bottom: calc(var(--space-lg) * 0.6);
  color: var(--text-heading);
}

.markdown-body code {
  background: var(--bg-panel-header);
  padding: calc(var(--space-xs) * 2) calc(var(--space-sm) * 2);
  border-radius: 3px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 0.9em;
  border: 1px solid var(--border-subtle);
  color: var(--text-primary);
}

.markdown-body pre {
  background: var(--bg-panel-header);
  padding: var(--space-lg);
  border-radius: var(--panel-radius);
  overflow: auto;
  border: 1px solid var(--border-color);
  margin: var(--space-lg) 0;
}

.markdown-body pre code {
  background: none;
  padding: 0;
  border: none;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.markdown-body ul, .markdown-body ol {
  padding-left: calc(var(--space-lg) * 2);
  margin: var(--space-sm) 0;
}

.markdown-body li {
  margin: calc(var(--space-xs) * 2.5) 0;
  color: var(--text-primary);
}

.markdown-body blockquote {
  border-left: 4px solid var(--border-color);
  padding-left: var(--space-lg);
  margin: var(--space-lg) 0;
  color: var(--text-secondary);
}

.markdown-body a {
  color: var(--accent-primary);
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body img {
  max-width: 100%;
  height: auto;
  border-radius: calc(var(--panel-radius) / 2);
}

.markdown-body table {
  border-collapse: collapse;
  width: 100%;
  margin: var(--space-lg) 0;
}

.markdown-body th,
.markdown-body td {
  border: 1px solid var(--border-color);
  padding: var(--space-sm);
  text-align: left;
  color: var(--text-primary);
}

.markdown-body th {
  background: var(--bg-panel-header);
  font-weight: 600;
  color: var(--text-heading);
}

.notepad-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: calc(var(--space-lg) * 0.75) var(--space-lg);
  background: var(--bg-panel-header);
  border-top: 1px solid var(--border-color);
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.footer-actions {
  display: flex;
  gap: var(--space-sm);
}

.footer-btn {
  padding: calc(var(--space-xs) * 3.2) calc(var(--space-sm) * 1.6);
  border: 1px solid var(--border-subtle);
  background: var(--bg-panel);
  cursor: pointer;
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.85rem;
  transition: all 0.2s;
  color: var(--text-primary);
}

.footer-btn:hover {
  background: var(--bg-panel-header);
  border-color: var(--border-color);
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-panel-header);
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}
</style>
