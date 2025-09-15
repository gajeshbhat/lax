use lax::{AnalysisReport, CommandHistory, ParseError, ShellCommand};
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_shell_command_creation() {
    let cmd = ShellCommand::new("ls -la".to_string(), 0);
    assert_eq!(cmd.text, "ls -la");
    assert_eq!(cmd.exit_code, 0);
    assert!(cmd.is_successful());
}

#[test]
fn test_shell_command_with_metadata() {
    let cmd = ShellCommand::with_metadata(
        "git status".to_string(),
        0,
        Some(1234567890),
        Some("/home/user".to_string()),
    );
    
    assert_eq!(cmd.text, "git status");
    assert_eq!(cmd.exit_code, 0);
    assert_eq!(cmd.timestamp, Some(1234567890));
    assert_eq!(cmd.directory, Some("/home/user".to_string()));
    assert!(cmd.is_successful());
}

#[test]
fn test_failed_command() {
    let cmd = ShellCommand::new("cat nonexistent.txt".to_string(), 1);
    assert!(!cmd.is_successful());
}

#[test]
fn test_csv_parsing_success() {
    let line = "ls -la, 0, /home/user, 1234567890";
    let cmd = ShellCommand::from_csv_line(line).unwrap();
    
    assert_eq!(cmd.text, "ls -la");
    assert_eq!(cmd.exit_code, 0);
    assert_eq!(cmd.directory, Some("/home/user".to_string()));
    assert_eq!(cmd.timestamp, Some(1234567890));
}

#[test]
fn test_csv_parsing_minimal() {
    let line = "git status, 0";
    let cmd = ShellCommand::from_csv_line(line).unwrap();
    
    assert_eq!(cmd.text, "git status");
    assert_eq!(cmd.exit_code, 0);
    assert_eq!(cmd.directory, None);
    assert_eq!(cmd.timestamp, None);
}

#[test]
fn test_csv_parsing_errors() {
    // Insufficient fields
    let result = ShellCommand::from_csv_line("ls");
    assert!(matches!(result, Err(ParseError::InsufficientFields)));
    
    // Invalid exit code
    let result = ShellCommand::from_csv_line("ls, invalid_code");
    assert!(matches!(result, Err(ParseError::InvalidExitCode)));
}

#[test]
fn test_bash_history_parsing() {
    let cmd = ShellCommand::from_bash_history_line("ls -la").unwrap();
    assert_eq!(cmd.text, "ls -la");
    assert_eq!(cmd.exit_code, 0); // Assumes success for bash history
    
    // Empty lines should return None
    assert!(ShellCommand::from_bash_history_line("").is_none());
    assert!(ShellCommand::from_bash_history_line("   ").is_none());
    
    // Comments should return None
    assert!(ShellCommand::from_bash_history_line("#1234567890").is_none());
}

#[test]
fn test_command_name_extraction() {
    let cmd = ShellCommand::new("git status --porcelain".to_string(), 0);
    assert_eq!(cmd.command_name(), "git");
    
    let cmd = ShellCommand::new("ls".to_string(), 0);
    assert_eq!(cmd.command_name(), "ls");
    
    let cmd = ShellCommand::new("".to_string(), 0);
    assert_eq!(cmd.command_name(), "");
}

#[test]
fn test_command_args_extraction() {
    let cmd = ShellCommand::new("git status --porcelain".to_string(), 0);
    assert_eq!(cmd.args(), vec!["status", "--porcelain"]);
    
    let cmd = ShellCommand::new("ls".to_string(), 0);
    assert_eq!(cmd.args(), Vec::<&str>::new());
}

#[test]
fn test_command_history_basic_operations() {
    let mut history = CommandHistory::new();
    assert!(history.is_empty());
    assert_eq!(history.len(), 0);
    
    let cmd = ShellCommand::new("ls -la".to_string(), 0);
    history.add_command(cmd);
    
    assert!(!history.is_empty());
    assert_eq!(history.len(), 1);
}

#[test]
fn test_command_history_csv_loading() {
    let mut history = CommandHistory::new();
    let lines = vec![
        "ls -la, 0, /home/user",
        "cat file.txt, 1, /home/user",
        "git status, 0, /repo",
        "invalid line", // This should cause an error
    ];
    
    let errors = history.load_from_csv_lines(&lines);
    
    assert_eq!(history.len(), 3);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].0, 4); // Line number
}

