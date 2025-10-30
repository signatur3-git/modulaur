#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adapters;
mod credentials;
mod dashboard;
mod db;
mod error;
mod models;
mod plugins; // M6: Plugin system
mod prompt_gen;
mod tickets; // Ticket/Kanban system
mod window; // Prompt Generator System
            // Phase 2: New services
mod data_sources;
mod pages;
mod plugin_data;
mod settings;

#[cfg(feature = "sidecar-db")]
mod sidecar;

use adapters::{AdapterConfig, AdapterRegistry};
use credentials::{
    get_machine_password, get_secure_credential, remove_secure_credential, store_secure_credential,
};
use dashboard::DashboardService;
use db::Database;
use models::Dashboard;
use plugins::PluginManager; // M6: Plugin manager
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(feature = "sidecar-db")]
use sidecar::SurrealDbSidecar;

// Global application state
#[cfg(feature = "embedded-db")]
pub struct AppState {
    pub dashboard_service: Arc<Mutex<DashboardService>>,
    pub plugin_manager: Arc<Mutex<PluginManager>>, // M6: Plugin manager
    pub adapter_registry: Arc<AdapterRegistry>,
    pub database: Arc<Mutex<Database>>,
    // Phase 2: New services (not using page_service - using direct DB access)
    pub data_source_service: Arc<Mutex<data_sources::DataSourceService>>,
    pub settings_service: Arc<Mutex<settings::SettingsService>>,
    pub plugin_data_service: Arc<Mutex<plugin_data::PluginDataService>>,
}

#[cfg(feature = "sidecar-db")]
struct AppState {
    dashboard_service: Arc<Mutex<DashboardService>>,
    plugin_manager: Arc<Mutex<PluginManager>>, // M6: Plugin manager
    adapter_registry: Arc<AdapterRegistry>,
    database: Arc<Mutex<Database>>,
    _sidecar: Arc<Mutex<SurrealDbSidecar>>, // Keep sidecar alive
    // Phase 2: New services
    page_service: Arc<Mutex<pages::PageService>>,
    data_source_service: Arc<Mutex<data_sources::DataSourceService>>,
    settings_service: Arc<Mutex<settings::SettingsService>>,
    plugin_data_service: Arc<Mutex<plugin_data::PluginDataService>>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Initialize dashboard service
    let dashboard_service =
        DashboardService::new().expect("Failed to initialize dashboard service");

    // Get data directory
    let data_dir = dirs::data_local_dir()
        .expect("Failed to get local data directory")
        .join("modulaur")
        .join("data");

    #[cfg(feature = "sidecar-db")]
    let sidecar = {
        // Start SurrealDB sidecar
        tracing::info!("Starting SurrealDB sidecar...");
        let sidecar =
            SurrealDbSidecar::start(data_dir.clone()).expect("Failed to start SurrealDB sidecar");

        // Wait for SurrealDB to be ready
        sidecar
            .wait_for_ready(30)
            .await
            .expect("SurrealDB sidecar failed to start");

        sidecar
    };

    #[cfg(feature = "embedded-db")]
    tracing::info!("Using embedded SurrealDB with SurrealKV");

    // Initialize database connection
    let database = Database::new(data_dir)
        .await
        .expect("Failed to connect to database");

    // M6: Initialize plugin manager
    // In dev mode, use project plugins directory
    // In production, use AppData
    let plugin_dir = if cfg!(debug_assertions) {
        // Development: use plugins directory in src-tauri folder
        // CARGO_MANIFEST_DIR is the directory containing Cargo.toml (src-tauri/)
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("plugins")
    } else {
        // Production: use AppData
        dirs::data_local_dir()
            .expect("Failed to get local data directory")
            .join("modulaur")
            .join("plugins")
    };

    tracing::info!("Plugin directory: {:?}", plugin_dir);

    // M6 DEBUG: VERY OBVIOUS MESSAGE - Use eprintln! for stderr (unbuffered)
    eprintln!("\n\n");
    eprintln!("═══════════════════════════════════════════");
    eprintln!("🚀 BACKEND STARTED WITH DEBUG CODE! 🚀");
    eprintln!("═══════════════════════════════════════════");
    eprintln!("\n");

