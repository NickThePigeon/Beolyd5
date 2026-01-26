#!/bin/bash
set -e

echo "=== Building Beolyd5 ==="
echo ""

# Add cargo to PATH
source $HOME/.cargo/env

# Navigate to UI directory
cd "$(dirname "$0")/src/ui"

# Install frontend dependencies
echo ">>> Installing Node dependencies..."
yarn install

# Build the application
echo ">>> Building Tauri application (this may take a while on Pi)..."
cargo tauri build --jobs 2

echo ""
echo "=== Build complete! ==="
echo "Binary located at: target/release/bs5-controller-ui"
echo ""
echo "To install, run: ./raspberry-os-install.sh"
