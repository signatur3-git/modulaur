<template>
  <div class="rss-feed-reader">
    <!-- Header -->
    <div class="reader-header">
      <h3>📰 RSS Feed Reader</h3>
      <div class="header-actions">
        <button @click="refreshAllFeeds" :disabled="isRefreshing" class="refresh-btn">
          <span v-if="isRefreshing">⟳</span>
          <span v-else>🔄</span>
          Refresh
        </button>
        <button @click="showFeedManager = true" class="manage-btn">⚙️ Feeds</button>
      </div>
    </div>

    <!-- Category Tabs -->
    <div class="category-tabs">
      <button
        v-for="category in categories"
        :key="category.id"
        @click="activeCategory = category.id"
        :class="{ active: activeCategory === category.id }"
        class="category-tab"
      >
        {{ category.name }}
        <span class="count" v-if="getUnreadCount(category.id) > 0">
          {{ getUnreadCount(category.id) }}
        </span>
      </button>
      <button @click="activeCategory = 'favorites'" :class="{ active: activeCategory === 'favorites' }" class="category-tab">
        ⭐ Favorites
        <span class="count" v-if="favorites.length > 0">{{ favorites.length }}</span>
      </button>
    </div>

    <!-- Content Area -->
    <div class="content-area">
      <!-- Feed Manager Modal -->
      <div v-if="showFeedManager" class="modal-overlay" @click="showFeedManager = false">
        <div class="modal-content" @click.stop>
          <h4>Manage RSS Feeds</h4>

          <!-- Add Feed Form -->
          <div class="add-feed-form">
            <input
              v-model="newFeedUrl"
              type="url"
              placeholder="https://example.com/feed.xml"
              class="feed-url-input"
            />
            <input
              v-model="newFeedName"
              type="text"
              placeholder="Feed Name (optional)"
              class="feed-name-input"
            />
            <select v-model="newFeedCategory" class="category-select">
              <option value="">Select Category</option>
              <option v-for="category in categories" :key="category.id" :value="category.id">
                {{ category.name }}
              </option>
            </select>
            <button @click="addFeed" :disabled="!newFeedUrl.trim()" class="add-btn">Add Feed</button>
          </div>

          <!-- Feed List -->
          <div class="feed-list">
            <div
              v-for="feed in feeds"
              :key="feed.id"
              class="feed-item"
              :class="{ 'feed-error': feed.error }"
            >
              <div class="feed-info">
                <h5>{{ feed.name || feed.url }}</h5>
                <small>{{ feed.url }}</small>
                <div class="feed-meta">
                  <span class="category-badge">{{ getCategoryName(feed.category) }}</span>
                  <span v-if="feed.lastFetched" class="last-updated">
                    Updated {{ formatDate(feed.lastFetched) }}
                  </span>
                  <span v-if="feed.error" class="error-badge">⚠️ Error</span>
                </div>
              </div>
              <div class="feed-actions">
                <button @click="refreshFeed(feed.id)" :disabled="feed.refreshing" class="refresh-feed-btn">
                  {{ feed.refreshing ? '⟳' : '🔄' }}
                </button>
                <button @click="removeFeed(feed.id)" class="remove-feed-btn">🗑️</button>
              </div>
            </div>
          </div>

          <!-- Category Management -->
          <div class="category-management">
            <h5>Categories</h5>
            <div class="category-list">
              <div v-for="category in categories" :key="category.id" class="category-item">
                <span>{{ category.name }}</span>
                <button @click="removeCategory(category.id)" class="remove-category-btn" :disabled="category.id === 'uncategorized'">×</button>
              </div>
            </div>
            <div class="add-category">
              <input v-model="newCategoryName" type="text" placeholder="New category name" />
              <button @click="addCategory" :disabled="!newCategoryName.trim()">Add</button>
            </div>
          </div>

          <div class="modal-actions">
            <button @click="showFeedManager = false" class="close-btn">Close</button>
          </div>
        </div>
      </div>

      <!-- Articles List -->
      <div v-if="activeCategory !== 'favorites'" class="articles-list">
        <div v-if="loading" class="loading">Loading articles...</div>
        <div v-else-if="!currentArticles.length" class="empty-state">
          <p>📰 No articles found</p>
          <p>Try refreshing feeds or add new feeds</p>
        </div>
        <div v-else>
          <div
            v-for="article in currentArticles"
            :key="article.id"
            class="article-item"
            :class="{ 'unread': !article.read, 'favorite': article.favorite }"
            @click="toggleReadStatus(article.id)"
          >
            <div class="article-header">
              <h4 class="article-title">{{ article.title }}</h4>
              <div class="article-actions">
                <button @click.stop="toggleFavorite(article.id)" class="favorite-btn" :title="article.favorite ? 'Remove from favorites' : 'Add to favorites'">
                  {{ article.favorite ? '⭐' : '☆' }}
                </button>
                <button @click.stop="openArticle(article)" class="open-btn" title="Open article">🔗</button>
              </div>
            </div>
            <div class="article-meta">
              <span class="feed-name">{{ getFeedName(article.feedId) }}</span>
              <span class="article-date">{{ formatDate(article.pubDate) }}</span>
            </div>
            <p v-if="article.description" class="article-description" v-html="truncateText(article.description, 200)"></p>
            <div v-if="article.image && showImages" class="article-image">
              <img :src="article.image" :alt="article.title" @error="handleImageError" />
            </div>
          </div>
        </div>
      </div>

      <!-- Favorites View -->
      <div v-else class="favorites-list">
        <div v-if="!favorites.length" class="empty-state">
          <p>⭐ No favorite articles yet</p>
          <p>Click the star icon on articles to add them to favorites</p>
        </div>
        <div v-else>
          <div
            v-for="article in favorites"
            :key="article.id"
            class="article-item favorite"
            @click="openArticle(article)"
          >
            <div class="article-header">
              <h4 class="article-title">{{ article.title }}</h4>
              <div class="article-actions">
                <button @click.stop="toggleFavorite(article.id)" class="favorite-btn" title="Remove from favorites">⭐</button>
                <button @click.stop="openArticle(article)" class="open-btn" title="Open article">🔗</button>
              </div>
            </div>
            <div class="article-meta">
              <span class="feed-name">{{ getFeedName(article.feedId) }}</span>
              <span class="article-date">{{ formatDate(article.pubDate) }}</span>
            </div>
            <p v-if="article.description" class="article-description" v-html="truncateText(article.description, 200)"></p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { formatDistanceToNow, parseISO, format } from 'date-fns';

