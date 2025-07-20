#!/bin/bash

set -e

echo "Checking if Rust is already installed..."

if command -v cargo >/dev/null 2>&1; then
    echo "Rust and Cargo are already installed."
    cargo --version
    rustc --version
    exit 0
fi

echo "📦 Installing Rust via rustup (https://rustup.rs)..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


export PATH="$HOME/.cargo/bin:$PATH"

echo "Rust installed successfully!"
echo "Verifying installation..."
cargo --version
rustc --version