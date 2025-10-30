# Build script for text-converter plugin
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "Building Text Converter Plugin" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan

$ErrorActionPreference = "Stop"
$pluginDir = $PSScriptRoot

try {
    Write-Host "`n[1/2] Installing dependencies..." -ForegroundColor Yellow
    Set-Location "$pluginDir\frontend"
    npm install
    if ($LASTEXITCODE -ne 0) { throw "npm install failed" }

    Write-Host "`n[2/2] Building UMD bundle..." -ForegroundColor Yellow
    npm run build
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }

    Write-Host "`n============================================" -ForegroundColor Green
    Write-Host "✅ Build Complete!" -ForegroundColor Green
    Write-Host "============================================" -ForegroundColor Green
    Write-Host "Output:"
    Write-Host "  - frontend/dist/text-converter.umd.js"
    Write-Host "  - frontend/dist/text-converter.css"
    Write-Host "`nBuilt directly to:"
    Write-Host "  - ../../src-tauri/plugins/text-converter/"
    Write-Host "`nNote: Vite config builds directly to src-tauri/plugins (no copy needed)"
    Write-Host "============================================" -ForegroundColor Green

} catch {
    Write-Host "`n❌ Build failed: $_" -ForegroundColor Red
    exit 1
}

