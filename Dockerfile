# === Builder Stage ===
FROM rust:1.80.0 AS builder

WORKDIR /product-service

# Copy only Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to precompile dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Build dependencies to leverage Docker cache
RUN cargo build --release && rm -rf src

# Now copy actual source
COPY . ./

# Build the real app
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

# Expose the default port if you want to document it
EXPOSE 3002

# Start the app
CMD ["./product-service"]
