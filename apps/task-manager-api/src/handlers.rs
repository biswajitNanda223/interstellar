use crate::error::AppError;
use crate::models::{CreateTask, Db, Task, UpdateTask};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// Handler to retrieve all tasks from the database.
///
/// ### RUST LEARN: Thread Locks & State Extraction
/// * `State(db)`: Axum automatically extracts our shared database state.
/// * `db.read()`: Acquires a read lock on the database. Multiple threads can read simultaneously.
/// * `.map_err(...)`: Converts the lock error (if another thread panicked while holding the lock)
///   into our custom `AppError::DatabaseLockError`.
pub async fn get_tasks(State(db): State<Db>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = db.read().map_err(|_| AppError::DatabaseLockError)?;

    // We clone the vector of tasks to return them. In a real database,
    // this would be fetched from Postgres/SQLite instead of cloning memory.
    Ok(Json(tasks.clone()))
}

/// Handler to retrieve a single task by its ID.
pub async fn get_task_by_id(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> Result<Json<Task>, AppError> {
    let tasks = db.read().map_err(|_| AppError::DatabaseLockError)?;

    // Use an iterator to search for the task with the matching ID.
    // `.find()` returns an `Option<&Task>` (borrowed reference to the task inside the vector).
    let task = tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or(AppError::TaskNotFound(id))?; // Converts Option to Result: if None, returns TaskNotFound error.

    Ok(Json(task.clone()))
}

/// Handler to create a new task.
pub async fn create_task(
    State(db): State<Db>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    // Acquire a write lock because we are modifying the vector.
    // Only one thread can write at a time.
    let mut tasks = db.write().map_err(|_| AppError::DatabaseLockError)?;

    // Calculate a new simple ID (max current ID + 1, or 1 if empty)
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let new_task = Task {
        id: new_id,
        title: payload.title,
        description: payload.description,
        completed: false, // Default to incomplete
    };

    tasks.push(new_task.clone());

    // Return HTTP 201 Created and the created task in JSON format
    Ok((StatusCode::CREATED, Json(new_task)))
}

/// Handler to perform a partial update on a task.
pub async fn update_task(
    Path(id): Path<u64>,
    State(db): State<Db>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, AppError> {
    let mut tasks = db.write().map_err(|_| AppError::DatabaseLockError)?;

    // Find the task index
    let index = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(AppError::TaskNotFound(id))?;

    // Borrow the mutable reference to the task in-place inside our vector.
    let task = &mut tasks[index];

    // ### RUST LEARN: Pattern Matching with `if let`
    // We update fields only if the client provided them (i.e., if they are `Some(val)`).
    if let Some(title) = payload.title {
        task.title = title;
    }
    if let Some(description) = payload.description {
        task.description = Some(description);
    }
    if let Some(completed) = payload.completed {
        task.completed = completed;
    }

    Ok(Json(task.clone()))
}

/// Handler to delete a task by its ID.
pub async fn delete_task(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> Result<StatusCode, AppError> {
    let mut tasks = db.write().map_err(|_| AppError::DatabaseLockError)?;

    // Find the task index
    let index = tasks
        .iter()
        .position(|t| t.id == id)
        .ok_or(AppError::TaskNotFound(id))?;

    // Remove the task from the vector
    tasks.remove(index);

    // Return HTTP 204 No Content for a successful deletion
    Ok(StatusCode::NO_CONTENT)
}
