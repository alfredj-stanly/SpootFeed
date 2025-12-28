use axum::{Router, routing::get};

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/v1", get(|| async { "Hey, there!" }))
}

async fn health() -> &'static str {
    "As strong as an Oak!\n"
}

async fn index() -> &'static str {
    "Hey, there is joy in every hello!\n"
}
