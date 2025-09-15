#!/bin/bash
# Lax Uninstallation Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="$HOME/.local/bin"
LAX_CONFIG_DIR="$HOME/.config/lax"
LAX_BINARY="$INSTALL_DIR/lax"

echo -e "${BLUE}🗑️  Lax Uninstallation Script${NC}"
echo "============================"

# Function to detect shell
detect_shell() {
    if [[ -n "$ZSH_VERSION" ]]; then
        echo "zsh"
    elif [[ -n "$BASH_VERSION" ]]; then
        echo "bash"
    else
        case "$SHELL" in
            */zsh) echo "zsh" ;;
            */bash) echo "bash" ;;
            *) echo "unknown" ;;
        esac
    fi
}

# Function to remove binary
remove_binary() {
    echo -e "${BLUE}Removing lax binary...${NC}"
    
    if [[ -f "$LAX_BINARY" ]]; then
        rm -f "$LAX_BINARY"
        echo -e "${GREEN}✓ Binary removed${NC}"
    else
        echo -e "${YELLOW}Binary not found${NC}"
    fi
}

# Function to remove shell integration
remove_shell_integration() {
    local shell_type="$(detect_shell)"
    echo -e "${BLUE}Removing shell integration for $shell_type...${NC}"
    
    # Determine shell config file
    local shell_config=""
    
    case "$shell_type" in
        bash)
            if [[ -f "$HOME/.bashrc" ]]; then
                shell_config="$HOME/.bashrc"
            elif [[ -f "$HOME/.bash_profile" ]]; then
                shell_config="$HOME/.bash_profile"
            fi
            ;;
        zsh)
            shell_config="$HOME/.zshrc"
            ;;
    esac
    
    if [[ -n "$shell_config" && -f "$shell_config" ]]; then
        # Create backup
        cp "$shell_config" "$shell_config.lax_backup"
        
        # Remove lax integration lines
        sed -i '/# Lax shell integration/,/^fi$/d' "$shell_config"
        sed -i '/# Added by lax installer/,/export PATH.*\.local\/bin/d' "$shell_config"
        
        echo -e "${GREEN}✓ Shell integration removed from $shell_config${NC}"
        echo -e "${BLUE}Backup saved as $shell_config.lax_backup${NC}"
    fi
}

# Function to remove config directory
remove_config() {
    echo -e "${BLUE}Removing configuration directory...${NC}"
    
    if [[ -d "$LAX_CONFIG_DIR" ]]; then
        rm -rf "$LAX_CONFIG_DIR"
        echo -e "${GREEN}✓ Configuration directory removed${NC}"
    else
        echo -e "${YELLOW}Configuration directory not found${NC}"
    fi
}

# Function to handle history file
handle_history() {
    local history_file="$HOME/.lax_history.csv"
    
    if [[ -f "$history_file" ]]; then
        echo -e "${YELLOW}Found lax history file: $history_file${NC}"
        read -p "Do you want to remove it? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -f "$history_file"
            echo -e "${GREEN}✓ History file removed${NC}"
        else
            echo -e "${BLUE}History file preserved${NC}"
        fi
    fi
}

# Main uninstallation process
main() {
    echo "This script will:"
    echo "1. Remove lax binary from $INSTALL_DIR"
    echo "2. Remove shell integration"
    echo "3. Remove configuration directory"
    echo "4. Optionally remove history file"
    echo ""
    
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Uninstallation cancelled."
        exit 0
    fi
    
    # Run uninstallation steps
    remove_binary
    remove_shell_integration
    remove_config
    handle_history
    
    echo ""
    echo -e "${GREEN}🎉 Uninstallation completed!${NC}"
    echo ""
    echo "To complete the removal:"
    echo "1. Restart your shell or run: source ~/.$(basename $SHELL)rc"
    echo "2. Remove the project directory if desired"
    echo ""
    echo "Thank you for using lax! 🦀"
}

main "$@"
