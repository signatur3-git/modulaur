// =============================================================================
// Prompt Generator Backend Module
// =============================================================================
//
// This module provides the backend commands for the prompt-generator plugin.
// While the frontend is loaded as a UMD plugin, these commands remain in the
// core Tauri app for database access. They are only used by the prompt-generator
// plugin and can be conditionally compiled out if the plugin is not needed.
//
// Plugin: plugins/featured/prompt-generator/
// Frontend: Vue components loaded via pluginLoader
// Backend: These Tauri commands (invoked via @tauri-apps/api)
//
// =============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ============================================
// DATA STRUCTURES
// ============================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub namespace: String,
    #[serde(default)]
    pub additional_namespaces: Vec<String>,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// PromptTemplate - @deprecated, kept for backward compatibility
/// Use PromptSection with is_entry_point=true instead
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub package_id: String,
    pub namespace: String,
    pub name: String,
    pub description: String,
    pub content: serde_json::Value,
    pub variables: Vec<serde_json::Value>,
    pub tags: Vec<String>,
    pub examples: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

/// PromptSection - The unified content unit
/// Can be an entry point (is_entry_point=true) or a reusable fragment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub package_id: String,
    pub namespace: String,
    pub name: String,
    pub description: String,
    pub content: serde_json::Value,
    /// Whether this section is a top-level entry point for rendering
    #[serde(default)]
    pub is_entry_point: bool,
    pub exportable: bool,
    pub required_variables: Vec<String>,
    /// Variable definitions (only used when is_entry_point=true)
    #[serde(default)]
    pub variables: Vec<serde_json::Value>,
    /// Tags for categorization (only used when is_entry_point=true)
    #[serde(default)]
    pub tags: Vec<String>,
    /// Example renderings (only used when is_entry_point=true)
    #[serde(default)]
    pub examples: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeparatorSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub package_id: String,
    pub namespace: String,
    pub name: String,
    pub description: String,
    pub rules: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub package_id: String,
    pub namespace: String,
    pub name: String,
    pub description: String,
    pub base_type: String,
    pub validation: Option<serde_json::Value>,
    pub format: Option<serde_json::Value>,
    pub examples: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub package_id: String,
    pub namespace: String,
    pub name: String,
    pub description: String,
    pub color: Option<String>,
    pub parent: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageExport {
    pub format_version: String,
    pub exported_at: String,
    pub package: PromptPackage,
    /// @deprecated - kept for backward compatibility
    #[serde(default)]
    pub templates: Vec<PromptTemplate>,
    pub sections: Vec<PromptSection>,
    pub separator_sets: Vec<SeparatorSet>,
    pub data_types: Vec<PromptDataType>,
    pub tags: Vec<PromptTag>,
}

fn get_timestamp() -> String {
    Utc::now().to_rfc3339()
}

fn extract_id(thing: &Option<Thing>) -> Option<String> {
    thing.as_ref().map(|t| match &t.id {
        surrealdb::sql::Id::String(s) => s.clone(),
        surrealdb::sql::Id::Number(n) => n.to_string(),
        _ => format!("{:?}", t.id),
    })
}

// ============================================
// COMMANDS
// ============================================

pub mod commands {
    use super::*;
    use crate::AppState;

