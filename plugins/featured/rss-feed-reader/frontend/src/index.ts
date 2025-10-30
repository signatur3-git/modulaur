import RSSFeedReaderPanel from './RSSFeedReaderPanel.vue';

const plugin = {
  install(app: any) {
    app.component('RSSFeedReaderPanel', RSSFeedReaderPanel);
  },
  // Export component directly for UMD access
  components: {
    RSSFeedReaderPanel
  },
  // Plugin metadata
  meta: {
    name: 'rss-feed-reader',
    version: '1.0.0',
    components: {
      RSSFeedReaderPanel: {
        display_name: 'RSS Feed Reader',
        description: 'Read RSS feeds with categories, read/unread status, and favorites - each panel has its own feeds',
        icon: '📰',
        category: 'news'
      }
    }
  }
};

export default plugin;
