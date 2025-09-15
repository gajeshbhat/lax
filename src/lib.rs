pub mod analysis;
pub mod cli;
pub mod command;
pub mod history;

pub use analysis::{suggest_alternatives, AnalysisReport};
pub use command::{ParseError, ShellCommand};
pub use history::CommandHistory;
