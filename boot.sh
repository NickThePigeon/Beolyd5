#!/bin/bash

# Create .xinitrc to start the app on X start
echo 'matchbox-window-manager -use_cursor no -use_titlebar no &' > ~/.xinitrc
echo 'exec WEBKIT_DISABLE_COMPOSITING_MODE=1 /usr/local/bin/bs5-controller-ui' >> ~/.xinitrc

# Modify .bashrc to start X on boot
echo '[[ -z $DISPLAY && $XDG_VTNR -eq 1 ]] && startx' >> ~/.bashrc
