// src-tauri/src/commands/mod.rs

pub mod file_commands;
pub mod graph_commands;

// Re-export the functions from these modules if necessary
pub use file_commands::*;
pub use graph_commands::*;
