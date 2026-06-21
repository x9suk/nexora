@echo off
REM Nexora Setup Script for Windows

echo.
echo  ==============================
echo    Nexora Language Setup
echo  ==============================
echo.

REM Check for Rust
echo [1/3] Checking Rust installation...
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo.
    echo  ERROR: Rust is not installed!
    echo  Install from: https://rustup.rs
    echo.
    exit /b 1
)
echo  OK - Rust found
rustc --version

REM Check for Cargo
echo.
echo [2/3] Checking Cargo...
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo.
    echo  ERROR: Cargo is not installed!
    echo.
    exit /b 1
)
echo  OK - Cargo found

REM Build the project
echo.
echo [3/3] Building Nexora...
cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo  ERROR: Build failed!
    echo.
    exit /b 1
)

echo.
echo  ==============================
echo    Setup Complete!
echo  ==============================
echo.
echo  Usage:
echo    nexora                     Start REPL
echo    nexora examples\hello.nx   Run a file
echo    nexora run file.nx         Run a file
echo.
echo  Examples:
    echo    target\release\nexora.exe examples\hello.nx
echo    target\release\nexora.exe examples\functions.nx
echo    target\release\nexora.exe examples\loops.nx
echo.
