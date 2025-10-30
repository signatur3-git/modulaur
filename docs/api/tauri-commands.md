# Tauri Commands API Reference

**Last Updated:** 2026-01-04

This document lists the Tauri commands available to the frontend and plugins.

## Overview

Modulaur's backend exposes functionality through Tauri commands that can be invoked from the frontend using `@tauri-apps/api`.

### Usage

```typescript
import { invoke } from '@tauri-apps/api/core'

// Call a command
const result = await invoke('command_name', {
  parameter1: value1,
  parameter2: value2
})
```

## Records Management

### get_records_by_type

Get all records of a specific type.

```typescript
const records = await invoke<StagedRecord[]>('get_records_by_type', {
  recordType: 'snippet'  // e.g., 'snippet', 'time_entry', 'project'
})
```

**Parameters:**
- `recordType: string` - Type of records to retrieve

**Returns:** `StagedRecord[]`

### upsert_record

Create or update a record.

```typescript
const record = await invoke<StagedRecord>('upsert_record', {
  record: {
    record_type: 'snippet',
    source: 'my-plugin',
    data: {
      title: 'My Snippet',
      content: 'Hello World'
    },
    metadata: {
      tags: ['example'],
      title: 'My Snippet'
    }
  }
})
```

**Parameters:**
- `record: StagedRecord` - Record to create/update

**Returns:** `StagedRecord`

### update_record

Update an existing record by ID.

```typescript
const updated = await invoke<StagedRecord>('update_record', {
  id: 'records:abc123',
  record: {
    record_type: 'snippet',
    source: 'my-plugin',
    data: {
      title: 'Updated Title'
    }
  }
})
```

**Parameters:**
- `id: string` - Record ID (e.g., "records:abc123")
- `record: StagedRecord` - Updated record data

**Returns:** `StagedRecord`

### delete_record

Delete a record by ID.

```typescript
await invoke('delete_record', {
  id: 'records:abc123'
})
```

**Parameters:**
- `id: string` - Record ID to delete

**Returns:** `void`

### get_record_count

Get total count of records.

```typescript
const count = await invoke<number>('get_record_count')
```

**Returns:** `number`

### get_staged_records

Get paginated records.

```typescript
const records = await invoke<StagedRecord[]>('get_staged_records', {
  limit: 50,
  offset: 0
})
```

**Parameters:**
- `limit: number` - Max records to return
- `offset: number` - Records to skip

**Returns:** `StagedRecord[]`

## Pages Management

### get_pages

Get all pages.

```typescript
import { Page } from '@/types'

const pages = await invoke<Page[]>('get_pages')
```

**Returns:** `Page[]`

### create_page

Create a new page.

```typescript
const page = await invoke<Page>('create_page', {
  page: {
    name: 'My Page',
    route: 'my-page',
    type: 'dashboard',
    icon: '📄',
    visible: true,
    order: 10,
    config: {}
  }
})
```

**Parameters:**
- `page: Page` - Page to create

**Returns:** `Page`

### update_page

Update an existing page.

```typescript
const updated = await invoke<Page>('update_page', {
  id: 'pages:xyz789',
  page: {
    name: 'Updated Page Name'
  }
})
```

**Parameters:**
- `id: string` - Page ID
- `page: Partial<Page>` - Fields to update

**Returns:** `Page`

### delete_page

Delete a page.

```typescript
await invoke('delete_page', {
  id: 'pages:xyz789'
})
```

**Parameters:**
- `id: string` - Page ID to delete

**Returns:** `void`

### reorder_pages

Update the order of pages.

```typescript
await invoke('reorder_pages', {
  pageIds: ['pages:abc', 'pages:def', 'pages:ghi']
})
```

**Parameters:**
- `pageIds: string[]` - Array of page IDs in desired order

**Returns:** `void`

## Dashboards Management

### get_dashboards

Get all dashboards (legacy file-based system).

```typescript
const dashboards = await invoke<Dashboard[]>('get_dashboards')
```

**Returns:** `Dashboard[]`

### get_dashboard

Get a specific dashboard by ID.

```typescript
const dashboard = await invoke<Dashboard>('get_dashboard', {
  id: 'my-dashboard'
})
```

**Parameters:**
- `id: string` - Dashboard ID

**Returns:** `Dashboard`

### save_dashboard

Create or update a dashboard.

```typescript
await invoke('save_dashboard', {
  dashboard: {
    id: 'my-dashboard',
    name: 'My Dashboard',
    layout: { /* ... */ }
  }
})
```

**Parameters:**
- `dashboard: Dashboard` - Dashboard to save

**Returns:** `void`

### delete_dashboard

Delete a dashboard.

```typescript
await invoke('delete_dashboard', {
  id: 'my-dashboard'
})
```

**Parameters:**
- `id: string` - Dashboard ID to delete

**Returns:** `void`

## Database Management

### get_database_stats

Get database statistics.

```typescript
const stats = await invoke<DatabaseStats>('get_database_stats')
```

**Returns:**
```typescript
{
  total_records: number
  size_bytes: number
  by_type: Record<string, number>
  by_source: Record<string, number>
}
```

### export_database

Export all database data to JSON.

```typescript
const exportData = await invoke<DatabaseExport>('export_database')
```

**Returns:** `DatabaseExport`

### import_database

Import database data from JSON export.

```typescript
const stats = await invoke<ImportStats>('import_database', {
  importData: exportData,
  mergeStrategy: 'merge'  // 'merge' | 'replace' | 'skip'
})
```

**Parameters:**
- `importData: DatabaseExport` - Data to import
- `mergeStrategy: string` - How to handle existing data

