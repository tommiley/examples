FROM rust:1.75 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/hello-world-api /app/

# Update the code to listen on 0.0.0.0 instead of 127.0.0.1
EXPOSE 8080

CMD ["./hello-world-api"] 