#!/bin/bash
set -e

echo "=== Beolyd5 Update from GitHub ==="
echo ""

# GitHub repo - change this to your fork
REPO="NickThePigeon/Beolyd5"
DEB_NAME="bs5-controller-ui_0.1.0_arm64.deb"

# Get latest release download URL
echo ">>> Fetching latest release..."
DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url.*\.deb" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: No release found. You can also manually trigger a build:"
    echo "  1. Go to https://github.com/$REPO/actions"
    echo "  2. Run the 'Raspberry Pi compile' workflow"
    echo "  3. Download the artifact and install with: sudo dpkg -i <file>.deb"
    exit 1
fi

echo ">>> Downloading from: $DOWNLOAD_URL"
curl -L -o /tmp/$DEB_NAME "$DOWNLOAD_URL"

echo ">>> Installing..."
sudo dpkg -i /tmp/$DEB_NAME

echo ">>> Cleaning up..."
rm /tmp/$DEB_NAME

echo ""
echo "=== Update complete! ==="
echo "Restart the application or reboot to apply changes."
