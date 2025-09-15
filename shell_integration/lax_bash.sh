#!/bin/bash
# Lax Shell Integration for Bash
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
LAX_LAST_EXIT_CODE=0
LAX_COMMAND_START_TIME=0

# Function to log commands with exit codes
_lax_log_command() {
    local exit_code=$?
    local end_time=$(date +%s)
    local current_dir=$(pwd)
    
    if [[ -n "$LAX_LAST_COMMAND" && "$LAX_ENABLED" == "1" ]]; then
        # Create CSV entry: command, exit_code, directory, timestamp
        echo "$LAX_LAST_COMMAND, $exit_code, $current_dir, $LAX_COMMAND_START_TIME" >> "$LAX_HISTORY_FILE"
    fi
    
    LAX_LAST_COMMAND=""
    return $exit_code
}

# Function to capture command before execution
_lax_capture_command() {
    LAX_LAST_COMMAND=$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
    LAX_COMMAND_START_TIME=$(date +%s)
}

# Enhanced completion function
_lax_completion() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Only provide suggestions if LAX is enabled and pattern is long enough
    if [[ "$LAX_ENABLED" != "1" || ${#cur} -lt $LAX_MIN_PATTERN_LENGTH ]]; then
        return 0
    fi
    
    # Don't interfere with file completion for certain commands
    case "${COMP_WORDS[0]}" in
        cd|ls|cat|vim|nano|emacs|less|more|head|tail|file|stat)
            return 0
            ;;
    esac
    
    # Get suggestions from lax
    if [[ -f "$LAX_HISTORY_FILE" ]]; then
        local suggestions
        suggestions=$($LAX_BINARY search "$cur" --file "$LAX_HISTORY_FILE" --successful-only 2>/dev/null | \
                     grep -E '^\[PASS\]' | \
                     sed 's/^[0-9]*\. \[PASS\] \([^(]*\) .*/\1/' | \
                     head -n $LAX_MAX_SUGGESTIONS)
        
        if [[ -n "$suggestions" ]]; then
            COMPREPLY=($(compgen -W "$suggestions" -- "$cur"))
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
        echo "Usage: lax_rate <command>"
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
    eval "$($LAX_BINARY toggle)"
}

# Function to show lax status
lax_status() {
    $LAX_BINARY status
}

# Function to initialize lax history from bash history
lax_init() {
    $LAX_BINARY init
}

# Install hooks only if lax is enabled
if [[ "$LAX_ENABLED" == "1" ]]; then
    # Set up command logging hooks
    trap '_lax_log_command' DEBUG

    # Set up pre-command hook
    if [[ -z "$PROMPT_COMMAND" ]]; then
        PROMPT_COMMAND="_lax_capture_command"
    else
        PROMPT_COMMAND="_lax_capture_command; $PROMPT_COMMAND"
    fi

    # Set up completion
    complete -F _lax_completion -o default -o bashdefault command
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
    echo "  lax --init      - Initialize history from bash_history"
    echo "  lax analyze     - Analyze command history"
    echo "  lax search      - Search command history"
    echo "  lax rate        - Show success rate for a command"
fi
