#!/bin/bash

# Define the installation directory
INSTALL_DIR="$HOME/.local/bin"

# Create the installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Get the directory of this script
SCRIPT_DIR=$(dirname "$(realpath "$0")")

# Copy the compiled binary to the installation directory
cp "$SCRIPT_DIR/target/release/zone_file_parser" "$INSTALL_DIR/zone_file_parser"

# Add the installation directory to the PATH if it's not already present
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "Adding $INSTALL_DIR to PATH"

  # Check if .bashrc or .zshrc exists, then add the directory to PATH
  if [ -f "$HOME/.bashrc" ]; then
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
    echo "Added $INSTALL_DIR to .bashrc"
  elif [ -f "$HOME/.zshrc" ]; then
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.zshrc"
    echo "Added $INSTALL_DIR to .zshrc"
  else
    echo "No .bashrc or .zshrc found. Please add $INSTALL_DIR to your PATH manually."
  fi
else
  echo "$INSTALL_DIR is already in your PATH."
fi

# Instructions for the user to apply changes to PATH
echo "Installation complete! Please restart your terminal or run the following command to apply changes to your PATH:"
echo "source ~/.bashrc  # or 'source ~/.zshrc' if you're using zsh"