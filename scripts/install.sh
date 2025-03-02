#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    *)          echo -e "${RED}Unsupported OS: ${OS}${NC}"; exit 1;;
esac

echo -e "${BLUE}Detected OS: ${OS}${NC}"

# Detect architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64*)    ARCH="x86_64";;
    arm64*)     ARCH="aarch64";;
    aarch64*)   ARCH="aarch64";;
    *)          echo -e "${RED}Unsupported architecture: ${ARCH}${NC}"; exit 1;;
esac

echo -e "${BLUE}Detected architecture: ${ARCH}${NC}"

# Installation directory
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "${INSTALL_DIR}"

# Add to PATH if not already there
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo -e "${YELLOW}Adding ${INSTALL_DIR} to PATH${NC}"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "${HOME}/.profile"
    export PATH="${INSTALL_DIR}:$PATH"
fi

# Build from source or download binary
if [ -f "Cargo.toml" ]; then
    echo -e "${BLUE}Building from source...${NC}"
    cargo build --release
    cp target/release/ctrlrs "${INSTALL_DIR}/ctrlrs"
else
    # This would be the URL to download pre-built binaries
    # For now, we'll just show a message
    echo -e "${RED}Pre-built binaries not available yet. Please build from source.${NC}"
    echo -e "${YELLOW}git clone https://github.com/yourusername/ctrlrs.git${NC}"
    echo -e "${YELLOW}cd ctrlrs${NC}"
    echo -e "${YELLOW}cargo build --release${NC}"
    echo -e "${YELLOW}cp target/release/ctrlrs ~/.local/bin/${NC}"
    exit 1
fi

# Make executable
chmod +x "${INSTALL_DIR}/ctrlrs"

echo -e "${GREEN}ctrlrs installed successfully to ${INSTALL_DIR}/ctrlrs${NC}"

# Shell integration
SHELL_NAME=$(basename "$SHELL")
case "${SHELL_NAME}" in
    bash)
        SHELL_RC="${HOME}/.bashrc"
        ;;
    zsh)
        SHELL_RC="${HOME}/.zshrc"
        ;;
    fish)
        SHELL_RC="${HOME}/.config/fish/config.fish"
        mkdir -p "$(dirname "${SHELL_RC}")"
        ;;
    *)
        echo -e "${YELLOW}Unknown shell: ${SHELL_NAME}. Please manually add shell integration.${NC}"
        exit 0
        ;;
esac

# Get the directory where the install script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Check if shell integration is already added
if grep -q "ctrlrs shell integration" "${SHELL_RC}" 2>/dev/null; then
    echo -e "${YELLOW}Shell integration already exists in ${SHELL_RC}${NC}"
else
    echo -e "${BLUE}Adding shell integration to ${SHELL_RC}${NC}"
    
    case "${SHELL_NAME}" in
        bash)
            # Check if bash integration script exists
            if [ -f "${SCRIPT_DIR}/bash_integration.sh" ]; then
                # Add source command to shell RC file
                echo -e "\n# Source ctrlrs shell integration" >> "${SHELL_RC}"
                echo "source ${SCRIPT_DIR}/bash_integration.sh" >> "${SHELL_RC}"
            else
                echo -e "${RED}Bash integration script not found at ${SCRIPT_DIR}/bash_integration.sh${NC}"
                exit 1
            fi
            ;;
        zsh)
            # Check if zsh integration script exists
            if [ -f "${SCRIPT_DIR}/zsh_integration.sh" ]; then
                # Add source command to shell RC file
                echo -e "\n# Source ctrlrs shell integration" >> "${SHELL_RC}"
                echo "source ${SCRIPT_DIR}/zsh_integration.sh" >> "${SHELL_RC}"
            else
                echo -e "${RED}Zsh integration script not found at ${SCRIPT_DIR}/zsh_integration.sh${NC}"
                exit 1
            fi
            ;;
        fish)
            # Check if fish integration script exists
            if [ -f "${SCRIPT_DIR}/fish_integration.fish" ]; then
                # Add source command to shell RC file
                echo -e "\n# Source ctrlrs shell integration" >> "${SHELL_RC}"
                echo "source ${SCRIPT_DIR}/fish_integration.fish" >> "${SHELL_RC}"
            else
                echo -e "${RED}Fish integration script not found at ${SCRIPT_DIR}/fish_integration.fish${NC}"
                exit 1
            fi
            ;;
    esac
    
    echo -e "${GREEN}Shell integration added to ${SHELL_RC}${NC}"
    echo -e "${YELLOW}Please restart your shell or run 'source ${SHELL_RC}' to apply changes${NC}"
fi

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${BLUE}Usage: Press Ctrl+R in your terminal to use the enhanced history search${NC}"
