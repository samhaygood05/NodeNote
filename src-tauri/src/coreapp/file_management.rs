use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json::json;
use tauri::{command, api::path::data_dir};

// Define a structure to deserialize your JSON data
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphInfo {
    name: String,
    path: String,
    created_at: String,
    last_opened: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphList {
    graphs: Vec<GraphInfo>,
}

#[command]
pub fn get_graph_list() -> Result<Vec<GraphInfo>, String> {
    let data_dir = tauri::api::path::data_dir().ok_or("Data directory not found")?;
    let app_data_dir = data_dir.join("NodeNote");
    let registry_path = app_data_dir.join("graph_registry.json");

    // Check if the registry file exists
    if !registry_path.exists() {
        return Err("Graph registry file does not exist".into());
    }

    // Validate the registries
    validate_graph_registries(true).map_err(|e| e.to_string())?;

    // Read the registry file
    let mut file = File::open(registry_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    // Deserialize the JSON content to GraphList
    let graph_list: GraphList = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

    Ok(graph_list.graphs)
}

#[command]
pub fn create_graph_directories(base_path: String, graph_name: String) -> Result<String, String> {

    let graph_path = Path::new(&base_path).join(&graph_name);
    let notes_path = graph_path.join(".graph");
    let internal_path = graph_path.join("nodenote");
    let nodes_path = internal_path.join("nodes");
    let edges_path = internal_path.join("edges");

    // Attempt to create the graph directory and subdirectories
    if let Err(e) = create_dir_all(&notes_path) {
        return Err(format!("Failed to create .graph directory: {}", e));
    }
    if let Err(e) = create_dir_all(&nodes_path) {
        return Err(format!("Failed to create nodes directory: {}", e));
    }

    if let Err(e) = create_dir_all(&edges_path) {
        return Err(format!("Failed to create edges directory: {}", e));
    }

    // Create the config file in the graph directory
    if let Err(e) = create_config_file(&internal_path) {
        return Err(format!("Failed to create config file: {}", e));
    }

    // Update the graph registry
    if let Err(e) = add_graph_registry(&base_path, &graph_name) {
        return Err(format!("Failed to update graph registry: {}", e.to_string()));
    }

    Ok(format!("Graph created successfully at {:?}", graph_path))
}

fn create_config_file(graph_path: &Path) -> Result<(), String> {
    let config_path = graph_path.join("config.edn");
    match File::create(&config_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(
                b"{:meta/version 1\n ;; Currently, there are no config settings.\n ;; This will change at some point.\n }"
            ) {
                return Err(format!("Failed to write to config file: {}", e));
            }
        },
        Err(e) => return Err(format!("Failed to create config file: {}", e)),
    }
    Ok(())
}

fn add_graph_registry(base_path: &str, graph_name: &str) -> io::Result<()> {
    // Determine the base directory for app data
    let base_data_dir = data_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Data directory not found"))?;
    let app_data_dir = base_data_dir.join("NodeNote");

    // Ensure the application-specific directory exists
    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir)?;
    }

    // Specify the path to the graph registry within the application-specific directory
    let registry_path = app_data_dir.join("graph_registry.json");

    // Create the graph_registry.json if it does not already exist
    let mut registry = if registry_path.exists() {
        let mut file = File::open(&registry_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({ "graphs": [] }))
    } else {
        json!({ "graphs": [] })
    };

    // Add new graph to the registry
    let new_entry = json!({
        "name": graph_name,
        "path": base_path,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "last_opened": chrono::Utc::now().to_rfc3339()
    });
    registry["graphs"].as_array_mut().unwrap().push(new_entry);

    // Write the updated registry back to the file
    let mut file = OpenOptions::new().write(true).create(true).open(registry_path)?;
    file.write_all(serde_json::to_string_pretty(&registry)?.as_bytes())?;

    Ok(())
}

fn update_graph_registry(base_path: &str, graph_name: &str) -> io::Result<()> {
    // Determine the base directory for app data
    let base_data_dir = data_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Data directory not found"))?;
    let app_data_dir = base_data_dir.join("NodeNote");

    // Ensure the application-specific directory exists
    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir)?;
    }

    // Specify the path to the graph registry within the application-specific directory
    let registry_path = app_data_dir.join("graph_registry.json");

    // Load the graph registry
    let mut registry = if registry_path.exists() {
        let mut file = File::open(&registry_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({ "graphs": [] }))
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Graph registry not found"));
    };

    // Update the "last_opened" field of the specified graph
    if let Some(graphs) = registry["graphs"].as_array_mut() {
        for graph in graphs.iter_mut() {
            if graph["name"] == graph_name && graph["path"] == base_path {
                graph["last_opened"] = json!(chrono::Utc::now().to_rfc3339());
                break;
            }
        }
    }

    // Write the updated registry back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(registry_path)?;
    file.write_all(serde_json::to_string_pretty(&registry)?.as_bytes())?;

    Ok(())
}

fn remove_graph_registry(base_path: &str, graph_name: &str) -> io::Result<()> {
    // Determine the base directory for app data
    let base_data_dir = data_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Data directory not found"))?;
    let app_data_dir = base_data_dir.join("NodeNote");

    // Ensure the application-specific directory exists
    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir)?;
    }

    // Specify the path to the graph registry within the application-specific directory
    let registry_path = app_data_dir.join("graph_registry.json");

    // Load the graph registry
    let mut registry = if registry_path.exists() {
        let mut file = File::open(&registry_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({ "graphs": [] }))
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Graph registry not found"));
    };

    // Remove the specified graph from the registry
    if let Some(graphs) = registry["graphs"].as_array_mut() {
        graphs.retain(|graph| !(graph["name"] == graph_name && graph["path"] == base_path));
    }

    // Write the updated registry back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(registry_path)?;
    file.write_all(serde_json::to_string_pretty(&registry)?.as_bytes())?;

    Ok(())
}

fn validate_graph_registries(clean_invalid: bool) -> io::Result<()> {
    // Determine the base directory for app data
    let base_data_dir = data_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Data directory not found"))?;
    let app_data_dir = base_data_dir.join("NodeNote");

    // Ensure the application-specific directory exists
    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir)?;
    }

    // Specify the path to the graph registry within the application-specific directory
    let registry_path = app_data_dir.join("graph_registry.json");

    // Load the graph registry
    let mut registry = if registry_path.exists() {
        let mut file = File::open(&registry_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({ "graphs": [] }))
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Graph registry not found"));
    };

    // Validate each graph registry entry
    if let Some(graphs) = registry["graphs"].as_array_mut() {
        let mut invalid_graphs = Vec::new();

        for graph in graphs.iter_mut() {
            let path = graph["path"].as_str().unwrap_or("");
            let name = graph["name"].as_str().unwrap_or("");
            if Path::new(path).join(name).exists() {
                graph["validated"] = json!(true);
            } else {
                graph["validated"] = json!(false);
                if clean_invalid {
                    invalid_graphs.push(graph.clone());
                }
            }
        }

        // Remove invalid graphs if clean_invalid is true
        if clean_invalid {
            for invalid_graph in invalid_graphs {
                graphs.retain(|graph| graph != &invalid_graph);
            }
        }
    }

    // Write the updated registry back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(registry_path)?;
    file.write_all(serde_json::to_string_pretty(&registry)?.as_bytes())?;

    Ok(())
}