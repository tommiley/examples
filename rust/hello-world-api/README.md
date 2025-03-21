# Hello World Rust Web API

A simple RESTful API built with Rust and Actix-web.

## Prerequisites

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)

## Running the API Locally

```bash
# Navigate to the project directory
cd hello-world-api

# Build and run the application
cargo run
```

## Deployment Options

### Using Docker

```bash
# Build the Docker image
docker build -t hello-world-api .

# Run the container
docker run -p 8080:8080 hello-world-api
```

### Using Cloud Platforms

This API can be deployed to cloud platforms that support Docker containers or Rust applications.

#### Environment Variables

- `PORT`: Set the port for the server (default: 8080)

## Available Endpoints

- `GET /`: Returns a "Hello, World!" message
- `GET /hello/{name}`: Returns a personalized greeting with the provided name

## Example Usage

```bash
# Get default greeting
curl http://localhost:8080/

# Get personalized greeting
curl http://localhost:8080/hello/Rustacean
``` 