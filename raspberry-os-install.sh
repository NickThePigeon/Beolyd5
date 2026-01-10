#!/bin/bash
set -e

echo "=== Installing Beolyd5 ==="
echo ""

SCRIPT_DIR="$(dirname "$0")"
BINARY="$SCRIPT_DIR/src/ui/target/release/bs5-controller-ui"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    echo "Please run ./raspberry-os-build.sh first"
    exit 1
fi

# Install binary
echo ">>> Installing binary to /usr/local/bin..."
sudo cp "$BINARY" /usr/local/bin/bs5-controller-ui
sudo chmod +x /usr/local/bin/bs5-controller-ui

# Create config directory
echo ">>> Creating config directory..."
mkdir -p ~/.config/beolyd5

# Create default config file with HEOS settings
if [ ! -f ~/.config/beolyd5/config.json ]; then
    cat > ~/.config/beolyd5/config.json << 'EOF'
{
    "heos": {
        "host": "192.168.1.2",
        "port": 1255
    }
}
EOF
    echo ">>> Created default config at ~/.config/beolyd5/config.json"
    echo "    Edit this file to set your HEOS device IP address"
fi

echo ""
echo "=== Installation complete! ==="
echo ""
echo "Next steps:"
echo "1. Configure HDMI (if not done):"
echo "   sudo nano /boot/config.txt"
echo "   Add: hdmi_group=2"
echo "   Add: hdmi_mode=16"
echo ""
echo "2. Configure auto-start:"
echo "   ./raspberry-os-boot.sh"
echo ""
echo "3. Reboot:"
echo "   sudo reboot"
