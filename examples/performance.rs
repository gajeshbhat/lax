use lax::{CommandHistory, ShellCommand};
use std::time::Instant;

fn main() {
    println!("Lax Performance Benchmarks");
    println!("==========================");

    // Test 1: Loading large CSV
    let start = Instant::now();
    let mut history = CommandHistory::new();
    
    // Generate 10,000 test commands
    let test_data: Vec<String> = (0..10_000)
        .map(|i| {
            let cmd = match i % 10 {
                0 => "ls -la",
                1 => "pwd",
                2 => "git status",
                3 => "cargo build",
                4 => "cat file.txt",
                5 => "grep pattern",
                6 => "find . -name",
                7 => "vim file.rs",
                8 => "cd directory",
                _ => "echo hello",
            };
            let exit_code = if i % 7 == 0 { 1 } else { 0 }; // ~14% failure rate
            format!("{}, {}, /home/user, {}", cmd, exit_code, 1234567890 + i)
        })
        .collect();

    let test_lines: Vec<&str> = test_data.iter().map(|s| s.as_str()).collect();
    let errors = history.load_from_csv_lines(&test_lines);
    let load_time = start.elapsed();

    println!("✓ Loaded {} commands in {:?} ({} errors)", 
             history.len(), load_time, errors.len());

    // Test 2: Analysis performance
    let start = Instant::now();
    let success_rate = history.success_rate();
    let analysis_time = start.elapsed();
    
    println!("✓ Calculated success rate ({:.1}%) in {:?}", 
             success_rate, analysis_time);

    // Test 3: Most used commands
    let start = Instant::now();
    let most_used = history.most_used_commands(10);
    let most_used_time = start.elapsed();
    
    println!("✓ Found top 10 most used commands in {:?}", most_used_time);

    // Test 4: Search performance
    let start = Instant::now();
    let git_commands = history.find_similar_commands("git");
    let search_time = start.elapsed();
    
    println!("✓ Found {} git commands in {:?}", git_commands.len(), search_time);

    // Test 5: Success rate by command
    let start = Instant::now();
    let rates = history.success_rate_by_command();
    let rates_time = start.elapsed();
    
    println!("✓ Calculated success rates for {} unique commands in {:?}", 
             rates.len(), rates_time);

    // Test 6: Memory usage estimation
    let command_size = std::mem::size_of::<ShellCommand>();
    let estimated_memory = history.len() * command_size;
    
    println!("✓ Estimated memory usage: {} bytes ({:.1} KB)", 
             estimated_memory, estimated_memory as f64 / 1024.0);

    println!("\nPerformance Summary:");
    println!("- Load rate: {:.0} commands/ms", 
             history.len() as f64 / load_time.as_millis() as f64);
    println!("- Analysis speed: {:.0} commands/ms", 
             history.len() as f64 / analysis_time.as_millis() as f64);
    println!("- Search speed: {:.0} commands/ms", 
             history.len() as f64 / search_time.as_millis() as f64);
}
