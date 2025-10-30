@echo off
echo =====================================
echo Building Time Tracker Plugin
echo =====================================
echo.

set PROJECT_ROOT=D:\workspaces\modulaur
set PLUGIN_ROOT=%PROJECT_ROOT%\plugins\featured\time-tracker
set FRONTEND_DIR=%PLUGIN_ROOT%\frontend
set TARGET_DIR=%PROJECT_ROOT%\src-tauri\plugins\time-tracker

echo [1/2] Building frontend...
cd /d %FRONTEND_DIR%

if not exist "node_modules" (
    echo   Installing dependencies...
    call npm install
)

echo   Running build...
call npm run build

if %ERRORLEVEL% neq 0 (
    echo   Frontend build failed!
    exit /b 1
)

echo   Frontend built successfully
echo.

echo [2/2] Verifying...
if exist "%TARGET_DIR%\manifest.json" (
    if exist "%TARGET_DIR%\frontend\dist\time-tracker.umd.js" (
        if exist "%TARGET_DIR%\frontend\dist\style.css" (
            echo   All files OK
            echo.
            echo =====================================
            echo BUILD COMPLETE!
            echo =====================================
            echo.
            echo Plugin installed at: %TARGET_DIR%
            echo Note: Vite config builds directly to src-tauri/plugins (no copy needed)
            echo Restart the app to load the plugin!
        ) else (
            echo   ERROR: style.css missing!
            exit /b 1
        )
    ) else (
        echo   ERROR: time-tracker.umd.js missing!
        exit /b 1
    )
) else (
    echo   ERROR: manifest.json missing!
    exit /b 1
)

