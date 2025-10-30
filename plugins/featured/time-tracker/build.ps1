Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Building Time Tracker Plugin" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"
$projectRoot = "D:\workspaces\modulaur"
$pluginRoot = "$projectRoot\plugins\featured\time-tracker"
$frontendDir = "$pluginRoot\frontend"
$targetDir = "$projectRoot\src-tauri\plugins\time-tracker"

Write-Host "[1/2] Building frontend..." -ForegroundColor Yellow
Set-Location $frontendDir

if (-not (Test-Path "node_modules")) {
    Write-Host "  Installing dependencies..." -ForegroundColor Gray
    npm install
}

Write-Host "  Running build..." -ForegroundColor Gray
npm run build

if ($LASTEXITCODE -ne 0) {
    Write-Host "  Frontend build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "  Frontend built successfully" -ForegroundColor Green
Write-Host ""

Write-Host "[2/2] Verifying..." -ForegroundColor Yellow
$manifestExists = Test-Path "$targetDir\manifest.json"
$umdExists = Test-Path "$targetDir\frontend\dist\time-tracker.umd.js"
$cssExists = Test-Path "$targetDir\frontend\dist\style.css"

if ($manifestExists -and $umdExists -and $cssExists) {
    Write-Host "  All files OK" -ForegroundColor Green
    Write-Host ""
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host "BUILD COMPLETE!" -ForegroundColor Green
    Write-Host "=====================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Plugin installed at: $targetDir" -ForegroundColor Gray
    Write-Host "Note: Vite config builds directly to src-tauri/plugins (no copy needed)" -ForegroundColor Gray
    Write-Host "Restart the app to load the plugin!" -ForegroundColor Yellow
} else {
    Write-Host "  Missing files:" -ForegroundColor Red
    if (-not $manifestExists) { Write-Host "    - manifest.json" -ForegroundColor Red }
    if (-not $umdExists) { Write-Host "    - time-tracker.umd.js" -ForegroundColor Red }
    if (-not $cssExists) { Write-Host "    - style.css" -ForegroundColor Red }
    exit 1
}

