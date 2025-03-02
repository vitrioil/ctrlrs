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
        # Clear the current line to avoid interference with TUI
        zle -I
        
        # Run ctrlrs in a way that allows TUI to display properly
        # but still captures the final selected command
        local result
        result=$("${ctrlrs_path}" </dev/tty >/dev/tty 2>/dev/null)
        
        # Update the command line with the selected command
        if [ -n "$result" ]; then
            BUFFER="$result"
            CURSOR=${#BUFFER}
        fi
        
        # Force the prompt to update
        zle reset-prompt
    else
        echo "ctrlrs not found. Please make sure it's installed." >/dev/tty
        zle reset-prompt
    fi
}

# Define the widget and bind it to Ctrl+R
zle -N enhanced_ctrl_r
bindkey '^R' enhanced_ctrl_r

# To install, add this to your ~/.zshrc:
# source /path/to/zsh_integration.sh
