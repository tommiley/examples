# Simple Rust Hello World API

A minimal Rust web API that returns "Hello, World!" at the root endpoint.

## Running with Docker

This project uses an Alpine-based Dockerfile for better portability and fewer compatibility issues.

Build and run the Docker container:

```bash
# Build the image
docker build -t rust-simple-api .

# Run the container
docker run -p 8080:8080 rust-simple-api
```

## Testing the API

Once running, test the API with:

```bash
curl http://localhost:8080/
```

You should see: `Hello, World!`

## Troubleshooting

If deployment fails, check the build logs from your platform (e.g., Dokploy). Common issues:
- **Network Errors**: During `apk add` or `cargo build`.
- **Build Dependencies**: Ensure `musl-dev` is installed correctly in the build stage.
- **Port Mappings**: Make sure the platform is correctly mapping external port 80 to the container's port 8080. 