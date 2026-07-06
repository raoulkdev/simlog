# -- Stage 1: Build -----------------------------------------------------------
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache deps before copying source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source + sqlx offline data
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations

ENV SQLX_OFFLINE=true

RUN touch src/main.rs && cargo build --release

# -- Stage 2: Runtime ---------------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/simlog_backend .

EXPOSE 3000
CMD ["./simlog_backend"]
