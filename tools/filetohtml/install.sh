#!/bin/bash

# Exit on any error
set -e

# Step 1: Check if Rust is installed
if ! command -v cargo &> /dev/null
then
    echo "Rust is not installed. Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# Step 2: Build the Rust project
echo "Building the project..."
cargo build --release

# Step 3: Install the binary globally using cargo
echo "Installing the tool globally..."
cargo install --path .

# Step 4: Ensure that ~/.cargo/bin is in the PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "$HOME/.cargo/bin is not in the PATH. Adding it now..."
    
    # Add to the appropriate shell configuration file based on the shell used
    if [ -n "$ZSH_VERSION" ]; then
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
        echo "Added to ~/.zshrc"
        source ~/.zshrc
    elif [ -n "$BASH_VERSION" ]; then
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
        echo "Added to ~/.bashrc"
        source ~/.bashrc
    else
        echo "Unknown shell. Please add ~/.cargo/bin to your PATH manually."
    fi
else
    echo "$HOME/.cargo/bin is already in the PATH."
fi

echo "Installation complete! You can now use the tool globally."