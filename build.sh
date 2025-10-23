#!/bin/bash
# Build script for GdBLE GDExtension

set -e

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

echo "Detected platform: $OS $ARCH"

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-unknown-linux-gnu"
        BIN_DIR="addons/gdble/bin/linux-x86_64"
        LIB_NAME="libgdble.so"
    elif [ "$ARCH" = "aarch64" ]; then
        TARGET="aarch64-unknown-linux-gnu"
        BIN_DIR="addons/gdble/bin/linux-arm64"
        LIB_NAME="libgdble.so"
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="x86_64-apple-darwin"
        BIN_DIR="addons/gdble/bin/macos-x86_64"
        LIB_NAME="libgdble.dylib"
    elif [ "$ARCH" = "arm64" ]; then
        TARGET="aarch64-apple-darwin"
        BIN_DIR="addons/gdble/bin/macos-arm64"
        LIB_NAME="libgdble.dylib"
    fi
else
    echo "Unsupported platform: $OS"
    exit 1
fi

echo "Building GdBLE for $TARGET..."
cargo build --release --target $TARGET

echo "Copying library to $BIN_DIR/..."
mkdir -p "$BIN_DIR"
cp "target/$TARGET/release/$LIB_NAME" "$BIN_DIR/$LIB_NAME"

echo ""
echo "Build complete!"
echo "Library location: $BIN_DIR/$LIB_NAME"
echo ""
echo "Copy the entire addons/gdble/ folder to your Godot project's addons/ directory"
