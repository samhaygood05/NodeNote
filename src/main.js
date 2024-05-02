const { invoke } = window.__TAURI__.tauri;

document.getElementById('create-graph-btn').addEventListener('click', function() {
  document.getElementById('modal').style.display = 'block';
});

document.getElementsByClassName('close')[0].addEventListener('click', function() {
  document.getElementById('modal').style.display = 'none';
});

document.getElementById('select-directory').addEventListener('click', async function() {
  try {
      const path = await window.__TAURI__.dialog.open({
          directory: true,  // Ensures the dialog is set to pick directories
          multiple: false,  // Ensures only one directory can be picked
          title: "Select a directory for the new graph"
      });

      if (path) {
          document.getElementById('selected-directory-path').textContent = `Selected Directory: ${path}`;
      } else {
          document.getElementById('selected-directory-path').textContent = "No directory selected";
      }
  } catch (error) {
      console.error('Error selecting directory:', error);
      document.getElementById('selected-directory-path').textContent = "Failed to select directory";
  }
});


document.getElementById('create-graph-form').addEventListener('submit', function(event) {
    event.preventDefault();
    const graphName = document.getElementById('graph-name').value;
    const directoryPath = document.getElementById('selected-directory-path').textContent.replace('Selected Directory: ', '');

    if (directoryPath && graphName) {
      window.__TAURI__.invoke('create_graph_directories', {
          basePath: directoryPath,
          graphName: graphName
      }).then(response => {
          console.log(response);  // Success message
          alert('Graph created successfully!');
      }).catch(error => {
          console.error('Error creating graph:', error);
          alert('Failed to create graph: ' + error);
      });
    
    }

    document.getElementById('modal').style.display = 'none';
});

