#!/bin/bash

if command -v cargo >/dev/null 2>&1; then
    echo "Cargo is installed. Running the program..."
    cargo --version
    rustc --version
    cargo run -- $1
else
    bash install_tools.sh
    exit 1
fi
