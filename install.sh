#!/bin/bash
set -e

APP_NAME="memwatch"
INSTALL_DIR="/Applications"
CLI_LINK="/usr/local/bin/memwatch"

echo "Installing memwatch..."

if [ ! -d "dist/${APP_NAME}.app" ]; then
    echo "Error: App bundle not found. Please run ./package.sh first."
    exit 1
fi

echo "Copying app to ${INSTALL_DIR}..."
sudo cp -R "dist/${APP_NAME}.app" "${INSTALL_DIR}/"

if [ ! -d "${INSTALL_DIR}/${APP_NAME}.app" ]; then
    echo "Error: Failed to copy app bundle"
    exit 1
fi

echo "Creating CLI symlink..."
sudo ln -sf "${INSTALL_DIR}/${APP_NAME}.app/Contents/MacOS/${APP_NAME}" "${CLI_LINK}"

echo ""
echo "Installation complete!"
echo ""
echo "You can now:"
echo "  • Launch the app from Applications folder or Spotlight"
echo "  • Use 'memwatch' command from terminal"
echo "  • Use 'memwatch toggle' to show/hide the window"
echo "  • Press ⌥⌘M (Option+Command+M) as global hotkey"
echo ""
echo "To uninstall, run:"
echo "  sudo rm -rf ${INSTALL_DIR}/${APP_NAME}.app"
echo "  sudo rm ${CLI_LINK}"
