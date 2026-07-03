use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use uuid::Uuid;

// Flight Struct
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct Flight {
    // ID
    id: Uuid,

    // Base flight info
    flight_date: NaiveDate,
    aircraft: String,
    simulator: String,

    // Airport info
    departure_airport: String,
    arrival_airport: String,

    // Flight time
    duration_minutes: i16,

    // Weather info
    weather_conditions: String,
    time_of_day: String,
    wind_conditions: String,

    // Takeoff and landing info
    takeoff_type: String,
    approach_type: String,
    landing_quality_rating: i8,

    // ATC info
    atc_used: bool,

    // Failures and remarks info
    failures_emergencies: String,
    remarks: String,

    // Timestamps
    created_at: DateTime<Utc>
}

// Flight payload struct
#[derive(Deserialize, Serialize, sqlx::FromRow, Decode, Encode)]
pub struct FlightPayload {
    // Base flight info
    flight_date: NaiveDate,
    aircraft: String,
    simulator: String,

    // Airport info
    departure_airport: String,
    arrival_airport: String,

    // Flight time
    duration_minutes: i16,

    // Weather info
    weather_conditions: String,
    time_of_day: String,
    wind_conditions: String,

    // Takeoff and landing info
    takeoff_type: String,
    approach_type: String,
    landing_quality_rating: i8,

    // ATC info
    atc_used: bool,

    // Failures and remarks info
    failures_emergencies: String,
    remarks: String
}