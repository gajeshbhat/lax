use crate::history::CommandHistory;
use std::collections::HashMap;

pub struct AnalysisReport {
    pub total_commands: usize,
    pub successful_commands: usize,
    pub failed_commands: usize,
    pub success_rate: f64,
    pub most_used: Vec<(String, usize)>,
    pub success_rates_by_command: HashMap<String, f64>,
    pub unique_commands: usize,
}

impl AnalysisReport {
    pub fn generate(history: &CommandHistory, top_n: usize) -> Self {
        let total_commands = history.len();
        let successful_commands = history.successful_commands().count();
        let failed_commands = history.failed_commands().count();
        let success_rate = history.success_rate();
        let most_used = history.most_used_commands(top_n);
        let success_rates_by_command = history.success_rate_by_command();
        let unique_commands = history.commands_by_name().len();

        Self {
            total_commands,
            successful_commands,
            failed_commands,
            success_rate,
            most_used,
            success_rates_by_command,
            unique_commands,
        }
    }

    pub fn print_summary(&self) {
        println!("Command History Analysis");
        println!("========================");
        println!("Total commands: {}", self.total_commands);
        println!("Unique commands: {}", self.unique_commands);
        println!("Successful: {}", self.successful_commands);
        println!("Failed: {}", self.failed_commands);
        println!("Success rate: {:.1}%", self.success_rate);
    }

    pub fn print_detailed(&self) {
        self.print_summary();
        
        println!("\nMost Used Commands:");
        println!("-------------------");
        for (i, (command, count)) in self.most_used.iter().enumerate() {
            let success_rate = self.success_rates_by_command
                .get(command)
                .map_or(0.0, |&rate| rate);
            println!("{}. {} ({} times, {:.1}% success)", 
                     i + 1, command, count, success_rate);
        }

        println!("\nCommand Success Rates:");
        println!("----------------------");
        let mut sorted_rates: Vec<_> = self.success_rates_by_command.iter().collect();
        sorted_rates.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        
        for (command, rate) in sorted_rates.iter().take(10) {
            println!("{}: {:.1}%", command, rate);
        }
    }

    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.success_rate < 80.0 {
            recommendations.push(
                "Consider reviewing failed commands to improve success rate".to_string()
            );
        }

        // Find commands with low success rates
        let problematic_commands: Vec<_> = self.success_rates_by_command
            .iter()
            .filter(|(_, rate)| **rate < 50.0)
            .collect();

        if !problematic_commands.is_empty() {
            recommendations.push(format!(
                "Commands with low success rates: {}",
                problematic_commands.iter()
                    .map(|(cmd, rate)| format!("{} ({:.1}%)", cmd, rate))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Suggest frequently used commands
        if let Some((most_used_cmd, count)) = self.most_used.first() {
            if *count > self.total_commands / 10 {
                recommendations.push(format!(
                    "Consider creating an alias for '{}' (used {} times)",
                    most_used_cmd, count
                ));
            }
        }

        recommendations
    }
}

pub fn suggest_alternatives(history: &CommandHistory, failed_command: &str) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    // Find similar successful commands
    let similar_commands = history.find_similar_commands(failed_command);
    let successful_similar: Vec<_> = similar_commands
        .into_iter()
        .filter(|cmd| cmd.is_successful())
        .take(5)
        .collect();

    for cmd in successful_similar {
        suggestions.push(cmd.text.clone());
    }

    // If no similar commands found, suggest most successful commands with same base command
    if suggestions.is_empty() {
        let base_command = failed_command.split_whitespace().next().unwrap_or("");
        let commands_by_name = history.commands_by_name();
        
        if let Some(commands) = commands_by_name.get(base_command) {
            let successful_variants: Vec<_> = commands
                .iter()
                .filter(|cmd| cmd.is_successful())
                .take(3)
                .collect();
                
            for cmd in successful_variants {
                suggestions.push(cmd.text.clone());
            }
        }
    }

    suggestions
}
