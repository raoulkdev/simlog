# Simlog Backend

Backend API for logging simulator flights and querying AI for deeper insight into each flight.

> This project was built for learning.

## Why I built this

I built Simlog Backend to practice designing and shipping a multi-service backend:
- building REST APIs in Rust
- working with PostgreSQL + migrations
- integrating a separate AI microservice
- containerizing services with Docker Compose

## What this backend does

- Accepts and stores detailed flight logs
- Returns all flights or a single flight by ID
- Sends flight context + a natural-language prompt to an AI service and returns the response

There is no authentication layer in this project.

## Tech stack

- **Rust** (Axum, SQLx, Tokio) - core API
- **Python** (FastAPI, Google GenAI SDK) - AI service
- **PostgreSQL** - data storage
- **Docker Compose** - local orchestration

## Run locally (Docker)

```bash
cd backend
docker compose up --build
```

API is available at: `http://localhost:3000`

## API routes

| Method | Route | Description |
| --- | --- | --- |
| `POST` | `/flights` | Create a new flight log |
| `GET` | `/flights` | Get all flight logs |
| `GET` | `/flights/{id}` | Get one flight log by UUID |
| `POST` | `/flights/query_ai` | Ask AI about a specific flight |

## Example curl requests

Create a flight:

```bash
curl -X POST http://localhost:3000/flights \
  -H "Content-Type: application/json" \
  -d '{
    "flight_date": "2026-06-01",
    "aircraft": "Airbus A380-800",
    "simulator": "X-Plane 12",
    "departure_airport": "OMDB",
    "arrival_airport": "YSSY",
    "duration_minutes": 862,
    "weather_conditions": "VFR",
    "time_of_day": "night",
    "wind_conditions": "130@12",
    "takeoff_type": "normal",
    "approach_type": "ILS",
    "landing_quality_rating": 5,
    "atc_used": true,
    "failures_emergencies": null,
    "remarks": "Dubai to Sydney long haul."
  }'
```

Get all flights:

```bash
curl http://localhost:3000/flights
```

Get one flight by ID:

```bash
curl http://localhost:3000/flights/<flight-uuid>
```

Query AI about a flight:

```bash
curl -X POST http://localhost:3000/flights/query_ai \
  -H "Content-Type: application/json" \
  -d '{
    "flight_id": "<flight-uuid>",
    "user_query": "Summarize this flight and point out what went well."
  }'
```
