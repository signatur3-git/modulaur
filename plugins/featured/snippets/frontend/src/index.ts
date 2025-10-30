import { App } from 'vue';
import SnippetsPage from './SnippetsPage.vue';
import SnippetsPanel from './components/SnippetsPanel.vue';

export default {
  install(app: App) {
    app.component('SnippetsPage', SnippetsPage);
    app.component('SnippetsPanel', SnippetsPanel);
  },

  components: {
    SnippetsPage,
    SnippetsPanel
  },

  meta: {
    name: 'snippets',
    version: '1.0.0',
    description: 'Code snippets with syntax highlighting and copy-to-clipboard',
    pages: [
      {
        id: 'snippets',
        name: 'Snippets',
        component: 'SnippetsPage',
        icon: '✂️',
        description: 'Save and manage reusable code snippets'
      }
    ],
    panels: [
      {
        id: 'snippets-panel',
        name: 'Snippets',
        component: 'SnippetsPanel',
        icon: '✂️',
        defaultSize: { w: 8, h: 8 }
      }
    ]
  }
};
