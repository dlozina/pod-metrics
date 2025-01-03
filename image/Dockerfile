# Build stage
FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app
# Install required dependencies for building
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install SSL certificates and clean up
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/pod_metrics /app/pod_metrics

# Create config directory for ConfigMap mounting
RUN mkdir -p /etc/config

# Set the binary as the entrypoint
ENTRYPOINT ["/app/pod_metrics"]