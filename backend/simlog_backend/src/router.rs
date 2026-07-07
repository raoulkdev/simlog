use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use reqwest::Method;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{
    create_flight, get_all_flights, get_flight_by_id, handle_user_flight_ai_query,
};

// Router
pub fn router(db: Arc<Pool<Postgres>>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);
    
    Router::new()
        .route("/flights", post(create_flight))
        .route("/flights", get(get_all_flights))
        .route("/flights/{capture}", get(get_flight_by_id))
        .route("/flights/query_ai", post(handle_user_flight_ai_query))
        .with_state(db.clone())
        .layer(cors)
}
