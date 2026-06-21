@echo off
echo ============================================
echo    Nexora Language Installer
echo ============================================
echo.

:: Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed!
    echo.
    echo Please install Rust first:
    echo   https://rustup.rs
    echo.
    echo After installing Rust, run this installer again.
    echo.
    pause
    exit /b 1
)

echo [1/3] Building Nexora...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    pause
    exit /b 1
)

echo [2/3] Installing Nexora...
set INSTALL_DIR=%USERPROFILE%\.nexora
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
copy "target\release\nexora.exe" "%INSTALL_DIR%\nexora.exe"

echo [3/3] Adding to PATH...
setx PATH "%PATH%;%INSTALL_DIR%"

echo.
echo ============================================
echo    Nexora installed successfully!
echo ============================================
echo.
echo You can now use Nexora:
echo   nexora                    - Start REPL
echo   nexora run file.nx        - Run a file
echo   nexora help               - Show help
echo.
echo Restart your terminal for PATH changes to take effect.
echo.
pause
