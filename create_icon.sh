#!/bin/bash
set -e

echo "Creating app icon for memwatch..."

# Create base icon with Python - gradient and letter M
echo "Generating base icon with gradient and M..."

python3 << 'PYEOF'
import struct
import zlib
import math

def create_gradient_icon(filename, width, height):
    """Create a gradient icon with letter M"""

    def png_pack(png_tag, data):
        chunk_head = png_tag
        return (struct.pack("!I", len(data)) +
                chunk_head + data +
                struct.pack("!I", 0xFFFFFFFF & zlib.crc32(chunk_head + data)))

    # Colors for gradient
    color_top = (74, 144, 226)      # #4A90E2
    color_bottom = (53, 122, 189)   # #357ABD

    # Create image with gradient
    raw_data = b''
    for y in range(height):
        raw_data += b'\x00'  # No filter
        # Calculate gradient color for this row
        t = y / height
        r = int(color_top[0] + (color_bottom[0] - color_top[0]) * t)
        g = int(color_top[1] + (color_bottom[1] - color_top[1]) * t)
        b = int(color_top[2] + (color_bottom[2] - color_top[2]) * t)

        for x in range(width):
            raw_data += bytes([r, g, b])

    # PNG signature
    png_data = b'\x89PNG\r\n\x1a\n'

    # IHDR chunk
    ihdr = struct.pack("!2I5B", width, height, 8, 2, 0, 0, 0)
    png_data += png_pack(b'IHDR', ihdr)

    # IDAT chunk
    compressed = zlib.compress(raw_data, 9)
    png_data += png_pack(b'IDAT', compressed)

    # IEND chunk
    png_data += png_pack(b'IEND', b'')

    with open(filename, 'wb') as f:
        f.write(png_data)

# Create gradient background
create_gradient_icon('/tmp/memwatch_gradient.png', 1024, 1024)
print("Gradient created")
PYEOF

if [ ! -f /tmp/memwatch_gradient.png ]; then
    echo "Error: Failed to create gradient"
    exit 1
fi

# Try to add rounded corners and M letter with ImageMagick
if command -v magick &> /dev/null; then
    echo "Using ImageMagick to add rounded corners and M..."
    magick /tmp/memwatch_gradient.png \
        \( +clone -alpha extract \
           -draw 'fill black polygon 0,0 0,180 180,0 fill white circle 180,180 180,0' \
           \( +clone -flip \) -compose Multiply -composite \
           \( +clone -flop \) -compose Multiply -composite \
        \) -alpha off -compose CopyOpacity -composite \
        -gravity center \
        -fill white -font Helvetica-Bold -pointsize 650 \
        -annotate +0-30 'M' \
        /tmp/memwatch_base.png
    echo "Enhanced icon created with ImageMagick"
elif command -v convert &> /dev/null; then
    echo "Using ImageMagick (legacy convert command)..."
    convert /tmp/memwatch_gradient.png \
        \( +clone -alpha extract \
           -draw 'fill black polygon 0,0 0,180 180,0 fill white circle 180,180 180,0' \
           \( +clone -flip \) -compose Multiply -composite \
           \( +clone -flop \) -compose Multiply -composite \
        \) -alpha off -compose CopyOpacity -composite \
        -gravity center \
        -fill white -font Helvetica-Bold -pointsize 650 \
        -annotate +0-30 'M' \
        /tmp/memwatch_base.png
    echo "Enhanced icon created"
else
    echo "ImageMagick not found, using gradient only..."
    cp /tmp/memwatch_gradient.png /tmp/memwatch_base.png
fi

if [ ! -f /tmp/memwatch_base.png ]; then
    echo "Error: Failed to create base icon"
    exit 1
fi

ICON_DIR="icon.iconset"
rm -rf "$ICON_DIR"
mkdir -p "$ICON_DIR"

# Note: For a fancier icon with gradient and "M" text, install ImageMagick:
# brew install imagemagick
# Then uncomment and run the SVG conversion code in this script

# Generate all required icon sizes
echo "Generating icon sizes..."
sips -z 16 16     /tmp/memwatch_base.png --out "$ICON_DIR/icon_16x16.png" >/dev/null 2>&1
sips -z 32 32     /tmp/memwatch_base.png --out "$ICON_DIR/icon_16x16@2x.png" >/dev/null 2>&1
sips -z 32 32     /tmp/memwatch_base.png --out "$ICON_DIR/icon_32x32.png" >/dev/null 2>&1
sips -z 64 64     /tmp/memwatch_base.png --out "$ICON_DIR/icon_32x32@2x.png" >/dev/null 2>&1
sips -z 128 128   /tmp/memwatch_base.png --out "$ICON_DIR/icon_128x128.png" >/dev/null 2>&1
sips -z 256 256   /tmp/memwatch_base.png --out "$ICON_DIR/icon_128x128@2x.png" >/dev/null 2>&1
sips -z 256 256   /tmp/memwatch_base.png --out "$ICON_DIR/icon_256x256.png" >/dev/null 2>&1
sips -z 512 512   /tmp/memwatch_base.png --out "$ICON_DIR/icon_256x256@2x.png" >/dev/null 2>&1
sips -z 512 512   /tmp/memwatch_base.png --out "$ICON_DIR/icon_512x512.png" >/dev/null 2>&1
sips -z 1024 1024 /tmp/memwatch_base.png --out "$ICON_DIR/icon_512x512@2x.png" >/dev/null 2>&1

# Convert to .icns
echo "Converting to .icns format..."
iconutil -c icns "$ICON_DIR" -o memwatch.icns

# Cleanup
rm -rf "$ICON_DIR"
rm -f /tmp/memwatch_icon.svg
rm -f /tmp/memwatch_base.png

echo ""
echo "✓ Icon created: memwatch.icns"
echo ""
