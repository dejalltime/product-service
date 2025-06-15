# === Builder Stage ===
FROM rust:latest AS builder

WORKDIR /product-service

# Copy only Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Prefetch dependencies without triggering full build
RUN cargo fetch

# Now copy the actual source code
COPY . ./

# Build the actual application
RUN cargo build --release

# === Runtime Stage ===
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Set the build argument for the app version number
ARG APP_VERSION=0.1.0

# Install required system libraries
RUN apt-get update && \
    apt-get install -y wget libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /product-service/target/release/product-service /app/

# Set version env var
ENV APP_VERSION=$APP_VERSION

# Expose the default port
EXPOSE 3002

# Start the app
CMD ["./product-service"]
