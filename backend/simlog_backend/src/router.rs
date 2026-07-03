use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

use crate::handlers::create_flight;

// Router
pub fn router(db: Arc<Pool<Postgres>>) -> Router {
    Router::new()
        .route("/flights", post(create_flight))
        .route("/flights", get("Get all flights"))
        .route("/flights/{capture}", get("Get a specific flight"))
        .with_state(db.clone())
}
