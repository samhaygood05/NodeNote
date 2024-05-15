// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod coreapp;
use coreapp::file_management::{create_graph_directories, get_graph_list};
use tauri::{command, App, Manager, Window};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_graph_directories, get_graph_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
