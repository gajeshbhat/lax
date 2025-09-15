use crate::command::{ParseError, ShellCommand};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct CommandHistory {
    commands: Vec<ShellCommand>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: ShellCommand) {
        self.commands.push(command);
    }

    pub fn load_from_csv_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<(usize, ParseError)>> {
        let content = fs::read_to_string(path)
            .context("Failed to read CSV file")?;
        
        let lines: Vec<&str> = content.lines().collect();
        Ok(self.load_from_csv_lines(&lines))
    }

    pub fn load_from_csv_lines(&mut self, lines: &[&str]) -> Vec<(usize, ParseError)> {
        let mut errors = Vec::new();

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            match ShellCommand::from_csv_line(line) {
                Ok(command) => self.add_command(command),
                Err(error) => errors.push((line_num + 1, error)),
            }
        }

        errors
    }

    pub fn load_from_bash_history<P: AsRef<Path>>(&mut self, path: P) -> Result<usize> {
        let content = fs::read_to_string(path)
            .context("Failed to read bash history file")?;
        
        let mut loaded_count = 0;
        for line in content.lines() {
            if let Some(command) = ShellCommand::from_bash_history_line(line) {
                self.add_command(command);
                loaded_count += 1;
            }
        }

        Ok(loaded_count)
    }

    pub fn save_to_csv_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut content = String::new();
        
        for command in &self.commands {
            let timestamp = command.timestamp.map_or(String::new(), |t| t.to_string());
            let directory = command.directory.as_deref().unwrap_or("");
            
            content.push_str(&format!(
                "{}, {}, {}, {}\n",
                command.text, command.exit_code, directory, timestamp
            ));
        }

        fs::write(path, content)
            .context("Failed to write CSV file")?;
        
        Ok(())
    }

    pub fn success_rate(&self) -> f64 {
        if self.commands.is_empty() {
            return 0.0;
        }

        let successful_count = self.successful_commands().count();
        (successful_count as f64 / self.commands.len() as f64) * 100.0
    }

    pub fn successful_commands(&self) -> impl Iterator<Item = &ShellCommand> {
        self.commands.iter().filter(|cmd| cmd.is_successful())
    }

    pub fn failed_commands(&self) -> impl Iterator<Item = &ShellCommand> {
        self.commands.iter().filter(|cmd| !cmd.is_successful())
    }

    pub fn commands_by_name(&self) -> HashMap<String, Vec<&ShellCommand>> {
        let mut map = HashMap::new();
        
        for command in &self.commands {
            let name = command.command_name().to_string();
            map.entry(name).or_insert_with(Vec::new).push(command);
        }
        
        map
    }

    pub fn most_used_commands(&self, limit: usize) -> Vec<(String, usize)> {
        let mut command_counts = HashMap::new();
        
        for command in &self.commands {
            let name = command.command_name().to_string();
            *command_counts.entry(name).or_insert(0) += 1;
        }
        
        let mut sorted: Vec<_> = command_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(limit);
        
        sorted
    }

    pub fn success_rate_by_command(&self) -> HashMap<String, f64> {
        let commands_by_name = self.commands_by_name();
        let mut success_rates = HashMap::new();
        
        for (name, commands) in commands_by_name {
            let total = commands.len();
            let successful = commands.iter().filter(|cmd| cmd.is_successful()).count();
            let rate = (successful as f64 / total as f64) * 100.0;
            success_rates.insert(name, rate);
        }
        
        success_rates
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ShellCommand> {
        self.commands.iter()
    }

    pub fn filter_by_success(&self, successful: bool) -> impl Iterator<Item = &ShellCommand> {
        self.commands.iter().filter(move |cmd| cmd.is_successful() == successful)
    }

    pub fn find_similar_commands(&self, pattern: &str) -> Vec<&ShellCommand> {
        self.commands
            .iter()
            .filter(|cmd| cmd.text.contains(pattern))
            .collect()
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}
