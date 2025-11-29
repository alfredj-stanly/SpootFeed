use axum::{Router, routing::get};

pub fn app() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "As strong as an Ox!\n"
}
