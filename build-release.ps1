# Build Release Script for Modulaur
# This script builds the complete release with frontend bundled into the executable

Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Modulaur Release Build" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Step 1: Build frontend
Write-Host "📦 Step 1: Building Frontend..." -ForegroundColor Yellow
Set-Location src
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Frontend build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Frontend built successfully" -ForegroundColor Green
Write-Host ""

# Step 2: Build Tauri app (this bundles frontend into the executable)
Write-Host "🦀 Step 2: Building Tauri Application..." -ForegroundColor Yellow
Set-Location ..\src-tauri
cargo tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tauri build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Tauri build successful" -ForegroundColor Green
Write-Host ""

# Step 3: Show output location
Write-Host "📂 Build Output:" -ForegroundColor Cyan
Write-Host ""

$bundleDir = "target\release\bundle"
if (Test-Path $bundleDir) {
    Write-Host "  Installers:" -ForegroundColor White

    # NSIS installer (Windows)
    $nsisPath = "$bundleDir\nsis"
    if (Test-Path $nsisPath) {
        Get-ChildItem $nsisPath -Filter "*.exe" | ForEach-Object {
            $size = [math]::Round($_.Length / 1MB, 2)
            Write-Host "    ✓ $($_.Name) ($size MB)" -ForegroundColor Green
            Write-Host "      Location: $($_.FullName)" -ForegroundColor Gray
        }
    }

    # MSI installer (Windows)
    $msiPath = "$bundleDir\msi"
    if (Test-Path $msiPath) {
        Get-ChildItem $msiPath -Filter "*.msi" | ForEach-Object {
            $size = [math]::Round($_.Length / 1MB, 2)
            Write-Host "    ✓ $($_.Name) ($size MB)" -ForegroundColor Green
            Write-Host "      Location: $($_.FullName)" -ForegroundColor Gray
        }
    }

    Write-Host ""
    Write-Host "  Standalone executable (with bundled frontend):" -ForegroundColor White
    $exePath = "target\release\modulaur.exe"
    if (Test-Path $exePath) {
        $exeInfo = Get-Item $exePath
        $size = [math]::Round($exeInfo.Length / 1MB, 2)
        Write-Host "    ✓ modulaur.exe ($size MB)" -ForegroundColor Green
        Write-Host "      Location: $($exeInfo.FullName)" -ForegroundColor Gray
    }
} else {
    Write-Host "  ⚠️  Bundle directory not found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  Build Complete! 🎉" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "To install, run the installer from the locations shown above." -ForegroundColor White
Write-Host "Or copy modulaur.exe to test the standalone version." -ForegroundColor White

