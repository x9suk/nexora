#!/bin/sh
# Nexora Language Installer
# Usage: curl -fsSL https://nexora.dev/install.sh | sh

set -e

VERSION="1.0.0"
INSTALL_DIR="$HOME/.nexora"
BIN_DIR="$INSTALL_DIR/bin"

echo ""
echo "  _   _                       _   _"
echo " | \ | |                     | \ | |"
echo " |  \| | _____  ___   _  ___|  \| | ___  _ __ ___   ___"
echo " | . \` |/ _ \ \/ / | | |/ __| . \` |/ _ \| '_ \` _ \ / _ \\"
echo " | |\  |  __/>  <| |_| | (__| |\  | (_) | | | | | |  __/"
echo " \_| \_/\___/_/\_\\__, |\___|\_| \_/\___/|_| |_| |_|\___|"
echo "                    __/ |"
echo "                   |___/  v$VERSION"
echo ""

# Create directories
echo "[1/4] Creating directories..."
mkdir -p "$BIN_DIR"

# Detect OS and Architecture
echo "[2/4] Detecting system..."
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  ARCH="x64" ;;
    aarch64) ARCH="arm64" ;;
    armv7l)  ARCH="arm" ;;
esac
echo "  OS: $OS | Arch: $ARCH"

# Download binary
echo "[3/4] Downloading Nexora v$VERSION..."

URLS="
https://github.com/nexora-lang/nexora/releases/download/v$VERSION/nexora-$OS-$ARCH.tar.gz
https://github.com/nexora-lang/nexora/releases/download/v$VERSION/nexora-$OS-$ARCH.zip
https://nexora.dev/download/$OS/$ARCH/nexora.tar.gz
"

DOWNLOADED=false
for url in $URLS; do
    echo "  Trying: $url"
    if curl -fsSL --connect-timeout 10 --max-time 60 -o /tmp/nexora.tar.gz "$url" 2>/dev/null; then
        echo "  Downloaded!"
        DOWNLOADED=true
        break
    fi
done

if [ "$DOWNLOADED" = false ]; then
    echo "  Error: Could not download Nexora"
    echo "  Manual install: https://nexora.dev/install"
    exit 1
fi

# Extract
echo "  Extracting..."
tar -xzf /tmp/nexora.tar.gz -C "$BIN_DIR" 2>/dev/null || unzip -o /tmp/nexora.zip -d "$BIN_DIR" 2>/dev/null
rm -f /tmp/nexora.tar.gz /tmp/nexora.zip

# Add to PATH
echo "[4/4] Adding to PATH..."
SHELL_RC=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
        echo "  Added to PATH in $SHELL_RC"
    else
        echo "  Already in PATH!"
    fi
fi

# Verify
echo ""
echo "Verifying installation..."
if [ -x "$BIN_DIR/nexora" ]; then
    "$BIN_DIR/nexora" --version
else
    echo "  Installed at: $BIN_DIR"
fi

echo ""
echo "========================================"
echo " Installation Complete!"
echo "========================================"
echo ""
echo "Restart your terminal, then run:"
echo "  nexora                    # Start REPL"
echo "  nexora run file.nx        # Run a file"
echo "  nxm init                  # Create project"
echo "  nxm install lodash-nx     # Install package"
echo ""
echo "Docs: https://nexora.dev/docs"
echo ""
