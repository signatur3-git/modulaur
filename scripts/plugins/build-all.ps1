# Build All Plugins
# Builds all featured plugins and deploys them to the target directory

param(
    [switch]$SkipDeploy,
    [switch]$ShowOutput
)

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

# Support both old and new folder names
$pluginSourceDirs = @(
    (Join-Path $rootDir "plugins\featured")
)

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "Build All Plugins" -ForegroundColor Cyan
Write-Host "===============================================`n" -ForegroundColor Cyan

$builtCount = 0
$skippedCount = 0
$failedPlugins = @()

# Get all plugin directories from both sources
$plugins = @()
foreach ($sourceDir in $pluginSourceDirs) {
    if (Test-Path $sourceDir) {
        $plugins += Get-ChildItem -Path $sourceDir -Directory
    }
}

Write-Host "Found $($plugins.Count) plugin(s) to process`n" -ForegroundColor Cyan

foreach ($plugin in $plugins) {
    $pluginName = $plugin.Name
    $frontendPath = Join-Path $plugin.FullName "frontend"

    Write-Host "[$pluginName]" -ForegroundColor Yellow

    # Check if plugin has a frontend directory with package.json
    if (!(Test-Path $frontendPath)) {
        Write-Host "  ⊘ No frontend directory - skipping" -ForegroundColor Gray
        $skippedCount++
        continue
    }

    $packageJsonPath = Join-Path $frontendPath "package.json"
    if (!(Test-Path $packageJsonPath)) {
        Write-Host "  ⊘ No package.json in frontend - skipping" -ForegroundColor Gray
        $skippedCount++
        continue
    }

    # Build the plugin
    Write-Host "  → Building frontend..." -ForegroundColor Cyan
    Push-Location $frontendPath

    try {
        # Expose repo root to plugin build (for Vite deploy paths)
        $env:SANDBOX_ROOT = $rootDir

        # Check if node_modules exists, if not run npm install
        if (!(Test-Path "node_modules")) {
            Write-Host "  → Installing dependencies..." -ForegroundColor Cyan
            if ($ShowOutput) {
                npm install
            } else {
                npm install 2>&1 | Out-Null
            }
            if ($LASTEXITCODE -ne 0) {
                throw "npm install failed"
            }
        }

        # Build the plugin
        if ($ShowOutput) {
            npm run build
        } else {
            npm run build 2>&1 | Out-Null
        }
        if ($LASTEXITCODE -ne 0) {
            throw "npm run build failed"
        }

        Write-Host "  [OK] Build successful" -ForegroundColor Green
        $builtCount++
    }
    catch {
        Write-Host "  [X] Build failed: $_" -ForegroundColor Red
        $failedPlugins += $pluginName
    }
    finally {
        Remove-Item Env:MODULAUR_ROOT -ErrorAction SilentlyContinue
        Pop-Location
    }

    Write-Host ""
}

Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "Build Summary" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "[OK] Built: $builtCount plugin(s)" -ForegroundColor Green

if ($skippedCount -gt 0) {
    Write-Host "[SKIP] Skipped: $skippedCount plugin(s)" -ForegroundColor Yellow
}

if ($failedPlugins.Count -gt 0) {
    Write-Host "[X] Failed: $($failedPlugins.Count) plugin(s)" -ForegroundColor Red
    Write-Host "  - $($failedPlugins -join ', ')" -ForegroundColor Red
    Write-Host ""
    exit 1
}

# Deploy plugins unless skipped
if (-not $SkipDeploy) {
    Write-Host "`nDeploying plugins..." -ForegroundColor Cyan
    $deployScript = Join-Path $PSScriptRoot "deploy.ps1"
    & $deployScript
    if ($LASTEXITCODE -ne 0) {
        Write-Host "`n[X] Deployment failed" -ForegroundColor Red
        exit 1
    }
}

Write-Host "`n[OK] All plugins built successfully!" -ForegroundColor Green
Write-Host ""