    #[tauri::command]
    pub async fn get_prompt_packages(
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<PromptPackage>, String> {
        let db = state.database.lock().await;
        let packages: Vec<PromptPackage> = db
            .db
            .select("prompt_packages")
            .await
            .map_err(|e| format!("Failed to get packages: {}", e))?;
        Ok(packages)
    }

    #[tauri::command]
    pub async fn get_prompt_package(
        id: String,
        state: tauri::State<'_, AppState>,
    ) -> Result<Option<PromptPackage>, String> {
        let db = state.database.lock().await;
        let package: Option<PromptPackage> = db
            .db
            .select(("prompt_packages", &id))
            .await
            .map_err(|e| format!("Failed to get package: {}", e))?;
        Ok(package)
    }

    #[tauri::command]
    pub async fn create_prompt_package(
        mut package: PromptPackage,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptPackage, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        package.created_at = timestamp.clone();
        package.updated_at = timestamp;
        package.id = None;

        let created: Option<PromptPackage> = db
            .db
            .create("prompt_packages")
            .content(package)
            .await
            .map_err(|e| format!("Failed to create package: {}", e))?;

        created.ok_or_else(|| "Failed to create package".to_string())
    }

    #[tauri::command]
    pub async fn update_prompt_package(
        id: String,
        mut package: PromptPackage,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptPackage, String> {
        let db = state.database.lock().await;
        package.updated_at = get_timestamp();

        let result: Option<PromptPackage> = db
            .db
            .update(("prompt_packages", &id))
            .content(package)
            .await
            .map_err(|e| format!("Failed to update package: {}", e))?;

        result.ok_or_else(|| "Package not found".to_string())
    }

    #[tauri::command]
    pub async fn delete_prompt_package(
        id: String,
        state: tauri::State<'_, AppState>,
    ) -> Result<(), String> {
        let db = state.database.lock().await;

        // Cascade delete all related data
        // Delete sections
        let _: Vec<PromptSection> = db
            .db
            .query("DELETE FROM prompt_sections WHERE package_id = $pkg_id")
            .bind(("pkg_id", id.clone()))
            .await
            .map_err(|e| format!("Failed to delete sections: {}", e))?
            .take(0)
            .unwrap_or_default();

        // Delete templates
        let _: Vec<PromptTemplate> = db
            .db
            .query("DELETE FROM prompt_templates WHERE package_id = $pkg_id")
            .bind(("pkg_id", id.clone()))
            .await
            .map_err(|e| format!("Failed to delete templates: {}", e))?
            .take(0)
            .unwrap_or_default();

        // Delete separator sets
        let _: Vec<SeparatorSet> = db
            .db
            .query("DELETE FROM prompt_separator_sets WHERE package_id = $pkg_id")
            .bind(("pkg_id", id.clone()))
            .await
            .map_err(|e| format!("Failed to delete separator sets: {}", e))?
            .take(0)
            .unwrap_or_default();

        // Delete data types
        let _: Vec<PromptDataType> = db
            .db
            .query("DELETE FROM prompt_data_types WHERE package_id = $pkg_id")
            .bind(("pkg_id", id.clone()))
            .await
            .map_err(|e| format!("Failed to delete data types: {}", e))?
            .take(0)
            .unwrap_or_default();

        // Delete tags
        let _: Vec<PromptTag> = db
            .db
            .query("DELETE FROM prompt_tags WHERE package_id = $pkg_id")
            .bind(("pkg_id", id.clone()))
            .await
            .map_err(|e| format!("Failed to delete tags: {}", e))?
            .take(0)
            .unwrap_or_default();

        // Finally delete the package itself
        let _: Option<PromptPackage> = db
            .db
            .delete(("prompt_packages", &id))
            .await
            .map_err(|e| format!("Failed to delete package: {}", e))?;
        Ok(())
    }

    #[tauri::command]
    pub async fn get_prompt_templates(
        package_id: Option<String>,
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<PromptTemplate>, String> {
        let db = state.database.lock().await;

        let templates: Vec<PromptTemplate> = if let Some(pkg_id) = package_id {
            let mut result = db
                .db
                .query("SELECT * FROM prompt_templates WHERE package_id = $package_id")
                .bind(("package_id", pkg_id))
                .await
                .map_err(|e| format!("Failed to query templates: {}", e))?;
            result
                .take(0)
                .map_err(|e| format!("Failed to extract templates: {}", e))?
        } else {
            db.db
                .select("prompt_templates")
                .await
                .map_err(|e| format!("Failed to get templates: {}", e))?
        };

        Ok(templates)
    }

    #[tauri::command]
    pub async fn create_prompt_template(
        mut template: PromptTemplate,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptTemplate, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        template.created_at = timestamp.clone();
        template.updated_at = timestamp;
        template.id = None;

        let created: Option<PromptTemplate> = db
            .db
            .create("prompt_templates")
            .content(template)
            .await
            .map_err(|e| format!("Failed to create template: {}", e))?;

        created.ok_or_else(|| "Failed to create template".to_string())
    }

    #[tauri::command]
    pub async fn update_prompt_template(
        id: String,
        mut template: PromptTemplate,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptTemplate, String> {
        let db = state.database.lock().await;
        template.updated_at = get_timestamp();

        let result: Option<PromptTemplate> = db
            .db
            .update(("prompt_templates", &id))
            .content(template)
            .await
            .map_err(|e| format!("Failed to update template: {}", e))?;

        result.ok_or_else(|| "Template not found".to_string())
    }

    #[tauri::command]
    pub async fn delete_prompt_template(
        id: String,
        state: tauri::State<'_, AppState>,
    ) -> Result<(), String> {
        let db = state.database.lock().await;
        let _: Option<PromptTemplate> = db
            .db
            .delete(("prompt_templates", &id))
            .await
            .map_err(|e| format!("Failed to delete template: {}", e))?;
        Ok(())
    }

    #[tauri::command]
    pub async fn get_prompt_sections(
        package_id: Option<String>,
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<PromptSection>, String> {
        let db = state.database.lock().await;

        let sections: Vec<PromptSection> = if let Some(pkg_id) = package_id {
            let mut result = db
                .db
                .query("SELECT * FROM prompt_sections WHERE package_id = $package_id")
                .bind(("package_id", pkg_id))
                .await
                .map_err(|e| format!("Failed to query sections: {}", e))?;
            result
                .take(0)
                .map_err(|e| format!("Failed to extract sections: {}", e))?
        } else {
            db.db
                .select("prompt_sections")
                .await
                .map_err(|e| format!("Failed to get sections: {}", e))?
        };

        Ok(sections)
    }

    #[tauri::command]
    pub async fn create_prompt_section(
        mut section: PromptSection,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptSection, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        section.created_at = timestamp.clone();
        section.updated_at = timestamp;
        section.id = None;

        let created: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(section)
            .await
            .map_err(|e| format!("Failed to create section: {}", e))?;

        created.ok_or_else(|| "Failed to create section".to_string())
    }

    #[tauri::command]
    pub async fn update_prompt_section(
        id: String,
        mut section: PromptSection,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptSection, String> {
        let db = state.database.lock().await;
        section.updated_at = get_timestamp();

        let result: Option<PromptSection> = db
            .db
            .update(("prompt_sections", &id))
            .content(section)
            .await
            .map_err(|e| format!("Failed to update section: {}", e))?;

        result.ok_or_else(|| "Section not found".to_string())
    }

    #[tauri::command]
    pub async fn delete_prompt_section(
        id: String,
        state: tauri::State<'_, AppState>,
    ) -> Result<(), String> {
        let db = state.database.lock().await;
        let _: Option<PromptSection> = db
            .db
            .delete(("prompt_sections", &id))
            .await
            .map_err(|e| format!("Failed to delete section: {}", e))?;
        Ok(())
    }

    #[tauri::command]
    pub async fn get_separator_sets(
        package_id: Option<String>,
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<SeparatorSet>, String> {
        let db = state.database.lock().await;

        let sets: Vec<SeparatorSet> = if let Some(pkg_id) = package_id {
            let mut result = db
                .db
                .query("SELECT * FROM prompt_separator_sets WHERE package_id = $package_id")
                .bind(("package_id", pkg_id))
                .await
                .map_err(|e| format!("Failed to query separator sets: {}", e))?;
            result
                .take(0)
                .map_err(|e| format!("Failed to extract separator sets: {}", e))?
        } else {
            db.db
                .select("prompt_separator_sets")
                .await
                .map_err(|e| format!("Failed to get separator sets: {}", e))?
        };

        Ok(sets)
    }

    #[tauri::command]
    pub async fn create_separator_set(
        mut separator_set: SeparatorSet,
        state: tauri::State<'_, AppState>,
    ) -> Result<SeparatorSet, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        separator_set.created_at = timestamp.clone();
        separator_set.updated_at = timestamp;
        separator_set.id = None;

        let created: Option<SeparatorSet> = db
            .db
            .create("prompt_separator_sets")
            .content(separator_set)
            .await
            .map_err(|e| format!("Failed to create separator set: {}", e))?;

        created.ok_or_else(|| "Failed to create separator set".to_string())
    }

    #[tauri::command]
    pub async fn get_prompt_data_types(
        package_id: Option<String>,
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<PromptDataType>, String> {
        let db = state.database.lock().await;

        let types: Vec<PromptDataType> = if let Some(pkg_id) = package_id {
            let mut result = db
                .db
                .query("SELECT * FROM prompt_data_types WHERE package_id = $package_id")
                .bind(("package_id", pkg_id))
                .await
                .map_err(|e| format!("Failed to query data types: {}", e))?;
            result
                .take(0)
                .map_err(|e| format!("Failed to extract data types: {}", e))?
        } else {
            db.db
                .select("prompt_data_types")
                .await
                .map_err(|e| format!("Failed to get data types: {}", e))?
        };

        Ok(types)
    }

    #[tauri::command]
    pub async fn create_prompt_data_type(
        mut data_type: PromptDataType,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptDataType, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        data_type.created_at = timestamp.clone();
        data_type.updated_at = timestamp;
        data_type.id = None;

        let created: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(data_type)
            .await
            .map_err(|e| format!("Failed to create data type: {}", e))?;

        created.ok_or_else(|| "Failed to create data type".to_string())
    }

    #[tauri::command]
    pub async fn get_prompt_tags(
        package_id: Option<String>,
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<PromptTag>, String> {
        let db = state.database.lock().await;

        let tags: Vec<PromptTag> = if let Some(pkg_id) = package_id {
            let mut result = db
                .db
                .query("SELECT * FROM prompt_tags WHERE package_id = $package_id")
                .bind(("package_id", pkg_id))
                .await
                .map_err(|e| format!("Failed to query tags: {}", e))?;
            result
                .take(0)
                .map_err(|e| format!("Failed to extract tags: {}", e))?
        } else {
            db.db
                .select("prompt_tags")
                .await
                .map_err(|e| format!("Failed to get tags: {}", e))?
        };

        Ok(tags)
    }

    #[tauri::command]
    pub async fn create_prompt_tag(
        mut tag: PromptTag,
        state: tauri::State<'_, AppState>,
    ) -> Result<PromptTag, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();
        tag.created_at = timestamp.clone();
        tag.updated_at = timestamp;
        tag.id = None;

        let created: Option<PromptTag> = db
            .db
            .create("prompt_tags")
            .content(tag)
            .await
            .map_err(|e| format!("Failed to create tag: {}", e))?;

        created.ok_or_else(|| "Failed to create tag".to_string())
    }

    #[tauri::command]
    pub async fn export_prompt_package(
        package_id: String,
        state: tauri::State<'_, AppState>,
    ) -> Result<PackageExport, String> {
        let db = state.database.lock().await;

        let package: PromptPackage = db
            .db
            .select(("prompt_packages", &package_id))
            .await
            .map_err(|e| format!("Failed to get package: {}", e))?
            .ok_or("Package not found")?;

        let mut result = db
            .db
            .query("SELECT * FROM prompt_templates WHERE package_id = $id")
            .bind(("id", package_id.clone()))
            .await
            .map_err(|e| format!("Failed to get templates: {}", e))?;
        let templates: Vec<PromptTemplate> = result.take(0).unwrap_or_default();

        let mut result = db
            .db
            .query("SELECT * FROM prompt_sections WHERE package_id = $id")
            .bind(("id", package_id.clone()))
            .await
            .map_err(|e| format!("Failed to get sections: {}", e))?;
        let sections: Vec<PromptSection> = result.take(0).unwrap_or_default();

        let mut result = db
            .db
            .query("SELECT * FROM prompt_separator_sets WHERE package_id = $id")
            .bind(("id", package_id.clone()))
            .await
            .map_err(|e| format!("Failed to get separator sets: {}", e))?;
        let separator_sets: Vec<SeparatorSet> = result.take(0).unwrap_or_default();

        let mut result = db
            .db
            .query("SELECT * FROM prompt_data_types WHERE package_id = $id")
            .bind(("id", package_id.clone()))
            .await
            .map_err(|e| format!("Failed to get data types: {}", e))?;
        let data_types: Vec<PromptDataType> = result.take(0).unwrap_or_default();

        let mut result = db
            .db
            .query("SELECT * FROM prompt_tags WHERE package_id = $id")
            .bind(("id", package_id.clone()))
            .await
            .map_err(|e| format!("Failed to get tags: {}", e))?;
        let tags: Vec<PromptTag> = result.take(0).unwrap_or_default();

        Ok(PackageExport {
            format_version: "1.0.0".to_string(),
            exported_at: get_timestamp(),
            package,
            templates,
            sections,
            separator_sets,
            data_types,
            tags,
        })
    }

    #[tauri::command]
    pub async fn import_prompt_package(
        export_data: PackageExport,
        state: tauri::State<'_, AppState>,
    ) -> Result<String, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();

        let mut package = export_data.package;
        package.created_at = timestamp.clone();
        package.updated_at = timestamp.clone();
        package.id = None;

        let created_package: Option<PromptPackage> = db
            .db
            .create("prompt_packages")
            .content(package)
            .await
            .map_err(|e| format!("Failed to import package: {}", e))?;

        let pkg = created_package.ok_or("Failed to import package")?;
        let package_id = extract_id(&pkg.id).ok_or("Failed to get created package ID")?;

        for mut template in export_data.templates {
            template.id = None;
            template.package_id = package_id.clone();
            template.created_at = timestamp.clone();
            template.updated_at = timestamp.clone();

            let _: Option<PromptTemplate> = db
                .db
                .create("prompt_templates")
                .content(template)
                .await
                .map_err(|e| format!("Failed to import template: {}", e))?;
        }

        for mut section in export_data.sections {
            section.id = None;
            section.package_id = package_id.clone();
            section.created_at = timestamp.clone();
            section.updated_at = timestamp.clone();

            let _: Option<PromptSection> =
                db.db
                    .create("prompt_sections")
                    .content(section)
                    .await
                    .map_err(|e| format!("Failed to import section: {}", e))?;
        }

        for mut set in export_data.separator_sets {
            set.id = None;
            set.package_id = package_id.clone();
            set.created_at = timestamp.clone();
            set.updated_at = timestamp.clone();

            let _: Option<SeparatorSet> = db
                .db
                .create("prompt_separator_sets")
                .content(set)
                .await
                .map_err(|e| format!("Failed to import separator set: {}", e))?;
        }

        for mut dt in export_data.data_types {
            dt.id = None;
            dt.package_id = package_id.clone();
            dt.created_at = timestamp.clone();
            dt.updated_at = timestamp.clone();

            let _: Option<PromptDataType> = db
                .db
                .create("prompt_data_types")
                .content(dt)
                .await
                .map_err(|e| format!("Failed to import data type: {}", e))?;
        }

        for mut tag in export_data.tags {
            tag.id = None;
            tag.package_id = package_id.clone();
            tag.created_at = timestamp.clone();
            tag.updated_at = timestamp.clone();

            let _: Option<PromptTag> = db
                .db
                .create("prompt_tags")
                .content(tag)
                .await
                .map_err(|e| format!("Failed to import tag: {}", e))?;
        }

        Ok(package_id)
    }

    /// Seed the database with example packages for demonstration
    /// If examples already exist, they will be deleted and recreated
    #[tauri::command]
    pub async fn seed_example_packages(
        state: tauri::State<'_, AppState>,
    ) -> Result<String, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();

        // Check if examples already exist and delete them
        let existing: Vec<PromptPackage> = db
            .db
            .query("SELECT * FROM prompt_packages WHERE namespace = 'examples'")
            .await
            .map_err(|e| format!("Failed to check existing: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to extract: {}", e))?;

        if !existing.is_empty() {
            // Delete all related data for existing example packages
            for pkg in &existing {
                if let Some(ref id) = pkg.id {
                    let pkg_id = match &id.id {
                        surrealdb::sql::Id::String(s) => s.clone(),
                        surrealdb::sql::Id::Number(n) => n.to_string(),
                        _ => format!("{:?}", id.id),
                    };

                    // Delete sections
                    let _: Vec<PromptSection> = db
                        .db
                        .query("DELETE FROM prompt_sections WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete sections: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete templates
                    let _: Vec<PromptTemplate> = db
                        .db
                        .query("DELETE FROM prompt_templates WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete templates: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete separator sets
                    let _: Vec<SeparatorSet> = db
                        .db
                        .query("DELETE FROM prompt_separator_sets WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete separator sets: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete data types
                    let _: Vec<PromptDataType> = db
                        .db
                        .query("DELETE FROM prompt_data_types WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete data types: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete tags
                    let _: Vec<PromptTag> = db
                        .db
                        .query("DELETE FROM prompt_tags WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete tags: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete the package itself
                    let _: Option<PromptPackage> = db
                        .db
                        .delete(("prompt_packages", pkg_id.as_str()))
                        .await
                        .map_err(|e| format!("Failed to delete package: {}", e))?;
                }
            }
        }

        // Create the examples package
        let package = PromptPackage {
            id: None,
            namespace: "examples".to_string(),
            additional_namespaces: vec!["examples-internal".to_string()],
            name: "Example Prompts".to_string(),
            version: "1.0.0".to_string(),
            description: "A collection of example prompts demonstrating various features"
                .to_string(),
            author: "System".to_string(),
            dependencies: vec![],
            exports: vec![
                "greeting".to_string(),
                "character-description".to_string(),
                "code-review".to_string(),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let created_package: Option<PromptPackage> = db
            .db
            .create("prompt_packages")
            .content(package)
            .await
            .map_err(|e| format!("Failed to create package: {}", e))?;

        let pkg = created_package.ok_or("Failed to create package")?;
        let package_id = extract_id(&pkg.id).ok_or("Failed to get package ID")?;

        // ============================================
        // SIMPLE ENTRY POINT: Greeting
        // ============================================
        let greeting_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Simple Greeting".to_string(),
            description: "A simple greeting that demonstrates list joining with Oxford comma"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Hello, " },
                    { "type": "list", "variable_id": "names", "separator_set_id": "oxford-comma" },
                    { "type": "text", "value": "! Welcome to our " },
                    { "type": "variable", "variable_id": "event_type" },
                    { "type": "text", "value": "." }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["names".to_string(), "event_type".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "names",
                    "name": "Names",
                    "description": "List of people to greet",
                    "type": "array",
                    "item_type": "string",
                    "required": true,
                    "min_items": 1
                }),
                serde_json::json!({
                    "id": "event_type",
                    "name": "Event Type",
                    "description": "Type of event",
                    "type": "string",
                    "required": true,
                    "default_value": "meeting"
                }),
            ],
            tags: vec!["simple".to_string(), "greeting".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Single person",
                    "variables": { "names": ["Alice"], "event_type": "meeting" },
                    "expected_output": "Hello, Alice! Welcome to our meeting."
                }),
                serde_json::json!({
                    "name": "Two people",
                    "variables": { "names": ["Alice", "Bob"], "event_type": "workshop" },
                    "expected_output": "Hello, Alice and Bob! Welcome to our workshop."
                }),
                serde_json::json!({
                    "name": "Three people",
                    "variables": { "names": ["Alice", "Bob", "Charlie"], "event_type": "conference" },
                    "expected_output": "Hello, Alice, Bob, and Charlie! Welcome to our conference."
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(greeting_section)
            .await
            .map_err(|e| format!("Failed to create greeting section: {}", e))?;

        // ============================================
        // MEDIUM ENTRY POINT: Character Description
        // ============================================
        let character_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Character Description".to_string(),
            description: "Generate a character description with conditional occupation and setting"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Create a detailed character description for " },
                    { "type": "variable", "variable_id": "name" },
                    {
                        "type": "conditional",
                        "condition": { "variable": "occupation", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", a " },
                                { "type": "variable", "variable_id": "occupation" }
                            ]
                        }
                    },
                    { "type": "text", "value": ". They should have the following traits: " },
                    { "type": "list", "variable_id": "traits", "separator_set_id": "oxford-comma" },
                    { "type": "text", "value": "." },
                    {
                        "type": "conditional",
                        "condition": { "variable": "setting", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": " The setting is " },
                                { "type": "variable", "variable_id": "setting", "format": { "case": "lower" } },
                                { "type": "text", "value": "." }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["name".to_string(), "traits".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "name",
                    "name": "Character Name",
                    "description": "The name of the character",
                    "type": "string",
                    "required": true
                }),
                serde_json::json!({
                    "id": "occupation",
                    "name": "Occupation",
                    "description": "The character's job or role (optional)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "traits",
                    "name": "Character Traits",
                    "description": "Personality traits for the character",
                    "type": "array",
                    "item_type": "string",
                    "required": true,
                    "min_items": 1,
                    "max_items": 5
                }),
                serde_json::json!({
                    "id": "setting",
                    "name": "Setting",
                    "description": "The world/genre setting (optional)",
                    "type": "enum",
                    "enum_values": ["Fantasy", "Sci-Fi", "Modern", "Historical"],
                    "required": false
                }),
            ],
            tags: vec![
                "medium".to_string(),
                "creative".to_string(),
                "character".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Simple character",
                    "variables": {
                        "name": "Aria",
                        "traits": ["brave", "curious"]
                    },
                    "expected_output": "Create a detailed character description for Aria. They should have the following traits: brave and curious."
                }),
                serde_json::json!({
                    "name": "Full character",
                    "variables": {
                        "name": "Aria",
                        "occupation": "blacksmith",
                        "traits": ["brave", "curious", "stubborn"],
                        "setting": "Fantasy"
                    },
                    "expected_output": "Create a detailed character description for Aria, a blacksmith. They should have the following traits: brave, curious, and stubborn. The setting is fantasy."
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(character_section)
            .await
            .map_err(|e| format!("Failed to create character section: {}", e))?;

        // ============================================
        // FRAGMENT: Review Guidelines (reusable)
        // ============================================
        let guidelines_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples-internal".to_string(),
            name: "review-guidelines".to_string(),
            description: "Standard code review guidelines (reusable fragment)".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "\n\nReview Guidelines:\n" },
                    { "type": "text", "value": "• Check for clear variable naming\n" },
                    { "type": "text", "value": "• Verify error handling is comprehensive\n" },
                    { "type": "text", "value": "• Look for potential performance issues\n" },
                    { "type": "text", "value": "• Ensure code follows project conventions" }
                ]
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(guidelines_fragment)
            .await
            .map_err(|e| format!("Failed to create guidelines fragment: {}", e))?;

        // ============================================
        // COMPLEX ENTRY POINT: Code Review
        // ============================================
        let code_review_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Code Review Request".to_string(),
            description: "A comprehensive code review prompt with focus areas, context, and reusable guidelines".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Please review the following " },
                    { "type": "variable", "variable_id": "language" },
                    { "type": "text", "value": " code" },
                    {
                        "type": "conditional",
                        "condition": { "variable": "focus_areas", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", focusing on " },
                                { "type": "list", "variable_id": "focus_areas", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    },
                    { "type": "text", "value": "." },
                    {
                        "type": "conditional",
                        "condition": { "variable": "context", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nContext: " },
                                { "type": "variable", "variable_id": "context" }
                            ]
                        }
                    },
                    { "type": "section-ref", "section_id": "examples-internal:review-guidelines" },
                    { "type": "text", "value": "\n\nReview depth: " },
                    { "type": "variable", "variable_id": "depth", "format": { "case": "title" } },
                    {
                        "type": "conditional",
                        "condition": { "variable": "specific_concerns", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nPlease pay special attention to:\n" },
                                { "type": "list", "variable_id": "specific_concerns", "separator_set_id": "bullet-list" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["language".to_string(), "depth".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "language",
                    "name": "Programming Language",
                    "description": "The language of the code being reviewed",
                    "type": "string",
                    "required": true,
                    "default_value": "TypeScript"
                }),
                serde_json::json!({
                    "id": "focus_areas",
                    "name": "Focus Areas",
                    "description": "Specific areas to focus the review on",
                    "type": "array",
                    "item_type": "enum",
                    "enum_values": ["performance", "security", "readability", "testing", "architecture"],
                    "required": false
                }),
                serde_json::json!({
                    "id": "context",
                    "name": "Context",
                    "description": "Additional context about the code",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "depth",
                    "name": "Review Depth",
                    "description": "How thorough the review should be",
                    "type": "enum",
                    "enum_values": ["quick-check", "thorough", "deep-dive"],
                    "required": true,
                    "default_value": "thorough"
                }),
                serde_json::json!({
                    "id": "specific_concerns",
                    "name": "Specific Concerns",
                    "description": "Specific issues or areas of concern",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                })
            ],
            tags: vec!["complex".to_string(), "code".to_string(), "review".to_string(), "development".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Simple review",
                    "variables": {
                        "language": "Python",
                        "depth": "quick-check"
                    },
                    "expected_output": "Please review the following Python code.\n\nReview Guidelines:\n• Check for clear variable naming\n• Verify error handling is comprehensive\n• Look for potential performance issues\n• Ensure code follows project conventions\n\nReview depth: Quick-Check"
                }),
                serde_json::json!({
                    "name": "Detailed review",
                    "variables": {
                        "language": "Rust",
                        "focus_areas": ["performance", "security"],
                        "context": "This is a hot path in our authentication system",
                        "depth": "deep-dive",
                        "specific_concerns": ["Memory allocation patterns", "Error handling edge cases"]
                    },
                    "expected_output": "Please review the following Rust code, focusing on performance and security.\n\nContext: This is a hot path in our authentication system\n\nReview Guidelines:\n• Check for clear variable naming\n• Verify error handling is comprehensive\n• Look for potential performance issues\n• Ensure code follows project conventions\n\nReview depth: Deep-Dive\n\nPlease pay special attention to:\n• Memory allocation patterns\n• Error handling edge cases"
                })
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(code_review_section)
            .await
            .map_err(|e| format!("Failed to create code review section: {}", e))?;

        // ============================================
        // LONG ENTRY POINT: AI Agent System Prompt
        // ============================================
        let agent_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "AI Agent System Prompt".to_string(),
            description: "A comprehensive AI agent system prompt with role, capabilities, constraints, and examples".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "You are " },
                    { "type": "variable", "variable_id": "role_article", "format": { "placeholder": "a" } },
                    { "type": "text", "value": " " },
                    { "type": "variable", "variable_id": "role_name" },
                    {
                        "type": "conditional",
                        "condition": { "variable": "expertise_areas", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": " with expertise in " },
                                { "type": "list", "variable_id": "expertise_areas", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    },
                    { "type": "text", "value": "." },
                    {
                        "type": "conditional",
                        "condition": { "variable": "capabilities", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nYou can:\n" },
                                { "type": "list", "variable_id": "capabilities", "separator_set_id": "bullet-list" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "constraints", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nImportant constraints:\n" },
                                { "type": "list", "variable_id": "constraints", "separator_set_id": "numbered-list" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "communication_style", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nCommunication style: " },
                                { "type": "variable", "variable_id": "communication_style", "format": { "case": "sentence" } },
                                { "type": "text", "value": "." }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "example_interactions", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nExample interactions:\n" },
                                { "type": "list", "variable_id": "example_interactions", "separator_set_id": "numbered-list" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "additional_instructions", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\n\nAdditional instructions:\n" },
                                { "type": "variable", "variable_id": "additional_instructions" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["role_name".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "role_article",
                    "name": "Article",
                    "description": "Article before role (a/an)",
                    "type": "enum",
                    "enum_values": ["a", "an"],
                    "required": false,
                    "default_value": "a"
                }),
                serde_json::json!({
                    "id": "role_name",
                    "name": "Role Name",
                    "description": "The role/persona of the AI agent",
                    "type": "string",
                    "required": true,
                    "default_value": "helpful assistant"
                }),
                serde_json::json!({
                    "id": "expertise_areas",
                    "name": "Expertise Areas",
                    "description": "Areas of expertise",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "capabilities",
                    "name": "Capabilities",
                    "description": "What the agent can do",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "constraints",
                    "name": "Constraints",
                    "description": "Rules the agent must follow",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "communication_style",
                    "name": "Communication Style",
                    "description": "How the agent should communicate",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "example_interactions",
                    "name": "Example Interactions",
                    "description": "Example Q&A or interactions",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "additional_instructions",
                    "name": "Additional Instructions",
                    "description": "Any additional custom instructions",
                    "type": "string",
                    "required": false
                })
            ],
            tags: vec!["complex".to_string(), "long".to_string(), "agent".to_string(), "system-prompt".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Simple agent",
                    "variables": {
                        "role_name": "technical writer"
                    },
                    "expected_output": "You are a technical writer."
                }),
                serde_json::json!({
                    "name": "Full agent",
                    "variables": {
                        "role_article": "a",
                        "role_name": "technical writer",
                        "expertise_areas": ["documentation", "API design", "developer experience"],
                        "capabilities": [
                            "Write clear technical documentation",
                            "Create API reference guides",
                            "Review and improve existing docs"
                        ],
                        "constraints": [
                            "Keep explanations concise",
                            "Use code examples when helpful",
                            "Avoid jargon without explanation"
                        ],
                        "communication_style": "professional but friendly",
                        "example_interactions": [
                            "User: How do I document a REST API? → Explain OpenAPI/Swagger, provide examples",
                            "User: This paragraph is confusing → Rewrite for clarity, explain changes"
                        ],
                        "additional_instructions": "When reviewing documentation, always suggest at least one improvement even if the content is good."
                    },
                    "expected_output": "You are a technical writer with expertise in documentation, API design, and developer experience.\n\nYou can:\n• Write clear technical documentation\n• Create API reference guides\n• Review and improve existing docs\n\nImportant constraints:\n1. Keep explanations concise\n2. Use code examples when helpful\n3. Avoid jargon without explanation\n\nCommunication style: Professional but friendly.\n\nExample interactions:\n1. User: How do I document a REST API? → Explain OpenAPI/Swagger, provide examples\n2. User: This paragraph is confusing → Rewrite for clarity, explain changes\n\nAdditional instructions:\nWhen reviewing documentation, always suggest at least one improvement even if the content is good."
                })
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(agent_section)
            .await
            .map_err(|e| format!("Failed to create agent section: {}", e))?;

        // ============================================
        // PLURALIZATION EXAMPLE: Task Summary
        // ============================================
        let task_summary_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Task Summary with Pluralization".to_string(),
            description: "Demonstrates pluralization, count-based switches, and natural language"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "You have " },
                    {
                        "type": "plural",
                        "count_variable": "tasks",
                        "zero": "no tasks",
                        "one": "1 task",
                        "two": "2 tasks",
                        "other": "{count} tasks"
                    },
                    { "type": "text", "value": " to complete" },
                    {
                        "type": "conditional",
                        "condition": { "variable": "tasks", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ": " },
                                { "type": "list", "variable_id": "tasks", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    },
                    { "type": "text", "value": ". " },
                    {
                        "type": "count-switch",
                        "count_variable": "tasks",
                        "cases": [
                            {
                                "count": "zero",
                                "content": { "type": "text", "value": "Great job staying on top of things!" }
                            },
                            {
                                "count": "one",
                                "content": { "type": "text", "value": "Almost done!" }
                            },
                            {
                                "count": "other",
                                "content": { "type": "text", "value": "Let's get started!" }
                            }
                        ]
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["tasks".to_string()],
            variables: vec![serde_json::json!({
                "id": "tasks",
                "name": "Tasks",
                "description": "List of tasks to complete",
                "type": "array",
                "item_type": "string",
                "required": true
            })],
            tags: vec!["pluralization".to_string(), "count-switch".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "No tasks",
                    "variables": { "tasks": [] },
                    "expected_output": "You have no tasks to complete. Great job staying on top of things!"
                }),
                serde_json::json!({
                    "name": "One task",
                    "variables": { "tasks": ["Review PR #123"] },
                    "expected_output": "You have 1 task to complete: Review PR #123. Almost done!"
                }),
                serde_json::json!({
                    "name": "Multiple tasks",
                    "variables": { "tasks": ["Review PR", "Update docs", "Deploy to staging"] },
                    "expected_output": "You have 3 tasks to complete: Review PR, Update docs, and Deploy to staging. Let's get started!"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(task_summary_section)
            .await
            .map_err(|e| format!("Failed to create task summary section: {}", e))?;

        // ============================================
        // ARTICLE SELECTION EXAMPLE: Item Description
        // ============================================
        let article_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Article Selection (a/an)".to_string(),
            description: "Demonstrates automatic a/an article selection based on following word"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "You found " },
                    {
                        "type": "article",
                        "word_variable": "item_type",
                        "style": "indefinite",
                        "capitalize": false
                    },
                    { "type": "text", "value": " " },
                    { "type": "variable", "variable_id": "item_type" },
                    { "type": "text", "value": "! " },
                    {
                        "type": "conditional",
                        "condition": { "variable": "item_rarity", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "It's " },
                                {
                                    "type": "article",
                                    "word_variable": "item_rarity",
                                    "style": "indefinite",
                                    "capitalize": false
                                },
                                { "type": "text", "value": " " },
                                { "type": "variable", "variable_id": "item_rarity" },
                                { "type": "text", "value": " item." }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["item_type".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "item_type",
                    "name": "Item Type",
                    "description": "The type of item found",
                    "type": "string",
                    "required": true
                }),
                serde_json::json!({
                    "id": "item_rarity",
                    "name": "Item Rarity",
                    "description": "The rarity level (optional)",
                    "type": "enum",
                    "enum_values": ["common", "uncommon", "rare", "epic", "legendary", "unique"],
                    "required": false
                }),
            ],
            tags: vec!["article".to_string(), "a-an".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Apple (vowel)",
                    "variables": { "item_type": "apple" },
                    "expected_output": "You found an apple!"
                }),
                serde_json::json!({
                    "name": "Sword (consonant)",
                    "variables": { "item_type": "sword", "item_rarity": "rare" },
                    "expected_output": "You found a sword! It's a rare item."
                }),
                serde_json::json!({
                    "name": "Umbrella (vowel)",
                    "variables": { "item_type": "umbrella", "item_rarity": "uncommon" },
                    "expected_output": "You found an umbrella! It's an uncommon item."
                }),
                serde_json::json!({
                    "name": "Unique item (special case - 'u' sounds like 'y')",
                    "variables": { "item_type": "unicorn", "item_rarity": "unique" },
                    "expected_output": "You found a unicorn! It's a unique item."
                }),
                serde_json::json!({
                    "name": "Hour (silent h)",
                    "variables": { "item_type": "hour glass", "item_rarity": "epic" },
                    "expected_output": "You found an hour glass! It's an epic item."
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(article_section)
            .await
            .map_err(|e| format!("Failed to create article section: {}", e))?;

        // ============================================
        // SWITCH EXAMPLE: Greeting by Time of Day
        // ============================================
        let greeting_switch_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Time-Based Greeting (Switch)".to_string(),
            description: "Demonstrates switch/case for value-based content selection".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "switch",
                        "variable_id": "time_of_day",
                        "cases": [
                            {
                                "value": "morning",
                                "content": { "type": "text", "value": "Good morning" }
                            },
                            {
                                "value": "afternoon",
                                "content": { "type": "text", "value": "Good afternoon" }
                            },
                            {
                                "value": "evening",
                                "content": { "type": "text", "value": "Good evening" }
                            },
                            {
                                "value": "night",
                                "content": { "type": "text", "value": "Good night" }
                            }
                        ],
                        "default_content": { "type": "text", "value": "Hello" }
                    },
                    { "type": "text", "value": ", " },
                    { "type": "variable", "variable_id": "name" },
                    { "type": "text", "value": "! " },
                    {
                        "type": "switch",
                        "variable_id": "time_of_day",
                        "cases": [
                            {
                                "value": "morning",
                                "content": { "type": "text", "value": "Ready to start the day?" }
                            },
                            {
                                "value": "afternoon",
                                "content": { "type": "text", "value": "Hope your day is going well." }
                            },
                            {
                                "value": "evening",
                                "content": { "type": "text", "value": "Wrapping up for the day?" }
                            },
                            {
                                "value": "night",
                                "content": { "type": "text", "value": "Sleep well!" }
                            }
                        ],
                        "default_content": { "type": "text", "value": "How can I help you?" }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["name".to_string(), "time_of_day".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "name",
                    "name": "Name",
                    "description": "Person's name",
                    "type": "string",
                    "required": true
                }),
                serde_json::json!({
                    "id": "time_of_day",
                    "name": "Time of Day",
                    "description": "Current time period",
                    "type": "enum",
                    "enum_values": ["morning", "afternoon", "evening", "night"],
                    "required": true,
                    "default_value": "morning"
                }),
            ],
            tags: vec!["switch".to_string(), "greeting".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Morning greeting",
                    "variables": { "name": "Alice", "time_of_day": "morning" },
                    "expected_output": "Good morning, Alice! Ready to start the day?"
                }),
                serde_json::json!({
                    "name": "Evening greeting",
                    "variables": { "name": "Bob", "time_of_day": "evening" },
                    "expected_output": "Good evening, Bob! Wrapping up for the day?"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(greeting_switch_section)
            .await
            .map_err(|e| format!("Failed to create greeting switch section: {}", e))?;

        // ============================================
        // FRAGMENT: Error Message Builder (reusable)
        // ============================================
        let error_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples-internal".to_string(),
            name: "error-message".to_string(),
            description: "Reusable error message fragment with severity".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "switch",
                        "variable_id": "severity",
                        "cases": [
                            { "value": "info", "content": { "type": "text", "value": "ℹ️ Info: " } },
                            { "value": "warning", "content": { "type": "text", "value": "⚠️ Warning: " } },
                            { "value": "error", "content": { "type": "text", "value": "❌ Error: " } },
                            { "value": "critical", "content": { "type": "text", "value": "🚨 CRITICAL: " } }
                        ],
                        "default_content": { "type": "text", "value": "Note: " }
                    },
                    { "type": "variable", "variable_id": "message" }
                ]
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec!["severity".to_string(), "message".to_string()],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(error_fragment)
            .await
            .map_err(|e| format!("Failed to create error fragment: {}", e))?;

        // ============================================
        // COMPLEX: Notification with Nested Sections
        // ============================================
        let notification_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Smart Notification".to_string(),
            description: "Complex notification with pluralization, section refs, and conditionals"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "📬 Notification Summary for " },
                    { "type": "variable", "variable_id": "user_name" },
                    { "type": "text", "value": "\n\n" },
                    // Messages section with pluralization
                    { "type": "text", "value": "Messages: " },
                    {
                        "type": "plural",
                        "count_variable": "messages",
                        "zero": "No new messages",
                        "one": "1 new message",
                        "other": "{count} new messages"
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "messages", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": " from " },
                                { "type": "list", "variable_id": "messages", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    },
                    { "type": "text", "value": "\n" },
                    // Alerts section with severity
                    {
                        "type": "conditional",
                        "condition": { "variable": "alerts", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": "\nAlerts:\n" },
                                {
                                    "type": "list",
                                    "variable_id": "alerts",
                                    "separator_set_id": "newline",
                                    "item_template": {
                                        "type": "section-ref",
                                        "section_id": "examples-internal:error-message"
                                    }
                                }
                            ]
                        }
                    },
                    // Status based on total items
                    { "type": "text", "value": "\n\nStatus: " },
                    {
                        "type": "count-switch",
                        "count_variable": "alerts",
                        "cases": [
                            { "count": "zero", "content": { "type": "text", "value": "✅ All clear!" } },
                            { "count": "one", "content": { "type": "text", "value": "⚠️ 1 item needs attention" } },
                            { "count": "other", "content": { "type": "text", "value": "🔴 Multiple items need attention" } }
                        ]
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["user_name".to_string()],
            variables: vec![
                serde_json::json!({
                    "id": "user_name",
                    "name": "User Name",
                    "description": "The user's name",
                    "type": "string",
                    "required": true
                }),
                serde_json::json!({
                    "id": "messages",
                    "name": "Messages",
                    "description": "List of message senders",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "alerts",
                    "name": "Alerts",
                    "description": "List of alert objects with severity and message",
                    "type": "array",
                    "item_type": "object",
                    "required": false
                }),
            ],
            tags: vec![
                "complex".to_string(),
                "notification".to_string(),
                "pluralization".to_string(),
                "section-ref".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "No activity",
                    "variables": {
                        "user_name": "Alice",
                        "messages": [],
                        "alerts": []
                    },
                    "expected_output": "📬 Notification Summary for Alice\n\nMessages: No new messages\n\nStatus: ✅ All clear!"
                }),
                serde_json::json!({
                    "name": "Full notification",
                    "variables": {
                        "user_name": "Bob",
                        "messages": ["Alice", "Charlie"],
                        "alerts": [
                            { "severity": "warning", "message": "Disk space low" },
                            { "severity": "error", "message": "Build failed" }
                        ]
                    },
                    "expected_output": "📬 Notification Summary for Bob\n\nMessages: 2 new messages from Alice and Charlie\n\nAlerts:\n⚠️ Warning: Disk space low\n❌ Error: Build failed\n\nStatus: 🔴 Multiple items need attention"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(notification_section)
            .await
            .map_err(|e| format!("Failed to create notification section: {}", e))?;

        // ============================================
        // DATA TYPE EXAMPLE: Create custom data types
        // ============================================
        let severity_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Severity".to_string(),
            description: "Alert severity levels".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": ["info", "warning", "error", "critical"]
            })),
            format: None,
            examples: vec![serde_json::json!("info"), serde_json::json!("error")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(severity_type)
            .await
            .map_err(|e| format!("Failed to create severity type: {}", e))?;

        let item_rarity_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "ItemRarity".to_string(),
            description: "RPG-style item rarity tiers".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": ["common", "uncommon", "rare", "epic", "legendary", "unique"]
            })),
            format: Some(serde_json::json!({
                "case": "title"
            })),
            examples: vec![serde_json::json!("common"), serde_json::json!("legendary")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(item_rarity_type)
            .await
            .map_err(|e| format!("Failed to create item rarity type: {}", e))?;

        // ============================================
        // DATA TYPE: Writing Styles
        // ============================================
        let writing_style_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "WritingStyle".to_string(),
            description: "Different writing styles for creative prompts".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": ["formal", "casual", "poetic", "technical", "humorous", "dramatic", "minimalist"]
            })),
            format: None,
            examples: vec![serde_json::json!("formal"), serde_json::json!("casual")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(writing_style_type)
            .await
            .map_err(|e| format!("Failed to create writing style type: {}", e))?;

        // ============================================
        // FRAGMENT: Random Adjective Pool
        // ============================================
        let adjective_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples-internal".to_string(),
            name: "random-adjective".to_string(),
            description: "Picks a random adjective from a pool".to_string(),
            content: serde_json::json!({
                "type": "random-value",
                "pool": ["mysterious", "ancient", "forgotten", "enchanted", "cursed", "legendary", "hidden", "sacred", "forbidden", "ethereal"]
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(adjective_fragment)
            .await
            .map_err(|e| format!("Failed to create adjective fragment: {}", e))?;

        // ============================================
        // FRAGMENT: Random Location
        // ============================================
        let location_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples-internal".to_string(),
            name: "random-location".to_string(),
            description: "Picks a random fantasy location".to_string(),
            content: serde_json::json!({
                "type": "pick-one",
                "candidates": [
                    { "type": "text", "value": "a towering castle on a cliff" },
                    { "type": "text", "value": "a dense forest shrouded in mist" },
                    { "type": "text", "value": "an underground cavern lit by crystals" },
                    { "type": "text", "value": "a floating island above the clouds" },
                    { "type": "text", "value": "a sunken temple beneath the waves" },
                    { "type": "text", "value": "a desert oasis guarded by sphinxes" }
                ]
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(location_fragment)
            .await
            .map_err(|e| format!("Failed to create location fragment: {}", e))?;

        // ============================================
        // FRAGMENT: Random Character Trait
        // ============================================
        let trait_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples-internal".to_string(),
            name: "random-trait".to_string(),
            description: "Picks a random character trait with weighted probability".to_string(),
            content: serde_json::json!({
                "type": "weighted-pick",
                "options": [
                    { "weight": 3, "content": { "type": "text", "value": "brave" } },
                    { "weight": 3, "content": { "type": "text", "value": "clever" } },
                    { "weight": 2, "content": { "type": "text", "value": "mysterious" } },
                    { "weight": 2, "content": { "type": "text", "value": "kind-hearted" } },
                    { "weight": 1, "content": { "type": "text", "value": "cunning" } },
                    { "weight": 1, "content": { "type": "text", "value": "reckless" } }
                ]
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(trait_fragment)
            .await
            .map_err(|e| format!("Failed to create trait fragment: {}", e))?;

        // ============================================
        // ENTRY POINT: Random Story Prompt Generator
        // ============================================
        let story_prompt_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Random Story Prompt".to_string(),
            description: "Generates unique story prompts by combining random elements".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Write a story about " },
                    {
                        "type": "article",
                        "word_content": { "type": "section-ref", "section_id": "examples-internal:random-trait" },
                        "style": "indefinite"
                    },
                    { "type": "text", "value": " " },
                    { "type": "section-ref", "section_id": "examples-internal:random-trait" },
                    { "type": "text", "value": " hero who discovers " },
                    {
                        "type": "article",
                        "word_content": { "type": "section-ref", "section_id": "examples-internal:random-adjective" },
                        "style": "indefinite"
                    },
                    { "type": "text", "value": " " },
                    { "type": "section-ref", "section_id": "examples-internal:random-adjective" },
                    { "type": "text", "value": " artifact in " },
                    { "type": "section-ref", "section_id": "examples-internal:random-location" },
                    { "type": "text", "value": "." }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![
                "random".to_string(),
                "creative".to_string(),
                "story".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Example output 1",
                    "variables": {},
                    "expected_output": "Write a story about a brave hero who discovers an ancient artifact in a towering castle on a cliff."
                }),
                serde_json::json!({
                    "name": "Example output 2",
                    "variables": {},
                    "expected_output": "Write a story about a mysterious hero who discovers a forbidden artifact in a dense forest shrouded in mist."
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(story_prompt_section)
            .await
            .map_err(|e| format!("Failed to create story prompt section: {}", e))?;

        // ============================================
        // ENTRY POINT: Random Character Generator
        // ============================================
        let character_gen_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Random Character Generator".to_string(),
            description: "Generates random character descriptions with pick-many traits"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Create a character named " },
                    {
                        "type": "pick-one",
                        "candidates": [
                            { "type": "text", "value": "Aldric" },
                            { "type": "text", "value": "Seraphina" },
                            { "type": "text", "value": "Thorne" },
                            { "type": "text", "value": "Lyra" },
                            { "type": "text", "value": "Caspian" },
                            { "type": "text", "value": "Isolde" }
                        ]
                    },
                    { "type": "text", "value": " who is " },
                    {
                        "type": "pick-many",
                        "candidates": [
                            { "type": "text", "value": "wise beyond their years" },
                            { "type": "text", "value": "haunted by their past" },
                            { "type": "text", "value": "searching for redemption" },
                            { "type": "text", "value": "fiercely loyal" },
                            { "type": "text", "value": "secretly royal" },
                            { "type": "text", "value": "gifted with magic" },
                            { "type": "text", "value": "trained in combat" },
                            { "type": "text", "value": "a master of disguise" }
                        ],
                        "count": { "min": 2, "max": 3 },
                        "separator_set_id": "oxford-comma"
                    },
                    { "type": "text", "value": ". They carry " },
                    {
                        "type": "article",
                        "word_content": { "type": "section-ref", "section_id": "examples-internal:random-adjective" },
                        "style": "indefinite"
                    },
                    { "type": "text", "value": " " },
                    { "type": "section-ref", "section_id": "examples-internal:random-adjective" },
                    { "type": "text", "value": " " },
                    {
                        "type": "pick-one",
                        "candidates": [
                            { "type": "text", "value": "sword" },
                            { "type": "text", "value": "staff" },
                            { "type": "text", "value": "amulet" },
                            { "type": "text", "value": "tome" },
                            { "type": "text", "value": "bow" }
                        ]
                    },
                    { "type": "text", "value": "." }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![
                "random".to_string(),
                "character".to_string(),
                "pick-many".to_string(),
            ],
            examples: vec![serde_json::json!({
                "name": "Example character",
                "variables": {},
                "expected_output": "Create a character named Seraphina who is wise beyond their years and gifted with magic. They carry an ancient staff."
            })],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(character_gen_section)
            .await
            .map_err(|e| format!("Failed to create character gen section: {}", e))?;

        // ============================================
        // ENTRY POINT: Random Quest Generator
        // ============================================
        let quest_gen_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Random Quest Generator".to_string(),
            description: "Generates random quests with objectives and rewards using shuffle"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "🎯 Quest: " },
                    {
                        "type": "pick-one",
                        "candidates": [
                            { "type": "text", "value": "The Lost Artifact" },
                            { "type": "text", "value": "Dragon's Bane" },
                            { "type": "text", "value": "The Forgotten Kingdom" },
                            { "type": "text", "value": "Shadow's Edge" },
                            { "type": "text", "value": "The Crystal Prophecy" }
                        ]
                    },
                    { "type": "text", "value": "\n\n📍 Location: " },
                    { "type": "section-ref", "section_id": "examples-internal:random-location" },
                    { "type": "text", "value": "\n\n📋 Objectives:\n" },
                    {
                        "type": "pick-many",
                        "candidates": [
                            { "type": "text", "value": "• Defeat the guardian" },
                            { "type": "text", "value": "• Solve the ancient riddle" },
                            { "type": "text", "value": "• Retrieve the artifact" },
                            { "type": "text", "value": "• Rescue the captive" },
                            { "type": "text", "value": "• Seal the dark portal" },
                            { "type": "text", "value": "• Gather the sacred ingredients" },
                            { "type": "text", "value": "• Decode the map" },
                            { "type": "text", "value": "• Forge an alliance" }
                        ],
                        "count": { "min": 2, "max": 4 },
                        "separator_set_id": "newline"
                    },
                    { "type": "text", "value": "\n\n🏆 Reward: " },
                    {
                        "type": "weighted-pick",
                        "options": [
                            { "weight": 5, "content": { "type": "text", "value": "500 gold coins" } },
                            { "weight": 3, "content": { "type": "text", "value": "A magical weapon" } },
                            { "weight": 2, "content": { "type": "text", "value": "Ancient spellbook" } },
                            { "weight": 1, "content": { "type": "text", "value": "Title of nobility" } }
                        ]
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![
                "random".to_string(),
                "quest".to_string(),
                "game".to_string(),
            ],
            examples: vec![serde_json::json!({
                "name": "Example quest",
                "variables": {},
                "expected_output": "🎯 Quest: The Lost Artifact\n\n📍 Location: a towering castle on a cliff\n\n📋 Objectives:\n• Defeat the guardian\n• Solve the ancient riddle\n• Retrieve the artifact\n\n🏆 Reward: 500 gold coins"
            })],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(quest_gen_section)
            .await
            .map_err(|e| format!("Failed to create quest gen section: {}", e))?;

        // ============================================
        // ENTRY POINT: Random Writing Prompt with Style
        // ============================================
        let writing_prompt_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Styled Writing Prompt".to_string(),
            description: "Generates writing prompts with random style from data type".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Write in a " },
                    {
                        "type": "random-value",
                        "data_type_id": "examples:WritingStyle"
                    },
                    { "type": "text", "value": " style about " },
                    {
                        "type": "pick-one",
                        "candidates": [
                            { "type": "text", "value": "a chance encounter that changes everything" },
                            { "type": "text", "value": "the last day of an era" },
                            { "type": "text", "value": "a secret that refuses to stay buried" },
                            { "type": "text", "value": "a journey with no destination" },
                            { "type": "text", "value": "the moment before everything changes" }
                        ]
                    },
                    { "type": "text", "value": ".\n\nInclude these elements: " },
                    {
                        "type": "pick-many",
                        "candidates": [
                            { "type": "text", "value": "a ticking clock" },
                            { "type": "text", "value": "an unexpected ally" },
                            { "type": "text", "value": "a moral dilemma" },
                            { "type": "text", "value": "a hidden truth" },
                            { "type": "text", "value": "a moment of doubt" },
                            { "type": "text", "value": "an act of courage" }
                        ],
                        "count": 3,
                        "separator_set_id": "oxford-comma"
                    },
                    { "type": "text", "value": "." }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![
                "random".to_string(),
                "writing".to_string(),
                "data-type".to_string(),
            ],
            examples: vec![serde_json::json!({
                "name": "Example writing prompt",
                "variables": {},
                "expected_output": "Write in a poetic style about a secret that refuses to stay buried.\n\nInclude these elements: a ticking clock, an unexpected ally, and a moral dilemma."
            })],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(writing_prompt_section)
            .await
            .map_err(|e| format!("Failed to create writing prompt section: {}", e))?;

        // ============================================
        // ENTRY POINT: Shuffle-Based Itinerary
        // ============================================
        let itinerary_section = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "examples".to_string(),
            name: "Random Day Itinerary".to_string(),
            description: "Creates a randomized itinerary by shuffling activities".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "text", "value": "Today's Adventure Plan:\n\n" },
                    {
                        "type": "shuffle",
                        "variable_id": "activities",
                        "count": 4,
                        "separator_set_id": "numbered-list",
                        "item_template": {
                            "type": "composite",
                            "parts": [
                                { "type": "variable", "variable_id": "item" }
                            ]
                        }
                    },
                    { "type": "text", "value": "\n\n✨ Special surprise: " },
                    {
                        "type": "pick-one",
                        "candidates": [
                            { "type": "text", "value": "A hidden gem awaits!" },
                            { "type": "text", "value": "Secret menu item unlocked!" },
                            { "type": "text", "value": "Bonus experience earned!" },
                            { "type": "text", "value": "Mystery reward revealed!" }
                        ]
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec!["activities".to_string()],
            variables: vec![serde_json::json!({
                "id": "activities",
                "name": "Activities",
                "description": "List of possible activities to shuffle and pick from",
                "type": "array",
                "item_type": "string",
                "required": true,
                "default_value": [
                    "Visit the museum",
                    "Explore the park",
                    "Try the local café",
                    "Browse the bookstore",
                    "Walk by the river",
                    "Check out street art",
                    "Visit the market",
                    "Relax at the garden"
                ]
            })],
            tags: vec![
                "random".to_string(),
                "shuffle".to_string(),
                "itinerary".to_string(),
            ],
            examples: vec![serde_json::json!({
                "name": "Example itinerary",
                "variables": {
                    "activities": ["Visit the museum", "Explore the park", "Try the local café", "Browse the bookstore", "Walk by the river"]
                },
                "expected_output": "Today's Adventure Plan:\n\n1. Explore the park\n2. Try the local café\n3. Visit the museum\n4. Walk by the river\n\n✨ Special surprise: A hidden gem awaits!"
            })],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(itinerary_section)
            .await
            .map_err(|e| format!("Failed to create itinerary section: {}", e))?;

        // ============================================
        // TAGS for categorization
        // ============================================
        let tags_to_create = vec![
            ("simple", "Simple examples", "#28a745"),
            ("medium", "Medium complexity", "#ffc107"),
            ("complex", "Complex examples", "#dc3545"),
            ("pluralization", "Demonstrates pluralization", "#17a2b8"),
            ("article", "Demonstrates a/an selection", "#6f42c1"),
            ("switch", "Demonstrates switch/case", "#fd7e14"),
            ("section-ref", "Uses section references", "#20c997"),
            ("random", "Uses random selection", "#e83e8c"),
            ("pick-many", "Picks multiple random items", "#6610f2"),
            ("shuffle", "Shuffles and selects items", "#007bff"),
        ];

        for (name, description, color) in tags_to_create {
            let tag = PromptTag {
                id: None,
                package_id: package_id.clone(),
                namespace: "examples".to_string(),
                name: name.to_string(),
                description: description.to_string(),
                color: Some(color.to_string()),
                parent: None,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
            };

            let _: Option<PromptTag> = db
                .db
                .create("prompt_tags")
                .content(tag)
                .await
                .map_err(|e| format!("Failed to create tag: {}", e))?;
        }

        Ok(
            "Created example package with 13 entry points, 5 fragments, 3 data types, and 10 tags"
                .to_string(),
        )
    }

    #[tauri::command]
    pub async fn seed_text2image_common_package(
        state: tauri::State<'_, AppState>,
    ) -> Result<String, String> {
        let db = state.database.lock().await;
        let timestamp = get_timestamp();

        // Check if text2image-common already exists and delete it
        let existing: Vec<PromptPackage> = db
            .db
            .query("SELECT * FROM prompt_packages WHERE namespace = 'text2image-common'")
            .await
            .map_err(|e| format!("Failed to check existing: {}", e))?
            .take(0)
            .map_err(|e| format!("Failed to extract: {}", e))?;

        if !existing.is_empty() {
            // Delete all related data for existing text2image-common packages
            for pkg in &existing {
                if let Some(ref id) = pkg.id {
                    let pkg_id = match &id.id {
                        surrealdb::sql::Id::String(s) => s.clone(),
                        surrealdb::sql::Id::Number(n) => n.to_string(),
                        _ => format!("{:?}", id.id),
                    };

                    // Delete sections
                    let _: Vec<PromptSection> = db
                        .db
                        .query("DELETE FROM prompt_sections WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete sections: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete data types
                    let _: Vec<PromptDataType> = db
                        .db
                        .query("DELETE FROM prompt_data_types WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete data types: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete tags
                    let _: Vec<PromptTag> = db
                        .db
                        .query("DELETE FROM prompt_tags WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete tags: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete separator sets
                    let _: Vec<SeparatorSet> = db
                        .db
                        .query("DELETE FROM prompt_separator_sets WHERE package_id = $pkg_id")
                        .bind(("pkg_id", pkg_id.clone()))
                        .await
                        .map_err(|e| format!("Failed to delete separator sets: {}", e))?
                        .take(0)
                        .unwrap_or_default();

                    // Delete the package itself
                    let _: Option<PromptPackage> = db
                        .db
                        .delete(("prompt_packages", pkg_id.as_str()))
                        .await
                        .map_err(|e| format!("Failed to delete package: {}", e))?;
                }
            }
        }

        // Create the text2image-common package
        let package = PromptPackage {
            id: None,
            namespace: "text2image-common".to_string(),
            additional_namespaces: vec!["t2i-internal".to_string()],
            name: "Text2Image Common Library".to_string(),
            version: "1.0.0".to_string(),
            description: "Common reusable components for text-to-image prompt generation including subjects, actions, environments, styles, and modifiers".to_string(),
            author: "System".to_string(),
            dependencies: vec![],
            exports: vec![
                "hero-description".to_string(),
                "scene-description".to_string(),
                "style-modifiers".to_string(),
                "lighting-atmosphere".to_string(),
                "camera-settings".to_string()
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };

        let created_package: Option<PromptPackage> = db
            .db
            .create("prompt_packages")
            .content(package)
            .await
            .map_err(|e| format!("Failed to create package: {}", e))?;

        let pkg = created_package.ok_or("Failed to create package")?;
        let package_id = extract_id(&pkg.id).ok_or("Failed to get package ID")?;

        // ============================================
        // DATA TYPES
        // ============================================

        // Hero Types
        let hero_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "HeroType".to_string(),
            description: "Types of heroes/main subjects".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "warrior", "mage", "rogue", "archer", "knight", "paladin", "necromancer", "druid",
                    "cyborg", "android", "space explorer", "pilot", "engineer", "scientist",
                    "detective", "spy", "superhero", "vigilante", "mercenary",
                    "princess", "queen", "king", "prince", "peasant", "merchant",
                    "monk", "samurai", "ninja", "viking", "barbarian",
                    "dragon", "demon", "angel", "elf", "dwarf", "orc", "goblin",
                    "alien", "robot", "mutant", "vampire", "werewolf", "zombie"
                ]
            })),
            format: None,
            examples: vec![serde_json::json!("warrior"), serde_json::json!("cyborg")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(hero_type)
            .await
            .map_err(|e| format!("Failed to create hero type: {}", e))?;

        // Action Types
        let action_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "ActionType".to_string(),
            description: "Actions/verbs for scenes".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "standing", "sitting", "running", "walking", "jumping", "flying", "floating", "hovering",
                    "fighting", "battling", "dueling", "defending", "attacking", "charging",
                    "casting spell", "channeling energy", "meditating", "praying",
                    "exploring", "discovering", "searching", "investigating",
                    "climbing", "swimming", "diving", "surfing",
                    "riding", "driving", "piloting",
                    "dancing", "performing", "singing", "playing instrument",
                    "crafting", "building", "forging", "smithing",
                    "reading", "writing", "studying", "teaching",
                    "resting", "sleeping", "dreaming",
                    "commanding", "leading", "ruling", "conquering"
                ]
            })),
            format: None,
            examples: vec![serde_json::json!("fighting"), serde_json::json!("flying")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(action_type)
            .await
            .map_err(|e| format!("Failed to create action type: {}", e))?;

        // Environment Types
        let environment_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "EnvironmentType".to_string(),
            description: "Background environments and settings".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "medieval castle", "ancient ruins", "mystical forest", "dark cave", "mountain peak", "volcanic wasteland",
                    "frozen tundra", "desert dunes", "tropical island", "underwater realm", "sky kingdom", "floating islands",
                    "futuristic city", "cyberpunk street", "space station", "alien planet", "post-apocalyptic wasteland",
                    "steampunk workshop", "crystal cavern", "enchanted garden", "haunted mansion", "gothic cathedral",
                    "throne room", "battlefield", "colosseum", "temple", "shrine", "monastery",
                    "laboratory", "library", "archive", "museum", "gallery",
                    "market square", "tavern", "inn", "port", "harbor",
                    "bridge", "crossroads", "gateway", "portal", "dimensional rift",
                    "void", "astral plane", "dream realm", "nightmare landscape", "heaven", "hell", "purgatory"
                ]
            })),
            format: None,
            examples: vec![
                serde_json::json!("mystical forest"),
                serde_json::json!("futuristic city"),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(environment_type)
            .await
            .map_err(|e| format!("Failed to create environment type: {}", e))?;

        // Art Style Types
        let art_style_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "ArtStyle".to_string(),
            description: "Artistic styles and rendering approaches".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "photorealistic", "hyperrealistic", "cinematic", "dramatic", "epic",
                    "oil painting", "watercolor", "digital painting", "concept art", "matte painting",
                    "anime", "manga", "cartoon", "comic book", "graphic novel",
                    "pixel art", "voxel art", "low poly", "isometric",
                    "sketch", "pencil drawing", "charcoal", "ink drawing", "line art",
                    "impressionist", "expressionist", "surreal", "abstract", "minimalist",
                    "art nouveau", "art deco", "baroque", "renaissance", "gothic",
                    "steampunk", "cyberpunk", "solarpunk", "dieselpunk",
                    "fantasy art", "sci-fi art", "dark fantasy", "high fantasy",
                    "studio ghibli style", "pixar style", "disney style",
                    "unreal engine", "octane render", "unity engine", "3d render"
                ]
            })),
            format: None,
            examples: vec![
                serde_json::json!("photorealistic"),
                serde_json::json!("anime"),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(art_style_type)
            .await
            .map_err(|e| format!("Failed to create art style type: {}", e))?;

        // Lighting Types
        let lighting_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "LightingType".to_string(),
            description: "Lighting conditions and effects".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "golden hour", "blue hour", "sunrise", "sunset", "noon sun", "harsh sunlight",
                    "soft lighting", "dramatic lighting", "studio lighting", "rim lighting", "back lighting",
                    "volumetric lighting", "god rays", "light shafts", "lens flare",
                    "moonlight", "starlight", "candlelight", "firelight", "torch light",
                    "neon lights", "bioluminescence", "magical glow", "ethereal light",
                    "fog", "mist", "haze", "smoke", "dust particles",
                    "dark", "shadows", "silhouette", "chiaroscuro",
                    "bright", "radiant", "glowing", "luminous", "shimmering"
                ]
            })),
            format: None,
            examples: vec![
                serde_json::json!("golden hour"),
                serde_json::json!("volumetric lighting"),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(lighting_type)
            .await
            .map_err(|e| format!("Failed to create lighting type: {}", e))?;

        // Camera Angle Types
        let camera_angle_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "CameraAngle".to_string(),
            description: "Camera angles and shot types".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "close-up", "extreme close-up", "medium shot", "wide shot", "extreme wide shot",
                    "portrait", "full body", "three-quarter view", "profile view",
                    "low angle", "high angle", "dutch angle", "birds eye view", "worms eye view",
                    "over the shoulder", "point of view", "first person",
                    "establishing shot", "aerial view", "drone shot",
                    "macro", "microscopic"
                ]
            })),
            format: None,
            examples: vec![
                serde_json::json!("close-up"),
                serde_json::json!("birds eye view"),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(camera_angle_type)
            .await
            .map_err(|e| format!("Failed to create camera angle type: {}", e))?;

        // Quality Modifiers
        let quality_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "QualityModifier".to_string(),
            description: "Quality and detail modifiers".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "8k", "4k", "high resolution", "ultra detailed", "highly detailed",
                    "intricate details", "fine details", "sharp focus", "crisp",
                    "trending on artstation", "award winning", "masterpiece", "professional",
                    "beautiful", "stunning", "gorgeous", "breathtaking", "mesmerizing"
                ]
            })),
            format: None,
            examples: vec![serde_json::json!("8k"), serde_json::json!("masterpiece")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(quality_type)
            .await
            .map_err(|e| format!("Failed to create quality type: {}", e))?;

        // Color Palette Types
        let color_palette_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "ColorPalette".to_string(),
            description: "Color schemes and palettes".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "vibrant colors", "muted colors", "pastel colors", "neon colors", "dark colors",
                    "warm tones", "cool tones", "monochromatic", "black and white", "sepia",
                    "golden", "silver", "bronze", "copper",
                    "blue palette", "red palette", "green palette", "purple palette", "orange palette",
                    "earth tones", "jewel tones", "autumn colors", "winter colors", "spring colors", "summer colors",
                    "complementary colors", "analogous colors", "triadic colors"
                ]
            })),
            format: None,
            examples: vec![
                serde_json::json!("vibrant colors"),
                serde_json::json!("warm tones"),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(color_palette_type)
            .await
            .map_err(|e| format!("Failed to create color palette type: {}", e))?;

        // Mood Types
        let mood_type = PromptDataType {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "MoodType".to_string(),
            description: "Emotional atmosphere and mood".to_string(),
            base_type: "enum".to_string(),
            validation: Some(serde_json::json!({
                "enum_values": [
                    "epic", "heroic", "triumphant", "victorious",
                    "dark", "ominous", "foreboding", "sinister", "menacing",
                    "peaceful", "serene", "tranquil", "calm", "relaxing",
                    "mysterious", "enigmatic", "cryptic",
                    "romantic", "dreamy", "whimsical", "magical",
                    "melancholic", "somber", "sad", "tragic",
                    "intense", "dramatic", "tense", "suspenseful",
                    "joyful", "cheerful", "happy", "uplifting",
                    "lonely", "isolated", "abandoned",
                    "chaotic", "frantic", "hectic",
                    "nostalgic", "vintage", "retro"
                ]
            })),
            format: None,
            examples: vec![serde_json::json!("epic"), serde_json::json!("mysterious")],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptDataType> = db
            .db
            .create("prompt_data_types")
            .content(mood_type)
            .await
            .map_err(|e| format!("Failed to create mood type: {}", e))?;

        // ============================================
        // FRAGMENTS (Reusable Sections)
        // ============================================

        // Random Hero Description
        let hero_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "t2i-internal".to_string(),
            name: "random-hero".to_string(),
            description: "Picks a random hero type from data pool".to_string(),
            content: serde_json::json!({
                "type": "random-value",
                "data_type_id": "text2image-common:HeroType"
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(hero_fragment)
            .await
            .map_err(|e| format!("Failed to create hero fragment: {}", e))?;

        // Random Action
        let action_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "t2i-internal".to_string(),
            name: "random-action".to_string(),
            description: "Picks a random action".to_string(),
            content: serde_json::json!({
                "type": "random-value",
                "data_type_id": "text2image-common:ActionType"
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(action_fragment)
            .await
            .map_err(|e| format!("Failed to create action fragment: {}", e))?;

        // Random Environment
        let environment_fragment = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "t2i-internal".to_string(),
            name: "random-environment".to_string(),
            description: "Picks a random environment".to_string(),
            content: serde_json::json!({
                "type": "random-value",
                "data_type_id": "text2image-common:EnvironmentType"
            }),
            is_entry_point: false,
            exportable: true,
            required_variables: vec![],
            variables: vec![],
            tags: vec![],
            examples: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(environment_fragment)
            .await
            .map_err(|e| format!("Failed to create environment fragment: {}", e))?;

        // ============================================
        // ENTRY POINTS (Exportable Templates)
        // ============================================

        // Hero Description Entry Point
        let hero_description_entry = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "Hero Description".to_string(),
            description: "Generates a detailed hero description with optional customization"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "conditional",
                        "condition": { "variable": "hero_type", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "hero_type" },
                        "else_content": { "type": "section-ref", "section_id": "t2i-internal:random-hero" }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "appearance_modifiers", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "list", "variable_id": "appearance_modifiers", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![
                serde_json::json!({
                    "id": "hero_type",
                    "name": "Hero Type",
                    "description": "Type of hero (optional, will be random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "appearance_modifiers",
                    "name": "Appearance Modifiers",
                    "description": "Additional appearance details (optional)",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
            ],
            tags: vec![
                "hero".to_string(),
                "character".to_string(),
                "subject".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Random hero",
                    "variables": {},
                    "expected_output": "warrior"
                }),
                serde_json::json!({
                    "name": "Custom hero with modifiers",
                    "variables": {
                        "hero_type": "cyborg",
                        "appearance_modifiers": ["glowing red eyes", "metallic armor", "lightning effects"]
                    },
                    "expected_output": "cyborg, glowing red eyes, metallic armor, and lightning effects"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(hero_description_entry)
            .await
            .map_err(|e| format!("Failed to create hero description entry: {}", e))?;

        // Scene Description Entry Point
        let scene_description_entry = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "Scene Description".to_string(),
            description: "Generates a complete scene with subject, action, and environment"
                .to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    { "type": "section-ref", "section_id": "text2image-common:hero-description" },
                    { "type": "text", "value": " " },
                    {
                        "type": "conditional",
                        "condition": { "variable": "action", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "action" },
                        "else_content": { "type": "section-ref", "section_id": "t2i-internal:random-action" }
                    },
                    { "type": "text", "value": " in " },
                    {
                        "type": "conditional",
                        "condition": { "variable": "environment", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "environment" },
                        "else_content": { "type": "section-ref", "section_id": "t2i-internal:random-environment" }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "objects", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", with " },
                                { "type": "list", "variable_id": "objects", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![
                serde_json::json!({
                    "id": "hero_type",
                    "name": "Hero Type",
                    "description": "Type of hero (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "action",
                    "name": "Action",
                    "description": "What the subject is doing (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "environment",
                    "name": "Environment",
                    "description": "Background setting (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "objects",
                    "name": "Objects",
                    "description": "Additional objects in the scene (optional)",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
            ],
            tags: vec!["scene".to_string(), "complete".to_string()],
            examples: vec![
                serde_json::json!({
                    "name": "Fully random scene",
                    "variables": {},
                    "expected_output": "warrior fighting in mystical forest"
                }),
                serde_json::json!({
                    "name": "Custom scene with objects",
                    "variables": {
                        "hero_type": "mage",
                        "action": "casting spell",
                        "environment": "ancient ruins",
                        "objects": ["glowing crystals", "floating runes", "magical tome"]
                    },
                    "expected_output": "mage casting spell in ancient ruins, with glowing crystals, floating runes, and magical tome"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(scene_description_entry)
            .await
            .map_err(|e| format!("Failed to create scene description entry: {}", e))?;

        // Style Modifiers Entry Point
        let style_modifiers_entry = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "Style Modifiers".to_string(),
            description: "Art style, quality, and color palette modifiers".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "conditional",
                        "condition": { "variable": "art_style", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "art_style" },
                        "else_content": { "type": "random-value", "data_type_id": "text2image-common:ArtStyle" }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "quality_modifiers", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "list", "variable_id": "quality_modifiers", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "color_palette", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "variable", "variable_id": "color_palette" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![
                serde_json::json!({
                    "id": "art_style",
                    "name": "Art Style",
                    "description": "Artistic style (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "quality_modifiers",
                    "name": "Quality Modifiers",
                    "description": "Quality descriptors (optional)",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "color_palette",
                    "name": "Color Palette",
                    "description": "Color scheme (optional)",
                    "type": "string",
                    "required": false
                }),
            ],
            tags: vec![
                "style".to_string(),
                "quality".to_string(),
                "modifiers".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Random style",
                    "variables": {},
                    "expected_output": "photorealistic"
                }),
                serde_json::json!({
                    "name": "Custom style with quality",
                    "variables": {
                        "art_style": "anime",
                        "quality_modifiers": ["8k", "highly detailed", "masterpiece"],
                        "color_palette": "vibrant colors"
                    },
                    "expected_output": "anime, 8k, highly detailed, and masterpiece, vibrant colors"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(style_modifiers_entry)
            .await
            .map_err(|e| format!("Failed to create style modifiers entry: {}", e))?;

        // Lighting and Atmosphere Entry Point
        let lighting_atmosphere_entry = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "Lighting and Atmosphere".to_string(),
            description: "Lighting, mood, and atmospheric effects".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "conditional",
                        "condition": { "variable": "lighting", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "lighting" },
                        "else_content": { "type": "random-value", "data_type_id": "text2image-common:LightingType" }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "mood", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "variable", "variable_id": "mood" },
                                { "type": "text", "value": " mood" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "atmospheric_effects", "operator": "has_items" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "list", "variable_id": "atmospheric_effects", "separator_set_id": "oxford-comma" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![
                serde_json::json!({
                    "id": "lighting",
                    "name": "Lighting",
                    "description": "Lighting type (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "mood",
                    "name": "Mood",
                    "description": "Emotional atmosphere (optional)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "atmospheric_effects",
                    "name": "Atmospheric Effects",
                    "description": "Additional atmospheric elements (optional)",
                    "type": "array",
                    "item_type": "string",
                    "required": false
                }),
            ],
            tags: vec![
                "lighting".to_string(),
                "atmosphere".to_string(),
                "mood".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Random lighting",
                    "variables": {},
                    "expected_output": "golden hour"
                }),
                serde_json::json!({
                    "name": "Custom atmosphere",
                    "variables": {
                        "lighting": "volumetric lighting",
                        "mood": "epic",
                        "atmospheric_effects": ["god rays", "dust particles", "lens flare"]
                    },
                    "expected_output": "volumetric lighting, epic mood, god rays, dust particles, and lens flare"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(lighting_atmosphere_entry)
            .await
            .map_err(|e| format!("Failed to create lighting atmosphere entry: {}", e))?;

        // Camera Settings Entry Point
        let camera_settings_entry = PromptSection {
            id: None,
            package_id: package_id.clone(),
            namespace: "text2image-common".to_string(),
            name: "Camera Settings".to_string(),
            description: "Camera angle, shot type, and technical settings".to_string(),
            content: serde_json::json!({
                "type": "composite",
                "parts": [
                    {
                        "type": "conditional",
                        "condition": { "variable": "camera_angle", "operator": "exists" },
                        "then_content": { "type": "variable", "variable_id": "camera_angle" },
                        "else_content": { "type": "random-value", "data_type_id": "text2image-common:CameraAngle" }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "focal_length", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "variable", "variable_id": "focal_length" },
                                { "type": "text", "value": "mm lens" }
                            ]
                        }
                    },
                    {
                        "type": "conditional",
                        "condition": { "variable": "depth_of_field", "operator": "exists" },
                        "then_content": {
                            "type": "composite",
                            "parts": [
                                { "type": "text", "value": ", " },
                                { "type": "variable", "variable_id": "depth_of_field" }
                            ]
                        }
                    }
                ]
            }),
            is_entry_point: true,
            exportable: true,
            required_variables: vec![],
            variables: vec![
                serde_json::json!({
                    "id": "camera_angle",
                    "name": "Camera Angle",
                    "description": "Camera perspective (optional, random if not provided)",
                    "type": "string",
                    "required": false
                }),
                serde_json::json!({
                    "id": "focal_length",
                    "name": "Focal Length",
                    "description": "Lens focal length in mm (optional)",
                    "type": "number",
                    "required": false
                }),
                serde_json::json!({
                    "id": "depth_of_field",
                    "name": "Depth of Field",
                    "description": "DOF description (e.g., 'shallow depth of field', 'bokeh') (optional)",
                    "type": "string",
                    "required": false
                }),
            ],
            tags: vec![
                "camera".to_string(),
                "technical".to_string(),
                "composition".to_string(),
            ],
            examples: vec![
                serde_json::json!({
                    "name": "Random camera",
                    "variables": {},
                    "expected_output": "close-up"
                }),
                serde_json::json!({
                    "name": "Custom camera settings",
                    "variables": {
                        "camera_angle": "low angle",
                        "focal_length": 85,
                        "depth_of_field": "shallow depth of field with bokeh"
                    },
                    "expected_output": "low angle, 85mm lens, shallow depth of field with bokeh"
                }),
            ],
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
        };
        let _: Option<PromptSection> = db
            .db
            .create("prompt_sections")
            .content(camera_settings_entry)
            .await
            .map_err(|e| format!("Failed to create camera settings entry: {}", e))?;

        // ============================================
        // TAGS for categorization
        // ============================================
        let tags_to_create = vec![
            ("text2image", "Text-to-image related", "#FF6B6B"),
            ("hero", "Hero/character components", "#4ECDC4"),
            ("scene", "Scene components", "#45B7D1"),
            ("style", "Style and quality", "#96CEB4"),
            ("lighting", "Lighting and atmosphere", "#FFEAA7"),
            ("camera", "Camera and composition", "#DFE6E9"),
            ("modifiers", "Modifier components", "#74B9FF"),
            ("subject", "Subject/main focus", "#A29BFE"),
            ("atmosphere", "Atmospheric effects", "#FD79A8"),
            ("mood", "Mood and emotion", "#FDCB6E"),
            ("quality", "Quality descriptors", "#6C5CE7"),
            ("technical", "Technical settings", "#00B894"),
            ("composition", "Composition elements", "#00CEC9"),
            ("complete", "Complete prompt templates", "#55EFC4"),
        ];

        for (name, description, color) in tags_to_create {
            let tag = PromptTag {
                id: None,
                package_id: package_id.clone(),
                namespace: "text2image-common".to_string(),
                name: name.to_string(),
                description: description.to_string(),
                color: Some(color.to_string()),
                parent: None,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
            };

            let _: Option<PromptTag> = db
                .db
                .create("prompt_tags")
                .content(tag)
                .await
                .map_err(|e| format!("Failed to create tag: {}", e))?;
        }

        Ok("Created Text2Image Common Library package with 9 data types, 3 internal fragments, 5 exportable entry points, and 14 tags".to_string())
    }
}
