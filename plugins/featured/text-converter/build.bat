@echo off
REM Build script for text-converter plugin
echo ============================================
echo Building Text Converter Plugin
echo ============================================

cd /d "%~dp0"

echo.
echo [1/2] Installing dependencies...
cd frontend
call npm install
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: npm install failed
    exit /b 1
)

echo.
echo [2/2] Building UMD bundle...
call npm run build
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Build failed
    exit /b 1
)

echo.
echo ============================================
echo ✅ Build Complete!
echo ============================================
echo Output:
echo   - frontend/dist/text-converter.umd.js
echo   - frontend/dist/text-converter.css
echo.
echo Built directly to:
echo   - ../../src-tauri/plugins/text-converter/
echo.
echo Note: Vite config builds directly to src-tauri/plugins (no copy needed)
echo ============================================

