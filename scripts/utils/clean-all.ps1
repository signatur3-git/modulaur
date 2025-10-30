# Clean All Build Artifacts
# Performs a deep clean of all build artifacts (frontend, backend, plugins)

param(
    [switch]$SkipInstall
)

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "Clean All Build Artifacts" -ForegroundColor Cyan
Write-Host "===============================================`n" -ForegroundColor Cyan

# Detect folder structure (old or new)
$frontendDir = $null
if (Test-Path (Join-Path $rootDir "src")) {
    $frontendDir = Join-Path $rootDir "src"
}

$backendDir = $null
if (Test-Path (Join-Path $rootDir "src-tauri")) {
    $backendDir = Join-Path $rootDir "src-tauri"
}

# Clean Backend (Rust/Tauri)
if ($backendDir) {
    Write-Host "[Backend] Cleaning Rust artifacts..." -ForegroundColor Yellow
    Push-Location $backendDir

    if (Test-Path "target") {
        Write-Host "  → Running cargo clean..." -ForegroundColor Cyan
        cargo clean
        Write-Host "  ✓ Rust artifacts cleaned" -ForegroundColor Green
    } else {
        Write-Host "  ⊘ No Rust artifacts to clean" -ForegroundColor Gray
    }

    Pop-Location
    Write-Host ""
} else {
    Write-Host "⚠ Backend directory not found, skipping" -ForegroundColor Yellow
    Write-Host ""
}

# Clean Frontend (Vue/Node)
if ($frontendDir) {
    Write-Host "[Frontend] Cleaning Node artifacts..." -ForegroundColor Yellow
    Push-Location $frontendDir

    $cleaned = $false

    if (Test-Path "node_modules") {
        Write-Host "  → Removing node_modules..." -ForegroundColor Cyan
        Remove-Item -Recurse -Force node_modules
        Write-Host "  ✓ node_modules removed" -ForegroundColor Green
        $cleaned = $true
    }

    if (Test-Path "dist") {
        Write-Host "  → Removing dist..." -ForegroundColor Cyan
        Remove-Item -Recurse -Force dist
        Write-Host "  ✓ dist removed" -ForegroundColor Green
        $cleaned = $true
    }

    if (Test-Path ".vite") {
        Write-Host "  → Removing .vite cache..." -ForegroundColor Cyan
        Remove-Item -Recurse -Force .vite
        Write-Host "  ✓ .vite cache removed" -ForegroundColor Green
        $cleaned = $true
    }

    if (-not $cleaned) {
        Write-Host "  ⊘ No frontend artifacts to clean" -ForegroundColor Gray
    }

    Pop-Location
    Write-Host ""
} else {
    Write-Host "⚠ Frontend directory not found, skipping" -ForegroundColor Yellow
    Write-Host ""
}

# Clean Plugin Builds
Write-Host "[Plugins] Cleaning plugin artifacts..." -ForegroundColor Yellow

$pluginDirs = @(
    (Join-Path $rootDir "plugins\featured"),
    (Join-Path $rootDir "plugins\examples")
)

$pluginsCleaned = 0
foreach ($dir in $pluginDirs) {
    if (Test-Path $dir) {
        $plugins = Get-ChildItem -Path $dir -Directory
        foreach ($plugin in $plugins) {
            $frontendPath = Join-Path $plugin.FullName "frontend"
            if (Test-Path $frontendPath) {
                Push-Location $frontendPath

                $cleaned = $false
                if (Test-Path "node_modules") {
                    Remove-Item -Recurse -Force node_modules -ErrorAction SilentlyContinue
                    $cleaned = $true
                }
                if (Test-Path "dist") {
                    Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
                    $cleaned = $true
                }

                if ($cleaned) {
                    Write-Host "  ✓ Cleaned: $($plugin.Name)" -ForegroundColor Green
                    $pluginsCleaned++
                }

                Pop-Location
            }
        }
    }
}

if ($pluginsCleaned -eq 0) {
    Write-Host "  ⊘ No plugin artifacts to clean" -ForegroundColor Gray
} else {
    Write-Host "  ✓ Cleaned $pluginsCleaned plugin(s)" -ForegroundColor Green
}

Write-Host ""

# Reinstall dependencies if requested
if (-not $SkipInstall) {
    if ($frontendDir) {
        Write-Host "[Frontend] Reinstalling dependencies..." -ForegroundColor Yellow
        Push-Location $frontendDir

        Write-Host "  → Running npm install..." -ForegroundColor Cyan
        npm install

        if ($LASTEXITCODE -eq 0) {
            Write-Host "  ✓ Frontend dependencies installed" -ForegroundColor Green
        } else {
            Write-Host "  ✗ Frontend installation failed!" -ForegroundColor Red
            Pop-Location
            exit 1
        }

        Pop-Location
        Write-Host ""
    }
}

Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "✓ Clean Complete!" -ForegroundColor Green
Write-Host "===============================================`n" -ForegroundColor Cyan

if ($SkipInstall) {
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  npm run install:all  # Reinstall dependencies" -ForegroundColor Gray
    Write-Host "  npm run build        # Build everything" -ForegroundColor Gray
    Write-Host ""
}

