use axum::{routing::get, Router};

use crate::handler::{get_route_encode_handler, health_checker_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/route/encode", get(get_route_encode_handler))
}
