#!/bin/bash
set -e

echo "=== Configuring Beolyd5 Auto-Start ==="

# Create .xinitrc to start the app on X start
cat > ~/.xinitrc << 'EOF'
# Hide cursor and disable window decorations
matchbox-window-manager -use_cursor no -use_titlebar no &

# Wait a moment for window manager
sleep 1

# Start Beolyd5
exec WEBKIT_DISABLE_COMPOSITING_MODE=1 /usr/local/bin/bs5-controller-ui
EOF

# Backup existing .bashrc
cp ~/.bashrc ~/.bashrc.backup 2>/dev/null || true

# Add auto-start X on login (only if not already added)
if ! grep -q "startx" ~/.bashrc; then
    echo '' >> ~/.bashrc
    echo '# Auto-start X for Beolyd5' >> ~/.bashrc
    echo '[[ -z $DISPLAY && $XDG_VTNR -eq 1 ]] && startx' >> ~/.bashrc
fi

echo ""
echo "=== Auto-start configured! ==="
echo "The app will start automatically on next boot."
echo "Reboot with: sudo reboot"
