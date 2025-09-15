# Lax Shell Integration Guide

This guide explains how to integrate Lax with your shell for intelligent command completion and history tracking.

## Quick Start

### Automatic Installation
```bash
./install.sh
```

### Manual Installation
1. Build lax: `cargo build --release`
2. Copy binary to PATH: `cp target/release/lax ~/.local/bin/`
3. Source integration script in your shell config

## Shell Support

### Bash Integration
Add to `~/.bashrc`:
```bash
# Lax shell integration
if [[ -f "$HOME/.config/lax/shell_integration/lax_bash.sh" ]]; then
    source "$HOME/.config/lax/shell_integration/lax_bash.sh"
fi
```

### ZSH Integration
Add to `~/.zshrc`:
```bash
# Lax shell integration
if [[ -f "$HOME/.config/lax/shell_integration/lax_zsh.sh" ]]; then
    source "$HOME/.config/lax/shell_integration/lax_zsh.sh"
fi
```

## Configuration

Set these environment variables before sourcing the integration script:

```bash
export LAX_ENABLED=1                           # Enable/disable lax (default: 1)
export LAX_HISTORY_FILE="$HOME/.lax_history.csv"  # History file location
export LAX_BINARY="lax"                       # Lax binary name/path
export LAX_MIN_PATTERN_LENGTH=2               # Minimum chars for suggestions
export LAX_MAX_SUGGESTIONS=5                  # Maximum suggestions to show
```

## Available Commands

### Core Commands
- `lax_status` - Show current status and configuration
- `lax_toggle` - Enable/disable lax functionality
- `lax_init` - Initialize history from shell history

### Analysis Commands
- `lax_analyze [options]` - Analyze command history
- `lax_search <pattern> [--successful-only]` - Search commands
- `lax_rate <command>` - Show success rate for a command

### Examples
```bash
# Check status
lax_status

# Analyze your command history
lax_analyze --detailed --top 10

# Search for git commands that succeeded
lax_search "git" --successful-only

# Check success rate of cargo commands
lax_rate "cargo"

# Disable lax temporarily
lax_toggle
```

## How It Works

### Command Logging
Lax automatically logs every command you run with:
- Command text
- Exit code (0 = success, non-zero = failure)
- Working directory
- Timestamp

### Smart Completion
When you type a command, lax:
1. Checks if the pattern is long enough (configurable)
2. Searches your history for similar successful commands
3. Provides suggestions based on success rate
4. Integrates with your shell's existing completion

### History Format
Commands are stored in CSV format:
```csv
command, exit_code, directory, timestamp
ls -la, 0, /home/user, 1234567890
cat nonexistent.txt, 1, /home/user, 1234567891
```

## Best Practices

### 1. Regular Analysis
Run `lax_analyze` periodically to:
- Identify frequently failing commands
- Find opportunities for aliases
- Understand your command patterns

### 2. Selective Completion
Lax doesn't interfere with file completion for commands like:
- `cd`, `ls`, `cat`, `vim` (file-focused commands)
- Use longer patterns for better suggestions

### 3. History Management
- History file grows over time - consider periodic cleanup
- Backup your history file before major changes
- Use `lax_init` to bootstrap from existing shell history

## Troubleshooting

### Common Issues

**Lax not working after installation:**
```bash
# Reload your shell configuration
source ~/.bashrc  # or ~/.zshrc

# Check if lax is in PATH
which lax

# Verify integration is loaded
lax_status
```

**No suggestions appearing:**
```bash
# Check minimum pattern length
echo $LAX_MIN_PATTERN_LENGTH

# Verify lax is enabled
lax_status

# Check if history file exists
ls -la ~/.lax_history.csv
```

**Commands not being logged:**
```bash
# Check if lax is enabled
lax_status

# Verify history file is writable
touch ~/.lax_history.csv

# Check recent entries
tail ~/.lax_history.csv
```

### Performance Tuning

For large history files (>10,000 commands):
```bash
# Reduce suggestion count
export LAX_MAX_SUGGESTIONS=3

# Increase minimum pattern length
export LAX_MIN_PATTERN_LENGTH=3

# Consider periodic history cleanup
head -5000 ~/.lax_history.csv > ~/.lax_history_trimmed.csv
mv ~/.lax_history_trimmed.csv ~/.lax_history.csv
```

## Uninstallation

### Automatic Uninstallation
```bash
./uninstall.sh
```

### Manual Uninstallation
1. Remove integration lines from shell config
2. Remove binary: `rm ~/.local/bin/lax`
3. Remove config: `rm -rf ~/.config/lax`
4. Optionally remove history: `rm ~/.lax_history.csv`

## Advanced Usage

### Custom History Location
```bash
export LAX_HISTORY_FILE="/path/to/custom/history.csv"
source ~/.config/lax/shell_integration/lax_bash.sh
```

### Disable for Specific Sessions
```bash
export LAX_ENABLED=0
# Start new shell or source config
```

### Integration with Other Tools
```bash
# Use with fzf for interactive search
lax_search "git" | fzf

# Export successful commands
lax_search "pattern" --successful-only > successful_commands.txt

# Analyze specific time periods
grep "$(date +%Y-%m-%d)" ~/.lax_history.csv | lax analyze --format csv
```

## Security Considerations

- History file contains your command history - protect it appropriately
- Consider excluding sensitive commands from logging
- History file is stored in plain text - be aware of sensitive data
- Use appropriate file permissions: `chmod 600 ~/.lax_history.csv`

## Contributing

Found a bug or want to improve the shell integration?
1. Check existing issues
2. Test your changes with both bash and zsh
3. Update documentation
4. Submit a pull request

## Support

For issues with shell integration:
1. Check this documentation
2. Run `lax_status` to verify configuration
3. Test with `./test_integration.sh`
4. Check shell-specific integration scripts in `shell_integration/`
