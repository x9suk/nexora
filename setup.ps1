# Nexora Local Installer
# Builds and installs from source

$ErrorActionPreference = "Stop"

$SOURCE_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path
$INSTALL_DIR = "$env:USERPROFILE\.nexora\bin"

Write-Host ""
Write-Host "Installing Nexora Package Manager (nxm)..." -ForegroundColor Cyan
Write-Host ""

# Create install directory
if (!(Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
}

# Build nxm if not exists
$nxmExe = "$SOURCE_DIR\nxm\target\release\nxm.exe"
if (!(Test-Path $nxmExe)) {
    Write-Host "Building nxm..." -ForegroundColor Yellow
    Push-Location "$SOURCE_DIR\nxm"
    cargo build --release
    Pop-Location
}

# Copy binary
Write-Host "Installing nxm to $INSTALL_DIR..." -ForegroundColor Yellow
Copy-Item $nxmExe "$INSTALL_DIR\nxm.exe" -Force

# Also copy nexora
$nexoraExe = "$SOURCE_DIR\target\release\nexora.exe"
if (Test-Path $nexoraExe) {
    Copy-Item $nexoraExe "$INSTALL_DIR\nexora.exe" -Force
}

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$INSTALL_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$INSTALL_DIR", "User")
    Write-Host "Added to PATH!" -ForegroundColor Green
}

# Refresh PATH for current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

Write-Host ""
Write-Host "nxm installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Run: nxm --help" -ForegroundColor Yellow
Write-Host ""
