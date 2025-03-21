#!/bin/bash
set -e

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Build the application
cargo build --release

# Prepare bin directory
mkdir -p bin
cp target/release/hello-world-api bin/

echo "Build completed successfully!" 