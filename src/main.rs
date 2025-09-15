mod analysis;
mod cli;
mod command;
mod history;

use analysis::{suggest_alternatives, AnalysisReport};
use anyhow::{Context, Result};
use clap::Parser;
use cli::{get_default_bash_history, get_default_csv_history, Cli, Commands};
use history::CommandHistory;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { file, format, top, detailed } => {
            handle_analyze(file, format, top, detailed)?;
        }
        Commands::Convert { input, output } => {
            handle_convert(input, output)?;
        }
        Commands::Search { pattern, file, successful_only } => {
            handle_search(pattern, file, successful_only)?;
        }
        Commands::Rate { command, file } => {
            handle_rate(command, file)?;
        }
        Commands::Status => {
            handle_shell_status()?;
        }
        Commands::Toggle => {
            handle_shell_toggle()?;
        }
        Commands::Init => {
            handle_shell_init()?;
        }
    }

    Ok(())
}

fn handle_analyze(file: Option<std::path::PathBuf>, format: String, top: usize, detailed: bool) -> Result<()> {
    let history_file = file.or_else(|| match format.as_str() {
        "csv" => get_default_csv_history(),
        _ => get_default_bash_history(),
    }).context("Could not determine history file path")?;

    let mut history = CommandHistory::new();

    match format.as_str() {
        "csv" => {
            let errors = history.load_from_csv_file(&history_file)?;
            if !errors.is_empty() {
                println!("Warning: {} parse errors in CSV file", errors.len());
            }
        }
        "bash" => {
            let loaded = history.load_from_bash_history(&history_file)?;
            println!("Loaded {} commands from bash history", loaded);
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Use 'bash' or 'csv'", format);
        }
    }

    if history.is_empty() {
        println!("No commands found in history file");
        return Ok(());
    }

    let report = AnalysisReport::generate(&history, top);

    if detailed {
        report.print_detailed();
    } else {
        report.print_summary();
    }

    let recommendations = report.get_recommendations();
    if !recommendations.is_empty() {
        println!("\nRecommendations:");
        for rec in recommendations {
            println!("• {}", rec);
        }
    }

    Ok(())
}

fn handle_convert(input: std::path::PathBuf, output: std::path::PathBuf) -> Result<()> {
    let mut history = CommandHistory::new();
    let loaded = history.load_from_bash_history(&input)?;
    history.save_to_csv_file(&output)?;

    println!("Converted {} commands from {} to {}",
             loaded, input.display(), output.display());

    Ok(())
}

fn handle_search(pattern: String, file: Option<std::path::PathBuf>, successful_only: bool) -> Result<()> {
    let history_file = file.or_else(get_default_csv_history)
        .context("Could not determine history file path")?;

    let mut history = CommandHistory::new();
    let _errors = history.load_from_csv_file(&history_file)?;

    let matching_commands = history.find_similar_commands(&pattern);
    let filtered_commands: Vec<_> = if successful_only {
        matching_commands.into_iter().filter(|cmd| cmd.is_successful()).collect()
    } else {
        matching_commands
    };

    if filtered_commands.is_empty() {
        println!("No matching commands found for pattern: {}", pattern);
        return Ok(());
    }

    println!("Found {} matching commands:", filtered_commands.len());
    for (i, cmd) in filtered_commands.iter().enumerate() {
        println!("{}. {}", i + 1, cmd);
    }

    Ok(())
}

fn handle_rate(command: String, file: Option<std::path::PathBuf>) -> Result<()> {
    let history_file = file.or_else(get_default_csv_history)
        .context("Could not determine history file path")?;

    let mut history = CommandHistory::new();
    let _errors = history.load_from_csv_file(&history_file)?;

    let success_rates = history.success_rate_by_command();

    if let Some(&rate) = success_rates.get(&command) {
        println!("Success rate for '{}': {:.1}%", command, rate);

        // Show some examples
        let commands_by_name = history.commands_by_name();
        if let Some(examples) = commands_by_name.get(&command) {
            let successful: Vec<_> = examples.iter().filter(|cmd| cmd.is_successful()).take(3).collect();
            let failed: Vec<_> = examples.iter().filter(|cmd| !cmd.is_successful()).take(3).collect();

            if !successful.is_empty() {
                println!("\nSuccessful examples:");
                for cmd in successful {
                    println!("  {}", cmd.text);
                }
            }

            if !failed.is_empty() {
                println!("\nFailed examples:");
                for cmd in &failed {
                    println!("  {}", cmd.text);
                }

                // Suggest alternatives
                let suggestions = suggest_alternatives(&history, &failed[0].text);
                if !suggestions.is_empty() {
                    println!("\nSuggested alternatives:");
                    for suggestion in suggestions.iter().take(3) {
                        println!("  {}", suggestion);
                    }
                }
            }
        }
    } else {
        println!("Command '{}' not found in history", command);
    }

    Ok(())
}

fn handle_shell_status() -> Result<()> {
    let lax_enabled = std::env::var("LAX_ENABLED").unwrap_or_else(|_| "1".to_string());
    let history_file = std::env::var("LAX_HISTORY_FILE")
        .unwrap_or_else(|_| format!("{}/.lax_history.csv", std::env::var("HOME").unwrap_or_default()));
    let binary = std::env::var("LAX_BINARY").unwrap_or_else(|_| "lax".to_string());
    let min_pattern = std::env::var("LAX_MIN_PATTERN_LENGTH").unwrap_or_else(|_| "2".to_string());
    let max_suggestions = std::env::var("LAX_MAX_SUGGESTIONS").unwrap_or_else(|_| "5".to_string());

    let status = if lax_enabled == "1" { "ENABLED" } else { "DISABLED" };
    println!("Lax Status: {}", status);
    println!("History file: {}", history_file);
    println!("Binary: {}", binary);
    println!("Min pattern length: {}", min_pattern);
    println!("Max suggestions: {}", max_suggestions);

    if std::path::Path::new(&history_file).exists() {
        let count = std::fs::read_to_string(&history_file)
            .map(|content| content.lines().count())
            .unwrap_or(0);
        println!("Commands in history: {}", count);
    } else {
        println!("History file not found");
    }

    Ok(())
}

fn handle_shell_toggle() -> Result<()> {
    let current = std::env::var("LAX_ENABLED").unwrap_or_else(|_| "1".to_string());
    let new_value = if current == "1" { "0" } else { "1" };

    println!("export LAX_ENABLED={}", new_value);

    if new_value == "1" {
        eprintln!("Lax enabled");
    } else {
        eprintln!("Lax disabled");
    }

    Ok(())
}

fn handle_shell_init() -> Result<()> {
    let shell = std::env::var("SHELL").unwrap_or_default();
    let home = std::env::var("HOME").unwrap_or_default();
    let history_file = std::env::var("LAX_HISTORY_FILE")
        .unwrap_or_else(|_| format!("{}/.lax_history.csv", home));

    let source_file = if shell.contains("zsh") {
        format!("{}/.zsh_history", home)
    } else {
        format!("{}/.bash_history", home)
    };

    if std::path::Path::new(&source_file).exists() {
        println!("Converting {} to lax format...", source_file);

        let mut history = CommandHistory::new();
        let loaded = history.load_from_bash_history(&source_file)?;
        history.save_to_csv_file(&history_file)?;

        println!("Converted {} commands to {}", loaded, history_file);
    } else {
        println!("No shell history found at {}", source_file);
    }

    Ok(())
}
