use crate::error::AppError;
use crate::models::Dashboard;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

pub struct DashboardService {
    storage_path: PathBuf,
}

impl DashboardService {
    pub fn new() -> Result<Self, AppError> {
        // Use local app data directory
        let app_dir = dirs::data_local_dir()
            .ok_or_else(|| AppError::Config("Cannot determine local data directory".to_string()))?
            .join("modulaur");

        let storage_path = app_dir.join("dashboards");

        // Create directory if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).map_err(AppError::Io)?;
            info!("Created dashboards directory at {:?}", storage_path);
        }

        Ok(Self { storage_path })
    }

    pub fn get_all(&self) -> Result<Vec<Dashboard>, AppError> {
        let mut dashboards = Vec::new();

        if !self.storage_path.exists() {
            return Ok(dashboards);
        }

        for entry in fs::read_dir(&self.storage_path).map_err(AppError::Io)? {
            let entry = entry.map_err(AppError::Io)?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str::<Dashboard>(&content) {
                        Ok(dashboard) => dashboards.push(dashboard),
                        Err(e) => {
                            error!("Failed to parse dashboard file {:?}: {}", path, e);
                        }
                    },
                    Err(e) => {
                        error!("Failed to read dashboard file {:?}: {}", path, e);
                    }
                }
            }
        }

        // Sort by updated_at descending
        dashboards.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(dashboards)
    }

    pub fn get(&self, id: &str) -> Result<Dashboard, AppError> {
        let file_path = self.storage_path.join(format!("{}.json", id));

        if !file_path.exists() {
            return Err(AppError::Config(format!("Dashboard not found: {}", id)));
        }

        let content = fs::read_to_string(&file_path).map_err(AppError::Io)?;

        let dashboard =
            serde_json::from_str::<Dashboard>(&content).map_err(AppError::Serialization)?;

        Ok(dashboard)
    }

    pub fn save(&self, dashboard: &Dashboard) -> Result<(), AppError> {
        let file_path = self.storage_path.join(format!("{}.json", dashboard.id));

        let content = serde_json::to_string_pretty(dashboard).map_err(AppError::Serialization)?;

        fs::write(&file_path, content).map_err(AppError::Io)?;

        info!("Saved dashboard: {} to {:?}", dashboard.id, file_path);

        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        let file_path = self.storage_path.join(format!("{}.json", id));

        if file_path.exists() {
            fs::remove_file(&file_path).map_err(AppError::Io)?;
            info!("Deleted dashboard: {}", id);
        }

        Ok(())
    }
}
