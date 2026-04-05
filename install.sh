#!/bin/bash
set -euo pipefail

REPO="vericontext/snact"
INSTALL_DIR="/usr/local/bin"
BINARY="snact"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() { echo -e "${GREEN}==>${NC} $1"; }
warn() { echo -e "${YELLOW}==>${NC} $1"; }
error() { echo -e "${RED}error:${NC} $1" >&2; exit 1; }

# Detect platform
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Darwin) os="darwin" ;;
        Linux)  os="linux" ;;
        *)      error "Unsupported OS: $(uname -s). snact supports macOS and Linux." ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)  arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac

    echo "snact-${os}-${arch}"
}

# Get latest release tag
get_latest_version() {
    local url="https://api.github.com/repos/${REPO}/releases/latest"
    local version

    if command -v curl &>/dev/null; then
        version=$(curl -fsSL "$url" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')
    elif command -v wget &>/dev/null; then
        version=$(wget -qO- "$url" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    if [ -z "$version" ]; then
        error "Could not determine latest version. Check https://github.com/${REPO}/releases"
    fi

    echo "$version"
}

# Download and install
install() {
    local platform version url tmpdir

    platform=$(detect_platform)
    info "Detected platform: ${platform}"

    # Use provided version or fetch latest
    version="${SNACT_VERSION:-$(get_latest_version)}"
    info "Installing snact ${version}"

    url="https://github.com/${REPO}/releases/download/${version}/${platform}.tar.gz"

    tmpdir=$(mktemp -d)
    trap 'rm -rf "${tmpdir:-}"' EXIT

    info "Downloading ${url}"
    if command -v curl &>/dev/null; then
        curl -fsSL "$url" -o "${tmpdir}/snact.tar.gz"
    else
        wget -qO "${tmpdir}/snact.tar.gz" "$url"
    fi

    info "Extracting"
    tar xzf "${tmpdir}/snact.tar.gz" -C "$tmpdir"

    # Install binary
    if [ -w "$INSTALL_DIR" ]; then
        mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
    else
        info "Installing to ${INSTALL_DIR} (requires sudo)"
        sudo mv "${tmpdir}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
    fi

    chmod +x "${INSTALL_DIR}/${BINARY}"

    info "snact ${version} installed to ${INSTALL_DIR}/${BINARY}"
    echo ""

    # Verify
    if command -v snact &>/dev/null; then
        echo "  Run 'snact --help' to get started."
    else
        warn "${INSTALL_DIR} is not in your PATH."
        echo "  Add it with: export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi
}

install
