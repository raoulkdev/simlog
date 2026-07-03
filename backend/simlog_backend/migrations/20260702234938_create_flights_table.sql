CREATE TABLE IF NOT EXISTS flights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    flight_date DATE NOT NULL,
    aircraft TEXT NOT NULL,
    simulator TEXT NOT NULL,

    departure_airport TEXT NOT NULL,
    arrival_airport TEXT NOT NULL,

    duration_minutes INTEGER NOT NULL CHECK (duration_minutes >= 0),

    weather_conditions TEXT NOT NULL CHECK (weather_conditions IN ('VFR', 'IFR')),
    time_of_day TEXT NOT NULL CHECK (time_of_day IN ('day', 'night')),
    wind_conditions TEXT,

    takeoff_type TEXT,
    approach_type TEXT,
    landing_quality_rating SMALLINT CHECK (landing_quality_rating BETWEEN 1 AND 5),

    atc_used BOOLEAN NOT NULL DEFAULT FALSE,

    failures_emergencies TEXT,
    remarks TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);