#!/bin/bash
# Lax Installation Script
# This script installs lax and sets up shell integration

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

echo -e "${BLUE}🦀 Lax Installation Script${NC}"
echo "=========================="

# Function to detect shell
detect_shell() {
    if [[ -n "$ZSH_VERSION" ]]; then
        echo "zsh"
    elif [[ -n "$BASH_VERSION" ]]; then
        echo "bash"
    else
        # Fallback to checking $SHELL
        case "$SHELL" in
            */zsh) echo "zsh" ;;
            */bash) echo "bash" ;;
            *) echo "unknown" ;;
        esac
    fi
}

# Function to build lax
build_lax() {
    echo -e "${BLUE}Building lax...${NC}"
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Rust/Cargo not found. Please install Rust first.${NC}"
        echo "Visit: https://rustup.rs/"
        exit 1
    fi
    
    cargo build --release
    echo -e "${GREEN}✓ Build completed${NC}"
}

# Function to install binary
install_binary() {
    echo -e "${BLUE}Installing lax binary...${NC}"
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Copy binary
    cp target/release/lax "$LAX_BINARY"
    chmod +x "$LAX_BINARY"
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo -e "${YELLOW}Adding $INSTALL_DIR to PATH${NC}"
        
        # Determine which shell config file to update
        local shell_config=""
        case "$(detect_shell)" in
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
        
        if [[ -n "$shell_config" ]]; then
            echo "" >> "$shell_config"
            echo "# Added by lax installer" >> "$shell_config"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_config"
            echo -e "${GREEN}✓ Added to $shell_config${NC}"
        fi
    fi
    
    echo -e "${GREEN}✓ Binary installed to $LAX_BINARY${NC}"
}

# Function to setup shell integration
setup_shell_integration() {
    local shell_type="$(detect_shell)"
    echo -e "${BLUE}Setting up shell integration for $shell_type...${NC}"
    
    # Create config directory
    mkdir -p "$LAX_CONFIG_DIR"
    
    # Copy shell integration files
    cp -r shell_integration "$LAX_CONFIG_DIR/"
    
    # Determine shell config file
    local shell_config=""
    local integration_file=""
    
    case "$shell_type" in
        bash)
            if [[ -f "$HOME/.bashrc" ]]; then
                shell_config="$HOME/.bashrc"
            elif [[ -f "$HOME/.bash_profile" ]]; then
                shell_config="$HOME/.bash_profile"
            fi
            integration_file="$LAX_CONFIG_DIR/shell_integration/lax_bash.sh"
            ;;
        zsh)
            shell_config="$HOME/.zshrc"
            integration_file="$LAX_CONFIG_DIR/shell_integration/lax_zsh.sh"
            ;;
        *)
            echo -e "${YELLOW}Unknown shell type. Manual integration required.${NC}"
            return 0
            ;;
    esac
    
    if [[ -n "$shell_config" && -f "$integration_file" ]]; then
        # Check if already integrated
        if grep -q "lax shell integration" "$shell_config" 2>/dev/null; then
            echo -e "${YELLOW}Shell integration already exists in $shell_config${NC}"
        else
            echo "" >> "$shell_config"
            echo "# Lax shell integration" >> "$shell_config"
            echo "# To disable: export LAX_ENABLED=0 before sourcing" >> "$shell_config"
            echo "if [[ -f \"$integration_file\" ]]; then" >> "$shell_config"
            echo "    source \"$integration_file\"" >> "$shell_config"
            echo "fi" >> "$shell_config"
            echo -e "${GREEN}✓ Shell integration added to $shell_config${NC}"
        fi
    fi
}

# Function to initialize history
initialize_history() {
    echo -e "${BLUE}Initializing command history...${NC}"
    
    # Set LAX_BINARY environment variable for the integration script
    export LAX_BINARY="$LAX_BINARY"
    
    local shell_type="$(detect_shell)"
    local history_file=""
    
    case "$shell_type" in
        bash)
            history_file="$HOME/.bash_history"
            ;;
        zsh)
            history_file="$HOME/.zsh_history"
            ;;
    esac
    
    if [[ -f "$history_file" ]]; then
        "$LAX_BINARY" convert --input "$history_file" --output "$HOME/.lax_history.csv"
        echo -e "${GREEN}✓ History initialized from $history_file${NC}"
    else
        echo -e "${YELLOW}No shell history found, starting with empty history${NC}"
        touch "$HOME/.lax_history.csv"
    fi
}

# Function to run tests
run_tests() {
    echo -e "${BLUE}Running tests...${NC}"
    cargo test --quiet
    echo -e "${GREEN}✓ All tests passed${NC}"
}

# Main installation process
main() {
    echo "This script will:"
    echo "1. Build lax from source"
    echo "2. Install binary to $INSTALL_DIR"
    echo "3. Set up shell integration"
    echo "4. Initialize command history"
    echo ""
    
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Installation cancelled."
        exit 0
    fi
    
    # Run installation steps
    build_lax
    run_tests
    install_binary
    setup_shell_integration
    initialize_history
    
    echo ""
    echo -e "${GREEN}🎉 Installation completed!${NC}"
    echo ""
    echo "To start using lax:"
    echo "1. Restart your shell or run: source ~/.$(basename $SHELL)rc"
    echo "2. Check status: lax_status"
    echo "3. Toggle on/off: lax_toggle"
    echo ""
    echo "Available commands:"
    echo "  lax_status    - Show lax status and configuration"
    echo "  lax_toggle    - Enable/disable lax"
    echo "  lax_analyze   - Analyze command history"
    echo "  lax_search    - Search command history"
    echo "  lax_rate      - Show success rate for a command"
    echo ""
    echo "Configuration:"
    echo "  Binary: $LAX_BINARY"
    echo "  Config: $LAX_CONFIG_DIR"
    echo "  History: $HOME/.lax_history.csv"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -f "src/main.rs" ]]; then
    echo -e "${RED}Error: Please run this script from the lax project directory${NC}"
    exit 1
fi

main "$@"
