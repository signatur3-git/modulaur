# Deploy Plugin Assets
# Creates junction for frontend serving and optionally copies legacy plugins
# Modern plugins build directly to src-tauri/plugins/ (no copy needed)

param(
    [switch]$ShowOutput,
    [switch]$CopyLegacyPlugins  # Only needed for plugins that haven't been updated
)

$ErrorActionPreference = "Stop"
$rootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)


# Target directories - support both old and new folder names
$tauriPluginsDir = $null
if (Test-Path (Join-Path $rootDir "src-tauri")) {
    $tauriPluginsDir = Join-Path $rootDir "src-tauri\plugins"
} else {
    Write-Host "[X] ERROR: Could not find Tauri directory (src-tauri)" -ForegroundColor Red
    exit 1
}

$vitePublicPlugins = $null
if (Test-Path (Join-Path $rootDir "src")) {
    $vitePublicPlugins = Join-Path $rootDir "src\public\plugins"
} else {
    Write-Host "[X] ERROR: Could not find frontend directory (src)" -ForegroundColor Red
    exit 1
}

Write-Host "`n===============================================" -ForegroundColor Cyan
Write-Host "Plugin Deployment" -ForegroundColor Cyan
Write-Host "===============================================`n" -ForegroundColor Cyan

# Create plugin directory if it doesn't exist
if (!(Test-Path $tauriPluginsDir)) {
    New-Item -ItemType Directory -Path $tauriPluginsDir -Force | Out-Null
    Write-Host "[OK] Created plugins directory" -ForegroundColor Green
}

# Check if Vite public/plugins is a junction pointing to tauri/plugins
$junctionOk = $false
if (Test-Path $vitePublicPlugins) {
    $item = Get-Item $vitePublicPlugins -Force

    # Check if it's a junction using LinkType property (most reliable)
    $isJunction = $item.LinkType -eq "Junction"

    if ($isJunction) {
        # Get the target from the Target property
        $currentTarget = $item.Target[0]
        $expectedTarget = $tauriPluginsDir

        if ($currentTarget -eq $expectedTarget) {
            $junctionOk = $true
            Write-Host "[OK] Frontend plugins folder is correctly linked" -ForegroundColor Green
            Write-Host "  $vitePublicPlugins -> $tauriPluginsDir" -ForegroundColor Gray
        } else {
            Write-Host "[WARN] Junction points to wrong target" -ForegroundColor Yellow
            Write-Host "  Current: $currentTarget" -ForegroundColor Gray
            Write-Host "  Expected: $expectedTarget" -ForegroundColor Gray
            Write-Host "  Recreating junction..." -ForegroundColor Cyan

            # Remove and recreate
            cmd /c "rmdir `"$vitePublicPlugins`"" 2>&1 | Out-Null
            cmd /c "mklink /J `"$vitePublicPlugins`" `"$tauriPluginsDir`"" 2>&1 | Out-Null
            $junctionOk = $true
            Write-Host "[OK] Junction recreated" -ForegroundColor Green
        }
    } else {
        Write-Host "[WARN] plugins is a regular directory (not a junction)" -ForegroundColor Yellow
        Write-Host "  Converting to junction for better dev experience..." -ForegroundColor Cyan

        # Remove directory and create junction
        Remove-Item -Path $vitePublicPlugins -Recurse -Force
        cmd /c "mklink /J `"$vitePublicPlugins`" `"$tauriPluginsDir`"" 2>&1 | Out-Null

        if ($LASTEXITCODE -eq 0) {
            $junctionOk = $true
            Write-Host "[OK] Converted to junction" -ForegroundColor Green
        } else {
            Write-Host "[X] Failed to create junction, will copy files instead" -ForegroundColor Red
        }
    }
} else {
    Write-Host "-> Creating frontend plugins junction..." -ForegroundColor Cyan

    # Ensure parent directory exists
    $publicDir = Split-Path -Parent $vitePublicPlugins
    if (!(Test-Path $publicDir)) {
        New-Item -ItemType Directory -Path $publicDir -Force | Out-Null
    }

    # Create junction using cmd (more reliable than PowerShell)
    cmd /c "mklink /J `"$vitePublicPlugins`" `"$tauriPluginsDir`"" 2>&1 | Out-Null

    if ($LASTEXITCODE -eq 0) {
        $junctionOk = $true
        Write-Host "[OK] Created junction: frontend/public/plugins -> tauri/plugins" -ForegroundColor Green
        Write-Host "  This eliminates copy overhead in dev mode!" -ForegroundColor Gray
    } else {
        Write-Host "[WARN] Could not create junction, will copy files instead" -ForegroundColor Yellow
        Write-Host "  (This is normal if you don't have admin privileges)" -ForegroundColor Gray
    }
}

Write-Host ""

# Summary
Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "Deployment Summary" -ForegroundColor Cyan
Write-Host "===============================================" -ForegroundColor Cyan

if ($junctionOk) {
    Write-Host "[OK] Junction configured" -ForegroundColor Green
    Write-Host "  Plugins build directly to: $tauriPluginsDir" -ForegroundColor Gray
    Write-Host "  Frontend serves from: $vitePublicPlugins (junction)" -ForegroundColor Gray
} else {
    Write-Host "[WARN] Junction not available" -ForegroundColor Yellow
    Write-Host "  Plugins in: $tauriPluginsDir" -ForegroundColor Gray
}

Write-Host "`n[OK] Deployment complete!" -ForegroundColor Green
Write-Host "  Modern plugins build directly to src-tauri/plugins/" -ForegroundColor Gray
Write-Host "  No copy step needed!" -ForegroundColor Gray
Write-Host ""

