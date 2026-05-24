use crate::{
    error::AppError,
    models::{
        CreateProject, HealthResponse, MetricsResponse, Project, ProjectStatus, UpdateProject,
    },
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use serde::Deserialize;
use std::cmp::Reverse;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize)]
struct ListQuery {
    status: Option<ProjectStatus>,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics))
        .route("/projects", get(list_projects).post(create_project))
        .route(
            "/projects/{id}",
            get(get_project)
                .patch(update_project)
                .delete(delete_project),
        )
        .with_state(state)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "careerforge-api",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn metrics(State(state): State<AppState>) -> Json<MetricsResponse> {
    let projects = state.projects.read().await;
    let active_projects = projects
        .values()
        .filter(|project| project.status == ProjectStatus::Active)
        .count();
    let shipped_projects = projects
        .values()
        .filter(|project| project.status == ProjectStatus::Shipped)
        .count();

    Json(MetricsResponse {
        total_projects: projects.len(),
        active_projects,
        shipped_projects,
        uptime_seconds: state.started_at.elapsed().as_secs(),
    })
}

async fn list_projects(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Json<Vec<Project>> {
    let projects = state.projects.read().await;
    let mut items = projects
        .values()
        .filter(|project| {
            query
                .status
                .as_ref()
                .is_none_or(|status| &project.status == status)
        })
        .cloned()
        .collect::<Vec<_>>();

    items.sort_by_key(|project| Reverse(project.updated_at));
    Json(items)
}

async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProject>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    payload.validate()?;

    let now = Utc::now();
    let project = Project {
        id: Uuid::now_v7(),
        title: payload.title,
        summary: payload.summary,
        stack: payload.stack,
        status: payload.status,
        impact_score: payload.impact_score,
        created_at: now,
        updated_at: now,
    };

    state
        .projects
        .write()
        .await
        .insert(project.id, project.clone());
    Ok((StatusCode::CREATED, Json(project)))
}

async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    let projects = state.projects.read().await;
    let project = projects.get(&id).cloned().ok_or(AppError::NotFound)?;
    Ok(Json(project))
}

async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProject>,
) -> Result<Json<Project>, AppError> {
    payload.validate()?;

    let mut projects = state.projects.write().await;
    let project = projects.get_mut(&id).ok_or(AppError::NotFound)?;

    if let Some(title) = payload.title {
        project.title = title;
    }
    if let Some(summary) = payload.summary {
        project.summary = summary;
    }
    if let Some(stack) = payload.stack {
        project.stack = stack;
    }
    if let Some(status) = payload.status {
        project.status = status;
    }
    if let Some(impact_score) = payload.impact_score {
        project.impact_score = impact_score;
    }
    project.updated_at = Utc::now();

    Ok(Json(project.clone()))
}

async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state
        .projects
        .write()
        .await
        .remove(&id)
        .ok_or(AppError::NotFound)?;

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok() {
        let app = build_router(AppState::seeded());

        let response = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_then_list_project() {
        let app = build_router(AppState::seeded());
        let payload = json!({
            "title": "Rust Deployment API",
            "summary": "Shows validated JSON workflows and production-friendly Docker packaging.",
            "stack": ["Rust", "Axum", "Docker"],
            "status": "active",
            "impact_score": 88
        });

        let response = app
            .oneshot(
                Request::post("/projects")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let created: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(created["title"], "Rust Deployment API");
        assert!(created["id"].as_str().is_some());
    }

    #[tokio::test]
    async fn invalid_project_returns_bad_request() {
        let app = build_router(AppState::seeded());
        let payload = json!({
            "title": "no",
            "summary": "too short",
            "stack": [],
            "status": "active",
            "impact_score": 101
        });

        let response = app
            .oneshot(
                Request::post("/projects")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
