#!/usr/bin/env fish

# ctrlrs shell integration for Fish

function fish_user_key_bindings
    set -g ctrlrs_path "$HOME/.local/bin/ctrlrs"
    if not test -x "$ctrlrs_path"
        set -g ctrlrs_path (command -v ctrlrs ^/dev/null)
    end

    if test -z "$ctrlrs_path"; or not test -x "$ctrlrs_path"
        echo "ctrlrs not found or not executable. Please install it." >/dev/tty
        return
    end

    function _enhanced_ctrl_r
        set -l temp_file (mktemp)

        # Debugging output
        echo "ctrlrs_path is: '$ctrlrs_path'" >/dev/tty

        echo "Running: $ctrlrs_path -o $temp_file" >/dev/tty
        $ctrlrs_path -o $temp_file

        if test -s "$temp_file"
            set -l result (cat "$temp_file")
            commandline -r -- $result
            commandline -f repaint
        end

        rm -f "$temp_file"
    end

    bind \cr _enhanced_ctrl_r
end

# To install, add this to your ~/.config/fish/config.fish:
# source /path/to/fish_integration.fish
