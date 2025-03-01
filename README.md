# ctrlrs

Enhanced Ctrl-R for shell history with n-dimensional search.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **N-dimensional search**: Press Ctrl-R once to filter by a first term, then press Ctrl-R again to add a second filter, and so on (up to 5 dimensions)
- **Auto-detection**: Automatically detects your shell (Bash, Zsh, Fish) and reads the appropriate history file
- **Real-time filtering**: See results update as you type
- **Easy navigation**: Use arrow keys to navigate through results
- **Cross-platform**: Works on Linux and macOS
- **Lightweight**: Fast and efficient with minimal dependencies

## Demo

![Demo GIF](docs/demo.gif)

## Installation

### Quick Install (Linux/macOS)

```bash
# Clone the repository
git clone https://github.com/yourusername/ctrlrs.git
cd ctrlrs

# Run the install script
./scripts/install.sh
```

The install script will:
1. Build the binary from source
2. Install it to `~/.local/bin/`
3. Add shell integration to your shell configuration file
4. Make sure `~/.local/bin` is in your PATH

### Manual Installation

#### 1. Build from source

```bash
# Clone the repository
git clone https://github.com/yourusername/ctrlrs.git
cd ctrlrs

# Build with Cargo
cargo build --release

# Copy the binary to a location in your PATH
cp target/release/ctrlrs ~/.local/bin/
chmod +x ~/.local/bin/ctrlrs
```

#### 2. Add shell integration

##### Bash

Add to your `~/.bashrc`:

```bash
# ctrlrs shell integration
function enhanced_ctrl_r() {
    local result=$(ctrlrs)
    if [ -n "$result" ]; then
        READLINE_LINE="$result"
        READLINE_POINT=${#READLINE_LINE}
    fi
}
# Override Ctrl+R with our enhanced version
bind -x '"\C-r": enhanced_ctrl_r'
```

##### Zsh

Add to your `~/.zshrc`:

```zsh
# ctrlrs shell integration
function enhanced_ctrl_r() {
    local result=$(ctrlrs)
    if [ -n "$result" ]; then
        BUFFER="$result"
        CURSOR=${#BUFFER}
    fi
}
# Override Ctrl+R with our enhanced version
zle -N enhanced_ctrl_r
bindkey '^R' enhanced_ctrl_r
```

##### Fish

Add to your `~/.config/fish/config.fish`:

```fish
# ctrlrs shell integration
function fish_user_key_bindings
    bind \cr 'commandline (ctrlrs)'
end
```

## Usage

1. Press `Ctrl+R` in your terminal to activate the enhanced history search
2. Type your first search term to filter commands
3. Navigate through results with `Up/Down` arrow keys
4. Press `Ctrl+R` again to enter a second search term for nested filtering
5. Continue pressing `Ctrl+R` to add more filters (up to 5 dimensions)
6. Press `Enter` to select a command or `Esc` to cancel

## Configuration

`ctrlrs` works out of the box with no configuration, but you can customize its behavior with command-line options:

```
USAGE:
    ctrlrs [OPTIONS]

OPTIONS:
    -d, --debug                 Enable debug logging
    -s, --shell <SHELL>         Specify shell type (auto-detected if not specified)
    -h, --history-file <PATH>   Specify history file path (auto-detected if not specified)
    -h, --help                  Print help information
    -V, --version               Print version information
```

## Uninstallation

```bash
# Run the uninstall script
./scripts/uninstall.sh
```

Or manually:
1. Remove the binary: `rm ~/.local/bin/ctrlrs`
2. Remove the shell integration from your shell configuration file

## Development

### Prerequisites

- Rust 1.56.0 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
