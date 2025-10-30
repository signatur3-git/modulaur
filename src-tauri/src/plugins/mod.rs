// Plugin system for Modulaur
//
// This module provides the infrastructure for loading and executing plugins
// that can extend the platform with custom adapters, visualizations, and more.
//
// Plugins are sandboxed using WebAssembly (WASM) for security and isolation.

mod http;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use wasmtime::*;
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;

use crate::adapters::AdapterConfig;
use crate::db::StagedRecord;
use crate::error::AppError;

// ============================================================================
// Plugin Metadata
// ============================================================================

/// Metadata describing a plugin's capabilities and requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub adapter_type: Option<String>, // If this plugin provides an adapter
    pub capabilities: Vec<String>,
    pub frontend: Option<FrontendConfig>, // Frontend configuration if available
}

// ============================================================================
// Plugin Manifest
// ============================================================================

/// Plugin manifest (manifest.json) structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub homepage: Option<String>,

    pub backend: Option<BackendConfig>,
    pub frontend: Option<FrontendConfig>,

    #[serde(default)]
    pub permissions: Vec<String>,

    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    #[serde(rename = "type")]
    pub type_: String, // "wasm" or "native"
    pub entry: String, // Path to .wasm file
    pub adapters: Vec<AdapterInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendConfig {
    pub entry: String,
    #[serde(default)]
    pub components: Vec<ComponentInfo>,
    #[serde(default)]
    pub styles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    #[serde(rename = "type")]
    pub type_: String, // "panel", "theme", "view"
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub config_schema: Option<ConfigSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSchema {
    pub fields: Vec<ConfigField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigField {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub type_: String, // "text", "textarea", "select", "number", "checkbox"
    pub options: Option<Vec<ConfigOption>>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub rows: Option<u32>,
    pub help: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub value: serde_json::Value, // Can be string, number, or boolean
    pub label: String,
}

// ============================================================================
// Plugin Context
// ============================================================================

/// Context provided to plugins for accessing app services
pub struct PluginContext {
    pub http_client: reqwest::Client,
    pub config_dir: PathBuf,
}

impl PluginContext {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config_dir,
        }
    }
}

// ============================================================================
// Plugin Trait
// ============================================================================

/// Main plugin trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Initialize the plugin
    async fn init(&mut self, context: PluginContext) -> Result<(), AppError>;

    /// Fetch data (for adapter plugins)
    async fn fetch(&self, config: &AdapterConfig) -> Result<Vec<StagedRecord>, AppError>;

    /// Test connection (for adapter plugins)
    async fn test_connection(&self, config: &AdapterConfig) -> Result<bool, AppError>;

    /// Shutdown the plugin
    async fn shutdown(&mut self) -> Result<(), AppError>;
}

// ============================================================================
// WASM Plugin Instance
// ============================================================================

/// A loaded WASM plugin instance
pub struct WasmPlugin {
    metadata: PluginMetadata,
    engine: Engine,
    module: Module,
}

impl WasmPlugin {
    /// Load a WASM plugin from file
    pub fn load(wasm_path: &Path, metadata: PluginMetadata) -> Result<Self, AppError> {
        tracing::info!("Loading WASM plugin from: {:?}", wasm_path);

        // Create WASM engine with default configuration
        let engine = Engine::default();

        // Load the WASM module
        let module = Module::from_file(&engine, wasm_path)
            .map_err(|e| AppError::Plugin(format!("Failed to load WASM module: {}", e)))?;

        Ok(Self {
            metadata,
            engine,
            module,
        })
    }

    /// Call a function in the WASM module
    async fn call_function(
        &self,
        function_name: &str,
        params: Vec<u8>,
    ) -> Result<Vec<u8>, AppError> {
        tracing::debug!(
            "Calling WASM function: {} with {} bytes",
            function_name,
            params.len()
        );

        // Create WASI context with preview1
        let wasi_ctx: WasiP1Ctx = WasiCtxBuilder::new().inherit_stdio().build_p1();

        // Create store with WASI context
        let mut store = Store::new(&self.engine, wasi_ctx);

        // Create linker with correct type
        let mut linker: Linker<WasiP1Ctx> = Linker::new(&self.engine);

        // Add WASI preview1 to linker
        preview1::add_to_linker_sync(&mut linker, |ctx: &mut WasiP1Ctx| ctx)
            .map_err(|e| AppError::Plugin(format!("Failed to add WASI to linker: {}", e)))?;

        // Add HTTP host functions to linker
        http::add_http_to_linker(&mut linker).map_err(|e| {
            AppError::Plugin(format!("Failed to add HTTP functions to linker: {}", e))
        })?;

        // Instantiate the module (sync instantiate with preview1)
        let instance = linker
            .instantiate(&mut store, &self.module)
            .map_err(|e| AppError::Plugin(format!("Failed to instantiate WASM module: {}", e)))?;

        // Get memory (for string passing)
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| AppError::Plugin("WASM module does not export memory".to_string()))?;

