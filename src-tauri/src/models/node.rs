use std::path::{PathBuf, Path};

/// Represents a general node in NodeNote, initialized with a name and a root path
#[derive(Debug, Clone)]
pub struct Node {
    /// This is the name of the node
    pub node_name: String,

    /// This is the folder of the node
    pub node_path: PathBuf,

    /// This tells NodeNote if the node has a subgraph
    pub subgraph: bool
}

impl Node {
    /// Constructs a new `Node` from a name and the root path.
    pub fn new(node_name: String, node_path: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&node_path.join(node_name)) {
            panic!("Failed to create node directory: {}", e);
        }

        Node {
            node_name,
            node_path,
            subgraph: false
        }
    }

    /// Creates the subgraph for the node
    pub fn create_subgraph(&mut self) -> std::io::Result<()> {
        let graph_path = self.node_path.join(node_name).join(".graph");

        if let Err(e) = fs::create_dir_all(&graph_path) {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to create .graph directory: {}", e)));
        }
        self.subgraph = true;
        Ok(())
    }

    /// Deletes the subgraph of the node if it is empty
    /// If `ignore_warning` is `true`, it will delete the subgraph even if it is not empty
    pub fn delete_subgraph(&mut self, ignore_warning: bool) -> io::Result<()> {
        let graph_path = self.node_path.join(node_name).join(".graph");
        
        if graph_path.exists() {
            if !ignore_warning {
                let is_empty = fs::read_dir(&graph_path)?.next().is_none();
                if !is_empty {
                    return Err(io::Error::new(io::ErrorKind::Other, "Warning: .graph folder is not empty"));
                }
            }
            if let Err(e) = fs::remove_dir_all(&graph_path) {
                return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to delete .graph directory: {}", e)));
            }
        }

        self.subgraph = false;
        Ok(())
    }

    /// Renames the node
    pub fn rename(&mut self, new_name: String) -> io::Result<()> {
        let old_path = self.node_path.join(node_name);
        let new_path = self.node_path.join(new_name);

        if Err(e) = fs::rename(&old_path, &new_path) {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to rename: {}", e)));
        }
        self.node_name = new_name;
        Ok(())
    }
}
