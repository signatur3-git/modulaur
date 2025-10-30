@echo off
REM Build script for WASI HTTP adapter plugin (Windows)

echo Building WASI HTTP adapter...

REM Build for wasm32-unknown-unknown (pure WASI)
cargo build --release --target wasm32-unknown-unknown

if %ERRORLEVEL% == 0 (
    echo Build successful!
    echo WASM binary: target\wasm32-unknown-unknown\release\wasi_http_adapter_template.wasm
    dir target\wasm32-unknown-unknown\release\wasi_http_adapter_template.wasm
) else (
    echo Build failed!
    exit /b 1
)