**Returns:** `ImportStats`

### clear_all_records

Delete all records from the database.

```typescript
const count = await invoke<number>('clear_all_records')
```

**Returns:** `number` - Number of records deleted

### cleanup_old_records

Delete records older than specified days.

```typescript
const result = await invoke<{ deleted: number }>('cleanup_old_records', {
  ttlDays: 30,
  source: 'optional-source-filter'  // optional
})
```

**Parameters:**
- `ttlDays: number` - Delete records older than this many days
- `source?: string` - Optional source filter

**Returns:** `{ deleted: number }`

## Tickets Management

### get_tickets

Get all tickets.

```typescript
const tickets = await invoke<Ticket[]>('get_tickets')
```

**Returns:** `Ticket[]`

### create_ticket

Create a new ticket.

```typescript
const ticket = await invoke<Ticket>('create_ticket', {
  ticket: {
    title: 'New Task',
    description: 'Task description',
    status: 'todo',
    priority: 'medium',
    tags: ['feature']
  }
})
```

**Parameters:**
- `ticket: CreateTicketRequest` - Ticket to create

**Returns:** `Ticket`

### update_ticket

Update an existing ticket.

```typescript
const updated = await invoke<Ticket>('update_ticket', {
  id: 'tickets:abc123',
  updates: {
    status: 'in_progress'
  }
})
```

**Parameters:**
- `id: string` - Ticket ID
- `updates: UpdateTicketRequest` - Fields to update

**Returns:** `Ticket`

### delete_ticket

Delete a ticket.

```typescript
await invoke('delete_ticket', {
  id: 'tickets:abc123'
})
```

**Parameters:**
- `id: string` - Ticket ID to delete

**Returns:** `void`

### move_ticket

Move a ticket to a different status column.

```typescript
const updated = await invoke<Ticket>('move_ticket', {
  id: 'tickets:abc123',
  newStatus: 'done'
})
```

**Parameters:**
- `id: string` - Ticket ID
- `newStatus: string` - New status (e.g., 'todo', 'in_progress', 'done')

**Returns:** `Ticket`

### add_comment

Add a comment to a ticket.

```typescript
const comment = await invoke<Comment>('add_comment', {
  ticketId: 'tickets:abc123',
  text: 'This is a comment'
})
```

**Parameters:**
- `ticketId: string` - Ticket ID
- `text: string` - Comment text

**Returns:** `Comment`

## Plugin Management

### get_installed_plugins

Get list of installed plugins.

```typescript
const plugins = await invoke<PluginManifest[]>('get_installed_plugins')
```

**Returns:** `PluginManifest[]`

### reload_plugins

Reload all plugins.

```typescript
const count = await invoke<number>('reload_plugins')
```

**Returns:** `number` - Number of plugins loaded

### get_plugin_info

Get detailed information about a specific plugin.

```typescript
const info = await invoke<PluginManifest>('get_plugin_info', {
  pluginId: 'markdown-notepad'
})
```

**Parameters:**
- `pluginId: string` - Plugin ID

**Returns:** `PluginManifest`

## Security & Credentials

### store_secure_credential

Store a credential securely in the system keychain.

```typescript
await invoke('store_secure_credential', {
  service: 'my-service',
  key: 'api-key',
  value: 'secret-value'
})
```

**Parameters:**
- `service: string` - Service name
- `key: string` - Credential key
- `value: string` - Credential value

**Returns:** `void`

### get_secure_credential

Retrieve a stored credential.

```typescript
const value = await invoke<string>('get_secure_credential', {
  service: 'my-service',
  key: 'api-key'
})
```

**Parameters:**
- `service: string` - Service name
- `key: string` - Credential key

**Returns:** `string | null`

### remove_secure_credential

Delete a stored credential.

```typescript
await invoke('remove_secure_credential', {
  service: 'my-service',
  key: 'api-key'
})
```

**Parameters:**
- `service: string` - Service name
- `key: string` - Credential key

**Returns:** `void`

## Types Reference

### StagedRecord

```typescript
interface StagedRecord {
  id?: string
  record_type: string
  source: string
  timestamp?: string
  data: Record<string, any>
  metadata: {
    tags: string[]
    status?: string
    title?: string
    description?: string
  }
}
```

### Page

```typescript
interface Page {
  id?: string
  name: string
  route: string
  type: 'dashboard' | 'plugin' | 'custom'
  icon?: string
  visible: boolean
  order: number
  config: Record<string, any>
  created_at?: string
  updated_at?: string
}
```

### Ticket

```typescript
interface Ticket {
  id?: string
  title: string
  description: string
  status: string
  priority: 'low' | 'medium' | 'high'
  tags: string[]
  created_at?: string
  updated_at?: string
}
```

## Error Handling

All commands return errors as rejected promises:

```typescript
try {
  const result = await invoke('some_command', { params })
  // Handle success
} catch (error) {
  console.error('Command failed:', error)
  // Handle error
}
```

## Best Practices

1. **Always handle errors** - Use try/catch blocks
2. **Type your responses** - Use TypeScript generics with invoke
3. **Validate input** - Check parameters before invoking
4. **Use meaningful sources** - Prefix records with your plugin ID
5. **Clean up data** - Delete records when no longer needed

## Future Commands

Commands planned for future releases:
- Data source management
- Settings management
- Plugin configuration
- Backup/restore operations
- Analytics and reporting

## Getting Help

- Check [Plugin Development Guide](../PLUGIN-DEVELOPMENT.md)
- Review example plugins in `plugins/featured/`
- Open issues on GitHub for bugs or questions

---

For the most up-to-date command list, check the source code in `src-tauri/src/main.rs`.

