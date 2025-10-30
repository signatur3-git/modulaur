# Check Binary Sizes
# Reports the sizes of built binaries

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "Binary Size Check" -ForegroundColor Cyan
Write-Host "===============================================`n" -ForegroundColor Cyan

# Detect backend folder
$backendDir = $null
if (Test-Path (Join-Path $rootDir "src-tauri")) {
    $backendDir = Join-Path $rootDir "src-tauri"
} else {
    Write-Host "✗ Backend directory not found (src-tauri)" -ForegroundColor Red
    exit 1
}

Push-Location $backendDir

# Check for release builds (multiple possible paths)
$possiblePaths = @(
    "target\release\modulaur.exe",
    "target\x86_64-pc-windows-msvc\release\modulaur.exe",
    "target\x86_64-pc-windows-gnu\release\modulaur.exe"
)

$exePath = $null
foreach ($path in $possiblePaths) {
    if (Test-Path $path) {
        $exePath = $path
        break
    }
}

if ($exePath) {
    Write-Host "✓ Found: modulaur.exe" -ForegroundColor Green
    $size = (Get-Item $exePath).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "  Size: $sizeMB MB" -ForegroundColor Cyan
    Write-Host ""
} else {
    Write-Host "⊘ No release build found" -ForegroundColor Yellow
    Write-Host "  Run 'npm run build:release' first" -ForegroundColor Gray
    Write-Host ""
}

# Check for debug build
$debugPath = "target\debug\modulaur.exe"
if (Test-Path $debugPath) {
    Write-Host "✓ Found: modulaur.exe (debug)" -ForegroundColor Green
    $size = (Get-Item $debugPath).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "  Size: $sizeMB MB" -ForegroundColor Cyan
    Write-Host ""
}

# Check for sidecar binaries (if used)
if (Test-Path "sidecar-binaries\surreal.exe") {
    Write-Host "✓ Found: surreal.exe (sidecar)" -ForegroundColor Green
    $size = (Get-Item "sidecar-binaries\surreal.exe").Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "  Size: $sizeMB MB" -ForegroundColor Cyan
    Write-Host ""
}

Pop-Location

Write-Host "===============================================`n" -ForegroundColor Cyan

