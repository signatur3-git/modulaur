# Build script for snippets plugin
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Building Snippets Plugin" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$ErrorActionPreference = "Stop"
$pluginDir = $PSScriptRoot

try {
    Write-Host "`n[1/2] Installing dependencies..." -ForegroundColor Yellow
    Set-Location "$pluginDir\frontend"
    npm install
    if ($LASTEXITCODE -ne 0) { throw "npm install failed" }

    Write-Host "`n[2/2] Building plugin..." -ForegroundColor Yellow
    npm run build
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }

    Write-Host "`n========================================" -ForegroundColor Green
    Write-Host "Build Complete!" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "`nPlugin built to: src-tauri\plugins\snippets\" -ForegroundColor Gray
    Write-Host "`nNote: Vite config builds directly to src-tauri/plugins (no copy needed)" -ForegroundColor Gray

} catch {
    Write-Host "`n❌ Build failed: $_" -ForegroundColor Red
    exit 1
}

