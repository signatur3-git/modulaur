# Download SurrealDB binary for Windows
# This script downloads the SurrealDB standalone binary for bundling as a sidecar

$version = "v2.1.3"
$url = "https://github.com/surrealdb/surrealdb/releases/download/$version/surreal-$version.windows-amd64.exe"
$outputDir = "sidecar-binaries"
$outputFile = "$outputDir\surreal.exe"

Write-Host "Downloading SurrealDB $version for Windows..."

# Create directory if it doesn't exist
if (!(Test-Path $outputDir)) {
    New-Item -ItemType Directory -Path $outputDir | Out-Null
}

# Download the binary
try {
    Invoke-WebRequest -Uri $url -OutFile $outputFile
    Write-Host "✅ Downloaded SurrealDB to $outputFile"
    Write-Host "File size: $((Get-Item $outputFile).Length / 1MB) MB"
} catch {
    Write-Error "❌ Failed to download SurrealDB: $_"
    exit 1
}

Write-Host "`n✅ SurrealDB binary ready for sidecar bundling!"
Write-Host "Binary location: $outputFile"

