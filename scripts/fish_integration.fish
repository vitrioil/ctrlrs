#!/usr/bin/env fish

# ctrlrs shell integration for Fish

function fish_user_key_bindings
    # Use full path to ensure the command is found
    set ctrlrs_path "$HOME/.local/bin/ctrlrs"
    if not test -x "$ctrlrs_path"
        # Try to find ctrlrs in PATH as fallback
        set ctrlrs_path (which ctrlrs 2>/dev/null)
    end
    
    if test -x "$ctrlrs_path"
        # Define a function to handle Ctrl+R
        function _enhanced_ctrl_r
            # Run ctrlrs and capture its output
            set -l result (eval $ctrlrs_path)
            if test -n "$result"
                # Set the command line to the selected command
                commandline -r $result
                commandline -f repaint
            end
        end
        
        # Override Ctrl+R with our enhanced version
        bind \cr _enhanced_ctrl_r
    else
        echo "ctrlrs not found. Please make sure it's installed."
    end
end

# To install, add this to your ~/.config/fish/config.fish:
# source /path/to/fish_integration.fish
