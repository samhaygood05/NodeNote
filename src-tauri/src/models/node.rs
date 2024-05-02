use std::path::{PathBuf, Path};

/// Represents a general node in your application, initialized with a markdown file.
#[derive(Debug, Clone)]
pub struct Node {
    /// The base name of the node, derived from the markdown file name without the extension.
    pub base_name: String,
    /// Path to the markdown file that serves as the 'root' of the node.
    pub markdown_path: PathBuf,
    /// A list of associated file paths, including various extensions and sublabels.
    pub associated_files: Vec<PathBuf>,
    /// Optional path to a subfolder representing a subgraph.
    pub subgraph_path: Option<PathBuf>,
}

impl Node {
    /// Constructs a new `Node` from a markdown file path.
    /// Panics if the provided path is not a markdown file.
    pub fn new(markdown_path: PathBuf) -> Self {
        if markdown_path.extension().map_or(true, |ext| ext != "md") {
            panic!("Provided file is not a markdown file");
        }

        let base_name = markdown_path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.split('.').next().unwrap_or(""))
            .unwrap_or("")
            .to_string();

        Node {
            base_name,
            markdown_path,
            associated_files: Vec::new(),
            subgraph_path: None,
        }
    }

    /// Constructs a new `Node` from a markdown file path and a prefix.
    /// Panics if the provided path is not a markdown file.
    pub fn new(markdown_path: PathBuf, prefix: String) -> Self {
        if markdown_path.extension().map_or(true, |ext| ext != "md") {
            panic!("Provided file is not a markdown file");
        }

        let base_name = markdown_path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.split('.').next().unwrap_or(""))
            .unwrap_or("")
            .to_string();

        Node {
            format!("{}{}", prefix, base_name),
            markdown_path,
            associated_files: Vec::new(),
            subgraph_path: None,
        }
    }

    /// Adds a file to the node's list of associated files.
    /// Files are added if they share the same base name regardless of sublabels, separated by periods.
    pub fn add_file(&mut self, file_path: PathBuf) {
        if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
            // Create a normalized version of the stem where sublabels are included in the comparison
            let normalized_stem = stem.split('.').next().unwrap_or("");
            if normalized_stem == self.base_name && file_path != self.markdown_path {
                self.associated_files.push(file_path);
            }
        }
    }


    /// Sets the subgraph path for this node, representing the location of the subgraph.
    pub fn set_subgraph_path(&mut self, path: PathBuf) {
        self.subgraph_path = Some(path);
    }
}
