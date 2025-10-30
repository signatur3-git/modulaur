# Export & Import Documentation

## Overview

Modulaur provides comprehensive data export and import functionality to backup, restore, and transfer data between environments (dev/prod).

## Database Export/Import

### What's Included
- **Pages**: All custom pages and their configurations
- **Records**: Staged records from adapters (snippets, time entries, etc.)
- **Dashboards**: Dashboard layouts and configurations
- **Data Sources**: Configured data source connections
- **Settings**: Application settings
- **Plugin Data**: Plugin-specific data stored in database
- **Tickets**: Kanban tickets and comments

### How to Use

#### Export Database
1. Navigate to **Database Management**
2. Click **"📥 Export to File"** in the Database Backup & Restore section
3. A JSON file will be downloaded: `database-export-YYYY-MM-DD.json`

#### Import Database
1. Navigate to **Database Management**
2. Select merge strategy:
   - **Merge**: Keep existing data, add new items
   - **Replace**: Delete all existing data first (⚠️ destructive!)
   - **Skip**: Skip items that already exist
3. Click **"📤 Import from File"**
4. Select your exported JSON file
5. Confirm the import

### Use Cases
- **Backup**: Regular backups before major changes
- **Dev to Prod**: Transfer tested configurations to production
- **Prod to Dev**: Copy production data for testing
- **Migration**: Move between different installations

## Plugin Panel Data (LocalStorage)

### What's Included
Some plugins store their content in browser LocalStorage:
- **Markdown Notepad**: Note content for each panel
- **RSS Feed Reader**: Feed URLs and cached content
- **Time Tracker**: Recent project descriptions

This data is **separate** from the database and must be exported/imported separately.

### How to Use

#### Export Panel Data
1. Navigate to **Database Management**
2. Click **"📥 Export Panel Data"** in the Plugin Panel Data section
3. A JSON file will be downloaded: `panel-data-export-YYYY-MM-DD.json`

#### Import Panel Data
1. Navigate to **Database Management**
2. Select merge strategy:
   - **Keep existing**: Don't overwrite existing panel data
   - **Overwrite all**: Replace all panel data with import
3. Click **"📤 Import Panel Data"**
4. Select your exported JSON file
5. Confirm the import

### Important Notes
- LocalStorage is browser-specific (Chrome, Edge, etc.)
- Panel data is **not** included in database exports
- Always export **both** database and panel data for complete backup

## Complete Backup Workflow

To create a complete backup of your Modulaur data:

1. **Export Database**
   - Go to Database Management
   - Click "📥 Export to File" → saves `database-export-YYYY-MM-DD.json`

2. **Export Panel Data**
   - Go to Database Management
   - Click "📥 Export Panel Data" → saves `panel-data-export-YYYY-MM-DD.json`

3. **Store Both Files Safely**
   - Keep both JSON files together
   - Label them clearly (dev/prod, date, purpose)

## Complete Restore Workflow

To restore from a complete backup:

1. **Import Database First**
   - Go to Database Management
   - Select merge strategy (usually "Replace" for clean restore)
   - Click "📤 Import from File"
   - Select `database-export-YYYY-MM-DD.json`

2. **Import Panel Data**
   - Go to Database Management
   - Select merge strategy (usually "Overwrite all" for clean restore)
   - Click "📤 Import Panel Data"
   - Select `panel-data-export-YYYY-MM-DD.json`

3. **Restart Application**
   - Close and reopen Modulaur
   - Verify all data is restored

## Transfer from Prod to Dev

To copy production data to development environment:

1. **In Production Build**:
   - Export database → `prod-database-YYYY-MM-DD.json`
   - Export panel data → `prod-panel-data-YYYY-MM-DD.json`

2. **In Development Build**:
   - Import database with "Replace" strategy
   - Import panel data with "Overwrite all" strategy
   - Restart dev environment

## File Formats

### Database Export Format
```json
{
  "version": "1.0",
  "exported_at": "2026-01-04T12:00:00Z",
  "data": {
    "records": [...],
    "pages": [...],
    "data_sources": [...],
    "settings": [...],
    "plugin_data": [...],
    "tickets": [...],
    "dashboards": [...]
  }
}
```

### Panel Data Export Format
```json
{
  "version": "1.0",
  "exported_at": "2026-01-04T12:00:00Z",
  "type": "localStorage_panel_data",
  "count": 15,
  "data": {
    "markdown-notes-panel1": "# My Notes...",
    "rss-reader-panel2": "{\"feeds\":[...]}",
    ...
  }
}
```

## Troubleshooting

### Import Shows 0 Items Imported
- **Check file format**: Ensure you're importing the correct file type (database vs panel data)
- **Check version compatibility**: File may be from a different version
- **Check merge strategy**: "Skip" strategy will skip all duplicates

### Panel Data Not Restored
- **LocalStorage limit**: Browser may have storage limit (usually 10MB)
- **Different browser**: LocalStorage is browser-specific
- **Clear cache**: Try clearing browser cache first

### Lost Data After Import
- **Replace strategy**: Using "Replace" deletes existing data first
- **Always export before import**: Create backup before importing

## Best Practices

1. **Regular Backups**: Export both database and panel data weekly
2. **Before Updates**: Always backup before updating Modulaur
3. **Test in Dev**: Test imports in dev environment first
4. **Label Files**: Use descriptive names with dates and purpose
5. **Keep Multiple Versions**: Don't overwrite old backups immediately
6. **Version Control**: Consider storing exports in version control (Git)

## API Reference

### Database Store Functions

```typescript
// Export database to JSON
await databaseStore.exportDatabase(): Promise<DatabaseExport>

// Import database from JSON
await databaseStore.importDatabase(data, mergeStrategy): Promise<ImportStats>

// Download database export as file
await databaseStore.downloadExport(): Promise<void>

// Export LocalStorage panel data
databaseStore.exportLocalStoragePanelData(): any

// Import LocalStorage panel data
databaseStore.importLocalStoragePanelData(data, strategy): number

// Download LocalStorage export as file
databaseStore.downloadLocalStorageExport(): void
```

### Tauri Commands

```rust
// Export all database data
#[tauri::command]
async fn export_database() -> Result<DatabaseExport, String>

// Import database data
#[tauri::command]
async fn import_database(
    import_data: DatabaseExport,
    merge_strategy: String
) -> Result<ImportStats, String>
```

