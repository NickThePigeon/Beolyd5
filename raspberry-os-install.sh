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

# Configure HDMI for BeoSound 5 display (no EDID)
echo ">>> Configuring HDMI for BeoSound 5 display..."
BOOT_CONFIG="/boot/config.txt"

# Backup config.txt if not already backed up
if [ ! -f "${BOOT_CONFIG}.backup" ]; then
    sudo cp "$BOOT_CONFIG" "${BOOT_CONFIG}.backup"
    echo "    Backed up config.txt"
fi

# Remove any existing HDMI settings to avoid conflicts
sudo sed -i '/^hdmi_force_hotplug/d' "$BOOT_CONFIG"
sudo sed -i '/^hdmi_ignore_edid/d' "$BOOT_CONFIG"
sudo sed -i '/^hdmi_group/d' "$BOOT_CONFIG"
sudo sed -i '/^hdmi_mode/d' "$BOOT_CONFIG"
sudo sed -i '/^hdmi_drive/d' "$BOOT_CONFIG"
sudo sed -i '/^config_hdmi_boost/d' "$BOOT_CONFIG"
sudo sed -i '/^disable_overscan/d' "$BOOT_CONFIG"

# Add BeoSound 5 HDMI configuration
sudo tee -a "$BOOT_CONFIG" > /dev/null << 'EOF'

# BeoSound 5 Display Configuration (no EDID)
hdmi_force_hotplug=1
hdmi_ignore_edid=0xa5000080
hdmi_group=2
hdmi_mode=16
hdmi_drive=2
config_hdmi_boost=4
disable_overscan=1
EOF

echo "    HDMI configured for 1024x768 @ 60Hz"

echo ""
echo "=== Installation complete! ==="
echo ""
echo "HDMI has been configured for BeoSound 5 (1024x768, no EDID)."
echo ""
echo "Next steps:"
echo "1. Configure auto-start:"
echo "   ./raspberry-os-boot.sh"
echo ""
echo "2. Reboot:"
echo "   sudo reboot"
