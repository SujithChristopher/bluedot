#!/bin/bash
# Build script for BlueDot GDExtension

set -e

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

echo "Detected platform: $OS $ARCH"

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-unknown-linux-gnu"
        BIN_DIR="addons/bluedot/bin/linux-x86_64"
        LIB_NAME="libbluedot.so"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
        BIN_DIR="addons/bluedot/bin/linux-arm64"
        LIB_NAME="libbluedot.so"
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-apple-darwin"
        BIN_DIR="addons/bluedot/bin/macos-x86_64"
        LIB_NAME="libbluedot.dylib"
    elif [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
        BIN_DIR="addons/bluedot/bin/macos-arm64"
        LIB_NAME="libbluedot.dylib"
    fi
else
    echo "Unsupported platform: $OS"
    exit 1
fi

echo "Building BlueDot for $TARGET..."
cargo build --release --target $TARGET

echo "Copying library to $BIN_DIR/..."
mkdir -p "$BIN_DIR"
cp "target/$TARGET/release/$LIB_NAME" "$BIN_DIR/$LIB_NAME"

echo ""
echo "Build complete!"
echo "Library location: $BIN_DIR/$LIB_NAME"
echo ""
echo "Copy the entire addons/bluedot/ folder to your Godot project's addons/ directory"
