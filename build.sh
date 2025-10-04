#!/bin/bash
set -e

echo "Building memwatch for macOS..."

if command -v rustup &> /dev/null; then
    if rustup target list | grep -q "aarch64-apple-darwin (installed)" && \
       rustup target list | grep -q "x86_64-apple-darwin (installed)"; then
        echo "Building universal binary..."

        cargo build --release --target aarch64-apple-darwin
        cargo build --release --target x86_64-apple-darwin

        mkdir -p target/universal/release

        lipo -create \
            target/aarch64-apple-darwin/release/memwatch \
            target/x86_64-apple-darwin/release/memwatch \
            -output target/universal/release/memwatch

        echo "Universal binary created at target/universal/release/memwatch"
        BINARY_PATH="target/universal/release/memwatch"
    else
        echo "Building for current architecture only..."
        cargo build --release
        BINARY_PATH="target/release/memwatch"
    fi
else
    echo "Building for current architecture only..."
    cargo build --release
    BINARY_PATH="target/release/memwatch"
fi

echo ""
echo "Build complete!"
echo "Binary location: $BINARY_PATH"
echo ""
echo "Binary size:"
ls -lh "$BINARY_PATH" | awk '{print $5, $9}'
