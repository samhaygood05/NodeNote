// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{command, App, Manager, Window};

#[command]
fn create_graph_directories(base_path: String, graph_name: String) -> Result<String, String> {
    use std::path::Path;
    use std::fs::create_dir_all;

    let graph_path = Path::new(&base_path).join(&graph_name);
    let notes_path = graph_path.join("Notes");
    let nodes_path = graph_path.join("Nodes");

    // Attempt to create the graph directory and subdirectories
    if let Err(e) = create_dir_all(&notes_path) {
        return Err(format!("Failed to create notes directory: {}", e));
    }
    if let Err(e) = create_dir_all(&nodes_path) {
        return Err(format!("Failed to create nodes directory: {}", e));
    }

    Ok(format!("Graph created successfully at {:?}", graph_path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_graph_directories])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
