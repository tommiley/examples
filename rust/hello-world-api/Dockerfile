FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/hello-world-api /app/

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./hello-world-api"] 