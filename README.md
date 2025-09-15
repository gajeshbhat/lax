# Lax - Smart Shell Autocomplete

A high-performance Rust tool that analyzes shell command history to provide intelligent autocomplete suggestions based on command success rates.

## Features

- 🚀 **High Performance**: Processes 10,000+ commands in milliseconds
- 📊 **Success Rate Analysis**: Track which commands succeed vs fail
- 🔍 **Smart Search**: Find similar commands with pattern matching
- 📈 **Detailed Analytics**: Command usage statistics and recommendations
- 💾 **Multiple Formats**: Support for bash history and CSV formats
- 🧪 **Well Tested**: Comprehensive test suite with 18+ integration tests

## Installation

### Quick Install (Recommended)
```bash
git clone https://github.com/yourusername/lax.git
cd lax
./scripts/install.sh
```

This will:
- Build lax from source
- Install binary to `~/.local/bin/lax`
- Set up shell integration
- Initialize command history

### Manual Install
```bash
git clone https://github.com/yourusername/lax.git
cd lax
cargo build --release
cp target/release/lax ~/.local/bin/
```

## Shell Integration

Lax provides intelligent shell integration that:
- 🎯 **Tracks command success/failure** automatically
- 🔍 **Provides smart suggestions** based on your history
- ⚡ **Integrates seamlessly** with existing shell completion
- 🎛️ **Can be toggled on/off** easily

### Quick Start
After installation, restart your shell or run:
```bash
source ~/.bashrc  # or ~/.zshrc for zsh users
```

### Available Shell Commands
```bash
lax_status      # Show status and configuration
lax_toggle      # Enable/disable lax
lax_analyze     # Analyze your command history
lax_search      # Search command patterns
lax_rate        # Check success rate of commands
```

See [docs/SHELL_INTEGRATION.md](docs/SHELL_INTEGRATION.md) for detailed documentation.

## Project Structure

```
lax/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library exports
│   ├── cli.rs             # Command-line interface
│   ├── command.rs         # Command data structures
│   ├── history.rs         # History management
│   └── analysis.rs        # Analysis and reporting
├── shell_integration/      # Shell integration scripts
│   ├── lax_bash.sh        # Bash integration
│   └── lax_zsh.sh         # ZSH integration
├── scripts/               # Installation and utility scripts
│   ├── install.sh         # Automated installation
│   └── uninstall.sh       # Clean uninstallation
├── docs/                  # Documentation
│   └── SHELL_INTEGRATION.md # Shell integration guide
├── tests/                 # Test suite
│   └── integration_tests.rs # Integration tests
└── examples/              # Example code and benchmarks
    └── performance.rs     # Performance benchmarks
```

## Usage

### Analyze Command History

```bash
# Analyze bash history
lax analyze --file ~/.bash_history --format bash --detailed

# Analyze CSV history with success/failure data
lax analyze --file history.csv --format csv --top 15
```

### Convert Bash History to CSV

```bash
# Convert bash history to trackable CSV format
lax convert --input ~/.bash_history --output ~/.lax_history.csv
```

### Search Commands

```bash
# Find all git commands
lax search "git" --file history.csv

# Find only successful git commands
lax search "git" --file history.csv --successful-only
```

### Check Command Success Rate

```bash
# Get success rate and examples for a specific command
lax rate "cargo" --file history.csv
```

## CSV Format

The CSV format tracks detailed command information:

```csv
command, exit_code, directory, timestamp
ls -la, 0, /home/user, 1234567890
cat nonexistent.txt, 1, /home/user, 1234567891
git status, 0, /home/user/project, 1234567892
```

## Architecture

### Core Components

- **`ShellCommand`**: Represents a single shell command with metadata
- **`CommandHistory`**: Manages collections of commands with analysis methods
- **`AnalysisReport`**: Generates detailed statistics and recommendations
- **CLI Interface**: Command-line interface with multiple subcommands

### Performance

- **Load Speed**: 2,500+ commands/ms
- **Analysis Speed**: Sub-millisecond for most operations
- **Memory Usage**: ~70 bytes per command
- **Search Speed**: Pattern matching across 10k commands in <1ms

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

Run performance benchmarks:

```bash
cargo run --release --example performance
```

## Example Output

### Analysis Report
```
Command History Analysis
========================
Total commands: 41
Unique commands: 12
Successful: 33
Failed: 8
Success rate: 80.5%

Most Used Commands:
-------------------
1. git (10 times, 80.0% success)
2. ls (7 times, 100.0% success)
3. cargo (6 times, 66.7% success)

Recommendations:
• Commands with low success rates: grep (0.0%), rm (0.0%)
• Consider creating an alias for 'git' (used 10 times)
```

### Command Rate Analysis
```
Success rate for 'cargo': 66.7%

Successful examples:
  cargo run
  cargo check
  cargo build

Failed examples:
  cargo build
  cargo test

Suggested alternatives:
  cargo build
```

## Code Quality

This project demonstrates production-quality Rust code with:

- **Proper Error Handling**: Custom error types with `thiserror`
- **Modular Architecture**: Clean separation of concerns
- **Iterator-Based Design**: Zero-cost abstractions
- **Comprehensive Testing**: Unit and integration tests
- **Performance Optimization**: Efficient algorithms and data structures
- **CLI Best Practices**: Using `clap` for argument parsing

## Uninstallation

To completely remove lax:

```bash
./scripts/uninstall.sh
```

This will:
- Remove the lax binary
- Remove shell integration from your config
- Remove configuration files
- Optionally remove command history

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## Future Enhancements

- [x] Real-time shell integration hooks
- [ ] Machine learning-based command prediction
- [x] Shell-specific completion integration
- [ ] Command similarity scoring
- [ ] Historical trend analysis
- [ ] Multi-user command sharing

## License

MIT License - see LICENSE file for details.
