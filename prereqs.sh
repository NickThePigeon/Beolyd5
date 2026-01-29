#!/bin/bash

# Increase swap for compiling on low-memory devices (Pi 3B+ has only 1GB)
echo ">>> Configuring swap (needed for compiling)..."
sudo dphys-swapfile swapoff 2>/dev/null || true
sudo sed -i 's/CONF_SWAPSIZE=.*/CONF_SWAPSIZE=2048/' /etc/dphys-swapfile
sudo dphys-swapfile setup
sudo dphys-swapfile swapon

# Update the package lists for upgrades and new package installations
sudo apt-get update

# Install curl if not installed
sudo apt-get install -y curl

# Install build essentials
sudo apt-get install -y build-essential

# Install prerequisite libraries for Tauri
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev libhidapi-dev libudev-dev

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add cargo to PATH
source $HOME/.cargo/env

# Install Tauri CLI v2 (limit jobs to avoid OOM on Pi)
CARGO_BUILD_JOBS=1 cargo install tauri-cli --version "^2" --force

# Install Xorg and Matchbox for GUI support
sudo apt-get install -y xorg matchbox-window-manager
