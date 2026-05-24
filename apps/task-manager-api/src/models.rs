use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

/// Represents a single To-Do Task in our application.
///
/// ### RUST LEARN: Attributes and Derives
/// The `#[derive(...)]` attribute automatically generates standard implementations for traits.
/// * `Serialize`: Converts this struct into JSON (used by Axum to return JSON responses).
/// * `Deserialize`: Converts JSON input from requests back into this Rust struct.
/// * `Clone`: Allows cheap duplicate creation of tasks (needed since we borrow and copy data inside locks).
/// * `Debug`: Allows us to format the struct with `{:?}` for logging and debugging.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>, // Option represents value that could be present or absent (None)
    pub completed: bool,
}

/// Request payload for creating a new Task.
#[derive(Deserialize, Debug)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

/// Request payload for updating an existing Task.
/// Every field is wrapped in `Option` so clients can perform partial updates (patching).
#[derive(Deserialize, Debug)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

/// ### RUST LEARN: Thread-Safe Shared State
/// In multi-threaded async web servers (like Axum/Tokio), multiple threads handle requests concurrently.
/// To share our list of tasks safely:
/// 1. **`Arc` (Atomic Reference Counter)**: A smart pointer that shares ownership of the data.
///    It keeps track of the number of references to the data and deallocates it when the count hits zero.
/// 2. **`RwLock` (Reader-Writer Lock)**: Provides mutual exclusion. Multiple threads can read concurrently,
///    but only one thread can write at a time. This prevents data races.
pub type Db = Arc<RwLock<Vec<Task>>>;

/// Helper function to initialize our in-memory database with some default mock tasks.
pub fn create_db() -> Db {
    Arc::new(RwLock::new(vec![
        Task {
            id: 1,
            title: String::from("Learn Rust Basics"),
            description: Some(String::from(
                "Read about Ownership, Borrowing, and Lifetimes.",
            )),
            completed: true,
        },
        Task {
            id: 2,
            title: String::from("Build modular Axum API"),
            description: Some(String::from(
                "Create a clean REST API with proper error handling.",
            )),
            completed: false,
        },
        Task {
            id: 3,
            title: String::from("Dockerize Rust Application"),
            description: Some(String::from(
                "Write a multi-stage Dockerfile and test running it.",
            )),
            completed: false,
        },
    ]))
}
