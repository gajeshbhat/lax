#!/bin/zsh
# Lax Shell Integration for ZSH
# Source this file to enable smart command completion based on success history

# Configuration
LAX_ENABLED=${LAX_ENABLED:-1}
LAX_HISTORY_FILE=${LAX_HISTORY_FILE:-"$HOME/.lax_history.csv"}
LAX_BINARY=${LAX_BINARY:-"lax"}
LAX_MIN_PATTERN_LENGTH=${LAX_MIN_PATTERN_LENGTH:-2}
LAX_MAX_SUGGESTIONS=${LAX_MAX_SUGGESTIONS:-5}

# Colors for output
LAX_COLOR_SUCCESS='\033[0;32m'
LAX_COLOR_FAIL='\033[0;31m'
LAX_COLOR_INFO='\033[0;34m'
LAX_COLOR_RESET='\033[0m'

# Global variables
LAX_LAST_COMMAND=""
LAX_COMMAND_START_TIME=0

# Function to log commands with exit codes
_lax_preexec() {
    LAX_LAST_COMMAND="$1"
    LAX_COMMAND_START_TIME=$(date +%s)
}

_lax_precmd() {
    local exit_code=$?
    local current_dir=$(pwd)
    
    if [[ -n "$LAX_LAST_COMMAND" && "$LAX_ENABLED" == "1" ]]; then
        # Create CSV entry: command, exit_code, directory, timestamp
        echo "$LAX_LAST_COMMAND, $exit_code, $current_dir, $LAX_COMMAND_START_TIME" >> "$LAX_HISTORY_FILE"
    fi
    
    LAX_LAST_COMMAND=""
}

# Enhanced completion function for ZSH
_lax_completion() {
    local context state line
    local -a suggestions
    
    # Get current word being completed
    local current_word="${words[CURRENT]}"
    
    # Only provide suggestions if LAX is enabled and pattern is long enough
    if [[ "$LAX_ENABLED" != "1" || ${#current_word} -lt $LAX_MIN_PATTERN_LENGTH ]]; then
        return 0
    fi
    
    # Don't interfere with file completion for certain commands
    case "${words[1]}" in
        cd|ls|cat|vim|nano|emacs|less|more|head|tail|file|stat)
            return 0
            ;;
    esac
    
    # Get suggestions from lax
    if [[ -f "$LAX_HISTORY_FILE" ]]; then
        local lax_suggestions
        lax_suggestions=($($LAX_BINARY search "$current_word" --file "$LAX_HISTORY_FILE" --successful-only 2>/dev/null | \
                         grep -E '^\[PASS\]' | \
                         sed 's/^[0-9]*\. \[PASS\] \([^(]*\) .*/\1/' | \
                         head -n $LAX_MAX_SUGGESTIONS))
        
        if [[ ${#lax_suggestions[@]} -gt 0 ]]; then
            _describe 'lax suggestions' lax_suggestions
        fi
    fi
}

# Function to show command success rate
lax_rate() {
    if [[ "$LAX_ENABLED" != "1" ]]; then
        echo -e "${LAX_COLOR_INFO}Lax is disabled${LAX_COLOR_RESET}"
        return 1
    fi

    if [[ -z "$1" ]]; then
        echo "Usage: lax rate <command>"
        return 1
    fi

    if [[ -f "$LAX_HISTORY_FILE" ]]; then
        $LAX_BINARY rate "$1" --file "$LAX_HISTORY_FILE"
    else
        echo -e "${LAX_COLOR_INFO}No history file found at $LAX_HISTORY_FILE${LAX_COLOR_RESET}"
    fi
}

# Function to search command history
lax_search() {
    if [[ "$LAX_ENABLED" != "1" ]]; then
        echo -e "${LAX_COLOR_INFO}Lax is disabled${LAX_COLOR_RESET}"
        return 1
    fi
    
    if [[ -z "$1" ]]; then
        echo "Usage: lax_search <pattern> [--successful-only]"
        return 1
    fi
    
    if [[ -f "$LAX_HISTORY_FILE" ]]; then
        $LAX_BINARY search "$@" --file "$LAX_HISTORY_FILE"
    else
        echo -e "${LAX_COLOR_INFO}No history file found at $LAX_HISTORY_FILE${LAX_COLOR_RESET}"
    fi
}

# Function to analyze command history
lax_analyze() {
    if [[ "$LAX_ENABLED" != "1" ]]; then
        echo -e "${LAX_COLOR_INFO}Lax is disabled${LAX_COLOR_RESET}"
        return 1
    fi
    
    if [[ -f "$LAX_HISTORY_FILE" ]]; then
        $LAX_BINARY analyze --file "$LAX_HISTORY_FILE" --format csv "$@"
    else
        echo -e "${LAX_COLOR_INFO}No history file found at $LAX_HISTORY_FILE${LAX_COLOR_RESET}"
    fi
}

# Function to enable/disable lax
lax_toggle() {
    eval "$($LAX_BINARY --toggle)"
}

# Function to show lax status
lax_status() {
    $LAX_BINARY --status
}

# Function to initialize lax history from zsh history
lax_init() {
    $LAX_BINARY --init
}

# Install hooks only if lax is enabled
if [[ "$LAX_ENABLED" == "1" ]]; then
    # Set up command logging hooks
    autoload -Uz add-zsh-hook
    add-zsh-hook preexec _lax_preexec
    add-zsh-hook precmd _lax_precmd

    # Set up completion
    autoload -Uz compinit
    compinit

    # Register completion function
    compdef _lax_completion command
fi

# Only show messages if LAX_VERBOSE is set
if [[ "$LAX_VERBOSE" == "1" ]]; then
    if [[ "$LAX_ENABLED" == "1" ]]; then
        echo -e "${LAX_COLOR_SUCCESS}Lax shell integration loaded${LAX_COLOR_RESET}"
    else
        echo -e "${LAX_COLOR_INFO}Lax shell integration loaded (disabled)${LAX_COLOR_RESET}"
    fi

    echo "Available commands:"
    echo "  lax --status    - Show lax status and configuration"
    echo "  lax --toggle    - Enable/disable lax"
    echo "  lax --init      - Initialize history from zsh_history"
    echo "  lax analyze     - Analyze command history"
    echo "  lax search      - Search command history"
    echo "  lax rate        - Show success rate for a command"
fi
