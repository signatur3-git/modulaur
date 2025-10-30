use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub i: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    #[serde(rename = "type")]
    pub panel_type: String,
    pub title: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: String,
    pub name: String,
    pub panels: Vec<Panel>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl Dashboard {
    #[allow(dead_code)] // Will be used when creating dashboards programmatically
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: format!("dashboard_{}", now),
            name,
            panels: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
