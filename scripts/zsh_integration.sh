#!/bin/zsh

# ctrlrs shell integration for Zsh
function enhanced_ctrl_r() {
    # Use full path to ensure the command is found
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        # Try to find ctrlrs in PATH as fallback
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi
    
    if [ -x "${ctrlrs_path}" ]; then
        # Run ctrlrs and capture its output
        local result=$("${ctrlrs_path}")
        if [ -n "$result" ]; then
            # Set the command line to the selected command
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

# To install, add this to your ~/.zshrc:
# source /path/to/zsh_integration.sh
