#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Installation directory
INSTALL_DIR="${HOME}/.local/bin"
BINARY_PATH="${INSTALL_DIR}/ctrlrs"

# Remove binary
if [ -f "${BINARY_PATH}" ]; then
    echo -e "${BLUE}Removing ${BINARY_PATH}${NC}"
    rm "${BINARY_PATH}"
    echo -e "${GREEN}Binary removed successfully${NC}"
else
    echo -e "${YELLOW}Binary not found at ${BINARY_PATH}${NC}"
fi

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
        ;;
    *)
        echo -e "${YELLOW}Unknown shell: ${SHELL_NAME}. Please manually remove shell integration.${NC}"
        exit 0
        ;;
esac

# Remove shell integration
if [ -f "${SHELL_RC}" ]; then
    if grep -q "ctrlrs shell integration" "${SHELL_RC}"; then
        echo -e "${BLUE}Removing shell integration from ${SHELL_RC}${NC}"
        
        # Create a temporary file
        TEMP_FILE=$(mktemp)
        
        # Remove the shell integration lines
        # This will remove both the old style integration and the new source-based integration
        case "${SHELL_NAME}" in
            bash)
                # Remove both old-style integration and new source-based integration
                sed -e '/# ctrlrs shell integration/,/bind -x/d' \
                    -e '/# Source ctrlrs shell integration/,+1d' \
                    "${SHELL_RC}" > "${TEMP_FILE}"
                ;;
            zsh)
                # Remove both old-style integration and new source-based integration
                sed -e '/# ctrlrs shell integration/,/bindkey/d' \
                    -e '/# Source ctrlrs shell integration/,+1d' \
                    "${SHELL_RC}" > "${TEMP_FILE}"
                ;;
            fish)
                # Remove both old-style integration and new source-based integration
                sed -e '/# ctrlrs shell integration/,/end/d' \
                    -e '/# Source ctrlrs shell integration/,+1d' \
                    "${SHELL_RC}" > "${TEMP_FILE}"
                ;;
        esac
        
        # Replace the original file
        mv "${TEMP_FILE}" "${SHELL_RC}"
        
        echo -e "${GREEN}Shell integration removed from ${SHELL_RC}${NC}"
        echo -e "${YELLOW}Please restart your shell or run 'source ${SHELL_RC}' to apply changes${NC}"
    else
        echo -e "${YELLOW}Shell integration not found in ${SHELL_RC}${NC}"
    fi
else
    echo -e "${YELLOW}Shell configuration file ${SHELL_RC} not found${NC}"
fi

echo -e "${GREEN}Uninstallation complete!${NC}"
