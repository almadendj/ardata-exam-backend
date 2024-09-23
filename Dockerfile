FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

# Run the backend binary
CMD ["backend"]

