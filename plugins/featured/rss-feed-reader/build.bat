@echo off
echo ========================================
echo Building RSS Feed Reader Plugin
echo ========================================

cd /d "%~dp0frontend"

echo.
echo [1/2] Installing dependencies...
call npm install

if errorlevel 1 (
    echo ERROR: npm install failed
    exit /b 1
)

echo.
echo [2/2] Building plugin...
call npm run build

if errorlevel 1 (
    echo ERROR: Build failed
    exit /b 1
)

echo.
echo ========================================
echo Build Complete!
echo ========================================
echo.
echo Plugin built to: src-tauri\plugins\rss-feed-reader\
echo.
echo Note: Vite config builds directly to src-tauri/plugins (no copy needed)
echo Source files remain in: plugins\featured\rss-feed-reader\
