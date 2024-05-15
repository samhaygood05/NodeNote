const { invoke } = window.__TAURI__.tauri;

document.getElementsByClassName('close')[0].addEventListener('click', function() {
  document.getElementById('modal').style.display = 'none';
});

document.getElementById('create-graph-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    const graphName = document.getElementById('graph-name').value;

    if (graphName) {
        try {
            const path = await window.__TAURI__.dialog.open({
                directory: true,  // Ensures the dialog is set to pick directories
                multiple: false,  // Ensures only one directory can be picked
                title: "Select a directory for the new graph"
            });

            if (path) {
                createGraph(graphName, path);
            } else {
                console.log("No directory selected");
                alert("No directory selected");
            }
        } catch (error) {
            console.error('Error selecting directory:', error);
            alert('Failed to select directory: ' + error);
        }
    } else {
        alert("Please enter a name for the graph");
    }
});

document.getElementById('refresh-graphs').addEventListener('click', function() {
    fetchAndDisplayGraphs(); 
});

function createGraph(graphName, directoryPath) {
    window.__TAURI__.invoke('create_graph_directories', {
        basePath: directoryPath,
        graphName: graphName
    }).then(response => {
        console.log(response);  // Success message
        alert('Graph created successfully!');
        document.getElementById('modal').style.display = 'none';
        fetchAndDisplayGraphs()
    }).catch(error => {
        console.error('Error creating graph:', error);
        alert('Failed to create graph: ' + error);
    });
}

function fetchAndDisplayGraphs() {
    invoke('get_graph_list')
        .then(graphs => {
            // Sort graphs by last_opened date
            graphs.sort((a, b) => new Date(b.last_opened) - new Date(a.last_opened));
            displayGraphs(graphs);
        })
        .catch(error => {
            console.error('Failed to fetch graphs:', error);
            alert('Failed to fetch graphs: ' + error);
        });
}
function displayGraphs(graphs) {
    const dropdown = document.getElementById('graph-dropdown');
    dropdown.innerHTML = '<option value="">Select a graph...</option>';  // Clear existing options
    dropdown.appendChild(new Option("Create New Graph", "create-new"));  // Add 'Create New Graph' option

    // Add a separator
    const separator = new Option("──────────", "", false, true);
    separator.disabled = true;  // Make separator non-selectable
    dropdown.appendChild(separator);

    // Add graph options
    graphs.forEach(graph => {
        const option = new Option(graph.name, graph.path);  // Using Option constructor for clarity
        dropdown.appendChild(option);
    });

    dropdown.selectedIndex = 0;
}

document.getElementById('graph-dropdown').addEventListener('change', function() {
    const selectedValue = this.value;
    if (selectedValue === "create-new") {
        // Trigger the modal to open for creating a new graph
        document.getElementById('modal').style.display = 'block';
        this.selectedIndex = 0;
    } else if (selectedValue) {
        console.log(`Graph selected: ${selectedValue}`);
        // Additional functionality to handle graph selection, e.g., load the graph data
    }
});



document.addEventListener('DOMContentLoaded', fetchAndDisplayGraphs);  // Call fetchAndDisplayGraphs when the page loads