#!/bin/bash
set -e

echo "=========================================="
echo "  Beolyd5 - BeoSound 5 Revival Project   "
echo "  Raspberry Pi 3B+ Full Setup            "
echo "=========================================="
echo ""

SCRIPT_DIR="$(dirname "$0")"
cd "$SCRIPT_DIR"

echo "This script will:"
echo "  1. Install all prerequisites"
echo "  2. Build the application"
echo "  3. Install the binary"
echo "  4. Configure auto-start"
echo ""
echo "This may take 30-60 minutes on a Raspberry Pi 3B+"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Step 1: Prerequisites
echo ""
echo ">>> Step 1/4: Installing prerequisites..."
./raspberry-os-prereqs.sh

# Reload shell environment
source $HOME/.cargo/env

# Step 2: Build
echo ""
echo ">>> Step 2/4: Building application..."
./raspberry-os-build.sh

# Step 3: Install
echo ""
echo ">>> Step 3/4: Installing..."
./raspberry-os-install.sh

# Step 4: Configure auto-start
echo ""
echo ">>> Step 4/4: Configuring auto-start..."
./raspberry-os-boot.sh

echo ""
echo "=========================================="
echo "  Setup Complete!                        "
echo "=========================================="
echo ""
echo "HDMI has been configured for BeoSound 5 (1024x768, no EDID)."
echo ""
echo "Edit your HEOS IP if needed:"
echo "  nano ~/.config/beolyd5/config.json"
echo ""
echo "Then reboot:"
echo "  sudo reboot"
