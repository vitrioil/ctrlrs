#!/bin/zsh
# ctrlrs shell integration for ZSH on macOS

# Function to find the ctrlrs executable
function _find_ctrlrs() {
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi
    echo "$ctrlrs_path"
}

function enhanced_ctrl_r() {
    local ctrlrs_path=$(_find_ctrlrs)
    
    if [ ! -x "${ctrlrs_path}" ]; then
        echo "ctrlrs not found. Please make sure it's installed." >/dev/tty
        zle reset-prompt
        return 1
    fi
    
    # Ensure proper terminal behavior
    zle -I
    zle reset-prompt
    
    # Create a temporary file to store the selected command
    local temp_file
    temp_file=$(mktemp "/tmp/ctrlrs.XXXXXX") || return
    
    # Run ctrlrs with the output file option using TTY redirection
    # This is the fix for macOS ZSH Ctrl+R issues
    <$TTY "${ctrlrs_path}" -o "$temp_file"
    
    # Read the selected command from the temp file if it exists and has content
    if [ -f "$temp_file" ] && [ -s "$temp_file" ]; then
        local result
        result=$(cat "$temp_file")
        
        # Set the command buffer if a result was selected
        BUFFER="$result"
        CURSOR=${#BUFFER}
    fi
    
    # Delete the temp file
    rm -f "$temp_file"
    
    # Refresh prompt
    zle reset-prompt
}

# Bind the function to Ctrl+R
zle -N enhanced_ctrl_r
bindkey '^R' enhanced_ctrl_r

# Also provide the 'c' command for backward compatibility
function c() {
    local ctrlrs_path=$(_find_ctrlrs)
    
    if [ ! -x "${ctrlrs_path}" ]; then
        echo "ctrlrs not found. Please make sure it's installed."
        return 1
    fi
    
    # Create a temporary file to store the selected command
    local temp_file
    temp_file=$(mktemp "/tmp/ctrlrs.XXXXXX") || return
    
    # Run ctrlrs with the output file option
    <$TTY "${ctrlrs_path}" -o "$temp_file"
    
    # Read the selected command from the temp file if it exists and has content
    if [ -f "$temp_file" ] && [ -s "$temp_file" ]; then
        local result
        result=$(cat "$temp_file")
        
        # Print the command to the terminal
        print -z "$result"
    fi
    
    # Delete the temp file
    rm -f "$temp_file"
}

# To install, add this to your ~/.zshrc:
# source /path/to/mac/zsh_integration.sh
