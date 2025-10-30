// Ticket system module
//
// Provides CRUD operations for tickets (native and external)
// Stores tickets in SurrealDB with a generic model

use crate::db::Database;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ============================================================================
// Ticket Model
// ============================================================================

/// Internal ticket structure that matches SurrealDB's response format (with Thing ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TicketRecord {
    pub id: Thing,
    pub source: TicketSource,
    pub source_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub ticket_type: TicketType,
    pub status: String,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
    pub estimate: Option<f64>,
    pub time_spent: Option<f64>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub parent_id: Option<String>,
    pub linked_tickets: Vec<String>,
    pub comments: Vec<Comment>,
    pub metadata: serde_json::Value,
}

/// User-facing ticket structure with String ID (for frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub source: TicketSource,
    pub source_id: Option<String>,

    pub title: String,
    pub description: Option<String>,
    pub ticket_type: TicketType,

    pub status: String,
    pub priority: Priority,
    pub tags: Vec<String>,

    pub assignee: Option<String>,
    pub reporter: Option<String>,

    pub estimate: Option<f64>,
    pub time_spent: Option<f64>,
    pub due_date: Option<String>,

    pub created_at: String,
    pub updated_at: String,

    pub parent_id: Option<String>,
    pub linked_tickets: Vec<String>,

    pub comments: Vec<Comment>,
    pub metadata: serde_json::Value,
}

