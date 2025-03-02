#!/bin/zsh
function enhanced_ctrl_r() {
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi

    if [ -x "${ctrlrs_path}" ]; then
        # Ensure proper terminal behavior
        zle -I
        zle reset-prompt

        # Create a temporary file to store the selected command
        local temp_file
        temp_file=$(mktemp)
        
        # Run ctrlrs with the output file option
        "$ctrlrs_path" -o "$temp_file" </dev/tty >/dev/tty 2>/dev/null
        
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
    else
        echo "ctrlrs not found. Please make sure it's installed." >/dev/tty
        zle reset-prompt
    fi
}

# Bind the function to Ctrl+R
zle -N enhanced_ctrl_r
bindkey '^R' enhanced_ctrl_r
