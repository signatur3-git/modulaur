// Adapter framework for data sources
//
// This module provides the core traits and types for building data source adapters.
// Each adapter can fetch data from external sources, transform it to a common format,
// and store it in the SurrealDB staging area.

use crate::db::StagedRecord;
use crate::error::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod rest;
// gitlab module removed - functionality provided by gitlab-adapter plugin

// ============================================================================
// Core Adapter Trait
// ============================================================================

/// Main adapter trait that all data source adapters must implement
#[async_trait]
pub trait Adapter: Send + Sync {
    /// Unique identifier for this adapter type (e.g., "rest_api", "gitlab")
    fn adapter_type(&self) -> &str;

    /// Human-readable name for this adapter
    #[allow(dead_code)] // Will be used in UI for displaying adapter names
    fn name(&self) -> &str;

    /// Fetch data from the source and return transformed records
    async fn fetch(&self, config: &AdapterConfig) -> Result<Vec<StagedRecord>, AppError>;

    /// Test the connection/configuration without fetching data
    async fn test_connection(&self, config: &AdapterConfig) -> Result<bool, AppError>;

    /// Get the default configuration template for this adapter
    fn default_config(&self) -> AdapterConfig;
}

// ============================================================================
// Adapter Configuration
// ============================================================================

/// Generic configuration for any adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    /// Adapter type identifier
    pub adapter_type: String,

    /// Source identifier (user-defined, e.g., "gitlab-project-123")
    pub source: String,

    /// Base URL or endpoint
    pub endpoint: String,

    /// Authentication configuration
    pub auth: Option<AuthConfig>,

    /// Adapter-specific parameters (flexible JSON)
    pub parameters: serde_json::Value,

    /// Polling interval in seconds (optional)
    pub polling_interval: Option<u64>,

    /// Whether this adapter is enabled
    pub enabled: bool,
}

impl AdapterConfig {
    pub fn new(adapter_type: &str, source: &str, endpoint: &str) -> Self {
        Self {
            adapter_type: adapter_type.to_string(),
            source: source.to_string(),
            endpoint: endpoint.to_string(),
            auth: None,
            parameters: serde_json::json!({}),
            polling_interval: None,
            enabled: true,
        }
    }
}

// ============================================================================
// Authentication Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    /// No authentication
    None,

    /// Bearer token authentication
    Bearer { token: String },

    /// OAuth2 Client Credentials flow
    OAuth2ClientCredentials {
        client_id: String,
        client_secret: String,
        token_url: String,
        scope: Option<String>,
    },

    /// OAuth2 Authorization Code flow (for user-based auth)
    OAuth2AuthorizationCode {
        client_id: String,
        client_secret: String,
        authorization_url: String,
        token_url: String,
        redirect_uri: String,
        scope: Option<String>,
    },

    /// Basic authentication
    Basic { username: String, password: String },

    /// API Key (header-based)
    ApiKey { header_name: String, key: String },

    /// GitLab Personal Access Token
    GitLabToken { token: String },
}

// ============================================================================
// Adapter Registry
// ============================================================================

/// Registry for managing available adapters
pub struct AdapterRegistry {
    adapters: std::collections::HashMap<String, Box<dyn Adapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            adapters: std::collections::HashMap::new(),
        };

        // Register built-in adapters
        registry.register(Box::new(rest::RestAdapter::new()));
        // GitLab adapter removed - functionality provided by gitlab-adapter plugin

        registry
    }

    /// Register a new adapter
    pub fn register(&mut self, adapter: Box<dyn Adapter>) {
        let adapter_type = adapter.adapter_type().to_string();
        self.adapters.insert(adapter_type, adapter);
    }

    /// Get an adapter by type
    pub fn get(&self, adapter_type: &str) -> Option<&dyn Adapter> {
        self.adapters.get(adapter_type).map(|b| b.as_ref())
    }

    /// Get all registered adapter types
    pub fn list_types(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Fetch data using the specified adapter
    pub async fn fetch(&self, config: &AdapterConfig) -> Result<Vec<StagedRecord>, AppError> {
        let adapter = self.get(&config.adapter_type).ok_or_else(|| {
            AppError::Adapter(format!("Unknown adapter type: {}", config.adapter_type))
        })?;

        adapter.fetch(config).await
    }

    /// Test connection for a configuration
    pub async fn test_connection(&self, config: &AdapterConfig) -> Result<bool, AppError> {
        let adapter = self.get(&config.adapter_type).ok_or_else(|| {
            AppError::Adapter(format!("Unknown adapter type: {}", config.adapter_type))
        })?;

        adapter.test_connection(config).await
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HTTP Client Helper
// ============================================================================

/// Helper for making authenticated HTTP requests
pub struct HttpClient;

impl HttpClient {
    /// Create a new reqwest client with timeout
    pub fn new_client() -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client")
    }

    /// Add authentication headers to a request builder
    pub fn add_auth(
        builder: reqwest::RequestBuilder,
        auth: &Option<AuthConfig>,
    ) -> reqwest::RequestBuilder {
        match auth {
            None | Some(AuthConfig::None) => builder,
            Some(AuthConfig::Bearer { token }) => {
                builder.header("Authorization", format!("Bearer {}", token))
            }
            Some(AuthConfig::Basic { username, password }) => {
                builder.basic_auth(username, Some(password))
            }
            Some(AuthConfig::ApiKey { header_name, key }) => builder.header(header_name, key),
            Some(AuthConfig::GitLabToken { token }) => builder.header("PRIVATE-TOKEN", token),
            Some(AuthConfig::OAuth2ClientCredentials { .. }) => {
                // OAuth2 token should be fetched first and converted to Bearer
                // This is handled by the adapter implementation
                builder
            }
            Some(AuthConfig::OAuth2AuthorizationCode { .. }) => {
                // Same as above
                builder
            }
        }
    }

    /// Fetch OAuth2 token using client credentials flow
    pub async fn fetch_oauth2_token(
        client_id: &str,
        client_secret: &str,
        token_url: &str,
        scope: Option<&str>,
    ) -> Result<String, AppError> {
        let client = Self::new_client();

        let mut params = vec![
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        if let Some(s) = scope {
            params.push(("scope", s));
        }

        let response = client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Http(format!("OAuth2 token request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Http(format!(
                "OAuth2 token request failed with status: {}",
                response.status()
            )));
        }

        let token_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::Http(format!("Failed to parse OAuth2 response: {}", e)))?;

        token_response["access_token"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Http("OAuth2 response missing access_token".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry::new();

        // Check that built-in adapters are registered
        let types = registry.list_types();
        assert!(types.contains(&"rest_api".to_string()));

        // GitLab is provided by a plugin in this repo, not a built-in adapter.
        assert!(!types.contains(&"gitlab".to_string()));
    }

    #[test]
    fn test_adapter_config_creation() {
        let config = AdapterConfig::new("test", "test-source", "https://example.com");

        assert_eq!(config.adapter_type, "test");
        assert_eq!(config.source, "test-source");
        assert_eq!(config.endpoint, "https://example.com");
        assert!(config.enabled);
        assert!(config.auth.is_none());
    }
}
