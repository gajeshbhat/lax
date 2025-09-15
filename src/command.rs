use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct ShellCommand {
    pub text: String,
    pub exit_code: i32,
    pub timestamp: Option<u64>,
    pub directory: Option<String>,
}

impl ShellCommand {
    pub fn new(text: String, exit_code: i32) -> Self {
        Self {
            text,
            exit_code,
            timestamp: None,
            directory: None,
        }
    }

    pub fn with_metadata(
        text: String,
        exit_code: i32,
        timestamp: Option<u64>,
        directory: Option<String>,
    ) -> Self {
        Self {
            text,
            exit_code,
            timestamp,
            directory,
        }
    }

    pub fn is_successful(&self) -> bool {
        self.exit_code == 0
    }

    pub fn from_csv_line(line: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            return Err(ParseError::InsufficientFields);
        }

        let text = parts[0].trim().to_string();
        let exit_code = parts[1].trim().parse::<i32>()
            .map_err(|_| ParseError::InvalidExitCode)?;

        let directory = if parts.len() > 2 && !parts[2].trim().is_empty() {
            Some(parts[2].trim().to_string())
        } else {
            None
        };

        let timestamp = if parts.len() > 3 {
            parts[3].trim().parse::<u64>().ok()
        } else {
            None
        };

        Ok(Self::with_metadata(text, exit_code, timestamp, directory))
    }

    pub fn from_bash_history_line(line: &str) -> Option<Self> {
        if line.trim().is_empty() || line.starts_with('#') {
            return None;
        }

        // For now, we assume all commands from history were successful
        // In a real implementation, we'd need to track exit codes separately
        Some(Self::new(line.trim().to_string(), 0))
    }

    pub fn command_name(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }

    pub fn args(&self) -> Vec<&str> {
        self.text.split_whitespace().skip(1).collect()
    }
}

impl fmt::Display for ShellCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_successful() { "PASS" } else { "FAIL" };
        write!(f, "[{}] {} (exit: {})", status, self.text, self.exit_code)?;
        
        if let Some(dir) = &self.directory {
            write!(f, " in {}", dir)?;
        }
        
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("insufficient fields in command line")]
    InsufficientFields,
    #[error("invalid exit code")]
    InvalidExitCode,
}
