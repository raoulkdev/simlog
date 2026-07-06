use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use uuid::Uuid;

// Flight Struct
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct Flight {
    // ID
    pub id: Uuid,

    // Base flight info
    pub flight_date: NaiveDate,
    pub aircraft: String,
    pub simulator: String,

    // Airport info
    pub departure_airport: String,
    pub arrival_airport: String,

    // Flight time
    pub duration_minutes: i32,

    // Weather info
    pub weather_conditions: String,
    pub time_of_day: String,
    pub wind_conditions: String,

    // Takeoff and landing info
    pub takeoff_type: Option<String>,
    pub approach_type: Option<String>,
    pub landing_quality_rating: i16,

    // ATC info
    pub atc_used: bool,

    // Failures and remarks info
    pub failures_emergencies: Option<String>,
    pub remarks: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
}

// Flight payload struct
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct FlightPayload {
    // Base flight info
    pub flight_date: NaiveDate,
    pub aircraft: String,
    pub simulator: String,

    // Airport info
    pub departure_airport: String,
    pub arrival_airport: String,

    // Flight time
    pub duration_minutes: i32,

    // Weather info
    pub weather_conditions: String,
    pub time_of_day: String,
    pub wind_conditions: String,

    // Takeoff and landing info
    pub takeoff_type: Option<String>,
    pub approach_type: Option<String>,
    pub landing_quality_rating: i16,

    // ATC info
    pub atc_used: bool,

    // Failures and remarks info
    pub failures_emergencies: Option<String>,
    pub remarks: Option<String>,
}

// User AI flight query payload model
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct FlightQueryPayload {
    pub flight_id: Uuid,
    pub user_query: String,
}

// User AI flight query response model
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct FlightQueryResponse {
    pub query_response: String,
}
