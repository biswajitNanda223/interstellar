use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// Declare the other modules in our project.
// Rust modules form a tree, and we must explicitly declare submodules here.
pub mod error;
pub mod handlers;
pub mod models;

/// ### RUST LEARN: Async Entry Point
/// Web servers are highly concurrent. `#[tokio::main]` converts our standard sync `main`
/// into an async entry point, spinning up Tokio's multi-threaded work-stealing scheduler.
#[tokio::main]
async fn main() {
    // 1. Initialize the shared thread-safe database state.
    let db = models::create_db();

    // 2. Configure CORS (Cross-Origin Resource Sharing) middleware
    // This allows web pages from other domains to talk to our API.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    // 3. Define our application router.
    // We map HTTP paths + methods directly to our async handler functions.
    let app = Router::new()
        // /tasks endpoints: GET to retrieve all, POST to create a new one.
        .route(
            "/tasks",
            get(handlers::get_tasks).post(handlers::create_task),
        )
        // /tasks/:id endpoints: GET specific task, PATCH to update, DELETE to remove.
        .route(
            "/tasks/:id",
            get(handlers::get_task_by_id)
                .patch(handlers::update_task)
                .delete(handlers::delete_task),
        )
        // Add middleware layer
        .layer(cors)
        // Provide the shared thread-safe database state to all handlers in the router.
        .with_state(db);

    // 4. Bind the TCP server.
    // We listen on 0.0.0.0 instead of 127.0.0.1 so that it is accessible
    // from outside a Docker container (mapping ports correctly).
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("=========================================");
    println!("     RUST TASK REST API RUNNING          ");
    println!("=========================================");
    println!("Listening on: http://localhost:3000");
    println!("Endpoints:");
    println!("  - GET   /tasks      (List all tasks)");
    println!("  - POST  /tasks      (Create a task)");
    println!("  - GET   /tasks/:id  (Get a single task)");
    println!("  - PATCH /tasks/:id  (Update a task)");
    println!("  - DEL   /tasks/:id  (Delete a task)");
    println!("=========================================");

    // 5. Start the web server.
    // It runs forever, serving incoming requests asynchronously!
    axum::serve(listener, app).await.unwrap();
}
