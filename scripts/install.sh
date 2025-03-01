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

# Check if shell integration is already added
if grep -q "ctrlrs shell integration" "${SHELL_RC}" 2>/dev/null; then
    echo -e "${YELLOW}Shell integration already exists in ${SHELL_RC}${NC}"
else
    echo -e "${BLUE}Adding shell integration to ${SHELL_RC}${NC}"
    
    case "${SHELL_NAME}" in
        bash)
            cat >> "${SHELL_RC}" << 'EOF'

# ctrlrs shell integration
function enhanced_ctrl_r() {
    # Use full path to ensure the command is found
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        # Try to find ctrlrs in PATH as fallback
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi
    
    if [ -x "${ctrlrs_path}" ]; then
        local result=$("${ctrlrs_path}")
        if [ -n "$result" ]; then
            READLINE_LINE="$result"
            READLINE_POINT=${#READLINE_LINE}
        fi
    else
        echo "ctrlrs not found. Please make sure it's installed."
    fi
}
# Override Ctrl+R with our enhanced version
bind -x '"\C-r": enhanced_ctrl_r'
EOF
            ;;
        zsh)
            cat >> "${SHELL_RC}" << 'EOF'

# ctrlrs shell integration
function enhanced_ctrl_r() {
    # Use full path to ensure the command is found
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        # Try to find ctrlrs in PATH as fallback
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi
    
    if [ -x "${ctrlrs_path}" ]; then
        local result=$("${ctrlrs_path}")
        if [ -n "$result" ]; then
            BUFFER="$result"
            CURSOR=${#BUFFER}
        fi
    else
        echo "ctrlrs not found. Please make sure it's installed."
        zle reset-prompt
    fi
}
# Override Ctrl+R with our enhanced version
zle -N enhanced_ctrl_r
bindkey '^R' enhanced_ctrl_r
EOF
            ;;
        fish)
            cat >> "${SHELL_RC}" << 'EOF'

# ctrlrs shell integration
function fish_user_key_bindings
    # Use full path to ensure the command is found
    set ctrlrs_path "$HOME/.local/bin/ctrlrs"
    if not test -x "$ctrlrs_path"
        # Try to find ctrlrs in PATH as fallback
        set ctrlrs_path (which ctrlrs 2>/dev/null)
    end
    
    if test -x "$ctrlrs_path"
        # Override Ctrl+R with our enhanced version
        bind \cr "commandline ($ctrlrs_path)"
    else
        echo "ctrlrs not found. Please make sure it's installed."
    end
end
EOF
            ;;
    esac
    
    echo -e "${GREEN}Shell integration added to ${SHELL_RC}${NC}"
    echo -e "${YELLOW}Please restart your shell or run 'source ${SHELL_RC}' to apply changes${NC}"
fi

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${BLUE}Usage: Press Ctrl+R in your terminal to use the enhanced history search${NC}"
