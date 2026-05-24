use crate::models::{Project, ProjectStatus};
use chrono::Utc;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub projects: Arc<RwLock<HashMap<Uuid, Project>>>,
    pub started_at: Instant,
}

impl AppState {
    pub fn seeded() -> Self {
        let now = Utc::now();
        let project = Project {
            id: Uuid::now_v7(),
            title: "CareerForge API".to_owned(),
            summary: "Production-shaped Rust API showing async routing, typed errors, validation, tests, observability, and Docker deployment.".to_owned(),
            stack: vec![
                "Rust".to_owned(),
                "Axum".to_owned(),
                "Tokio".to_owned(),
                "Docker".to_owned(),
            ],
            status: ProjectStatus::Shipped,
            impact_score: 92,
            created_at: now,
            updated_at: now,
        };

        Self {
            projects: Arc::new(RwLock::new(HashMap::from([(project.id, project)]))),
            started_at: Instant::now(),
        }
    }
}
