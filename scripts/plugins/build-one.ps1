# Build Single Plugin
# Builds and deploys a specific plugin by name

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$PluginName,

    [switch]$SkipDeploy,
    [switch]$ShowOutput
)

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

$pluginSourceDirs = @(
    (Join-Path $rootDir "plugins\featured")
)

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "Build Plugin: $PluginName" -ForegroundColor Cyan
Write-Host "===============================================`n" -ForegroundColor Cyan

# Find the plugin
$pluginPath = $null
foreach ($sourceDir in $pluginSourceDirs) {
    $testPath = Join-Path $sourceDir $PluginName
    if (Test-Path $testPath) {
        $pluginPath = $testPath
        break
    }
}

if (-not $pluginPath) {
    Write-Host "✗ ERROR: Plugin '$PluginName' not found" -ForegroundColor Red
    Write-Host "`nSearched in:" -ForegroundColor Gray
    foreach ($dir in $pluginSourceDirs) {
        Write-Host "  - $dir" -ForegroundColor Gray
    }
    Write-Host ""
    exit 1
}

$frontendPath = Join-Path $pluginPath "frontend"

# Check if plugin has a frontend
if (!(Test-Path $frontendPath)) {
    Write-Host "✗ ERROR: Plugin has no frontend directory" -ForegroundColor Red
    exit 1
}

$packageJsonPath = Join-Path $frontendPath "package.json"
if (!(Test-Path $packageJsonPath)) {
    Write-Host "✗ ERROR: No package.json found in frontend" -ForegroundColor Red
    exit 1
}

# Build the plugin
Write-Host "→ Building frontend..." -ForegroundColor Cyan
Push-Location $frontendPath

try {
    # Expose repo root to plugin build (for Vite deploy paths)
    $env:SANDBOX_ROOT = $rootDir

    # Check if node_modules exists
    if (!(Test-Path "node_modules")) {
        Write-Host "→ Installing dependencies..." -ForegroundColor Cyan
        if ($ShowOutput) {
            npm install
        } else {
            npm install 2>&1 | Out-Null
        }
        if ($LASTEXITCODE -ne 0) {
            throw "npm install failed"
        }
    }

    # Build
    if ($ShowOutput) {
        npm run build
    } else {
        npm run build 2>&1 | Out-Null
    }
    if ($LASTEXITCODE -ne 0) {
        throw "npm run build failed"
    }

    Write-Host "✓ Build successful" -ForegroundColor Green
}
catch {
    Write-Host "`n✗ Build failed: $_" -ForegroundColor Red
    Pop-Location
    exit 1
}
finally {
    # Clean up env var to avoid leaking into caller session
    Remove-Item Env:MODULAUR_ROOT -ErrorAction SilentlyContinue
    Pop-Location
}

# Deploy plugin unless skipped
if (-not $SkipDeploy) {
    Write-Host "`n→ Deploying plugin..." -ForegroundColor Cyan

    # Deploy just this plugin
    $deployScript = Join-Path $PSScriptRoot "deploy.ps1"
    & $deployScript

    if ($LASTEXITCODE -ne 0) {
        Write-Host "`n✗ Deployment failed" -ForegroundColor Red
        exit 1
    }
}

Write-Host "`n✓ Plugin '$PluginName' built successfully!" -ForegroundColor Green
Write-Host ""
