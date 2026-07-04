use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

use crate::handlers::{create_flight, get_all_flights, get_flight_by_id};

// Router
pub fn router(db: Arc<Pool<Postgres>>) -> Router {
    Router::new()
        .route("/flights", post(create_flight))
        .route("/flights", get(get_all_flights))
        .route("/flights/{capture}", get(get_flight_by_id))
        .with_state(db.clone())
}