        // Allocate space in WASM memory for the input string
        let alloc_fn = instance
            .get_typed_func::<u32, u32>(&mut store, "alloc")
            .ok();

        let input_ptr = if let Some(alloc) = alloc_fn {
            // Use plugin's allocator if available
            let size = params.len() as u32;
            alloc
                .call(&mut store, size)
                .map_err(|e| AppError::Plugin(format!("Failed to allocate memory: {}", e)))?
        } else {
            // Fallback: write at a safe offset (assuming memory is large enough)
            1024u32
        };

        // Write input data to WASM memory
        memory
            .write(&mut store, input_ptr as usize, &params)
            .map_err(|e| AppError::Plugin(format!("Failed to write to WASM memory: {}", e)))?;

        // Add null terminator for C string
        let null_byte = [0u8];
        memory
            .write(&mut store, (input_ptr as usize) + params.len(), &null_byte)
            .map_err(|e| AppError::Plugin(format!("Failed to write null terminator: {}", e)))?;

        // Get and call the target function
        let func = instance
            .get_typed_func::<u32, u32>(&mut store, function_name)
            .map_err(|e| {
                AppError::Plugin(format!("Function '{}' not found: {}", function_name, e))
            })?;

        let result_ptr = func
            .call(&mut store, input_ptr)
            .map_err(|e| AppError::Plugin(format!("Failed to call WASM function: {}", e)))?;

        // Read result from WASM memory
        // Support up to 10MB responses for large deep fetch results
        // Read in 4KB chunks for better performance
        let mut result = Vec::new();
        let mut offset = result_ptr as usize;
        const MAX_RESULT_SIZE: usize = 10 * 1024 * 1024; // 10MB
        const CHUNK_SIZE: usize = 4096; // 4KB chunks

        loop {
            // Read a chunk
            let remaining = MAX_RESULT_SIZE - result.len();
            if remaining == 0 {
                return Err(AppError::Plugin(format!(
                    "Plugin result exceeds maximum size of {} bytes",
                    MAX_RESULT_SIZE
                )));
            }

            let chunk_size = remaining.min(CHUNK_SIZE);
            let mut chunk = vec![0u8; chunk_size];

            memory
                .read(&store, offset, &mut chunk)
                .map_err(|e| AppError::Plugin(format!("Failed to read from WASM memory: {}", e)))?;

            // Find null terminator in chunk
            if let Some(null_pos) = chunk.iter().position(|&b| b == 0) {
                result.extend_from_slice(&chunk[..null_pos]);
                break;
            }

            // No null terminator found, add whole chunk and continue
            result.extend_from_slice(&chunk);
            offset += chunk_size;
        }

        // Free the result string if free_string function exists
        if let Ok(free_fn) = instance.get_typed_func::<u32, ()>(&mut store, "free_string") {
            let _ = free_fn.call(&mut store, result_ptr);
        }

        tracing::debug!("WASM function returned {} bytes", result.len());
        Ok(result)
    }
}

#[async_trait]
impl Plugin for WasmPlugin {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    async fn init(&mut self, _context: PluginContext) -> Result<(), AppError> {
        tracing::info!("Initializing plugin: {}", self.metadata.name);
        // TODO: Call plugin's init function via WASM
        Ok(())
    }

    async fn fetch(&self, config: &AdapterConfig) -> Result<Vec<StagedRecord>, AppError> {
        tracing::info!("Fetching data using plugin: {}", self.metadata.name);

        // Serialize config to JSON for passing to WASM
        let config_json = serde_json::to_vec(config)
            .map_err(|e| AppError::Plugin(format!("Failed to serialize config: {}", e)))?;

        // Call the WASM fetch function (wasm_bindgen exports as "plugin_fetch")
        let result = self.call_function("plugin_fetch", config_json).await?;

        // Deserialize the result
        let records: Vec<StagedRecord> = serde_json::from_slice(&result)
            .map_err(|e| AppError::Plugin(format!("Failed to deserialize plugin result: {}", e)))?;

        tracing::info!("Plugin returned {} records", records.len());
        Ok(records)
    }

