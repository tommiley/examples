# Hello World Rust Web API

A simple RESTful API built with Rust and Actix-web.

## Prerequisites

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)

## Running the API

```bash
# Navigate to the project directory
cd hello-world-api

# Build and run the application
cargo run
```

## Available Endpoints

- `GET /`: Returns a "Hello, World!" message
- `GET /hello/{name}`: Returns a personalized greeting with the provided name

## Example Usage

```bash
# Get default greeting
curl http://127.0.0.1:8080/

# Get personalized greeting
curl http://127.0.0.1:8080/hello/Rustacean
``` 