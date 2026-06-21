# Nexora Language Installer
# PowerShell se Nexora install karna
# Usage: irm https://nexora.dev/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Config
$VERSION = "1.0.0"
$INSTALL_DIR = "$env:USERPROFILE\.nexora"
$BIN_DIR = "$INSTALL_DIR\bin"

Write-Host ""
Write-Host "  _   _                       _   _" -ForegroundColor Cyan
Write-Host " | \ | |                     | \ | |" -ForegroundColor Cyan
Write-Host " |  \| | _____  ___   _  ___|  \| | ___  _ __ ___   ___" -ForegroundColor Cyan
Write-Host " | . ` |/ _ \ \/ / | | |/ __| . ` |/ _ \| '_ ` _ \ / _ \" -ForegroundColor Cyan
Write-Host " | |\  |  __/>  <| |_| | (__| |\  | (_) | | | | | |  __/" -ForegroundColor Cyan
Write-Host " \_| \_/\___/_/\_\\__, |\___|\_| \_/\___/|_| |_| |_|\___|" -ForegroundColor Cyan
Write-Host "                    __/ |" -ForegroundColor Cyan
Write-Host "                   |___/  v$VERSION" -ForegroundColor Cyan
Write-Host ""

# Create directories
Write-Host "[1/4] Creating directories..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path $BIN_DIR -Force | Out-Null

# Detect OS and Architecture
Write-Host "[2/4] Detecting system..." -ForegroundColor Yellow
$os = if ($IsWindows -or $env:OS -eq "Windows_NT") { "windows" } elseif ($IsMacOS) { "darwin" } else { "linux" }
$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
Write-Host "  OS: $os | Arch: $arch" -ForegroundColor Gray

# Download binary
Write-Host "[3/4] Downloading Nexora v$VERSION..." -ForegroundColor Yellow

# Try GitHub releases first, then fallback to direct
$urls = @(
    "https://github.com/nexora-lang/nexora/releases/download/v$VERSION/nexora-$os-$arch.zip",
    "https://github.com/nexora-lang/nexora/releases/download/v$VERSION/nexora-$os-$arch.tar.gz",
    "https://nexora.dev/download/$os/$arch/nexora.zip"
)

$downloaded = $false
foreach ($url in $urls) {
    try {
        Write-Host "  Trying: $url" -ForegroundColor Gray
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        
        $zipPath = "$env:TEMP\nexora.zip"
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing -TimeoutSec 30
        
        # Extract
        Expand-Archive -Path $zipPath -DestinationPath $BIN_DIR -Force
        Remove-Item $zipPath -Force
        $downloaded = $true
        Write-Host "  Downloaded!" -ForegroundColor Green
        break
    } catch {
        Write-Host "  Failed, trying next..." -ForegroundColor Gray
    }
}

# If download failed, try to copy from local build
if (-not $downloaded) {
    Write-Host "  Download failed. Checking for local build..." -ForegroundColor Yellow
    $localPaths = @(
        "C:\Users\USER\Downloads\LofiLink\nx\nexora\target\release\nexora.exe",
        ".\nexora.exe",
        ".\nexora"
    )
    
    foreach ($localPath in $localPaths) {
        if (Test-Path $localPath) {
            Copy-Item $localPath "$BIN_DIR\nexora.exe" -Force
            $downloaded = $true
            Write-Host "  Copied from local!" -ForegroundColor Green
            break
        }
    }
}

if (-not $downloaded) {
    Write-Host ""
    Write-Host "  Error: Could not download Nexora" -ForegroundColor Red
    Write-Host "  Manual install: https://nexora.dev/install" -ForegroundColor Yellow
    exit 1
}

# Also install nxm (package manager)
Write-Host "  Installing nxm (package manager)..." -ForegroundColor Gray
$nxmLocal = "C:\Users\USER\Downloads\LofiLink\nx\nexora\nxm\target\release\nxm.exe"
if (Test-Path $nxmLocal) {
    Copy-Item $nxmLocal "$BIN_DIR\nxm.exe" -Force
    Write-Host "  nxm installed!" -ForegroundColor Green
}

# Add to PATH
Write-Host "[4/4] Adding to PATH..." -ForegroundColor Yellow
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$BIN_DIR*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$BIN_DIR", "User")
    $env:Path = "$env:Path;$BIN_DIR"
    Write-Host "  Added to PATH!" -ForegroundColor Green
} else {
    Write-Host "  Already in PATH!" -ForegroundColor Green
}

# Verify
Write-Host ""
Write-Host "Verifying installation..." -ForegroundColor Yellow
try {
    $ver = & "$BIN_DIR\nexora.exe" --version 2>&1
    Write-Host "  $ver" -ForegroundColor Green
} catch {
    Write-Host "  Installed at: $BIN_DIR" -ForegroundColor Gray
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Installation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Restart PowerShell, then run:" -ForegroundColor Yellow
Write-Host "  nexora                    # Start REPL" -ForegroundColor White
Write-Host "  nexora run file.nx        # Run a file" -ForegroundColor White
Write-Host "  nxm init                  # Create project" -ForegroundColor White
Write-Host "  nxm install lodash-nx     # Install package" -ForegroundColor White
Write-Host ""
Write-Host "Docs: https://nexora.dev/docs" -ForegroundColor Gray
Write-Host ""
