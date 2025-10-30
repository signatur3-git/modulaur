// REST API Adapter
//
// Generic REST adapter with OAuth2 support for fetching data from HTTP endpoints

use crate::adapters::{Adapter, AdapterConfig, AuthConfig, HttpClient};
use crate::db::{RecordMetadata, StagedRecord};
use crate::error::AppError;
use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;

pub struct RestAdapter;

impl RestAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Extract records from JSON response based on configuration
    async fn transform_response(
        &self,
        response: Value,
        config: &AdapterConfig,
    ) -> Result<Vec<StagedRecord>, AppError> {
        let mut records = Vec::new();

        // Get the data path from parameters (e.g., "data", "results", or empty for root)
        let data_path = config.parameters["data_path"].as_str().unwrap_or("");

        // Navigate to the data array
        let data_array = if data_path.is_empty() {
            &response
        } else {
            response.get(data_path).unwrap_or(&response)
        };

        // If it's an array, process each item
        if let Some(array) = data_array.as_array() {
            for item in array {
                let record = self.create_record(item.clone(), config)?;
                records.push(record);
            }
        } else {
            // Single object response
            let record = self.create_record(data_array.clone(), config)?;
            records.push(record);
        }

        Ok(records)
    }

    /// Create a staged record from a JSON item
    fn create_record(&self, data: Value, config: &AdapterConfig) -> Result<StagedRecord, AppError> {
        // Extract metadata fields if they exist
        let tags = config.parameters["default_tags"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let title = data
            .get("title")
            .or_else(|| data.get("name"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let description = data
            .get("description")
            .and_then(|v| v.as_str())
            .map(String::from);

        let status = data
            .get("status")
            .and_then(|v| v.as_str())
            .map(String::from);

        let metadata = RecordMetadata {
            tags,
            status,
            title,
            description,
        };

        Ok(StagedRecord {
            id: None, // Will be set by SurrealDB
            record_type: self.adapter_type().to_string(),
            source: config.source.clone(),
            timestamp: Utc::now(),
            data,
            metadata,
        })
    }

    /// Get OAuth2 bearer token if needed
    async fn get_auth_token(&self, auth: &Option<AuthConfig>) -> Result<Option<String>, AppError> {
        if let Some(AuthConfig::OAuth2ClientCredentials {
            client_id,
            client_secret,
            token_url,
            scope,
        }) = auth
        {
            let token = HttpClient::fetch_oauth2_token(
                client_id,
                client_secret,
                token_url,
                scope.as_deref(),
            )
            .await?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl Adapter for RestAdapter {
    fn adapter_type(&self) -> &str {
        "rest_api"
    }

    fn name(&self) -> &str {
        "REST API Adapter"
    }

    async fn fetch(&self, config: &AdapterConfig) -> Result<Vec<StagedRecord>, AppError> {
        tracing::info!("Fetching data from REST API: {}", config.endpoint);

        // Get OAuth2 token if using OAuth2 client credentials
        let oauth_token = self.get_auth_token(&config.auth).await?;

        // Build the HTTP client and request
        let client = HttpClient::new_client();
        let mut request = client.get(&config.endpoint);

        // Add authentication
        if let Some(token) = oauth_token {
            // Convert OAuth2 token to Bearer
            request = request.header("Authorization", format!("Bearer {}", token));
        } else {
            request = HttpClient::add_auth(request, &config.auth);
        }

        // Add custom headers if specified
        if let Some(headers) = config.parameters.get("headers").and_then(|h| h.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key, value_str);
                }
            }
        }

        // Make the request
        let response = request
            .send()
            .await
            .map_err(|e| AppError::Http(format!("REST request failed: {}", e)))?;

        // Check status
        if !response.status().is_success() {
            return Err(AppError::Http(format!(
                "REST API returned error status: {}",
                response.status()
            )));
        }

        // Parse JSON response
        let json: Value = response
            .json()
            .await
            .map_err(|e| AppError::Http(format!("Failed to parse JSON response: {}", e)))?;

        tracing::debug!("REST API response: {:?}", json);

        // Transform to staged records
        let records = self.transform_response(json, config).await?;

        tracing::info!("Fetched {} records from REST API", records.len());

        Ok(records)
    }

    async fn test_connection(&self, config: &AdapterConfig) -> Result<bool, AppError> {
        tracing::info!("Testing connection to REST API: {}", config.endpoint);

        // Get OAuth2 token if needed
        let oauth_token = self.get_auth_token(&config.auth).await?;

        // Build a simple HEAD request to test connectivity
        let client = HttpClient::new_client();
        let mut request = client.head(&config.endpoint);

        // Add authentication
        if let Some(token) = oauth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        } else {
            request = HttpClient::add_auth(request, &config.auth);
        }

        // Make the request
        let response = request
            .send()
            .await
            .map_err(|e| AppError::Http(format!("Connection test failed: {}", e)))?;

        Ok(response.status().is_success())
    }

    fn default_config(&self) -> AdapterConfig {
        let mut config = AdapterConfig::new(
            self.adapter_type(),
            "rest-source",
            "https://api.example.com/data",
        );

        config.parameters = serde_json::json!({
            "data_path": "data",
            "default_tags": ["rest"],
            "headers": {
                "Accept": "application/json"
            }
        });

        config.polling_interval = Some(300); // 5 minutes

        config
    }
}

impl Default for RestAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_adapter_identity() {
        let adapter = RestAdapter::new();
        assert_eq!(adapter.adapter_type(), "rest_api");
        assert_eq!(adapter.name(), "REST API Adapter");
    }

    #[test]
    fn test_default_config() {
        let adapter = RestAdapter::new();
        let config = adapter.default_config();

        assert_eq!(config.adapter_type, "rest_api");
        assert_eq!(config.polling_interval, Some(300));
    }

    #[tokio::test]
    async fn test_transform_array_response() {
        let adapter = RestAdapter::new();
        let mut config = AdapterConfig::new("rest_api", "test", "http://test");
        config.parameters = json!({
            "data_path": "",
            "default_tags": ["test"]
        });

        let response = json!([
            {"id": 1, "title": "Test 1", "status": "active"},
            {"id": 2, "title": "Test 2", "status": "inactive"}
        ]);

        let records = adapter.transform_response(response, &config).await.unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].metadata.title, Some("Test 1".to_string()));
        assert_eq!(records[0].metadata.status, Some("active".to_string()));
        assert_eq!(records[0].metadata.tags, vec!["test".to_string()]);
    }

    #[tokio::test]
    async fn test_transform_nested_response() {
        let adapter = RestAdapter::new();
        let mut config = AdapterConfig::new("rest_api", "test", "http://test");
        config.parameters = json!({
            "data_path": "results",
            "default_tags": []
        });

        let response = json!({
            "status": "ok",
            "results": [
                {"id": 1, "name": "Item 1"}
            ]
        });

        let records = adapter.transform_response(response, &config).await.unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].metadata.title, Some("Item 1".to_string()));
    }
}
