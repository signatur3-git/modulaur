# System Architecture Overview

**Last Updated:** 2025-01-18

## High-Level Architecture

Modulaur follows a **desktop-first, plugin-extensible architecture** built on modern web and native technologies.

```
┌─────────────────────────────────────────────────────┐
│                Desktop Application                   │
│              (Tauri Native Shell)                    │
├─────────────────────────────────────────────────────┤
│                                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │         Vue 3 Frontend (TypeScript)          │  │
│  │  ┌────────────┐  ┌───────────────────────┐  │  │
│  │  │  Components │  │   Pinia State Stores   │  │  │
│  │  │  & Views    │  │   - pageStore          │  │  │
│  │  │             │  │   - settingsStore      │  │  │
│  │  │  - Dashboard│  │   - databaseStore      │  │  │
│  │  │  - Layout   │  │   - themeStore         │  │  │
│  │  │  - Settings │  │                        │  │  │
│  │  └────────────┘  └───────────────────────┘  │  │
│  │                                              │  │
│  │  ┌────────────────────────────────────────┐ │  │
│  │  │        Vue Router (Hash Mode)          │ │  │
│  │  │  /page/:id  |  /dashboard/:id          │ │  │
│  │  └────────────────────────────────────────┘ │  │
│  │                                              │  │
│  │  ┌────────────────────────────────────────┐ │  │
│  │  │       Plugin Loader (UMD/ESM)          │ │  │
│  │  │  Dynamically loads Vue components      │ │  │
│  │  └────────────────────────────────────────┘ │  │
│  └──────────────────────────────────────────────┘  │
│                         │                           │
│                    Tauri Bridge                     │
│                    (IPC/invoke)                     │
│                         │                           │
│  ┌──────────────────────────────────────────────┐  │
│  │         Rust Backend (Async)                 │  │
│  │  ┌────────────┐  ┌────────────────────────┐ │  │
│  │  │   Tauri    │  │   Database Layer       │ │  │
│  │  │  Commands  │  │   (SurrealDB)          │ │  │
│  │  │            │  │                        │ │  │
│  │  │ - pages    │  │  Tables:               │ │  │
│  │  │ - records  │  │  - pages               │ │  │
│  │  │ - tickets  │  │  - records             │ │  │
│  │  │ - settings │  │  - tickets             │ │  │
│  │  └────────────┘  │  - data_sources        │ │  │
│  │                  │  - settings             │ │  │
│  │  ┌────────────┐  │  - plugin_data         │ │  │
│  │  │  Plugin    │  └────────────────────────┘ │  │
│  │  │  System    │  ┌────────────────────────┐ │  │
│  │  │  (WASM)    │  │   Stage Manager        │ │  │
│  │  │            │  │   Dev / Prod           │ │  │
│  │  └────────────┘  └────────────────────────┘ │  │
│  └──────────────────────────────────────────────┘  │
│                                                      │
└─────────────────────────────────────────────────────┘
                         │
           ┌─────────────┴─────────────┐
           │                           │
    ┌──────▼──────┐           ┌────────▼────────┐
    │ File System │           │  External APIs  │
    │  (Plugins,  │           │  (HTTP, OAuth)  │
    │   Database) │           └─────────────────┘
    └─────────────┘
```

## Core Components

### 1. Frontend Layer (Vue 3)

**Technology:** Vue 3 + TypeScript + Vite

**Responsibilities:**

- User interface rendering
- User input handling
- State management (Pinia)
- Client-side routing
- Plugin component loading
- Theme management

**Key Files:**

- `sandbox-app-gui-spa/src/main.ts` - Application entry point
- `sandbox-app-gui-spa/src/App.vue` - Root component
- `sandbox-app-gui-spa/src/router/index.ts` - Route configuration
- `sandbox-app-gui-spa/src/stores/*` - State management

### 2. Backend Layer (Rust)

**Technology:** Rust + Tauri + async/tokio

**Responsibilities:**

- Database operations
- File system access
- Native OS integration
- Plugin WASM execution
- Background tasks
- Security boundaries

**Key Files:**

