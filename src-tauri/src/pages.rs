use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[cfg(feature = "embedded-db")]
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub name: String,
    pub route: String,
    #[serde(rename = "type")]
    pub page_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    pub order: i32,
    pub visible: bool,
    pub created_at: String,
    pub updated_at: String,
}

// Helper command to clear all pages (for cleanup during development)
#[tauri::command]
pub async fn clear_pages_table(state: tauri::State<'_, AppState>) -> Result<String, String> {
    tracing::info!("Clearing pages table");

    let db = state.database.lock().await;

    // Delete all pages
    let _result = db
        .db
        .query("DELETE pages")
        .await
        .map_err(|e| format!("Failed to clear pages: {}", e))?;

    Ok("Pages table cleared successfully".to_string())
}

#[tauri::command]
pub async fn get_pages(state: tauri::State<'_, AppState>) -> Result<Vec<Page>, String> {
    tracing::info!("Getting all pages");

    let db = state.database.lock().await;
    let query = "SELECT * FROM pages ORDER BY order ASC";
    let mut result = db
        .db
        .query(query)
        .await
        .map_err(|e| format!("Failed to query pages: {}", e))?;

    let pages: Vec<Page> = result
        .take(0)
        .map_err(|e| format!("Failed to extract pages: {}", e))?;

    tracing::info!("Found {} pages", pages.len());

    Ok(pages)
}

#[tauri::command]
pub async fn create_page(
    mut page: Page,
    state: tauri::State<'_, AppState>,
) -> Result<Page, String> {
    tracing::info!("Creating page: {}", page.name);

    let db = state.database.lock().await;

    // Check if route already exists
    let check_query = format!("SELECT * FROM pages WHERE route = '{}'", page.route);
    let mut check_result = db
        .db
        .query(&check_query)
        .await
        .map_err(|e| format!("Failed to check route: {}", e))?;

    let existing: Vec<Page> = check_result.take(0).unwrap_or_default();
    if !existing.is_empty() {
        return Err(format!("Page with route '{}' already exists", page.route));
    }

    // Create page - let SurrealDB generate the Thing ID
    page.id = None; // Clear any provided ID
    let created: Option<Page> = db
        .db
        .create("pages")
        .content(page)
        .await
        .map_err(|e| format!("Failed to create page: {}", e))?;

    created.ok_or_else(|| "Failed to create page".to_string())
}

#[tauri::command]
pub async fn update_page(
    id: String,
    updates: serde_json::Value,
    state: tauri::State<'_, AppState>,
) -> Result<Page, String> {
    tracing::info!("Updating page: {}", id);

    let db = state.database.lock().await;

    // If route is being updated, check it doesn't conflict
    if let Some(new_route) = updates.get("route").and_then(|v| v.as_str()) {
        let check_query = format!(
            "SELECT * FROM pages WHERE route = '{}' AND id != '{}'",
            new_route, id
        );
        let mut check_result = db
            .db
            .query(&check_query)
            .await
            .map_err(|e| format!("Failed to check route: {}", e))?;

        let existing: Vec<Page> = check_result.take(0).unwrap_or_default();
        if !existing.is_empty() {
            return Err(format!("Page with route '{}' already exists", new_route));
        }
    }

    let updated: Option<Page> = db
        .db
        .update(("pages", id.as_str()))
        .merge(updates)
        .await
        .map_err(|e| format!("Failed to update page: {}", e))?;

    updated.ok_or_else(|| format!("Page with id '{}' not found", id))
}

#[tauri::command]
pub async fn delete_page(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Deleting page: {}", id);

    let db = state.database.lock().await;
    let _: Option<Page> = db
        .db
        .delete(("pages", id.as_str()))
        .await
        .map_err(|e| format!("Failed to delete page: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn reorder_pages(
    page_ids: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Reordering {} pages", page_ids.len());

    let db = state.database.lock().await;
    for (index, page_id) in page_ids.iter().enumerate() {
        let _: Option<Page> = db
            .db
            .update(("pages", page_id.as_str()))
            .merge(serde_json::json!({ "order": index as i32 }))
            .await
            .map_err(|e| format!("Failed to update page order: {}", e))?;
    }

    Ok(())
}