    async fn test_connection(&self, config: &AdapterConfig) -> Result<bool, AppError> {
        tracing::info!("Testing connection using plugin: {}", self.metadata.name);

        // Serialize config to JSON
        let config_json = serde_json::to_vec(config)
            .map_err(|e| AppError::Plugin(format!("Failed to serialize config: {}", e)))?;

        // Call the WASM test_connection function (wasm_bindgen exports as "plugin_test_connection")
        let result = self
            .call_function("plugin_test_connection", config_json)
            .await?;

        // For now, assume success if no error
        Ok(!result.is_empty())
    }

    async fn shutdown(&mut self) -> Result<(), AppError> {
        tracing::info!("Shutting down plugin: {}", self.metadata.name);
        Ok(())
    }
}

// ============================================================================
// Plugin Manager
// ============================================================================

/// Manages all loaded plugins
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>, // Backend plugins (WASM)
    manifests: HashMap<String, PluginManifest>, // All plugin manifests (including frontend-only)
    plugin_dir: PathBuf,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            manifests: HashMap::new(),
            plugin_dir,
        }
    }

    /// Scan plugin directory and load all plugins
    pub async fn load_plugins(&mut self) -> Result<usize, AppError> {
        eprintln!("🔍 PluginManager::load_plugins() called");
        eprintln!("   Current plugins in HashMap: {}", self.plugins.len());
        tracing::info!("Scanning for plugins in: {:?}", self.plugin_dir);

        if !self.plugin_dir.exists() {
            eprintln!("⚠️  Plugin directory does not exist: {:?}", self.plugin_dir);
            tracing::warn!(
                "Plugin directory does not exist, creating: {:?}",
                self.plugin_dir
            );
            std::fs::create_dir_all(&self.plugin_dir).map_err(|e| {
                AppError::Plugin(format!("Failed to create plugin directory: {}", e))
            })?;
            return Ok(0);
        }

        let entries = std::fs::read_dir(&self.plugin_dir)
            .map_err(|e| AppError::Plugin(format!("Failed to read plugin directory: {}", e)))?;

        let mut count = 0;

        for entry in entries {
            let entry =
                entry.map_err(|e| AppError::Plugin(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                eprintln!("Attempting to load plugin from: {:?}", path);
                match self.load_plugin(&path).await {
                    Ok(_) => {
                        count += 1;
                        eprintln!("✅ Successfully loaded plugin from: {:?}", path);
                        tracing::info!("Successfully loaded plugin from: {:?}", path);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to load plugin {:?}: {}", path, e);
                        tracing::warn!("Failed to load plugin {:?}: {}", path, e);
                    }
                }
            }
        }

        eprintln!(
            "📊 load_plugins() complete: {} plugins in HashMap",
            self.plugins.len()
        );
        tracing::info!("Loaded {} plugins", count);
        Ok(count)
    }

    /// Load a single plugin from directory
    async fn load_plugin(&mut self, path: &Path) -> Result<(), AppError> {
        // 1. Read manifest.json
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            return Err(AppError::Plugin(format!(
                "No manifest.json found in {:?}",
                path
            )));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| AppError::Plugin(format!("Failed to read manifest: {}", e)))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| AppError::Plugin(format!("Failed to parse manifest: {}", e)))?;

        tracing::info!("Loading plugin: {} v{}", manifest.name, manifest.version);

        // 2. Store manifest (for all plugins, including frontend-only)
        self.manifests
            .insert(manifest.name.clone(), manifest.clone());

        // 3. Validate permissions
        self.validate_permissions(&manifest)?;

        // 4. Load backend module if present
        if let Some(backend) = &manifest.backend {
            if backend.type_ == "wasm" {
                let wasm_path = path.join(&backend.entry);
                if !wasm_path.exists() {
                    return Err(AppError::Plugin(format!(
                        "WASM file not found: {:?}",
                        wasm_path
                    )));
                }

                // Create metadata from manifest
                let metadata = PluginMetadata {
                    name: manifest.name.clone(),
                    version: manifest.version.clone(),
                    author: manifest.author.clone(),
                    description: manifest.description.clone(),
                    adapter_type: backend.adapters.first().map(|a| a.type_.clone()),
                    capabilities: backend
                        .adapters
                        .first()
                        .map(|a| a.capabilities.clone())
                        .unwrap_or_default(),
                    frontend: manifest.frontend.clone(), // Include frontend config
                };

                // Load the WASM plugin
                let plugin = WasmPlugin::load(&wasm_path, metadata)?;

                self.plugins.insert(manifest.name.clone(), Box::new(plugin));
            } else {
                return Err(AppError::Plugin(format!(
                    "Unsupported backend type: {}",
                    backend.type_
                )));
            }
        } else if manifest.frontend.is_some() {
            // Frontend-only plugin (no backend)
            // For now, we just track it in metadata without loading a WASM module
            // The frontend will handle loading the Vue components
            eprintln!(
                "✅ Frontend-only plugin: {} (no backend to load)",
                manifest.name
            );
            tracing::info!("Frontend-only plugin registered: {}", manifest.name);

            // Note: We don't add it to self.plugins because it has no backend implementation
            // The frontend will query the manifest directly via get_installed_plugins
        }

        Ok(())
    }

    /// Validate plugin permissions
    fn validate_permissions(&self, manifest: &PluginManifest) -> Result<(), AppError> {
        // TODO: Implement permission validation
        // For now, just log the permissions
        tracing::info!(
            "Plugin {} requests permissions: {:?}",
            manifest.name,
            manifest.permissions
        );
        Ok(())
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }

    /// Get all loaded plugins
    pub fn get_all_plugins(&self) -> Vec<PluginMetadata> {
        // Return metadata from ALL manifests (including frontend-only plugins)
        let plugins: Vec<PluginMetadata> = self
            .manifests
            .values()
            .map(|manifest| {
                // Check if there's a loaded backend plugin for additional info
                let backend_metadata = self.plugins.get(&manifest.name).map(|p| p.metadata());

                // Create metadata from manifest
                PluginMetadata {
                    name: manifest.name.clone(),
                    version: manifest.version.clone(),
                    author: manifest.author.clone(),
                    description: manifest.description.clone(),
                    adapter_type: backend_metadata
                        .as_ref()
                        .and_then(|m| m.adapter_type.clone())
                        .or_else(|| {
                            manifest
                                .backend
                                .as_ref()
                                .and_then(|b| b.adapters.first().map(|a| a.type_.clone()))
                        }),
                    capabilities: backend_metadata
                        .as_ref()
                        .map(|m| m.capabilities.clone())
                        .unwrap_or_else(|| {
                            manifest
                                .backend
                                .as_ref()
                                .and_then(|b| b.adapters.first().map(|a| a.capabilities.clone()))
                                .unwrap_or_default()
                        }),
                    frontend: manifest.frontend.clone(),
                }
            })
            .collect();

        eprintln!(
            "📋 get_all_plugins() called: returning {} plugins",
            plugins.len()
        );
        for plugin in &plugins {
            eprintln!(
                "   - {} (frontend: {})",
                plugin.name,
                plugin.frontend.is_some()
            );
        }
        plugins
    }

    /// Get a plugin by adapter type (for Phase 3.3 plugin-first lookup)
    pub fn get_plugin_by_adapter_type(&self, adapter_type: &str) -> Option<&dyn Plugin> {
        // Check all loaded backend plugins for matching adapter type
        for (name, plugin) in &self.plugins {
            let metadata = plugin.metadata();
            if metadata.adapter_type.as_deref() == Some(adapter_type) {
                eprintln!(
                    "🔌 Found plugin '{}' for adapter type '{}'",
                    name, adapter_type
                );
                tracing::info!("Found plugin {} for adapter type {}", name, adapter_type);
                return Some(plugin.as_ref());
            }
        }

        eprintln!("❌ No plugin found for adapter type '{}'", adapter_type);
        tracing::debug!("No plugin found for adapter type {}", adapter_type);
        None
    }

    /// Unload a plugin
    pub async fn unload_plugin(&mut self, name: &str) -> Result<(), AppError> {
        if let Some(mut plugin) = self.plugins.remove(name) {
            plugin.shutdown().await?;
            tracing::info!("Unloaded plugin: {}", name);
        }
        Ok(())
    }

    /// Shutdown all plugins
    pub async fn shutdown_all(&mut self) -> Result<(), AppError> {
        tracing::info!("Shutting down all plugins");

        for (name, mut plugin) in self.plugins.drain() {
            if let Err(e) = plugin.shutdown().await {
                tracing::error!("Error shutting down plugin {}: {}", name, e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest_parse() {
        let json = r#"{
            "name": "test-plugin",
            "version": "1.0.0",
            "author": "Test Author",
            "description": "Test plugin",
            "permissions": ["network:https://example.com"]
        }"#;

        let manifest: PluginManifest = serde_json::from_str(json).unwrap();
        assert_eq!(manifest.name, "test-plugin");
        assert_eq!(manifest.version, "1.0.0");
    }
}
