#!/bin/bash
set -e

APP_NAME="memwatch"
VERSION="0.1.0"
BUNDLE_ID="com.memwatch.memwatch"

echo "Packaging memwatch for macOS..."

./build.sh

if [ -f "target/universal/release/memwatch" ]; then
    BINARY_PATH="target/universal/release/memwatch"
else
    BINARY_PATH="target/release/memwatch"
fi

APP_DIR="dist/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Creating app bundle structure..."
rm -rf dist
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

cp "${BINARY_PATH}" "${MACOS_DIR}/${APP_NAME}"
chmod +x "${MACOS_DIR}/${APP_NAME}"

# Copy icon if it exists
if [ -f "memwatch.icns" ]; then
    echo "Adding app icon..."
    cp memwatch.icns "${RESOURCES_DIR}/"
fi

cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>memwatch</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>LSMinimumSystemVersion</key>
    <string>13.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>CFBundleIconFile</key>
    <string>memwatch</string>
</dict>
</plist>
EOF

echo "Creating DMG..."
DMG_NAME="memwatch-${VERSION}-macos.dmg"

rm -f "dist/${DMG_NAME}"

hdiutil create -volname "${APP_NAME}" -srcfolder "dist/${APP_NAME}.app" -ov -format UDZO "dist/${DMG_NAME}"

echo ""
echo "Packaging complete!"
echo "App bundle: ${APP_DIR}"
echo "DMG: dist/${DMG_NAME}"
echo ""
echo "DMG size:"
ls -lh "dist/${DMG_NAME}" | awk '{print $5, $9}'