    // M6 DEBUG: Print to stderr so we can see it (eprintln! is unbuffered)
    eprintln!("============================================");
    eprintln!("PLUGIN DISCOVERY DEBUG");
    eprintln!("============================================");
    eprintln!("Plugin directory: {:?}", plugin_dir);
    eprintln!("Directory exists: {}", plugin_dir.exists());
    if plugin_dir.exists() {
        eprintln!("Contents:");
        if let Ok(entries) = std::fs::read_dir(&plugin_dir) {
            for entry in entries.flatten() {
                eprintln!("  - {:?}", entry.file_name());
                let manifest_path = entry.path().join("manifest.json");
                eprintln!("    manifest.json exists: {}", manifest_path.exists());
            }
        }
    }
    eprintln!("============================================");

    let mut plugin_manager = PluginManager::new(plugin_dir);

    // Load plugins
    match plugin_manager.load_plugins().await {
        Ok(count) => tracing::info!("Loaded {} plugins", count),
        Err(e) => tracing::warn!("Failed to load plugins: {}", e),
    }

    // Initialize adapter registry
    let adapter_registry = AdapterRegistry::new();
    tracing::info!("Registered adapters: {:?}", adapter_registry.list_types());

    tracing::info!("Application initialized successfully");

    // Phase 2: Initialize new services
    // Services will share the database reference through Arc<Mutex<Database>>
    // Note: Pages use direct DB access via Tauri commands (no service layer)
    let data_source_service =
        data_sources::DataSourceService::new(Arc::new(Mutex::new(database.clone())));
    let settings_service = settings::SettingsService::new(Arc::new(Mutex::new(database.clone())));
    let plugin_data_service =
        plugin_data::PluginDataService::new(Arc::new(Mutex::new(database.clone())));

    #[cfg(feature = "embedded-db")]
    let app_state = AppState {
        dashboard_service: Arc::new(Mutex::new(dashboard_service)),
        database: Arc::new(Mutex::new(database)),
        adapter_registry: Arc::new(adapter_registry),
        plugin_manager: Arc::new(Mutex::new(plugin_manager)),
        data_source_service: Arc::new(Mutex::new(data_source_service)),
        settings_service: Arc::new(Mutex::new(settings_service)),
        plugin_data_service: Arc::new(Mutex::new(plugin_data_service)),
    };

    #[cfg(feature = "sidecar-db")]
    let app_state = AppState {
        dashboard_service: Arc::new(Mutex::new(dashboard_service)),
        database: Arc::new(Mutex::new(database)),
        adapter_registry: Arc::new(adapter_registry),
        _sidecar: Arc::new(Mutex::new(sidecar)),
        plugin_manager: Arc::new(Mutex::new(plugin_manager)),
        page_service: Arc::new(Mutex::new(page_service)),
        data_source_service: Arc::new(Mutex::new(data_source_service)),
        settings_service: Arc::new(Mutex::new(settings_service)),
        plugin_data_service: Arc::new(Mutex::new(plugin_data_service)),
    };

    #[cfg(feature = "sidecar-db")]
    let sidecar_for_cleanup = app_state._sidecar.clone();