impl From<TicketRecord> for Ticket {
    fn from(record: TicketRecord) -> Self {
        Ticket {
            id: record.id.to_string(),
            source: record.source,
            source_id: record.source_id,
            title: record.title,
            description: record.description,
            ticket_type: record.ticket_type,
            status: record.status,
            priority: record.priority,
            tags: record.tags,
            assignee: record.assignee,
            reporter: record.reporter,
            estimate: record.estimate,
            time_spent: record.time_spent,
            due_date: record.due_date,
            created_at: record.created_at,
            updated_at: record.updated_at,
            parent_id: record.parent_id,
            linked_tickets: record.linked_tickets,
            comments: record.comments,
            metadata: record.metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketSource {
    Native,
    Jira,
    GitLab,
    GitHub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketType {
    Task,
    Bug,
    Feature,
    Epic,
    Story,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub text: String,
    pub created_at: String,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub ticket_type: TicketType,
    pub priority: Option<Priority>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
    pub estimate: Option<f64>,
    pub due_date: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<Priority>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
    pub estimate: Option<f64>,
    pub time_spent: Option<f64>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TicketFilters {
    pub source: Option<TicketSource>,
    pub ticket_type: Option<TicketType>,
    pub status: Option<String>,
    pub priority: Option<Priority>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub author: String,
    pub text: String,
}

// ============================================================================
// Ticket Operations
// ============================================================================

impl Database {
    /// Create a new native ticket
    pub async fn create_ticket(&self, req: CreateTicketRequest) -> Result<Ticket, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        // Create ticket data without id - SurrealDB will generate it
        let ticket_data = serde_json::json!({
            "source": "native",
            "source_id": serde_json::Value::Null,
            "title": req.title,
            "description": req.description,
            "ticket_type": req.ticket_type,
            "status": "todo",
            "priority": req.priority.unwrap_or(Priority::Medium),
            "tags": req.tags.unwrap_or_default(),
            "assignee": req.assignee,
            "reporter": serde_json::Value::Null,
            "estimate": req.estimate,
            "time_spent": serde_json::Value::Null,
            "due_date": req.due_date,
            "created_at": &now,
            "updated_at": &now,
            "parent_id": serde_json::Value::Null,
            "linked_tickets": Vec::<String>::new(),
            "comments": Vec::<Comment>::new(),
            "metadata": req.metadata.unwrap_or_else(|| serde_json::json!({}))
        });

        // Store in database - use simple CREATE, SurrealDB will generate ID
        let query = format!("CREATE tickets CONTENT {}", ticket_data);
        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to create ticket: {}", e)))?;

        let created: Option<TicketRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse created ticket: {}", e)))?;

        created
            .map(|record| record.into())
            .ok_or_else(|| AppError::Database("Ticket creation returned no result".to_string()))
    }

    /// Update an existing ticket
    pub async fn update_ticket(
        &self,
        id: &str,
        req: UpdateTicketRequest,
    ) -> Result<Ticket, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let id_owned = id.to_string();

        // Build update query dynamically based on provided fields
        let mut updates = Vec::new();

        if let Some(title) = req.title {
            updates.push(format!("title = '{}'", title.replace("'", "''")));
        }
        if let Some(description) = req.description {
            updates.push(format!(
                "description = '{}'",
                description.replace("'", "''")
            ));
        }
        if let Some(status) = req.status {
            updates.push(format!("status = '{}'", status.replace("'", "''")));
        }
        if let Some(priority) = req.priority {
            updates.push(format!("priority = '{:?}'", priority).to_lowercase());
        }
        if let Some(assignee) = req.assignee {
            updates.push(format!("assignee = '{}'", assignee.replace("'", "''")));
        }
        if let Some(tags) = req.tags {
            let tags_json = serde_json::to_string(&tags).unwrap_or_else(|_| "[]".to_string());
            updates.push(format!("tags = {}", tags_json));
        }
        if let Some(estimate) = req.estimate {
            updates.push(format!("estimate = {}", estimate));
        }
        if let Some(time_spent) = req.time_spent {
            updates.push(format!("time_spent = {}", time_spent));
        }
        if let Some(due_date) = req.due_date {
            updates.push(format!("due_date = '{}'", due_date.replace("'", "''")));
        }

        updates.push(format!("updated_at = '{}'", now));

        if updates.is_empty() {
            return Err(AppError::Validation("No fields to update".to_string()));
        }

        let query = format!("UPDATE {} SET {}", id_owned, updates.join(", "));
        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update ticket: {}", e)))?;

        let updated: Option<TicketRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse updated ticket: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| AppError::NotFound(format!("Ticket not found: {}", id_owned)))
    }

    /// Delete a ticket
    pub async fn delete_ticket(&self, id: &str) -> Result<(), AppError> {
        let id_owned = id.to_string();
        let query = format!("DELETE {}", id_owned);
        self.db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete ticket: {}", e)))?;

        Ok(())
    }

    /// Get tickets with optional filters
    pub async fn get_tickets(
        &self,
        filters: Option<TicketFilters>,
    ) -> Result<Vec<Ticket>, AppError> {
        let mut query = "SELECT * FROM tickets".to_string();
        let mut conditions = Vec::new();

        if let Some(f) = filters {
            if let Some(source) = f.source {
                conditions.push(format!("source = '{:?}'", source).to_lowercase());
            }
            if let Some(ticket_type) = f.ticket_type {
                conditions.push(format!("ticket_type = '{:?}'", ticket_type).to_lowercase());
            }
            if let Some(status) = f.status {
                conditions.push(format!("status = '{}'", status));
            }
            if let Some(priority) = f.priority {
                conditions.push(format!("priority = '{:?}'", priority).to_lowercase());
            }
            if let Some(assignee) = f.assignee {
                conditions.push(format!("assignee = '{}'", assignee));
            }
            if let Some(tags) = f.tags {
                for tag in tags {
                    conditions.push(format!("'{}' IN tags", tag));
                }
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY created_at DESC");

        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to query tickets: {}", e)))?;

        let tickets: Vec<TicketRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse tickets: {}", e)))?;

        Ok(tickets.into_iter().map(|r| r.into()).collect())
    }

    /// Move ticket to different status
    pub async fn move_ticket(&self, id: &str, new_status: &str) -> Result<Ticket, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let id_owned = id.to_string();
        let status_owned = new_status.to_string();

        let query = format!(
            "UPDATE {} SET status = '{}', updated_at = '{}'",
            id_owned,
            status_owned.replace("'", "''"),
            now
        );
        let mut result = self
            .db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to move ticket: {}", e)))?;

        let updated: Option<TicketRecord> = result
            .take(0)
            .map_err(|e| AppError::Database(format!("Failed to parse moved ticket: {}", e)))?;

        updated
            .map(|record| record.into())
            .ok_or_else(|| AppError::NotFound(format!("Ticket not found: {}", id_owned)))
    }

    /// Add comment to ticket
    pub async fn add_comment(
        &self,
        ticket_id: &str,
        req: CreateCommentRequest,
    ) -> Result<Comment, AppError> {
        let ticket_id_owned = ticket_id.to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let comment = Comment {
            id: uuid::Uuid::new_v4().to_string(),
            author: req.author,
            text: req.text,
            created_at: now.clone(),
        };

        let comment_json = serde_json::to_string(&comment)
            .map_err(|e| AppError::Database(format!("Failed to serialize comment: {}", e)))?;

        let query = format!(
            "UPDATE {} SET comments += {}, updated_at = '{}'",
            ticket_id_owned, comment_json, now
        );
        self.db
            .query(query)
            .await
            .map_err(|e| AppError::Database(format!("Failed to add comment: {}", e)))?;

        Ok(comment)
    }
}