#[test]
fn test_success_rate_calculation() {
    let mut history = CommandHistory::new();
    
    // Add 3 successful and 1 failed command
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("pwd".to_string(), 0));
    history.add_command(ShellCommand::new("whoami".to_string(), 0));
    history.add_command(ShellCommand::new("cat nonexistent".to_string(), 1));
    
    assert_eq!(history.success_rate(), 75.0);
    assert_eq!(history.successful_commands().count(), 3);
    assert_eq!(history.failed_commands().count(), 1);
}

#[test]
fn test_most_used_commands() {
    let mut history = CommandHistory::new();
    
    // Add commands with different frequencies
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("ls".to_string(), 1));
    history.add_command(ShellCommand::new("git".to_string(), 0));
    history.add_command(ShellCommand::new("git".to_string(), 0));
    history.add_command(ShellCommand::new("pwd".to_string(), 0));
    
    let most_used = history.most_used_commands(3);
    
    assert_eq!(most_used.len(), 3);
    assert_eq!(most_used[0], ("ls".to_string(), 3));
    assert_eq!(most_used[1], ("git".to_string(), 2));
    assert_eq!(most_used[2], ("pwd".to_string(), 1));
}

#[test]
fn test_success_rate_by_command() {
    let mut history = CommandHistory::new();
    
    // ls: 2 success, 1 failure = 66.7%
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("ls".to_string(), 1));
    
    // git: 2 success, 0 failure = 100%
    history.add_command(ShellCommand::new("git".to_string(), 0));
    history.add_command(ShellCommand::new("git".to_string(), 0));
    
    let rates = history.success_rate_by_command();
    
    assert!((rates["ls"] - 66.66666666666667).abs() < 0.001);
    assert_eq!(rates["git"], 100.0);
}

#[test]
fn test_find_similar_commands() {
    let mut history = CommandHistory::new();
    
    history.add_command(ShellCommand::new("git status".to_string(), 0));
    history.add_command(ShellCommand::new("git commit".to_string(), 0));
    history.add_command(ShellCommand::new("git push".to_string(), 1));
    history.add_command(ShellCommand::new("ls -la".to_string(), 0));
    
    let git_commands = history.find_similar_commands("git");
    assert_eq!(git_commands.len(), 3);
    
    let status_commands = history.find_similar_commands("status");
    assert_eq!(status_commands.len(), 1);
    assert_eq!(status_commands[0].text, "git status");
}

#[test]
fn test_analysis_report_generation() {
    let mut history = CommandHistory::new();
    
    // Add test data
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("ls".to_string(), 0));
    history.add_command(ShellCommand::new("git".to_string(), 1));
    history.add_command(ShellCommand::new("pwd".to_string(), 0));
    
    let report = AnalysisReport::generate(&history, 5);
    
    assert_eq!(report.total_commands, 4);
    assert_eq!(report.successful_commands, 3);
    assert_eq!(report.failed_commands, 1);
    assert_eq!(report.success_rate, 75.0);
    assert_eq!(report.unique_commands, 3);
    assert_eq!(report.most_used.len(), 3);
}

#[test]
fn test_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut history = CommandHistory::new();
    
    // Add some test commands
    history.add_command(ShellCommand::with_metadata(
        "ls -la".to_string(),
        0,
        Some(1234567890),
        Some("/home/user".to_string()),
    ));
    history.add_command(ShellCommand::new("pwd".to_string(), 1));
    
    // Test CSV save/load
    let temp_file = NamedTempFile::new()?;
    history.save_to_csv_file(temp_file.path())?;
    
    let mut loaded_history = CommandHistory::new();
    let errors = loaded_history.load_from_csv_file(temp_file.path())?;
    
    assert_eq!(errors.len(), 0);
    assert_eq!(loaded_history.len(), 2);
    assert_eq!(loaded_history.success_rate(), 50.0);
    
    Ok(())
}

#[test]
fn test_bash_history_file_loading() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = NamedTempFile::new()?;
    let bash_history_content = "ls -la\npwd\n#1234567890\nwhoami\n\n";
    fs::write(temp_file.path(), bash_history_content)?;
    
    let mut history = CommandHistory::new();
    let loaded_count = history.load_from_bash_history(temp_file.path())?;
    
    assert_eq!(loaded_count, 3); // ls, pwd, whoami (comment and empty line ignored)
    assert_eq!(history.len(), 3);
    
    // All commands from bash history should be marked as successful
    assert_eq!(history.success_rate(), 100.0);
    
    Ok(())
}
