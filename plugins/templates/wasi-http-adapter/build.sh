#!/bin/bash
# Build script for WASI HTTP adapter plugin

echo "🔨 Building WASI HTTP adapter..."

# Build for wasm32-unknown-unknown (pure WASI)
cargo build --release --target wasm32-unknown-unknown

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📦 WASM binary: target/wasm32-unknown-unknown/release/wasi_http_adapter_template.wasm"

    # Show size
    ls -lh target/wasm32-unknown-unknown/release/wasi_http_adapter_template.wasm
else
    echo "❌ Build failed!"
    exit 1
fi

