# RSS Feed Reader Plugin

A comprehensive RSS feed reader plugin for Modulaur with categories, read/unread status tracking, and favorites.

## Features

- **Per-Panel Isolation**: Each panel instance has its own feeds and settings
- **Categories**: Organize feeds into custom categories
- **Read/Unread Status**: Track which articles you've read
- **Favorites**: Mark articles as favorites for quick access
- **Auto-Refresh**: Configurable automatic feed refreshing
- **Article Images**: Optional display of article images
- **Link Behavior**: Configurable link opening (new tab or same tab)
- **CORS Proxy Fallback**: Automatically tries proxy when direct fetch fails

## Configuration Options

Each RSS Feed Reader panel can be configured with:

- **Auto Refresh Interval**: How often to refresh feeds (5-1440 minutes)
- **Max Articles per Feed**: Maximum articles to keep per feed (10-500)
- **Show Images**: Display article images when available
- **Open Links in New Tab**: Open article links in new browser tabs

## Usage

- **Add Feeds**: Enter RSS feed URLs with optional custom names and categories
2. Click the "⚙️ Feeds" button to open the feed manager
- **Authentication**: Configure authentication for intranet feeds
4. Articles will automatically load and be organized by category
5. Click articles to mark them as read/unread
6. Use the star icon to add/remove articles from favorites
## Authentication Support

For intranet RSS feeds that require authentication, the plugin supports several authentication methods:

### Basic Authentication
- **Username/Password**: Standard HTTP basic auth
- Use for feeds protected by simple username/password

### Bearer Token Authentication
- **Bearer Token**: JWT or API tokens
- Use for feeds protected by OAuth2 or API tokens

### API Key Authentication
- **Custom Header**: Any header name/value pair
- Use for feeds requiring custom API keys or headers

### Configuration
1. When adding a feed, check "Configure Authentication"
2. Select the authentication type
3. Enter the required credentials
4. The backend will include authentication headers with requests

**Symptoms:**
- Error: "Access to fetch at 'https://example.com/feed' has been blocked by CORS policy"
- Error: "Failed to load resource: net::ERR_FAILED"
- Error: "Feed fetch error: TypeError: Failed to fetch"

**Solutions:**

1. **Use CORS-enabled RSS feeds** (recommended for development):
   - `https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml` (NYT)
   - `https://feeds.npr.org/1001/rss.xml` (NPR)
   - `https://www.theguardian.com/world/rss` (Guardian)
   - `https://feeds.bbci.co.uk/news/world/rss.xml` (BBC)

2. **Use RSS-to-JSON proxy services**:
   - `https://api.rss2json.com/v1/api.json?rss_url=YOUR_FEED_URL`
   - `https://rss-to-json-serverless-api.vercel.app/api?feedURL=YOUR_FEED_URL`

3. **Run in production mode** (when built as Tauri app, CORS restrictions are lifted)

4. **Use local RSS feeds** or feeds from the same domain

**Example with proxy:**
Instead of: `https://hnrss.org/frontpage`
Use: `https://api.rss2json.com/v1/api.json?rss_url=https://hnrss.org/frontpage`

## Technical Details

- Uses **Tauri backend** for RSS feed fetching (bypasses CORS completely)
- Browser-native `DOMParser` for RSS feed parsing (no Node.js dependencies)
- Stores all data locally using panel-specific localStorage keys
- Supports RSS 2.0 and Atom feeds
- Automatic article deduplication based on GUID or URL
- Configurable article limits per feed to manage storage
- **Security**: Backend validates URLs and prevents SSRF attacks

## Dependencies

- `date-fns`: ^2.30.0 (for date formatting)
