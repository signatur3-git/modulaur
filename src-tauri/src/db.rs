use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

// Conditional imports based on feature flags
#[cfg(feature = "embedded-db")]
use surrealdb::engine::local::{Db, SurrealKv};

#[cfg(feature = "sidecar-db")]
use surrealdb::engine::remote::ws::{Client, Ws};

#[cfg(feature = "sidecar-db")]
use surrealdb::opt::auth::Root;

use crate::error::AppError;

/// Generic record stored in SurrealDB
/// This flexible structure allows adapters to store different types of data
/// while maintaining a queryable schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub record_type: String, // "rest_api", "gitlab_pipeline", etc.
    pub source: String,      // adapter identifier
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value, // flexible JSON payload
    pub metadata: RecordMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMetadata {
    pub tags: Vec<String>,
    pub status: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl StagedRecord {
    #[allow(dead_code)] // Will be used when creating records programmatically
    pub fn new(record_type: String, source: String, data: serde_json::Value) -> Self {
        Self {
            id: None, // Will be set by SurrealDB
            record_type,
            source,
            timestamp: Utc::now(),
            data,
            metadata: RecordMetadata {
                tags: Vec::new(),
                status: None,
                title: None,
                description: None,
            },
        }
    }
}

/// Database handle for SurrealDB operations
/// Supports both embedded (SurrealKV) and sidecar (WebSocket) modes
#[cfg(feature = "embedded-db")]
#[derive(Clone)]
pub struct Database {
    pub db: Surreal<Db>,
}

#[cfg(feature = "sidecar-db")]
#[derive(Clone)]
pub struct Database {
    pub db: Surreal<Client>,
}

#[cfg(feature = "embedded-db")]
impl Database {
    /// Initialize embedded SurrealDB with SurrealKv storage
    /// Pure Rust implementation, no C++ compiler needed
    ///
    /// Environment-specific databases:
    /// - Dev mode (debug_assertions): Uses `data_dir/dev/db`
    /// - Prod mode (release): Uses `data_dir/prod/db`
    pub async fn new(data_dir: PathBuf) -> Result<Self, AppError> {
        tracing::info!("Initializing embedded SurrealDB with SurrealKv");

        // Create data directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| AppError::Database(format!("Failed to create data directory: {}", e)))?;

        // Choose environment-specific subdirectory
        let env_subdir = if cfg!(debug_assertions) {
            "dev"
        } else {
            "prod"
        };

        let env_data_dir = data_dir.join(env_subdir);
        std::fs::create_dir_all(&env_data_dir).map_err(|e| {
            AppError::Database(format!(
                "Failed to create environment data directory: {}",
                e
            ))
        })?;

        // Path to database file
        let db_path = env_data_dir.join("db");
        let db_path_str = db_path
            .to_str()
            .ok_or_else(|| AppError::Database("Invalid database path".to_string()))?;

        tracing::info!("Database path ({}): {}", env_subdir, db_path_str);

        // Create embedded database with SurrealKv (pure Rust)
        let db = Surreal::new::<SurrealKv>(db_path_str).await.map_err(|e| {
            AppError::Database(format!("Failed to create embedded database: {}", e))
        })?;

        // Use namespace and database
        db.use_ns("modulaur")
            .use_db("main")
            .await
            .map_err(|e| AppError::Database(format!("Failed to use namespace/database: {}", e)))?;

        tracing::info!(
            "Successfully initialized embedded SurrealDB in {} mode",
            env_subdir
        );

        Ok(Self { db })
    }

    /// Initialize connection to legacy database (pre-stage-separation)
    /// This connects to the old database path without environment subdirectories
    /// Used for migrating historical data to the new stage-separated structure
    pub async fn new_legacy(data_dir: PathBuf) -> Result<Self, AppError> {
        tracing::info!("Initializing connection to legacy SurrealDB (pre-stage-separation)");

        // Path to legacy database (directly in data_dir, no env subdirectory)
        let db_path = data_dir.join("db");

        // Check if legacy database exists
        if !db_path.exists() {
            return Err(AppError::Database(format!(
                "Legacy database not found at: {:?}",
                db_path
            )));
        }

        let db_path_str = db_path
            .to_str()
            .ok_or_else(|| AppError::Database("Invalid database path".to_string()))?;

        tracing::info!("Legacy database path: {}", db_path_str);

        // Create embedded database with SurrealKv (pure Rust)
        let db = Surreal::new::<SurrealKv>(db_path_str).await.map_err(|e| {
            AppError::Database(format!("Failed to connect to legacy database: {}", e))
        })?;

        // Use namespace and database
        db.use_ns("modulaur")
            .use_db("main")
            .await
            .map_err(|e| AppError::Database(format!("Failed to use namespace/database: {}", e)))?;

        tracing::info!("Successfully connected to legacy SurrealDB");

        Ok(Self { db })
    }
}

#[cfg(feature = "sidecar-db")]
impl Database {
    /// Initialize SurrealDB connection to sidecar process
    /// Connects via WebSocket to external SurrealDB server for persistence
    pub async fn new(_data_dir: PathBuf) -> Result<Self, AppError> {
        tracing::info!("Connecting to SurrealDB sidecar via WebSocket");

        // Connect to SurrealDB sidecar (will be started by Tauri)
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.map_err(|e| {
            AppError::Database(format!("Failed to connect to SurrealDB sidecar: {}", e))
        })?;

        // Authenticate (using root credentials for local sidecar)
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map_err(|e| AppError::Database(format!("Failed to authenticate with SurrealDB: {}", e)))?;

        // Use namespace and database
        db.use_ns("modulaur")
            .use_db("main")
            .await
            .map_err(|e| AppError::Database(format!("Failed to use namespace/database: {}", e)))?;

        tracing::info!("Successfully connected to SurrealDB sidecar");

        Ok(Self { db })
    }
}

// Shared methods that work with both embedded and sidecar modes
impl Database {
    /// Create a new record
    pub async fn create_record(&self, record: StagedRecord) -> Result<StagedRecord, AppError> {
        // Create record and let SurrealDB generate the ID
        let created: Option<StagedRecord> = self
            .db
            .create("records")
            .content(record)
            .await
            .map_err(|e| AppError::Database(format!("Failed to create record: {}", e)))?;

        // Get the created record with its ID
        let result =
            created.ok_or_else(|| AppError::Database("Failed to create record".to_string()))?;

        Ok(result)
    }

    /// Upsert a record (update if exists, create if not)
    /// Uses source + record_type + external_id to determine uniqueness
    pub async fn upsert_record(&self, record: StagedRecord) -> Result<StagedRecord, AppError> {
        // Extract external ID from the data payload
        let external_id = record.data.get("id").and_then(|v| v.as_u64()).or_else(|| {
            record
                .data
                .get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse::<u64>().ok())
        });

        if let Some(ext_id) = external_id {
            // Create a deterministic record ID: source_type_externalid
            // e.g., "qcc-gitlab-project_gitlab_pipeline_12345"
            let record_id = format!(
                "{}_{}_{}",
                record.source.replace("-", "_"),
                record.record_type.replace("-", "_"),
                ext_id
            );

            // Use UPSERT with explicit ID
            let created: Option<StagedRecord> = self
                .db
                .upsert(("records", record_id.as_str()))
                .content(record)  // Owned value, no borrowing issue
                .await
                .map_err(|e| AppError::Database(format!("Failed to upsert record: {}", e)))?;

            created.ok_or_else(|| AppError::Database("Failed to upsert record".to_string()))
        } else {
            // No external ID, fall back to regular create (will create duplicates)
            tracing::warn!("Record has no external ID, using create instead of upsert");
            self.create_record(record).await
        }
    }

    /// Get a record by ID
    #[allow(dead_code)] // Will be used in UI for viewing individual records
    pub async fn get_record(&self, id: &str) -> Result<Option<StagedRecord>, AppError> {
        let record: Option<StagedRecord> = self
            .db
            .select(("records", id))
            .await
            .map_err(|e| AppError::Database(format!("Failed to get record: {}", e)))?;

        Ok(record)
    }

    /// Get all records of a specific type
    pub async fn get_records_by_type(
        &self,
        record_type: &str,
    ) -> Result<Vec<StagedRecord>, AppError> {
        tracing::debug!("🔍 Querying records by type: {}", record_type);

        let query = "SELECT * FROM records WHERE record_type = $type ORDER BY timestamp DESC";

        let mut result = self
            .db
            .query(query)
            .bind(("type", record_type.to_string()))
            .await
            .map_err(|e| AppError::Database(format!("Failed to query records: {}", e)))?;

        let records: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract records: {}", e)))?;

        tracing::info!(
            "🔍 Found {} records of type '{}'",
            records.len(),
            record_type
        );

        // Log source values for debugging snippet visibility issues
        if !records.is_empty() && record_type == "snippet" {
            for (i, record) in records.iter().enumerate() {
                tracing::info!("  [{}] source='{}', id={:?}", i, record.source, record.id);
            }
        }

        // Log first few IDs for debugging
        if !records.is_empty() && record_type == "time_entry" {
            for (i, record) in records.iter().take(5).enumerate() {
                if let Some(id) = &record.id {
                    tracing::debug!("  [{}] ID: {:?}", i, id);
                }
            }
        }

        Ok(records)
    }

    /// Get records by source adapter
    #[allow(dead_code)] // Will be used in UI for filtering by source
    pub async fn get_records_by_source(&self, source: &str) -> Result<Vec<StagedRecord>, AppError> {
        let query = "SELECT * FROM records WHERE source = $source ORDER BY timestamp DESC";

        let mut result = self
            .db
            .query(query)
            .bind(("source", source.to_string()))
            .await
            .map_err(|e| AppError::Database(format!("Failed to query records: {}", e)))?;

        let records: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract records: {}", e)))?;

        Ok(records)
    }

    /// Get all records with pagination
    pub async fn get_all_records(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<StagedRecord>, AppError> {
        let query = "SELECT * FROM records ORDER BY timestamp DESC LIMIT $limit START $offset";

        let mut result = self
            .db
            .query(query)
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await
            .map_err(|e| AppError::Database(format!("Failed to query records: {}", e)))?;

        let records: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract records: {}", e)))?;

        Ok(records)
    }

    /// Normalize a record id coming from the frontend.
    ///
    /// The SurrealDB Rust SDK APIs in this code use tuple form ("records", id)
    /// which expects `id` to be the *bare* id part, not "records:<id>".
    /// Some frontend code paths may accidentally pass the fully-qualified
    /// thing id ("records:<id>").
    fn normalize_record_id(id: &str) -> &str {
        id.strip_prefix("records:").unwrap_or(id)
    }

    /// Delete a record by ID
    /// Delete a single record by ID
    pub async fn delete_record(&self, id: &str) -> Result<(), AppError> {
        let id = Self::normalize_record_id(id);
        tracing::info!("🗄️  Database delete_record called for ID: {}", id);

        let deleted: Option<StagedRecord> = self.db.delete(("records", id)).await.map_err(|e| {
            tracing::error!("🗄️  SurrealDB delete failed for {}: {}", id, e);
            AppError::Database(format!("Failed to delete record: {}", e))
        })?;

        if deleted.is_some() {
            tracing::info!("🗄️  Record {} was found and deleted", id);
        } else {
            tracing::warn!("🗄️  Record {} not found (delete returned None)", id);
        }

        Ok(())
    }

    /// Update an existing record by ID
    pub async fn update_record(
        &self,
        id: &str,
        mut record: StagedRecord,
    ) -> Result<StagedRecord, AppError> {
        let id = Self::normalize_record_id(id);
        // Clear the ID from the record to avoid conflicts
        record.id = None;

        // Use UPDATE with merge to modify an existing record
        let updated: Option<StagedRecord> = self
            .db
            .update(("records", id))
            .merge(record)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update record: {}", e)))?;

        updated.ok_or_else(|| AppError::Database(format!("Record not found: {}", id)))
    }

    /// Delete all records from a specific source
    #[allow(dead_code)] // Will be used in UI for clearing adapter data
    pub async fn delete_records_by_source(&self, source: &str) -> Result<usize, AppError> {
        let query = "DELETE records WHERE source = $source RETURN BEFORE";

        let mut result = self
            .db
            .query(query)
            .bind(("source", source.to_string()))
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete records: {}", e)))?;

        let deleted: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract deleted records: {}", e)))?;

        Ok(deleted.len())
    }

    /// M5: Delete all records of a specific type (e.g., "gitlab_pipeline")
    pub async fn delete_records_by_type(&self, record_type: &str) -> Result<usize, AppError> {
        let query = "DELETE records WHERE record_type = $type RETURN BEFORE";

        let mut result = self
            .db
            .query(query)
            .bind(("type", record_type.to_string()))
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete records by type: {}", e)))?;

        let deleted: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract deleted records: {}", e)))?;

        tracing::info!(
            "Deleted {} records of type '{}'",
            deleted.len(),
            record_type
        );
        Ok(deleted.len())
    }

    /// M5: Delete records by source AND type (e.g., source="qcc-gitlab" AND type="gitlab_job")
    pub async fn delete_records_by_source_and_type(
        &self,
        source: &str,
        record_type: &str,
    ) -> Result<usize, AppError> {
        let query = "DELETE records WHERE source = $source AND record_type = $type RETURN BEFORE";

        let mut result = self
            .db
            .query(query)
            .bind(("source", source.to_string()))
            .bind(("type", record_type.to_string()))
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete records: {}", e)))?;

        let deleted: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract deleted records: {}", e)))?;

        tracing::info!(
            "Deleted {} records of type '{}' from source '{}'",
            deleted.len(),
            record_type,
            source
        );
        Ok(deleted.len())
    }

    /// Count total records
    pub async fn count_records(&self) -> Result<usize, AppError> {
        let query = "SELECT count() FROM records GROUP ALL";

        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to count records: {}", e)))?;

        #[derive(Deserialize)]
        struct CountResult {
            count: usize,
        }

        let counts: Vec<CountResult> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract count: {}", e)))?;

        Ok(counts.first().map(|c| c.count).unwrap_or(0))
    }

    /// Search records by tags
    #[allow(dead_code)] // Will be used in UI for tag-based filtering
    pub async fn search_by_tags(&self, tags: Vec<String>) -> Result<Vec<StagedRecord>, AppError> {
        let query =
            "SELECT * FROM records WHERE metadata.tags CONTAINSANY $tags ORDER BY timestamp DESC";

        let mut result = self
            .db
            .query(query)
            .bind(("tags", tags))
            .await
            .map_err(|e| AppError::Database(format!("Failed to search records: {}", e)))?;

        let records: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract records: {}", e)))?;

        Ok(records)
    }

    /// Clear all records from the database
    pub async fn clear_all_records(&self) -> Result<usize, AppError> {
        let query = "DELETE records RETURN BEFORE";

        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete all records: {}", e)))?;

        let deleted: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract deleted records: {}", e)))?;

        tracing::info!("Cleared {} records from database", deleted.len());
        Ok(deleted.len())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats, AppError> {
        let total = self.count_records().await?;

        // Get count by record type
        let query = "SELECT record_type, count() FROM records GROUP BY record_type";
        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to get stats: {}", e)))?;

        #[derive(Deserialize)]
        struct TypeCount {
            record_type: String,
            count: usize,
        }

        let type_counts: Vec<TypeCount> = result.take(0).unwrap_or_default();

        let by_type: std::collections::HashMap<String, usize> = type_counts
            .into_iter()
            .map(|tc| (tc.record_type, tc.count))
            .collect();

        // Get count by source
        let query = "SELECT source, count() FROM records GROUP BY source";
        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to get stats: {}", e)))?;

        #[derive(Deserialize)]
        struct SourceCount {
            source: String,
            count: usize,
        }

        let source_counts: Vec<SourceCount> = result.take(0).unwrap_or_default();

        let by_source: std::collections::HashMap<String, usize> = source_counts
            .into_iter()
            .map(|sc| (sc.source, sc.count))
            .collect();

        // Estimate database size (rough calculation based on record count)
        // Each record is approximately 500 bytes on average
        let size_bytes = (total as u64) * 500;

        Ok(DatabaseStats {
            total_records: total,
            size_bytes,
            by_type,
            by_source,
        })
    }

    /// M5 Phase 3: Clean up old records based on TTL
    /// Deletes records older than ttl_days
    /// If source is provided, only deletes records from that source
    pub async fn cleanup_old_records(
        &self,
        ttl_days: i64,
        source: Option<&str>,
    ) -> Result<usize, AppError> {
        use chrono::{Duration, Utc};

        let cutoff = Utc::now() - Duration::days(ttl_days);

        // Convert source to owned String to satisfy 'static lifetime requirement
        let source_owned = source.map(|s| s.to_string());

        let (query, mut result) = if let Some(src) = &source_owned {
            let res = self
                .db
                .query(
                    "DELETE records WHERE timestamp < $cutoff AND source = $source RETURN BEFORE",
                )
                .bind(("cutoff", cutoff))
                .bind(("source", src.clone()))
                .await
                .map_err(|e| AppError::Database(format!("Failed to cleanup old records: {}", e)))?;
            ("with source filter", res)
        } else {
            let res = self
                .db
                .query("DELETE records WHERE timestamp < $cutoff RETURN BEFORE")
                .bind(("cutoff", cutoff))
                .await
                .map_err(|e| AppError::Database(format!("Failed to cleanup old records: {}", e)))?;
            ("all sources", res)
        };

        let deleted: Vec<StagedRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract deleted records: {}", e)))?;

        tracing::info!(
            "Cleaned up {} records older than {} days ({})",
            deleted.len(),
            ttl_days,
            query
        );

        Ok(deleted.len())
    }

    /// Export all data from the database to JSON
    /// Returns a JSON object containing all tables and their data
    pub async fn export_all_data(&self) -> Result<serde_json::Value, AppError> {
        use serde_json::json;

        tracing::info!("Starting database export");

        // Export records
        let records_query = "SELECT * FROM records ORDER BY timestamp DESC";
        let mut records_result = self
            .db
            .query(records_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export records: {}", e)))?;

        let records: Vec<StagedRecord> = records_result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to extract records: {}", e)))?;

        // Export pages (if table exists)
        let pages_query = "SELECT * FROM pages";
        let mut pages_result = self
            .db
            .query(pages_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export pages: {}", e)))?;

        let pages: Vec<serde_json::Value> = pages_result.take(0).unwrap_or_default();

        // Export data_sources (if table exists)
        let data_sources_query = "SELECT * FROM data_sources";
        let mut data_sources_result = self
            .db
            .query(data_sources_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export data_sources: {}", e)))?;

        let data_sources: Vec<serde_json::Value> = data_sources_result.take(0).unwrap_or_default();

        // Export settings (if table exists)
        let settings_query = "SELECT * FROM settings";
        let mut settings_result = self
            .db
            .query(settings_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export settings: {}", e)))?;

        let settings: Vec<serde_json::Value> = settings_result.take(0).unwrap_or_default();

        // Export plugin_data (if table exists)
        let plugin_data_query = "SELECT * FROM plugin_data";
        let mut plugin_data_result = self
            .db
            .query(plugin_data_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export plugin_data: {}", e)))?;

        let plugin_data: Vec<serde_json::Value> = plugin_data_result.take(0).unwrap_or_default();

        // Export tickets (if table exists)
        let tickets_query = "SELECT * FROM tickets";
        let mut tickets_result = self
            .db
            .query(tickets_query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to export tickets: {}", e)))?;

        let tickets: Vec<serde_json::Value> = tickets_result.take(0).unwrap_or_default();

        let export = json!({
            "version": "1.0",
            "exported_at": chrono::Utc::now().to_rfc3339(),
            "data": {
                "records": records,
                "pages": pages,
                "data_sources": data_sources,
                "settings": settings,
                "plugin_data": plugin_data,
                "tickets": tickets,
                "dashboards": [], // Placeholder - will be filled by main.rs
            }
        });

        tracing::info!("Export complete: {} records, {} pages, {} data_sources, {} settings, {} plugin_data, {} tickets",
            records.len(), pages.len(), data_sources.len(), settings.len(), plugin_data.len(), tickets.len());

        Ok(export)
    }

    /// Import data from JSON export
    /// Accepts a JSON object with the same structure as export_all_data()
    /// merge_strategy: "replace" (clear existing), "merge" (keep both), "skip" (keep existing if conflict)
    pub async fn import_data(
        &self,
        import_data: serde_json::Value,
        merge_strategy: &str,
    ) -> Result<ImportStats, AppError> {
        tracing::info!("Starting database import with strategy: {}", merge_strategy);

        let mut stats = ImportStats {
            records_imported: 0,
            pages_imported: 0,
            data_sources_imported: 0,
            settings_imported: 0,
            plugin_data_imported: 0,
            tickets_imported: 0,
            dashboards_imported: 0,
            errors: Vec::new(),
        };

        // Validate import structure
        let data = import_data.get("data").ok_or_else(|| {
            AppError::Database("Invalid import format: missing 'data' field".to_string())
        })?;

        // If replace mode, clear existing data first
        if merge_strategy == "replace" {
            tracing::info!("Clearing existing data (replace mode)");
            let _ = self.clear_all_records().await;
            let _ = self.db.query("DELETE pages").await;
            let _ = self.db.query("DELETE data_sources").await;
            let _ = self.db.query("DELETE settings").await;
            let _ = self.db.query("DELETE plugin_data").await;
            let _ = self.db.query("DELETE tickets").await;
        }

        // Import records
        if let Some(records) = data.get("records").and_then(|v| v.as_array()) {
            for record in records {
                match serde_json::from_value::<StagedRecord>(record.clone()) {
                    Ok(mut staged_record) => {
                        // Clear ID to let database assign new one (or use upsert logic)
                        staged_record.id = None;

                        match self.upsert_record(staged_record).await {
                            Ok(_) => stats.records_imported += 1,
                            Err(e) => stats.errors.push(format!("Failed to import record: {}", e)),
                        }
                    }
                    Err(e) => stats.errors.push(format!("Failed to parse record: {}", e)),
                }
            }
        }

        // Import pages
        if let Some(pages) = data.get("pages").and_then(|v| v.as_array()) {
            for page in pages {
                match self.db.create("pages").content(page.clone()).await {
                    Ok::<Option<serde_json::Value>, _>(_) => stats.pages_imported += 1,
                    Err(e) => stats.errors.push(format!("Failed to import page: {}", e)),
                }
            }
        }

        // Import data_sources
        if let Some(data_sources) = data.get("data_sources").and_then(|v| v.as_array()) {
            for ds in data_sources {
                match self.db.create("data_sources").content(ds.clone()).await {
                    Ok::<Option<serde_json::Value>, _>(_) => stats.data_sources_imported += 1,
                    Err(e) => stats
                        .errors
                        .push(format!("Failed to import data_source: {}", e)),
                }
            }
        }

        // Import settings
        if let Some(settings) = data.get("settings").and_then(|v| v.as_array()) {
            for setting in settings {
                match self.db.create("settings").content(setting.clone()).await {
                    Ok::<Option<serde_json::Value>, _>(_) => stats.settings_imported += 1,
                    Err(e) => stats
                        .errors
                        .push(format!("Failed to import setting: {}", e)),
                }
            }
        }

        // Import plugin_data
        if let Some(plugin_data) = data.get("plugin_data").and_then(|v| v.as_array()) {
            for pd in plugin_data {
                match self.db.create("plugin_data").content(pd.clone()).await {
                    Ok::<Option<serde_json::Value>, _>(_) => stats.plugin_data_imported += 1,
                    Err(e) => stats
                        .errors
                        .push(format!("Failed to import plugin_data: {}", e)),
                }
            }
        }

        // Import tickets
        if let Some(tickets) = data.get("tickets").and_then(|v| v.as_array()) {
            for ticket in tickets {
                match self.db.create("tickets").content(ticket.clone()).await {
                    Ok::<Option<serde_json::Value>, _>(_) => stats.tickets_imported += 1,
                    Err(e) => stats.errors.push(format!("Failed to import ticket: {}", e)),
                }
            }
        }

        tracing::info!("Import complete: {} records, {} pages, {} data_sources, {} settings, {} plugin_data, {} tickets, {} dashboards, {} errors",
            stats.records_imported, stats.pages_imported, stats.data_sources_imported,
            stats.settings_imported, stats.plugin_data_imported, stats.tickets_imported,
            stats.dashboards_imported, stats.errors.len());

        Ok(stats)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportStats {
    pub records_imported: usize,
    pub pages_imported: usize,
    pub data_sources_imported: usize,
    pub settings_imported: usize,
    pub plugin_data_imported: usize,
    pub tickets_imported: usize,
    pub dashboards_imported: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_records: usize,
    pub size_bytes: u64,
    pub by_type: std::collections::HashMap<String, usize>,
    pub by_source: std::collections::HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_database_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let db = Database::new(temp_dir.path().to_path_buf()).await;
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_create_and_get_record() {
        let temp_dir = TempDir::new().unwrap();
        let db = Database::new(temp_dir.path().to_path_buf()).await.unwrap();

        let record = StagedRecord::new(
            "test_type".to_string(),
            "test_source".to_string(),
            serde_json::json!({"key": "value"}),
        );

        let created = db.create_record(record).await.unwrap();
        let created_id = created
            .id
            .clone()
            .expect("created record should have an id");

        // Thing formats as "records:<id>"; our get_record expects the bare id part.
        let created_id_str = created_id.to_string();
        let bare_id = created_id_str
            .strip_prefix("records:")
            .unwrap_or(created_id_str.as_str());

        let fetched = db.get_record(bare_id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().record_type, "test_type");
    }
}