- `src-tauri/src/main.rs` - Application lifecycle
- `src-tauri/src/db.rs` - Database layer
- `src-tauri/src/plugin_system.rs` - Plugin management
- `src-tauri/src/stage.rs` - Dev/Prod separation

### 3. Database Layer (SurrealDB)

**Technology:** SurrealDB (embedded mode)

**Responsibilities:**

- Data persistence
- Query execution
- Schema management
- Transaction handling
- Stage-based isolation

**Tables:**

- `pages` - Page metadata and configuration
- `records` - Generic data records (polymorphic)
- `tickets` - Kanban ticket data
- `data_sources` - External data source configs
- `settings` - Application settings
- `plugin_data` - Plugin-specific data

### 4. Plugin System

**Technology:** WASM (backend) + UMD/ESM (frontend)

**Responsibilities:**

- Dynamic code loading
- Isolated execution
- API surface for extensions
- Manifest parsing

**Extension Points:**

- Panel components (Vue)
- Page types (Vue)
- Data adapters (WASM)
- Background workers (WASM)

## Data Flow

### Typical User Interaction

```
User Action (Click/Type)
    ↓
Vue Component
    ↓
Pinia Store (State Update)
    ↓
Tauri invoke() call
    ↓
Rust Command Handler
    ↓
Database Operation (SurrealDB)
    ↓
Return Result
    ↓
Update Pinia Store
    ↓
Vue Reactivity Updates UI
```

### Plugin Loading Flow

```
App Startup
    ↓
Backend: Scan plugins/ folder
    ↓
Backend: Parse manifest.json files
    ↓
Backend: Register plugin metadata
    ↓
Frontend: Request plugin list
    ↓
Frontend: Load UMD scripts dynamically
    ↓
Frontend: Register Vue components
    ↓
Ready for Use
```

## Technology Stack

### Frontend

- **Vue 3** - UI framework (Composition API)
- **TypeScript** - Type safety
- **Pinia** - State management
- **Vue Router** - Client-side routing
- **Vite** - Build tool and dev server
- **Grid Layout Plus** - Dashboard layouts
- **Vue Draggable Plus** - Drag-and-drop

### Backend

- **Rust** - Systems programming language
- **Tauri 2.x** - Desktop application framework
- **SurrealDB** - Embedded database
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **wasmtime** - WASM runtime (for plugins)

### Development Tools

- **npm** - JavaScript package manager
- **cargo** - Rust package manager
- **git** - Version control
- **PowerShell** - Build scripts

## Design Principles

### 1. **Offline First**

The application works completely offline. Network access is optional for features like RSS feeds or API integrations.

### 2. **Plugin Everything**

Core functionality is minimal. Most features come from plugins that can be added, removed, or replaced.

### 3. **Database First**

All application state goes through the database, not LocalStorage or in-memory caches (except for UI state).

### 4. **Type Safety**

TypeScript on frontend, Rust on backend. Strong typing catches errors early.

### 5. **Reactive by Default**

Vue's reactivity system keeps UI in sync with state automatically.

### 6. **Stage Separation**

Development and Production environments are completely isolated to prevent accidental data mixing.

## Security Considerations

### Isolation

- Plugins run in isolated contexts
- WASM provides sandboxing for backend plugins
- Tauri's permission system controls native API access

### Data Protection

- Database is local-only (no cloud by default)
- Sensitive credentials encrypted at rest
- No telemetry or tracking

### Input Validation

- All user inputs validated on backend
- SQL injection prevented by SurrealDB's query model
- XSS prevented by Vue's template escaping

## Performance Characteristics

### Startup Performance

- Cold start: ~2-3 seconds
- Includes database initialization and plugin loading

### Runtime Performance

- UI updates: <16ms (60 FPS)
- Database queries: <10ms for typical operations
- Plugin loading: <500ms per plugin

### Resource Usage

- Memory: 80-150 MB typical
- Disk: ~20 MB binary + database
- CPU: Minimal when idle

## Deployment Model

### Development

- Frontend: Vite dev server (hot reload)
- Backend: Cargo watch (manual restart)
- Database: Embedded, dev stage

### Production

- Single binary executable
- Embedded web assets
- Embedded database
- No external dependencies
