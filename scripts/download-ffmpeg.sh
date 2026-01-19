#!/bin/bash

# Directory for binaries
BIN_DIR="src-tauri/binaries"
mkdir -p "$BIN_DIR"

# URL for macOS ARM64 static FFmpeg (Martin Riedl's build)
URL="https://ffmpeg.martin-riedl.de/redirect/latest/macos/arm64/release/ffmpeg.zip"

echo "Downloading static FFmpeg binary..."
echo "Source: $URL"
curl -L -o ffmpeg.zip "$URL"

if [ $? -ne 0 ]; then
    echo "Error: Download failed."
    exit 1
fi

echo "Extracting archive..."
unzip -o ffmpeg.zip

if [ $? -ne 0 ]; then
    echo "Error: Extraction failed."
    rm ffmpeg.zip
    exit 1
fi

# Target file name must match the target triple for Tauri sidecar
TARGET="$BIN_DIR/ffmpeg-aarch64-apple-darwin"

echo "Installing binary to $TARGET..."
mv ffmpeg "$TARGET"
chmod +x "$TARGET"

# Cleanup
rm ffmpeg.zip

echo "Success! FFmpeg binary installed."
echo "Version:"
"$TARGET" -version | head -n 1
