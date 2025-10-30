// Plugin data management service
// Handles CRUD operations for plugin-specific data storage

use crate::db::Database;
use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ============================================================================
// Plugin Data Models
// ============================================================================

/// Plugin data record as stored in database (with Thing ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PluginDataRecord {
    pub id: Thing,
    pub plugin_id: String,
    pub panel_id: Option<String>,
    pub scope: String,
    pub key: String,
    pub value: String,
    pub data_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User-facing plugin data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginData {
    pub id: String,
    pub plugin_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_id: Option<String>,
    pub scope: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<PluginDataRecord> for PluginData {
    fn from(record: PluginDataRecord) -> Self {
        PluginData {
            id: record.id.to_string(),
            plugin_id: record.plugin_id,
            panel_id: record.panel_id,
            scope: record.scope,
            key: record.key,
            value: record.value,
            data_type: record.data_type,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}

// ============================================================================
// Plugin Data Service
// ============================================================================

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PluginDataService {
    db: Arc<Mutex<Database>>,
}

impl PluginDataService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    /// Get plugin data by plugin_id, panel_id (optional), and key
    pub async fn get_plugin_data(
        &self,
        plugin_id: &str,
        panel_id: Option<&str>,
        key: &str,
    ) -> Result<Option<String>, AppError> {
        let query = if let Some(pid) = panel_id {
            format!(
                "SELECT * FROM plugin_data WHERE plugin_id = '{}' AND panel_id = '{}' AND key = '{}'",
                plugin_id, pid, key
            )
        } else {
            format!(
                "SELECT * FROM plugin_data WHERE plugin_id = '{}' AND panel_id = NONE AND key = '{}'",
                plugin_id, key
            )
        };

        let db = self.db.lock().await;
        let mut result = db
            .db
            .query(&query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to query plugin data: {}", e)))?;

        let data: Vec<PluginDataRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse plugin data: {}", e)))?;

        Ok(data.first().map(|r| r.value.clone()))
    }

    /// Save plugin data
    pub async fn save_plugin_data(
        &self,
        plugin_id: &str,
        panel_id: Option<&str>,
        key: &str,
        value: &str,
        data_type: &str,
    ) -> Result<(), AppError> {
        // Validate type
        self.validate_data_type(data_type)?;

        // Validate scope
        let scope = if panel_id.is_some() {
            "panel"
        } else {
            "global"
        };

        let now = Utc::now();

        // Check if exists
        let existing = self.get_plugin_data(plugin_id, panel_id, key).await?;

        let db = self.db.lock().await;

        if existing.is_some() {
            // Update existing
            let query = if let Some(pid) = panel_id {
                format!(
                    "UPDATE plugin_data SET value = '{}', data_type = '{}', updated_at = $now \
                     WHERE plugin_id = '{}' AND panel_id = '{}' AND key = '{}'",
                    value, data_type, plugin_id, pid, key
                )
            } else {
                format!(
                    "UPDATE plugin_data SET value = '{}', data_type = '{}', updated_at = $now \
                     WHERE plugin_id = '{}' AND panel_id = NONE AND key = '{}'",
                    value, data_type, plugin_id, key
                )
            };

            db.db
                .query(&query)
                .await
                .map_err(|e| AppError::Database(format!("Failed to update plugin data: {}", e)))?;
        } else {
            // Create new with unique ID
            let id = format!("{}:{}:{}", plugin_id, panel_id.unwrap_or("global"), key);

            let record = PluginDataRecord {
                id: Thing::from(("plugin_data", id.as_str())),
                plugin_id: plugin_id.to_string(),
                panel_id: panel_id.map(|s| s.to_string()),
                scope: scope.to_string(),
                key: key.to_string(),
                value: value.to_string(),
                data_type: data_type.to_string(),
                created_at: now,
                updated_at: now,
            };

            let _: Option<PluginDataRecord> = db
                .db
                .create("plugin_data")
                .content(record)
                .await
                .map_err(|e| AppError::Database(format!("Failed to create plugin data: {}", e)))?;
        }

        tracing::info!("Saved plugin data: {}:{:?}:{}", plugin_id, panel_id, key);
        Ok(())
    }

    /// Delete plugin data
    pub async fn delete_plugin_data(
        &self,
        plugin_id: &str,
        panel_id: Option<&str>,
        key: Option<&str>,
    ) -> Result<(), AppError> {
        let db = self.db.lock().await;

        let query = match (panel_id, key) {
            (Some(pid), Some(k)) => {
                // Delete specific key for specific panel
                format!(
                    "DELETE FROM plugin_data WHERE plugin_id = '{}' AND panel_id = '{}' AND key = '{}'",
                    plugin_id, pid, k
                )
            }
            (Some(pid), None) => {
                // Delete all data for specific panel
                format!(
                    "DELETE FROM plugin_data WHERE plugin_id = '{}' AND panel_id = '{}'",
                    plugin_id, pid
                )
            }
            (None, Some(k)) => {
                // Delete specific key for all panels (global)
                format!(
                    "DELETE FROM plugin_data WHERE plugin_id = '{}' AND panel_id = NONE AND key = '{}'",
                    plugin_id, k
                )
            }
            (None, None) => {
                // Delete all data for plugin
                format!("DELETE FROM plugin_data WHERE plugin_id = '{}'", plugin_id)
            }
        };

        db.db
            .query(&query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete plugin data: {}", e)))?;

        tracing::info!(
            "Deleted plugin data: {}:{:?}:{:?}",
            plugin_id,
            panel_id,
            key
        );
        Ok(())
    }

    /// Get all data for a plugin
    pub async fn get_all_plugin_data(&self, plugin_id: &str) -> Result<Vec<PluginData>, AppError> {
        let query = format!(
            "SELECT * FROM plugin_data WHERE plugin_id = '{}'",
            plugin_id
        );

        let db = self.db.lock().await;
        let mut result = db
            .db
            .query(&query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to query plugin data: {}", e)))?;

        let data: Vec<PluginDataRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse plugin data: {}", e)))?;

        Ok(data.into_iter().map(|d| d.into()).collect())
    }

    // Private helper
    fn validate_data_type(&self, data_type: &str) -> Result<(), AppError> {
        match data_type {
            "string" | "number" | "boolean" | "json" => Ok(()),
            _ => Err(AppError::Config(format!(
                "Invalid data type: {}. Must be 'string', 'number', 'boolean', or 'json'",
                data_type
            ))),
        }
    }
}