    let app = tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            check_app_size,
            get_config,
            get_dashboards,
            get_dashboard,
            save_dashboard,
            delete_dashboard,
            // M6: Plugin system
            get_installed_plugins,
            reload_plugins,
            get_plugin_info,
            unload_plugin,
            test_plugin_fetch,
            // M3: Data staging commands
            get_staged_records,
            get_records_by_type,
            get_record_count,
            upsert_record,
            update_record,
            delete_record,
            // M3: Adapter commands
            list_adapters,
            get_adapter_default_config,
            test_adapter_connection,
            fetch_adapter_data,
            // M5: Database management
            clear_all_records,
            get_database_stats,
            cleanup_old_records,
            delete_records_by_type,
            delete_records_by_source_and_type,
            // Database export/import
            export_database,
            import_database,
            // M5 Phase 5: Secure credential storage
            store_secure_credential,
            get_secure_credential,
            remove_secure_credential,
            get_machine_password,
            // Ticket/Kanban system
            create_ticket,
            update_ticket,
            delete_ticket,
            get_tickets,
            move_ticket,
            add_comment,
            // RSS Feed Reader
            fetch_rss_feed,
            // Phase 2 M10: Page management
            pages::get_pages,
            pages::create_page,
            pages::update_page,
            pages::delete_page,
            pages::reorder_pages,
            pages::clear_pages_table,
            // ============================================
            // PLUGIN: prompt-generator
            // Backend commands for plugins/featured/prompt-generator/
            // ============================================
            prompt_gen::commands::get_prompt_packages,
            prompt_gen::commands::get_prompt_package,
            prompt_gen::commands::create_prompt_package,
            prompt_gen::commands::update_prompt_package,
            prompt_gen::commands::delete_prompt_package,
            prompt_gen::commands::get_prompt_templates,
            prompt_gen::commands::create_prompt_template,
            prompt_gen::commands::update_prompt_template,
            prompt_gen::commands::delete_prompt_template,
            prompt_gen::commands::get_prompt_sections,
            prompt_gen::commands::create_prompt_section,
            prompt_gen::commands::update_prompt_section,
            prompt_gen::commands::delete_prompt_section,
            prompt_gen::commands::get_separator_sets,
            prompt_gen::commands::create_separator_set,
            prompt_gen::commands::get_prompt_data_types,
            prompt_gen::commands::create_prompt_data_type,
            prompt_gen::commands::get_prompt_tags,
            prompt_gen::commands::create_prompt_tag,
            prompt_gen::commands::export_prompt_package,
            prompt_gen::commands::import_prompt_package,
            prompt_gen::commands::seed_example_packages,
            prompt_gen::commands::seed_text2image_common_package,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // Register cleanup handler before running
    #[cfg(feature = "sidecar-db")]
    app.run(move |_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            tracing::info!("Application exiting, cleaning up SurrealDB sidecar...");
            if let Ok(mut sidecar) = sidecar_for_cleanup.try_lock() {
                sidecar.stop();
            } else {
                tracing::warn!("Could not acquire lock on sidecar for cleanup");
            }
        }
    });

    #[cfg(feature = "embedded-db")]
    app.run(|_app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            tracing::info!("Application exiting (embedded mode)...");
        }
    });
}

// ============================================================================
// M6: Plugin System Commands
// ============================================================================

#[tauri::command]
async fn get_installed_plugins(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<plugins::PluginMetadata>, String> {
    let plugin_manager = state.plugin_manager.lock().await;
    Ok(plugin_manager.get_all_plugins())
}

#[tauri::command]
async fn reload_plugins(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    let mut plugin_manager = state.plugin_manager.lock().await;

    // Shutdown existing plugins
    plugin_manager
        .shutdown_all()
        .await
        .map_err(|e| e.to_string())?;

    // Reload plugins
    plugin_manager
        .load_plugins()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_plugin_info(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<Option<plugins::PluginMetadata>, String> {
    let plugin_manager = state.plugin_manager.lock().await;

    Ok(plugin_manager.get_plugin(&name).map(|p| p.metadata()))
}

#[tauri::command]
async fn unload_plugin(name: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut plugin_manager = state.plugin_manager.lock().await;

    plugin_manager
        .unload_plugin(&name)
        .await
        .map_err(|e| e.to_string())
}

/// M6: Test plugin fetch functionality
#[tauri::command]
async fn test_plugin_fetch(
    plugin_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    tracing::info!("Testing plugin fetch for: {}", plugin_name);

    let plugin_manager = state.plugin_manager.lock().await;

    // Create a test adapter config
    let config = adapters::AdapterConfig {
        adapter_type: "example".to_string(),
        source: format!("{}-test-source", plugin_name),
        endpoint: "https://example.com".to_string(),
        auth: None,
        parameters: serde_json::json!({}),
        polling_interval: None,
        enabled: true,
    };

    // Get the plugin and call fetch
    let plugin = plugin_manager
        .get_plugin(&plugin_name)
        .ok_or_else(|| format!("Plugin '{}' not found", plugin_name))?;

    let records = plugin.fetch(&config).await.map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "plugin": plugin_name,
        "record_count": records.len(),
        "records": records,
    }))
}

fn init_logging() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_target(true)
        .with_thread_ids(true)
        .init();
    tracing::info!("Logging initialized");
}

#[tauri::command]
async fn check_app_size() -> Result<AppSize, String> {
    // Stub: Will check actual binary size in production
    let size_mb = 15.5;
    let is_acceptable = size_mb < 20.0;

    tracing::info!(
        "App size check: {:.1}MB (acceptable: {})",
        size_mb,
        is_acceptable
    );

    Ok(AppSize {
        size_mb,
        is_acceptable,
        threshold_mb: 20.0,
    })
}

