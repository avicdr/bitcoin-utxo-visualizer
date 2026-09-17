# Multi-stage Dockerfile for Rust Backend
FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY . .
RUN cargo build --release

# Final runtime image
FROM debian:bullseye-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/bitcoin-utxo-backend /usr/local/bin/bitcoin-utxo-backend
COPY migrations /app/migrations

ENV PORT=8080
EXPOSE 8080
CMD ["bitcoin-utxo-backend"]