interface Feed {
  auth?: {
    type: 'basic' | 'bearer' | 'api-key';
    username?: string;
    password?: string;
    token?: string;
    headerName?: string;
    headerValue?: string;
  };
  id: string;
  url: string;
  name: string;
  category: string;
  lastFetched?: Date;
  error?: string;
  refreshing?: boolean;
}

interface Article {
  id: string;
  feedId: string;
  title: string;
  description?: string;
  content?: string;
  link: string;
  pubDate: string;
  author?: string;
  image?: string;
  read: boolean;
  favorite: boolean;
  guid?: string;
}

interface Category {
  id: string;
  name: string;
}

const props = defineProps<{
  panel: {
    i: string;
    config?: {
      autoRefreshInterval?: number;
      maxArticlesPerFeed?: number;
      showImages?: boolean;
      openLinksInNewTab?: boolean;
    };
  };
}>();

// Panel-specific storage key
const storageKey = computed(() => `rss-reader-${props.panel.i}`);

// Reactive data
const feeds = ref<Feed[]>([]);
const articles = ref<Article[]>([]);
const categories = ref<Category[]>([
  { id: 'uncategorized', name: 'Uncategorized' },
  { id: 'news', name: 'News' },
  { id: 'tech', name: 'Technology' },
  { id: 'sports', name: 'Sports' }
]);

const activeCategory = ref('uncategorized');
const showFeedManager = ref(false);
const loading = ref(false);
const isRefreshing = ref(false);

// Feed management
const newFeedUrl = ref('');
const newFeedName = ref('');
const newFeedCategory = ref('');

// Category management
const newCategoryName = ref('');

// Auto-refresh timer
let refreshTimer: number | null = null;

// Computed properties
const config = computed(() => ({
  autoRefreshInterval: props.panel.config?.autoRefreshInterval || 30,
  maxArticlesPerFeed: props.panel.config?.maxArticlesPerFeed || 50,
  showImages: props.panel.config?.showImages !== false,
  openLinksInNewTab: props.panel.config?.openLinksInNewTab !== false
}));

