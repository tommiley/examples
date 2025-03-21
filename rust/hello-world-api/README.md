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

### Using Build Scripts

This repository includes scripts to simplify the build and run process:

```bash
# Build the application
./build.sh

# Run the application
./start.sh
```

### Custom Platform Deployment (Dokploy, etc.)

For platforms like Dokploy, this project includes:

- `Dockerfile`: For Docker-based deployments
- `nixpacks.toml`: For Nixpacks-based deployments
- `buildpack.yml`: For buildpack-based deployments
- `Procfile`: For Heroku-like platforms
- `build.sh` and `start.sh`: For script-based deployments

#### Other PaaS Platforms
For other PaaS platforms that support Docker:
1. Push this repository to GitHub
2. Connect your hosting platform to the GitHub repository
3. Deploy using the Dockerfile method

## Troubleshooting Deployment Issues

### Common Issues:
- **Missing Rust**: The deployment environment may not have Rust installed. The Dockerfile should handle this.
- **Port Configuration**: Ensure the application is configured to listen on `0.0.0.0` (not `127.0.0.1`) and on the port specified by the environment.
- **Build Failures**: Check the build logs for specific error messages. The Dockerfile uses a multi-stage build to handle dependencies.
- **Script Execution**: Ensure build.sh and start.sh have executable permissions (`chmod +x build.sh start.sh`)

## Environment Variables

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