#[tauri::command]
async fn get_config() -> Result<serde_json::Value, String> {
    // Stub: Will load config from file in M2+
    Ok(serde_json::json!({
        "app": {
            "name": "Modulaur",
            "version": "0.2.0"
        },
        "database": {
            "type": "surrealdb",
            "mode": "embedded"
        }
    }))
}

#[tauri::command]
async fn get_dashboards(state: tauri::State<'_, AppState>) -> Result<Vec<Dashboard>, String> {
    let service = state.dashboard_service.lock().await;

    service.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_dashboard(id: String, state: tauri::State<'_, AppState>) -> Result<Dashboard, String> {
    let service = state.dashboard_service.lock().await;

    service.get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_dashboard(
    dashboard: Dashboard,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let service = state.dashboard_service.lock().await;

    service.save(&dashboard).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_dashboard(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let service = state.dashboard_service.lock().await;

    service.delete(&id).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct AppSize {
    size_mb: f32,
    is_acceptable: bool,
    threshold_mb: f32,
}

// ============================================================================
// M3: Data Staging Commands
// ============================================================================

#[tauri::command]
async fn get_staged_records(
    limit: Option<usize>,
    offset: Option<usize>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<db::StagedRecord>, String> {
    let db = state.database.lock().await;

    db.get_all_records(limit.unwrap_or(100), offset.unwrap_or(0))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_records_by_type(
    record_type: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<db::StagedRecord>, String> {
    let db = state.database.lock().await;

    db.get_records_by_type(&record_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_record_count(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    let db = state.database.lock().await;

    db.count_records().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn upsert_record(
    record: db::StagedRecord,
    state: tauri::State<'_, AppState>,
) -> Result<db::StagedRecord, String> {
    let db = state.database.lock().await;

    db.upsert_record(record).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_record(
    id: String,
    record: db::StagedRecord,
    state: tauri::State<'_, AppState>,
) -> Result<db::StagedRecord, String> {
    let db = state.database.lock().await;

    db.update_record(&id, record)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_record(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    tracing::info!("🗑️  delete_record called with ID: {}", id);

    let db = state.database.lock().await;

    match db.delete_record(&id).await {
        Ok(_) => {
            tracing::info!("🗑️  Successfully deleted record: {}", id);
            Ok(())
        }
        Err(e) => {
            tracing::error!("🗑️  Failed to delete record {}: {}", id, e);
            Err(e.to_string())
        }
    }
}

// ============================================================================
// M3: Adapter Management Commands
// ============================================================================

/// List all available adapter types
#[tauri::command]
async fn list_adapters(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    Ok(state.adapter_registry.list_types())
}

/// Get default configuration for an adapter type
#[tauri::command]
async fn get_adapter_default_config(
    adapter_type: String,
    state: tauri::State<'_, AppState>,
) -> Result<AdapterConfig, String> {
    let adapter = state
        .adapter_registry
        .get(&adapter_type)
        .ok_or_else(|| format!("Unknown adapter type: {}", adapter_type))?;

    Ok(adapter.default_config())
}

/// Test connection for an adapter configuration
#[tauri::command]
async fn test_adapter_connection(
    config: AdapterConfig,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    // Phase 3.3: Check if plugin exists first
    let has_plugin = {
        let plugin_manager = state.plugin_manager.lock().await;
        plugin_manager
            .get_plugin_by_adapter_type(&config.adapter_type)
            .is_some()
    };

    if has_plugin {
        eprintln!("✅ Testing connection with PLUGIN: {}", config.adapter_type);
        let plugin_manager = state.plugin_manager.lock().await;
        let plugin = plugin_manager
            .get_plugin_by_adapter_type(&config.adapter_type)
            .expect("Plugin should exist");

        plugin
            .test_connection(&config)
            .await
            .map_err(|e| e.to_string())
    } else {
        eprintln!(
            "📦 Testing connection with BUILT-IN: {}",
            config.adapter_type
        );

        state
            .adapter_registry
            .test_connection(&config)
            .await
            .map_err(|e| e.to_string())
    }
}

/// Fetch data using an adapter and store in database
#[tauri::command]
async fn fetch_adapter_data(
    config: AdapterConfig,
    state: tauri::State<'_, AppState>,
) -> Result<usize, String> {
    tracing::info!("Fetching data with adapter: {}", config.adapter_type);

    // Phase 3.3: Check if plugin exists first
    let has_plugin = {
        let plugin_manager = state.plugin_manager.lock().await;
        plugin_manager
            .get_plugin_by_adapter_type(&config.adapter_type)
            .is_some()
    };

    let records = if has_plugin {
        eprintln!("✅ Using PLUGIN for adapter: {}", config.adapter_type);
        tracing::info!("Using plugin for adapter: {}", config.adapter_type);

        let plugin_manager = state.plugin_manager.lock().await;
        let plugin = plugin_manager
            .get_plugin_by_adapter_type(&config.adapter_type)
            .expect("Plugin should exist");

        eprintln!("🔌 Found plugin for adapter type '{}'", config.adapter_type);

        eprintln!("📤 Calling plugin.fetch() with config...");
        eprintln!("📤 Config adapter_type: {}", config.adapter_type);
        eprintln!("📤 Config endpoint: {}", config.endpoint);
        eprintln!("📤 Config source: {}", config.source);
        eprintln!("📤 Config parameters: {:?}", config.parameters);

        match plugin.fetch(&config).await {
            Ok(records) => {
                eprintln!("✅ Plugin fetch succeeded! Got {} records", records.len());
                records
            }
            Err(e) => {
                eprintln!("❌ Plugin fetch failed: {}", e);
                tracing::error!("Plugin fetch failed for {}: {}", config.adapter_type, e);
                return Err(format!("Plugin fetch failed: {}", e));
            }
        }
    } else {
        eprintln!("❌ No plugin found for adapter: {}", config.adapter_type);
        tracing::error!("No plugin found for adapter type: {}", config.adapter_type);
        return Err(format!(
            "No plugin found for adapter type: {}. Please install the appropriate plugin.",
            config.adapter_type
        ));
    };

    let count = records.len();
    tracing::info!("Fetched {} records, storing in database", count);

    // Store all records in database (using upsert to prevent duplicates)
    let db = state.database.lock().await;
    let mut upserted = 0;
    for record in records {
        db.upsert_record(record).await.map_err(|e| e.to_string())?;
        upserted += 1;
    }

    tracing::info!(
        "Upserted {} records successfully (updates existing, creates new)",
        upserted
    );

    Ok(count)
}

/// Clear all records from the database
#[tauri::command]
async fn clear_all_records(state: tauri::State<'_, AppState>) -> Result<usize, String> {
    tracing::info!("Clearing all records from database");

    let db = state.database.lock().await;
    let count = db.clear_all_records().await.map_err(|e| e.to_string())?;

    tracing::info!("Cleared {} records", count);
    Ok(count)
}

/// Get database statistics
#[tauri::command]
async fn get_database_stats(
    state: tauri::State<'_, AppState>,
) -> Result<db::DatabaseStats, String> {
    let db = state.database.lock().await;
    db.get_stats().await.map_err(|e| e.to_string())
}

/// M5 Phase 3: Clean up old records based on TTL
#[tauri::command]
async fn cleanup_old_records(
    ttl_days: i64,
    source: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    tracing::info!(
        "Cleaning up records older than {} days for source: {:?}",
        ttl_days,
        source
    );

    let db = state.database.lock().await;
    let deleted = db
        .cleanup_old_records(ttl_days, source.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    tracing::info!("Deleted {} old records", deleted);

    Ok(serde_json::json!({
        "deleted": deleted
    }))
}

/// M5: Delete records by type (e.g., "gitlab_pipeline")
#[tauri::command]
async fn delete_records_by_type(
    record_type: String,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    tracing::info!("Deleting all records of type: {}", record_type);

    let db = state.database.lock().await;
    let deleted = db
        .delete_records_by_type(&record_type)
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "deleted": deleted
    }))
}

/// M5: Delete records by source and type
#[tauri::command]
async fn delete_records_by_source_and_type(
    source: String,
    record_type: String,
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    tracing::info!(
        "Deleting records of type '{}' from source '{}'",
        record_type,
        source
    );

    let db = state.database.lock().await;
    let deleted = db
        .delete_records_by_source_and_type(&source, &record_type)
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "deleted": deleted
    }))
}

/// M9: Export all database data to JSON
/// Can be used to migrate data from dev to prod or vice versa
#[tauri::command]
async fn export_database(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    tracing::info!("Exporting database data");

    let db = state.database.lock().await;
    let mut export = db.export_all_data().await.map_err(|e| e.to_string())?;

    // Also export file-based dashboards (legacy format)
    drop(db); // Release database lock before acquiring dashboard service lock
    let dashboard_service = state.dashboard_service.lock().await;
    let dashboards = dashboard_service
        .get_all()
        .map_err(|e| format!("Failed to export dashboards: {}", e))?;

    // Add dashboards to export
    if let Some(data) = export.get_mut("data") {
        if let Some(data_obj) = data.as_object_mut() {
            data_obj.insert(
                "dashboards".to_string(),
                serde_json::to_value(&dashboards).unwrap_or(serde_json::json!([])),
            );
        }
    }

    tracing::info!(
        "Database export complete (including {} dashboards)",
        dashboards.len()
    );
    Ok(export)
}

/// M9: Import database data from JSON
/// merge_strategy options:
/// - "replace": Clear existing data first, then import
/// - "merge": Keep existing data, add imported data (may create duplicates)
/// - "skip": Keep existing data on conflict
#[tauri::command]
async fn import_database(
    import_data: serde_json::Value,
    merge_strategy: String,
    state: tauri::State<'_, AppState>,
) -> Result<db::ImportStats, String> {
    tracing::info!("Importing database data with strategy: {}", merge_strategy);

    let db = state.database.lock().await;
    let mut stats = db
        .import_data(import_data.clone(), &merge_strategy)
        .await
        .map_err(|e| e.to_string())?;

    // Also import file-based dashboards (legacy format)
    drop(db); // Release database lock before acquiring dashboard service lock

    if let Some(dashboards) = import_data
        .get("data")
        .and_then(|d| d.get("dashboards"))
        .and_then(|d| d.as_array())
    {
        let dashboard_service = state.dashboard_service.lock().await;

        // If replace mode, delete existing dashboards first
        if merge_strategy == "replace" {
            // Get all dashboard IDs and delete them
            if let Ok(existing) = dashboard_service.get_all() {
                for dash in existing {
                    let _ = dashboard_service.delete(&dash.id);
                }
            }
        }

        // Import dashboards
        for dashboard in dashboards {
            match serde_json::from_value::<Dashboard>(dashboard.clone()) {
                Ok(dashboard) => match dashboard_service.save(&dashboard) {
                    Ok(_) => stats.dashboards_imported += 1,
                    Err(e) => stats
                        .errors
                        .push(format!("Failed to import dashboard: {}", e)),
                },
                Err(e) => stats
                    .errors
                    .push(format!("Failed to parse dashboard: {}", e)),
            }
        }

        tracing::info!("Imported {} dashboards", stats.dashboards_imported);
    }

    tracing::info!("Database import complete");
    Ok(stats)
}

// ============================================================================
// Ticket System Command Wrappers
// ============================================================================

#[tauri::command]
async fn create_ticket(
    ticket: tickets::CreateTicketRequest,
    state: tauri::State<'_, AppState>,
) -> Result<tickets::Ticket, String> {
    let db = state.database.lock().await;
    db.create_ticket(ticket).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_ticket(
    id: String,
    updates: tickets::UpdateTicketRequest,
    state: tauri::State<'_, AppState>,
) -> Result<tickets::Ticket, String> {
    let db = state.database.lock().await;
    db.update_ticket(&id, updates)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_ticket(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.database.lock().await;
    db.delete_ticket(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_tickets(state: tauri::State<'_, AppState>) -> Result<Vec<tickets::Ticket>, String> {
    let db = state.database.lock().await;
    db.get_tickets(None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn move_ticket(
    id: String,
    new_status: String,
    state: tauri::State<'_, AppState>,
) -> Result<tickets::Ticket, String> {
    let db = state.database.lock().await;
    db.move_ticket(&id, &new_status)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_comment(
    ticket_id: String,
    text: String,
    state: tauri::State<'_, AppState>,
) -> Result<tickets::Comment, String> {
    let db = state.database.lock().await;
    let req = tickets::CreateCommentRequest {
        author: "User".to_string(), // TODO: Get from auth context
        text,
    };
    db.add_comment(&ticket_id, req)
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// RSS Feed Reader Command Wrapper
// ============================================================================

#[tauri::command]
async fn fetch_rss_feed(url: String) -> Result<serde_json::Value, String> {
    use reqwest;

    tracing::info!("Fetching RSS feed: {}", url);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch RSS feed: {}", e))?;

    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read RSS feed content: {}", e))?;

    // Parse RSS/Atom feed (simplified - you might want to use a proper RSS parser crate)
    // For now, just return the raw XML as a string wrapped in JSON
    Ok(serde_json::json!({
        "url": url,
        "content": content
    }))
}