const currentArticles = computed(() => {
  if (activeCategory.value === 'all') {
    return articles.value;
  }
  return articles.value.filter(article => {
    const feed = feeds.value.find(f => f.id === article.feedId);
    return feed?.category === activeCategory.value;
  });
});

const favorites = computed(() =>
  articles.value.filter(article => article.favorite)
);

const unreadCount = computed(() => {
  const counts: Record<string, number> = {};
  categories.value.forEach(cat => {
    counts[cat.id] = articles.value.filter(article => {
      const feed = feeds.value.find(f => f.id === article.feedId);
      return feed?.category === cat.id && !article.read;
    }).length;
  });
  return counts;
});

// Browser-compatible RSS parser
function parseRSSFeed(xmlText: string, feedUrl: string): { feed: any, items: any[] } {
  const parser = new DOMParser();
  const xmlDoc = parser.parseFromString(xmlText, 'text/xml');
  // Check for parser errors
  const parserError = xmlDoc.querySelector('parsererror');
  if (parserError) {
    throw new Error('Invalid XML format');
  }
  // Get RSS channel info
  const channel = xmlDoc.querySelector('channel') || xmlDoc.querySelector('feed');
  if (!channel) {
    throw new Error('No RSS channel found');
  }
  // Extract feed metadata
  const feed = {
    title: getTextContent(channel, 'title') || 'Unknown Feed',
    description: getTextContent(channel, 'description') || getTextContent(channel, 'subtitle'),
    link: getTextContent(channel, 'link'),
    language: getTextContent(channel, 'language'),
    lastBuildDate: getTextContent(channel, 'lastBuildDate') || getTextContent(channel, 'updated')
  };
  // Extract items/articles
  const itemElements = xmlDoc.querySelectorAll('item, entry');
  const items = Array.from(itemElements).map(item => ({
    title: getTextContent(item, 'title'),
    description: getTextContent(item, 'description') || getTextContent(item, 'summary') || getTextContent(item, 'content'),
    content: getTextContent(item, 'content') || getTextContent(item, 'description'),
    link: getTextContent(item, 'link') || getAttribute(item, 'link', 'href'),
    pubDate: getTextContent(item, 'pubDate') || getTextContent(item, 'published') || getTextContent(item, 'updated'),
    author: getTextContent(item, 'author') || getTextContent(item, 'creator') || getTextContent(item, 'dc\\:creator'),
    guid: getTextContent(item, 'guid') || getAttribute(item, 'id'),
    categories: Array.from(item.querySelectorAll('category')).map(cat => cat.textContent || '')
  })).filter(item => item.title); // Only include items with titles
  return { feed, items };
}
function getTextContent(element: Element | null, selector: string): string | undefined {
  if (!element) return undefined;
  const child = element.querySelector(selector);
  return child?.textContent?.trim();
}
function getAttribute(element: Element | null, selector: string, attr: string): string | undefined {
  if (!element) return undefined;
  const child = element.querySelector(selector);
  return child?.getAttribute(attr) || undefined;
}
// Convert RSS2JSON format back to RSS XML for our parser
function jsonToRssXml(jsonData: any): string {
  if (!jsonData.items || !Array.isArray(jsonData.items)) {
    throw new Error('Invalid RSS2JSON format');
  }
  const itemsXml = jsonData.items.map((item: any) => `
    <item>
      <title><![CDATA[${item.title || ''}]]></title>
      <description><![CDATA[${item.description || ''}]]></description>
      <link>${item.link || ''}</link>
      <guid>${item.guid || item.link || ''}</guid>
      <pubDate>${item.pubDate || new Date().toISOString()}</pubDate>
      <author><![CDATA[${item.author || ''}]]></author>
    </item>`).join('');
  const rssXml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title><![CDATA[${jsonData.feed?.title || 'Unknown Feed'}]]></title>
    <description><![CDATA[${jsonData.feed?.description || ''}]]></description>
    <link>${jsonData.feed?.link || ''}</link>
    ${itemsXml}
  </channel>
</rss>`;
  return rssXml;
}
// Methods
function getUnreadCount(categoryId: string): number {
  return unreadCount.value[categoryId] || 0;
}

function getFeedName(feedId: string): string {
  const feed = feeds.value.find(f => f.id === feedId);
  return feed?.name || feed?.url || 'Unknown Feed';
}

function getCategoryName(categoryId: string): string {
  const category = categories.value.find(c => c.id === categoryId);
  return category?.name || 'Uncategorized';
}

function formatDate(dateString: string): string {
  try {
    const date = parseISO(dateString);
    return formatDistanceToNow(date, { addSuffix: true });
  } catch {
    return dateString;
  }
}

function truncateText(text: string, maxLength: number): string {
  // Remove HTML tags for truncation
  const plainText = text.replace(/<[^>]*>/g, '');
  if (plainText.length <= maxLength) return text;
  return plainText.substring(0, maxLength) + '...';
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
}

async function addFeed() {
  if (!newFeedUrl.value.trim()) return;

  const feed: Feed = {
    id: Date.now().toString(),
    url: newFeedUrl.value.trim(),
    name: newFeedName.value.trim() || '',
    category: newFeedCategory.value || 'uncategorized'
  };

  feeds.value.push(feed);
  saveData();

  // Clear form
  newFeedUrl.value = '';
  newFeedName.value = '';
  newFeedCategory.value = '';

  // Fetch the new feed
  await refreshFeed(feed.id);
}

function removeFeed(feedId: string) {
  const index = feeds.value.findIndex(f => f.id === feedId);
  if (index > -1) {
    feeds.value.splice(index, 1);
    // Remove articles from this feed
    articles.value = articles.value.filter(a => a.feedId !== feedId);
    saveData();
  }
}

async function refreshFeed(feedId: string) {
  const feed = feeds.value.find(f => f.id === feedId);
  if (!feed) return;

  feed.refreshing = true;
  feed.error = undefined;

  try {
    let xmlText: string;

    // Use Tauri backend to fetch RSS feed (bypasses CORS)
    xmlText = await invoke('fetch_rss_feed', { url: feed.url });

    // Debug: Log first 500 characters of response
    console.log('RSS Feed Response (first 500 chars):', xmlText.substring(0, 500));
    console.log('RSS Feed Response length:', xmlText.length);

    // Parse the RSS feed
    const { feed: feedInfo, items } = parseRSSFeed(xmlText, feed.url);
    // Update feed name if not set
    if (!feed.name && feedInfo.title) {
      feed.name = feedInfo.title;
    }
    // Process articles
    const newArticles: Article[] = [];
    for (const item of items.slice(0, config.value.maxArticlesPerFeed)) {
      const articleId = `${feedId}-${item.guid || item.link || item.title}`;
      // Check if article already exists
      const existingArticle = articles.value.find(a => a.id === articleId);
      if (existingArticle) continue;
      // Extract image from content or description
      let image: string | undefined;
      const content = item.content || item.description || '';
      const imgMatch = content.match(/<img[^>]+src="([^"]+)"/i);
      if (imgMatch) {
        image = imgMatch[1];
      }
      newArticles.push({
        id: articleId,
        feedId,
        title: item.title || 'Untitled',
        description: item.description,
        content: item.content,
        link: item.link || '',
        pubDate: item.pubDate || new Date().toISOString(),
        author: item.author,
        image,
        read: false,
        favorite: false,
        guid: item.guid
      });
    }
    // Add new articles
    articles.value.unshift(...newArticles);
    // Limit articles per feed
    const feedArticles = articles.value.filter(a => a.feedId === feedId);
    if (feedArticles.length > config.value.maxArticlesPerFeed) {
      const toRemove = feedArticles.slice(config.value.maxArticlesPerFeed);
      articles.value = articles.value.filter(a => !toRemove.includes(a));
    }
    feed.lastFetched = new Date();
    saveData();

  } catch (error: any) {
    feed.error = error.message || 'Failed to fetch feed';
    console.error('Feed fetch error:', error);
  } finally {
    feed.refreshing = false;
  }
}

async function refreshAllFeeds() {
  if (isRefreshing.value) return;

  isRefreshing.value = true;
  loading.value = true;

  try {
    const promises = feeds.value.map(feed => refreshFeed(feed.id));
    await Promise.all(promises);
  } finally {
    isRefreshing.value = false;
    loading.value = false;
  }
}

function toggleReadStatus(articleId: string) {
  const article = articles.value.find(a => a.id === articleId);
  if (article) {
    article.read = !article.read;
    saveData();
  }
}

function toggleFavorite(articleId: string) {
  const article = articles.value.find(a => a.id === articleId);
  if (article) {
    article.favorite = !article.favorite;
    saveData();
  }
}

function openArticle(article: Article) {
  if (config.value.openLinksInNewTab) {
    window.open(article.link, '_blank');
  } else {
    window.open(article.link, '_self');
  }
}

function addCategory() {
  if (!newCategoryName.value.trim()) return;

  const category: Category = {
    id: newCategoryName.value.toLowerCase().replace(/\s+/g, '-'),
    name: newCategoryName.value.trim()
  };

  categories.value.push(category);
  newCategoryName.value = '';
  saveData();
}

function removeCategory(categoryId: string) {
  if (categoryId === 'uncategorized') return; // Can't remove default category

  const index = categories.value.findIndex(c => c.id === categoryId);
  if (index > -1) {
    categories.value.splice(index, 1);
    // Move feeds in this category to uncategorized
    feeds.value.forEach(feed => {
      if (feed.category === categoryId) {
        feed.category = 'uncategorized';
      }
    });
    saveData();
  }
}

function saveData() {
  const data = {
    feeds: feeds.value,
    articles: articles.value,
    categories: categories.value
  };
  localStorage.setItem(storageKey.value, JSON.stringify(data));
}

function loadData() {
  try {
    const data = localStorage.getItem(storageKey.value);
    if (data) {
      const parsed = JSON.parse(data);
      feeds.value = parsed.feeds || [];
      articles.value = parsed.articles || [];
      categories.value = parsed.categories || categories.value;
    }
  } catch (error) {
    console.error('Failed to load RSS data:', error);
  }
}

// Auto-refresh setup
function setupAutoRefresh() {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }

  const interval = config.value.autoRefreshInterval * 60 * 1000; // Convert minutes to milliseconds
  refreshTimer = window.setInterval(() => {
    refreshAllFeeds();
  }, interval);
}

function cleanupAutoRefresh() {
  if (refreshTimer) {
    clearInterval(refreshTimer);
    refreshTimer = null;
  }
}

// Watch for config changes
watch(() => props.panel.config, () => {
  setupAutoRefresh();
}, { deep: true });

// Lifecycle
onMounted(() => {
  loadData();
  setupAutoRefresh();
});

onUnmounted(() => {
  cleanupAutoRefresh();
});
</script>

<style scoped>
.rss-feed-reader {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  overflow: hidden;
}

.reader-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-md) var(--space-lg);
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-panel-header);
}

.reader-header h3 {
  margin: 0;
  color: var(--text-heading);
  font-size: 1.125rem;
}

.header-actions {
  display: flex;
  gap: var(--space-sm);
}

.refresh-btn, .manage-btn {
  padding: calc(var(--space-xs) * 1.5) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.refresh-btn:hover, .manage-btn:hover {
  background: var(--bg-panel-header);
  border-color: var(--accent-primary);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.category-tabs {
  display: flex;
  gap: var(--space-xs);
  padding: var(--space-md) var(--space-lg);
  border-bottom: 1px solid var(--border-color);
  overflow-x: auto;
}

.category-tab {
  padding: calc(var(--space-xs) * 1.5) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 0.875rem;
  white-space: nowrap;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: calc(var(--space-xs));
}

.category-tab:hover {
  background: var(--bg-panel-header);
  border-color: var(--accent-primary);
}

.category-tab.active {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border-color: var(--accent-primary);
}

.count {
  background: var(--accent-danger);
  color: white;
  padding: 0 calc(var(--space-xs) * 0.75);
  border-radius: 10px;
  font-size: 0.75rem;
  font-weight: 600;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-md);
}

.articles-list, .favorites-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.article-item {
  padding: var(--space-md);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  cursor: pointer;
  transition: all 0.2s;
}

.article-item:hover {
  border-color: var(--border-color);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.article-item.unread {
  border-left: 3px solid var(--accent-primary);
  background: var(--bg-panel-header);
}

.article-item.favorite {
  border-left: 3px solid var(--accent-warning);
}

.article-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-sm);
}

.article-title {
  margin: 0;
  color: var(--text-primary);
  font-size: 1rem;
  line-height: 1.4;
  flex: 1;
}

.article-actions {
  display: flex;
  gap: calc(var(--space-xs));
  margin-left: var(--space-sm);
}

.favorite-btn, .open-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  padding: calc(var(--space-xs) * 0.5);
  border-radius: calc(var(--panel-radius) / 4);
  transition: background 0.2s;
}

.favorite-btn:hover, .open-btn:hover {
  background: var(--bg-panel-header);
}

.article-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-sm);
  font-size: 0.875rem;
  color: var(--text-muted);
}

.feed-name {
  font-weight: 500;
  color: var(--accent-primary);
}

.article-description {
  color: var(--text-secondary);
  font-size: 0.875rem;
  line-height: 1.5;
  margin-bottom: var(--space-sm);
}

.article-description :deep(p) {
  margin: 0;
}

.article-image {
  margin-top: var(--space-sm);
}

.article-image img {
  max-width: 100%;
  height: auto;
  border-radius: calc(var(--panel-radius) / 4);
}

.loading, .empty-state {
  text-align: center;
  padding: var(--space-xl);
  color: var(--text-muted);
}

.empty-state p:first-child {
  font-size: 2rem;
  margin-bottom: var(--space-sm);
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-panel);
  border-radius: var(--panel-radius);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  padding: var(--space-xl);
  border: 1px solid var(--border-color);
}

.modal-content h4 {
  margin: 0 0 var(--space-lg) 0;
  color: var(--text-heading);
}

.add-feed-form {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr auto;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
}

.feed-url-input, .feed-name-input {
  padding: calc(var(--space-sm) * 0.75) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  color: var(--text-primary);
}

.category-select {
  padding: calc(var(--space-sm) * 0.75) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  color: var(--text-primary);
}

.add-btn {
  padding: calc(var(--space-sm) * 0.75) var(--space-md);
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  font-weight: 500;
}

.add-btn:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.feed-list {
  margin-bottom: var(--space-lg);
}

.feed-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-md);
  border: 1px solid var(--border-subtle);
  border-radius: calc(var(--panel-radius) / 2);
  margin-bottom: var(--space-sm);
  background: var(--bg-panel);
}

.feed-item.feed-error {
  border-color: var(--accent-danger);
}

.feed-info {
  flex: 1;
}

.feed-info h5 {
  margin: 0 0 calc(var(--space-xs)) 0;
  color: var(--text-primary);
}

.feed-info small {
  color: var(--text-muted);
  display: block;
  margin-bottom: calc(var(--space-xs));
}

.feed-meta {
  display: flex;
  gap: var(--space-sm);
  align-items: center;
  font-size: 0.75rem;
}

.category-badge {
  background: var(--accent-primary);
  color: var(--text-on-accent);
  padding: 0 calc(var(--space-xs) * 0.75);
  border-radius: calc(var(--panel-radius) / 4);
  font-size: 0.7rem;
}

.last-updated {
  color: var(--text-muted);
}

.error-badge {
  color: var(--accent-danger);
  font-weight: 500;
}

.feed-actions {
  display: flex;
  gap: calc(var(--space-xs));
}

.refresh-feed-btn, .remove-feed-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  padding: calc(var(--space-xs));
  border-radius: calc(var(--panel-radius) / 4);
  transition: background 0.2s;
}

.refresh-feed-btn:hover, .remove-feed-btn:hover {
  background: var(--bg-panel-header);
}

.category-management {
  margin-bottom: var(--space-lg);
}

.category-management h5 {
  margin: 0 0 var(--space-md) 0;
  color: var(--text-heading);
}

.category-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
  margin-bottom: var(--space-md);
}

.category-item {
  display: flex;
  align-items: center;
  gap: calc(var(--space-xs));
  padding: calc(var(--space-xs) * 1.5) var(--space-md);
  background: var(--bg-panel-header);
  border-radius: calc(var(--panel-radius) / 2);
  font-size: 0.875rem;
}

.remove-category-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-muted);
  font-size: 1.2rem;
  line-height: 1;
  padding: 0;
}

.remove-category-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.add-category {
  display: flex;
  gap: var(--space-sm);
}

.add-category input {
  flex: 1;
  padding: calc(var(--space-sm) * 0.75) var(--space-md);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  background: var(--bg-panel);
  color: var(--text-primary);
}

.add-category button {
  padding: calc(var(--space-sm) * 0.75) var(--space-md);
  background: var(--accent-primary);
  color: var(--text-on-accent);
  border: none;
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  font-weight: 500;
}

.add-category button:disabled {
  background: var(--text-muted);
  cursor: not-allowed;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-sm);
}

.close-btn {
  padding: calc(var(--space-sm) * 0.75) var(--space-lg);
  background: var(--bg-panel-header);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: calc(var(--panel-radius) / 2);
  cursor: pointer;
  font-weight: 500;
}
</style>
