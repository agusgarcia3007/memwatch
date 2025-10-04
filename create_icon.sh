#!/bin/bash
set -e

echo "Creating app icon for memwatch..."

# Create base icon with Python (simple solid color)
echo "Generating base icon..."

python3 << 'PYEOF'
import struct
import zlib

def create_simple_png(filename, width, height, color):
    """Create a simple solid color PNG"""
    def png_pack(png_tag, data):
        chunk_head = png_tag
        return (struct.pack("!I", len(data)) +
                chunk_head + data +
                struct.pack("!I", 0xFFFFFFFF & zlib.crc32(chunk_head + data)))

    # PNG signature
    png_data = b'\x89PNG\r\n\x1a\n'

    # IHDR chunk
    ihdr = struct.pack("!2I5B", width, height, 8, 2, 0, 0, 0)
    png_data += png_pack(b'IHDR', ihdr)

    # IDAT chunk - simple solid color
    raw_data = b''
    r, g, b = color
    for y in range(height):
        raw_data += b'\x00'  # No filter
        raw_data += (bytes([r, g, b]) * width)

    compressed = zlib.compress(raw_data, 9)
    png_data += png_pack(b'IDAT', compressed)

    # IEND chunk
    png_data += png_pack(b'IEND', b'')

    with open(filename, 'wb') as f:
        f.write(png_data)

# Create blue icon (memwatch brand color)
create_simple_png('/tmp/memwatch_base.png', 1024, 1024, (74, 144, 226))
print("Base icon created")
PYEOF

if [ ! -f /tmp/memwatch_base.png ]; then
    echo "Error: Failed to create base icon"
    exit 1
fi

ICON_DIR="icon.iconset"
rm -rf "$ICON_DIR"
mkdir -p "$ICON_DIR"

# Alternative: if you want to create an SVG-based icon with ImageMagick, uncomment:
# cat > /tmp/memwatch_icon.svg << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<svg width="1024" height="1024" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#4A90E2;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#357ABD;stop-opacity:1" />
    </linearGradient>
  </defs>

  <!-- Rounded rectangle background -->
  <rect width="1024" height="1024" rx="180" fill="url(#grad)"/>

  <!-- Memory bars visualization -->
  <g opacity="0.3">
    <rect x="200" y="650" width="80" height="200" fill="white" rx="10"/>
    <rect x="320" y="550" width="80" height="300" fill="white" rx="10"/>
    <rect x="440" y="450" width="80" height="400" fill="white" rx="10"/>
    <rect x="560" y="350" width="80" height="500" fill="white" rx="10"/>
    <rect x="680" y="500" width="80" height="350" fill="white" rx="10"/>
  </g>

  <!-- Large "M" for memwatch -->
  <text x="512" y="650"
        font-family="SF Pro Display, -apple-system, system-ui, sans-serif"
        font-size="600"
        font-weight="bold"
        fill="white"
        text-anchor="middle">M</text>
</svg>
EOF

# Check if we can convert SVG (requires rsvg-convert or similar)
if command -v rsvg-convert &> /dev/null; then
    echo "Using rsvg-convert to generate icon..."
    rsvg-convert -w 1024 -h 1024 /tmp/memwatch_icon.svg -o /tmp/memwatch_base.png
elif command -v convert &> /dev/null; then
    echo "Using ImageMagick to generate icon..."
    convert /tmp/memwatch_icon.svg -resize 1024x1024 /tmp/memwatch_base.png
else
    echo "Creating simple fallback icon with sips..."
    # Create a simple colored square as fallback
    # This is a basic approach - icon won't be as nice but will work
    python3 << 'PYEOF'
from PIL import Image, ImageDraw, ImageFont
import os

# Create base image
img = Image.new('RGB', (1024, 1024), color='#4A90E2')
draw = ImageDraw.Draw(img)

# Draw rounded rectangle (approximate)
# Draw memory bars
bars = [(200, 650, 80, 200), (320, 550, 80, 300), (440, 450, 80, 400),
        (560, 350, 80, 500), (680, 500, 80, 350)]
for x, y, w, h in bars:
    draw.rectangle([x, y, x+w, y+h], fill=(255, 255, 255, 76))

# Draw "M" text
try:
    font = ImageFont.truetype("/System/Library/Fonts/SFNS.ttf", 600)
except:
    font = ImageFont.load_default()

draw.text((512, 400), "M", fill='white', font=font, anchor="mm")

img.save('/tmp/memwatch_base.png')
print("Icon created with Python/PIL")
PYEOF

    if [ $? -ne 0 ]; then
        echo "Warning: Could not create fancy icon. Creating simple colored icon..."
        # Ultra-simple fallback: just create a blue square with sips
        sips -c 1024 1024 -s format png --out /tmp/memwatch_base.png /dev/null 2>/dev/null || {
            # Last resort: create with pure shell
            echo "Creating minimal icon..."
            cat > /tmp/memwatch_base.png << 'PNGEOF'
iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==
PNGEOF
        }
    fi
fi

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
