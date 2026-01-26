#!/bin/bash
set -e

echo "=== Beolyd5 Raspberry Pi Setup ==="
echo ""

# Update the package lists for upgrades and new package installations
echo ">>> Updating packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install curl if not installed
sudo apt-get install -y curl wget git

# Install build essentials
echo ">>> Installing build tools..."
sudo apt-get install -y build-essential pkg-config

# Install prerequisite libraries for Tauri
echo ">>> Installing Tauri dependencies..."
# Try webkit2gtk-4.1 first (Debian Bookworm/newer), fall back to 4.0 (Debian Bullseye/older)
if apt-cache show libwebkit2gtk-4.1-dev > /dev/null 2>&1; then
    echo "Using webkit2gtk-4.1 (Debian Bookworm)"
    sudo apt-get install -y libwebkit2gtk-4.1-dev
else
    echo "Using webkit2gtk-4.0 (Debian Bullseye)"
    sudo apt-get install -y libwebkit2gtk-4.0-dev
fi
sudo apt-get install -y libappindicator3-dev libsecret-1-dev || true
sudo apt-get install -y libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Install hidapi for BeoSound5 controller
echo ">>> Installing hidapi for USB controller..."
sudo apt-get install -y libhidapi-dev libudev-dev

# Install Node.js and Yarn
echo ">>> Installing Node.js and Yarn..."
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g yarn

# Install Rust
echo ">>> Installing Rust..."
if command -v rustc &> /dev/null; then
    echo "Rust already installed"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# Add cargo to PATH
source $HOME/.cargo/env

# Install Tauri CLI
echo ">>> Installing Tauri CLI..."
cargo install tauri-cli --force

# Install Xorg and Matchbox for GUI support
echo ">>> Installing X server..."
sudo apt-get install -y xorg matchbox-window-manager

# Setup udev rules for BeoSound5 controller (USB HID)
echo ">>> Setting up USB permissions for BeoSound5 controller..."
sudo tee /etc/udev/rules.d/99-beosound5.rules > /dev/null << 'EOF'
# BeoSound5 Controller
SUBSYSTEM=="usb", ATTR{idVendor}=="0cd4", ATTR{idProduct}=="1112", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0cd4", ATTRS{idProduct}=="1112", MODE="0666"
EOF
sudo udevadm control --reload-rules
sudo udevadm trigger

echo ""
echo "=== Prerequisites installed! ==="
echo "Now run: ./raspberry-os-build.sh"
