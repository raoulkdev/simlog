use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::models::{Flight, FlightPayload, FlightQueryPayload, FlightQueryResponse};

// Add a new flight to database
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
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Error adding flight: {e}")),
        )
            .into_response(),
    }
}

// Get all flights in the database
pub async fn get_all_flights(State(db): State<Arc<Pool<Postgres>>>) -> impl IntoResponse {
    match sqlx::query_as::<_, Flight>("SELECT * FROM flights")
        .fetch_all(&*db)
        .await
    {
        Ok(all_flights) => (StatusCode::OK, Json(all_flights)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Error getting all flights: {e}")),
        )
            .into_response(),
    }
}

// Get an individual flight in the database by UUID
pub async fn get_flight_by_id(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Flight>("SELECT * FROM flights WHERE id = $1")
        .bind(&id)
        .fetch_one(&*db)
        .await
    {
        Ok(flight) => (StatusCode::OK, Json(flight)).into_response(),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            Json(format!("No flight found with id:{id}")),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Error getting flight with id:{id}, Error: {e}")),
        )
            .into_response(),
    }
}

//  Handle user ai request
pub async fn handle_user_flight_ai_query(
    Json(payload): Json<FlightQueryPayload>,
) -> impl IntoResponse {
    let ai_service_url =
        std::env::var("AI_SERVICE_URL").expect(".env error: no value set for AI_SERVICE_URL");

    match reqwest::Client::new()
        .post(format!("{ai_service_url}/query_ai"))
        .json(&payload)
        .send()
        .await
    {
        Ok(response_raw) => match response_raw.json::<FlightQueryResponse>().await {
            Ok(response) => (StatusCode::OK, Json(response)).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Error getting ai response, Error: {e}")),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Error getting ai response, Error: {e}")),
        )
            .into_response(),
    }
}
