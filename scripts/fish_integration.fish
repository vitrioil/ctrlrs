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
        # Override Ctrl+R with our enhanced version
        bind \cr "commandline ($ctrlrs_path)"
    else
        echo "ctrlrs not found. Please make sure it's installed."
    end
end

# To install, add this to your ~/.config/fish/config.fish:
# source /path/to/fish_integration.fish
