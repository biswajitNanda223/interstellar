use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planned,
    Active,
    Shipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub summary: String,
    pub stack: Vec<String>,
    pub status: ProjectStatus,
    pub impact_score: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProject {
    #[validate(length(min = 3, max = 80))]
    pub title: String,
    #[validate(length(min = 12, max = 280))]
    pub summary: String,
    #[validate(length(min = 1, max = 8))]
    pub stack: Vec<String>,
    pub status: ProjectStatus,
    #[validate(range(min = 1, max = 100))]
    pub impact_score: u8,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProject {
    #[validate(length(min = 3, max = 80))]
    pub title: Option<String>,
    #[validate(length(min = 12, max = 280))]
    pub summary: Option<String>,
    #[validate(length(min = 1, max = 8))]
    pub stack: Option<Vec<String>>,
    pub status: Option<ProjectStatus>,
    #[validate(range(min = 1, max = 100))]
    pub impact_score: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub total_projects: usize,
    pub active_projects: usize,
    pub shipped_projects: usize,
    pub uptime_seconds: u64,
}
