// Data source management service
// Handles CRUD operations for data source configurations

use crate::db::Database;
use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ============================================================================
// Data Source Models
// ============================================================================

/// Data source record as stored in database (with Thing ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataSourceRecord {
    pub id: Thing,
    pub name: String,
    pub adapter_type: String,
    pub source: String,
    pub endpoint: String,
    pub auth_type: Option<String>,
    pub auth_credential_key: Option<String>,
    pub parameters: serde_json::Value,
    pub environment: String,
    pub enabled: bool,
    pub auto_refresh: bool,
    pub refresh_interval: Option<i32>,
    pub data_ttl_days: i32,
    pub last_fetch: Option<DateTime<Utc>>,
    pub last_fetch_count: Option<i32>,
    pub total_records: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User-facing data source structure with String ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    pub adapter_type: String,
    pub source: String,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_credential_key: Option<String>,
    pub parameters: serde_json::Value,
    pub environment: String,
    pub enabled: bool,
    pub auto_refresh: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<i32>,
    pub data_ttl_days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetch: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetch_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<DataSourceRecord> for DataSource {
    fn from(record: DataSourceRecord) -> Self {
        DataSource {
            id: record.id.to_string(),
            name: record.name,
            adapter_type: record.adapter_type,
            source: record.source,
            endpoint: record.endpoint,
            auth_type: record.auth_type,
            auth_credential_key: record.auth_credential_key,
            parameters: record.parameters,
            environment: record.environment,
            enabled: record.enabled,
            auto_refresh: record.auto_refresh,
            refresh_interval: record.refresh_interval,
            data_ttl_days: record.data_ttl_days,
            last_fetch: record.last_fetch,
            last_fetch_count: record.last_fetch_count,
            total_records: record.total_records,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

// ============================================================================
// Data Source Service
// ============================================================================

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DataSourceService {
    db: Arc<Mutex<Database>>,
}

impl DataSourceService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    /// Get all data sources
    pub async fn get_all_data_sources(&self) -> Result<Vec<DataSource>, AppError> {
        let query = "SELECT * FROM data_sources ORDER BY name ASC";

        let db = self.db.lock().await;
        let mut result = db
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to query data sources: {}", e)))?;

        let sources: Vec<DataSourceRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse data sources: {}", e)))?;

        Ok(sources.into_iter().map(|s| s.into()).collect())
    }

    /// Get a specific data source by ID
    pub async fn get_data_source(&self, id: &str) -> Result<Option<DataSource>, AppError> {
        let db = self.db.lock().await;
        let result: Option<DataSourceRecord> = db
            .db
            .select(("data_sources", id))
            .await
            .map_err(|e| AppError::Database(format!("Failed to get data source: {}", e)))?;

        Ok(result.map(|r| r.into()))
    }

    /// Create or update a data source
    pub async fn save_data_source(&self, source: &DataSource) -> Result<(), AppError> {
        // Validate environment
        self.validate_environment(&source.environment)?;

        let now = Utc::now();

        let db = self.db.lock().await;

        // Check if exists
        let exists: Option<DataSourceRecord> = db
            .db
            .select(("data_sources", source.id.as_str()))
            .await
            .map_err(|e| {
                AppError::Database(format!("Failed to check data source existence: {}", e))
            })?;

        let record = if let Some(existing) = exists {
            // Update existing
            DataSourceRecord {
                id: Thing::from(("data_sources", source.id.as_str())),
                name: source.name.clone(),
                adapter_type: source.adapter_type.clone(),
                source: source.source.clone(),
                endpoint: source.endpoint.clone(),
                auth_type: source.auth_type.clone(),
                auth_credential_key: source.auth_credential_key.clone(),
                parameters: source.parameters.clone(),
                environment: source.environment.clone(),
                enabled: source.enabled,
                auto_refresh: source.auto_refresh,
                refresh_interval: source.refresh_interval,
                data_ttl_days: source.data_ttl_days,
                last_fetch: source.last_fetch,
                last_fetch_count: source.last_fetch_count,
                total_records: source.total_records,
                created_at: existing.created_at,
                updated_at: now,
            }
        } else {
            // Create new
            DataSourceRecord {
                id: Thing::from(("data_sources", source.id.as_str())),
                name: source.name.clone(),
                adapter_type: source.adapter_type.clone(),
                source: source.source.clone(),
                endpoint: source.endpoint.clone(),
                auth_type: source.auth_type.clone(),
                auth_credential_key: source.auth_credential_key.clone(),
                parameters: source.parameters.clone(),
                environment: source.environment.clone(),
                enabled: source.enabled,
                auto_refresh: source.auto_refresh,
                refresh_interval: source.refresh_interval,
                data_ttl_days: source.data_ttl_days,
                last_fetch: None,
                last_fetch_count: None,
                total_records: None,
                created_at: now,
                updated_at: now,
            }
        };

        let _: Option<DataSourceRecord> = db
            .db
            .update(("data_sources", source.id.as_str()))
            .content(record)
            .await
            .map_err(|e| AppError::Database(format!("Failed to save data source: {}", e)))?;

        tracing::info!("Saved data source: {} ({})", source.name, source.id);
        Ok(())
    }

    /// Delete a data source
    pub async fn delete_data_source(&self, id: &str) -> Result<(), AppError> {
        let db = self.db.lock().await;
        let _deleted: Option<DataSourceRecord> = db
            .db
            .delete(("data_sources", id))
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete data source: {}", e)))?;

        tracing::info!("Deleted data source: {}", id);
        Ok(())
    }

    /// Validate that a data source is compatible with current environment
    pub async fn validate_data_source(&self, id: &str) -> Result<bool, AppError> {
        let source = self.get_data_source(id).await?;

        match source {
            None => Ok(false),
            Some(s) => {
                // Check environment compatibility
                let is_production = !cfg!(debug_assertions);

                match s.environment.as_str() {
                    "dev" if is_production => {
                        tracing::warn!("Dev-only data source {} in production", id);
                        Ok(false)
                    }
                    "production" if !is_production => {
                        tracing::warn!("Prod-only data source {} in dev", id);
                        Ok(false)
                    }
                    _ => Ok(true), // "both" or matching environment
                }
            }
        }
    }

    /// Update fetch statistics
    pub async fn update_fetch_stats(&self, id: &str, record_count: i32) -> Result<(), AppError> {
        let db = self.db.lock().await;
        let query = format!(
            "UPDATE data_sources:{} SET last_fetch = $now, last_fetch_count = {}",
            id, record_count
        );

        db.db
            .query(&query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update fetch stats: {}", e)))?;

        Ok(())
    }

    // Private helper
    fn validate_environment(&self, env: &str) -> Result<(), AppError> {
        match env {
            "dev" | "production" | "both" => Ok(()),
            _ => Err(AppError::Config(format!(
                "Invalid environment: {}. Must be 'dev', 'production', or 'both'",
                env
            ))),
        }
    }
}
