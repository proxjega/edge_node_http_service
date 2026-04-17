# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/edge_node_http_service .

# Copy configuration file
COPY config.json .

# Expose port (adjust if your app uses a different port)
EXPOSE 8080

# Run the application
CMD ["./edge_node_http_service"]
