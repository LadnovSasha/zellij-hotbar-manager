#!/bin/bash
set -e

echo "Building zellij-hotbar-manager plugin..."

# Add WASM target if not present
rustup target add wasm32-wasip1 2>/dev/null || true

# Build the plugin in release mode
cargo build --target wasm32-wasip1 --release

# Create plugins directory if it doesn't exist
mkdir -p ~/.config/zellij/plugins

# Copy the compiled plugin to Zellij plugins directory
cp target/wasm32-wasip1/release/zellij-hotbar-manager.wasm ~/.config/zellij/plugins/zellij-hotbar-manager.wasm

echo "Plugin built and installed successfully!"
echo "Location: ~/.config/zellij/plugins/zellij-hotbar-manager.wasm"
