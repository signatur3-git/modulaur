import MarkdownNotepadPanel from './MarkdownNotepadPanel.vue';
const plugin = {
  install(app: any) {
    app.component('MarkdownNotepadPanel', MarkdownNotepadPanel);
  },
  // Export component directly for UMD access
  components: {
    MarkdownNotepadPanel
  },
  // Plugin metadata
  meta: {
    name: 'markdown-notepad',
    version: '1.0.0',
    components: {
      MarkdownNotepadPanel: {
        display_name: 'Markdown Notepad',
        description: 'Take notes with markdown formatting - each panel has its own documents',
        icon: '📝',
        category: 'utility'
      }
    }
  }
};
export default plugin;
// For UMD build - attach to window
if (typeof window !== 'undefined') {
  (window as any).MarkdownNotepadPlugin = plugin;
}
