// src-tauri/src/core/mod.rs

pub mod file_management;
pub mod graph_analysis;

// Re-export for easier accessibility elsewhere in the project
pub use file_management::*;
pub use graph_analysis::*;
