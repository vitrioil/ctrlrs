#!/bin/bash
# ctrlrs shell integration for Bash
function enhanced_ctrl_r() {
    # Use full path to ensure the command is found
    local ctrlrs_path="${HOME}/.local/bin/ctrlrs"
    if [ ! -x "${ctrlrs_path}" ]; then
        # Try to find ctrlrs in PATH as fallback
        ctrlrs_path=$(which ctrlrs 2>/dev/null)
    fi

    if [ -x "${ctrlrs_path}" ]; then
        # Create a temporary file to store the selected command
        local temp_file
        temp_file=$(mktemp "/tmp/ctrlrs.XXXXXX") || return

        # Run ctrlrs with the output file option
        "${ctrlrs_path}" -o "$temp_file"

        # Read the selected command from the temp file if it exists and has content
        if [ -f "$temp_file" ] && [ -s "$temp_file" ]; then
            local result
            result=$(cat "$temp_file")
	    echo $result

            # Update the command line with the selected command
            READLINE_LINE="$result"
            READLINE_POINT=${#READLINE_LINE}
        fi

        # Delete the temp file
        #rm -f "$temp_file"
    else
        echo "ctrlrs not found. Please make sure it's installed." >/dev/tty
    fi
}

# Override Ctrl+R with our enhanced version
bind -x '"\C-r": enhanced_ctrl_r'

# To install, add this to your ~/.bashrc:
# source /path/to/bash_integration.sh