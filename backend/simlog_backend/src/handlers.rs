use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use sqlx::{Pool, Postgres};

use crate::flight::{Flight, FlightPayload};

pub async fn create_flight(
    State(db): State<Arc<Pool<Postgres>>>,
    Json(payload): Json<FlightPayload>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Flight>(
        "
        INSERT INTO flights (
            flight_date, 
            aircraft, 
            simulator,
            departure_airport,
            arrival_airport,
            duration_minutes,
            weather_conditions,
            time_of_day,
            wind_conditions,
            takeoff_type,
            approach_type,
            landing_quality_rating,
            atc_used,
            failures_emergencies,
            remarks
        )
        
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        
        RETURNING *
    ",
    )
    .bind(&payload.flight_date)
    .bind(&payload.aircraft)
    .bind(&payload.simulator)
    .bind(&payload.departure_airport)
    .bind(&payload.arrival_airport)
    .bind(&payload.duration_minutes)
    .bind(&payload.weather_conditions)
    .bind(&payload.time_of_day)
    .bind(&payload.wind_conditions)
    .bind(&payload.takeoff_type)
    .bind(&payload.approach_type)
    .bind(&payload.landing_quality_rating)
    .bind(&payload.atc_used)
    .bind(&payload.failures_emergencies)
    .bind(&payload.remarks)
    .fetch_one(&*db)
    .await
    {
        Ok(added_flight) => (StatusCode::OK, Json(added_flight)).into_response(),
        Err(e) => (StatusCode::OK, Json(format!("Error adding flight: {e}"))).into_response(),
    }
}
