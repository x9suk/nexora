@echo off
echo ============================================
echo    Nexora Language - Quick Install
echo ============================================
echo.

:: Create install directory
set INSTALL_DIR=%USERPROFILE%\.nexora
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo Copying nexora.exe...
copy "target\release\nexora.exe" "%INSTALL_DIR%\nexora.exe"

if %errorlevel% neq 0 (
    echo [ERROR] nexora.exe not found!
    echo Please run this from the nexora folder.
    pause
    exit /b 1
)

echo Adding to PATH...
setx PATH "%PATH%;%INSTALL_DIR%" >nul 2>nul

echo.
echo ============================================
echo    Done! Nexora installed to:
echo    %INSTALL_DIR%
echo ============================================
echo.
echo Restart your terminal, then use:
echo   nexora                - Start REPL
echo   nexora run file.nx    - Run a file
echo.
pause
