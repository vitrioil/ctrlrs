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

# Note: Ctrl+R binding is not reliable on macOS with ZSH due to terminal I/O issues
# We recommend using the 'r' command instead (defined below)
# This function is kept here for reference but is not bound to Ctrl+R by default
function enhanced_ctrl_r() {
    # This function is intentionally left empty as it's not reliable on macOS
    # Use the 'r' command instead
    zle -M "Please use the 'r' command instead of Ctrl+R on macOS"
    zle reset-prompt
}

# We don't bind Ctrl+R by default as it causes IO errors on macOS
# zle -N enhanced_ctrl_r
# bindkey '^R' enhanced_ctrl_r

# Also provide the 'r' command as an alternative
function r() {
    local ctrlrs_path=$(_find_ctrlrs)
    
    if [ ! -x "${ctrlrs_path}" ]; then
        echo "ctrlrs not found. Please make sure it's installed."
        return 1
    fi
    
    # Create a temporary file to store the selected command
    local temp_file
    temp_file=$(mktemp "/tmp/ctrlrs.XXXXXX") || return
    
    # Run ctrlrs with the output file option
    "${ctrlrs_path}" -o "$temp_file"
    
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
