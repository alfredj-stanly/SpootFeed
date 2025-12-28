# Stage 1: Build
FROM rust:1.91-bookworm AS builder

# Install protobuf compiler (needed for proto crate)
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release --package feeder

# Stage 2: Runtime (slim)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/feeder /app/feeder

# Expose port
EXPOSE 7878

# Run
CMD ["./feeder"]
