// Settings management service
// Handles CRUD operations for application settings

use crate::db::Database;
use crate::error::AppError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::sql::Thing;

// ============================================================================
// Settings Models
// ============================================================================

/// Setting record as stored in database (with Thing ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SettingRecord {
    pub id: Thing,
    pub key: String,
    pub value: String,
    pub setting_type: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// User-facing setting structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub setting_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<SettingRecord> for Setting {
    fn from(record: SettingRecord) -> Self {
        Setting {
            key: record.key,
            value: record.value,
            setting_type: record.setting_type,
            category: record.category,
            description: record.description,
            updated_at: record.updated_at,
        }
    }
}

// ============================================================================
// Settings Service
// ============================================================================

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SettingsService {
    db: Arc<Mutex<Database>>,
}

impl SettingsService {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    /// Get all settings as a HashMap
    pub async fn get_all_settings(&self) -> Result<HashMap<String, String>, AppError> {
        let query = "SELECT * FROM settings";

        let db = self.db.lock().await;
        let mut result = db
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to query settings: {}", e)))?;

        let settings: Vec<SettingRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse settings: {}", e)))?;

        let mut map = HashMap::new();
        for setting in settings {
            map.insert(setting.key, setting.value);
        }

        Ok(map)
    }

    /// Get a specific setting by key
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>, AppError> {
        let db = self.db.lock().await;
        let result: Option<SettingRecord> = db
            .db
            .select(("settings", key))
            .await
            .map_err(|e| AppError::Database(format!("Failed to get setting: {}", e)))?;

        Ok(result.map(|r| r.value))
    }

    /// Save a setting
    pub async fn save_setting(
        &self,
        key: &str,
        value: &str,
        setting_type: &str,
        category: Option<String>,
    ) -> Result<(), AppError> {
        // Validate type
        self.validate_setting_type(setting_type)?;

        let now = Utc::now();

        let db = self.db.lock().await;

        let record = SettingRecord {
            id: Thing::from(("settings", key)),
            key: key.to_string(),
            value: value.to_string(),
            setting_type: setting_type.to_string(),
            category,
            description: None,
            updated_at: now,
        };

        let _: Option<SettingRecord> = db
            .db
            .update(("settings", key))
            .content(record)
            .await
            .map_err(|e| AppError::Database(format!("Failed to save setting: {}", e)))?;

        tracing::info!("Saved setting: {} = {}", key, value);
        Ok(())
    }

    /// Delete a setting
    pub async fn delete_setting(&self, key: &str) -> Result<(), AppError> {
        let db = self.db.lock().await;
        let _deleted: Option<SettingRecord> = db
            .db
            .delete(("settings", key))
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete setting: {}", e)))?;

        tracing::info!("Deleted setting: {}", key);
        Ok(())
    }

    /// Get settings by category
    pub async fn get_settings_by_category(&self, category: &str) -> Result<Vec<Setting>, AppError> {
        let query = format!("SELECT * FROM settings WHERE category = '{}'", category);

        let db = self.db.lock().await;
        let mut result = db.db.query(&query).await.map_err(|e| {
            AppError::Database(format!("Failed to query settings by category: {}", e))
        })?;

        let settings: Vec<SettingRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse settings: {}", e)))?;

        Ok(settings.into_iter().map(|s| s.into()).collect())
    }

    // Private helper
    fn validate_setting_type(&self, setting_type: &str) -> Result<(), AppError> {
        match setting_type {
            "string" | "number" | "boolean" | "json" => Ok(()),
            _ => Err(AppError::Config(format!(
                "Invalid setting type: {}. Must be 'string', 'number', 'boolean', or 'json'",
                setting_type
            ))),
        }
    }
}
