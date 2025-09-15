use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lax")]
#[command(about = "Smart shell autocomplete based on command success history")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze command history and show statistics
    Analyze {
        /// Path to history file (defaults to ~/.bash_history)
        #[arg(short, long)]
        file: Option<PathBuf>,
        
        /// Input format: bash, csv
        #[arg(short = 'f', long, default_value = "bash")]
        format: String,
        
        /// Show top N most used commands
        #[arg(short = 'n', long, default_value = "10")]
        top: usize,
        
        /// Show detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Load history from bash and save as CSV for tracking success/failure
    Convert {
        /// Input bash history file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output CSV file
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Find commands similar to a pattern
    Search {
        /// Search pattern
        pattern: String,
        
        /// History file to search in
        #[arg(short, long)]
        file: Option<PathBuf>,
        
        /// Only show successful commands
        #[arg(short, long)]
        successful_only: bool,
    },
    
    /// Show success rate for specific commands
    Rate {
        /// Command name to analyze
        command: String,

        /// History file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Show shell integration status
    #[command(name = "status")]
    Status,

    /// Toggle shell integration on/off
    #[command(name = "toggle")]
    Toggle,

    /// Initialize history from shell history
    #[command(name = "init")]
    Init,
}

impl Commands {
    pub fn get_history_file(&self) -> Option<PathBuf> {
        match self {
            Commands::Analyze { file, .. } => file.clone(),
            Commands::Search { file, .. } => file.clone(),
            Commands::Rate { file, .. } => file.clone(),
            Commands::Convert { .. } => None,
            Commands::Status | Commands::Toggle | Commands::Init => None,
        }
    }
}

pub fn get_default_bash_history() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".bash_history"))
}

pub fn get_default_csv_history() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".lax_history.csv"))
}
