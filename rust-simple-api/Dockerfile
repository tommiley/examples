FROM rust:alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

COPY . .

# Build the application
RUN cargo build --release

# Create a minimal runtime image
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the binary from the builder
COPY --from=builder /app/target/release/rust-simple-api /app/

# Make sure it's executable
RUN chmod +x /app/rust-simple-api

EXPOSE 8080

CMD ["./rust-simple-api"] 