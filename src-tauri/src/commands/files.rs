use std::fs;
use std::path::{Path, PathBuf};
use tauri::Manager;

// Get path to Infinium folder
fn infinium_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let home_dir = match app.path().home_dir() {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    Ok(home_dir.join("Infinium"))
}

// Gives javascript way to read rust struct as json
#[derive(serde::Serialize)]
pub struct FileNode {
    name: String,
    path: String,
    is_folder: bool,
    children: Option<Vec<FileNode>>,
}

// Build file tree
fn build_tree(path: &Path, root: &Path) -> FileNode {
    let name = path.file_name()
        .unwrap_or_default()
        .to_string_lossy() // to UTF-8
        .to_string();

    // Relative path from Infinium root
    let relative_path = path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    if path.is_dir() {
        let children = fs::read_dir(path)
            .map(|entries| { // Go through every entry in directory
                let mut nodes: Vec<FileNode> = entries
                    .filter_map(|e| e.ok()) // Go through and ignore all errors
                    .map(|e| build_tree(&e.path(), root))
                    .collect(); // Get itterable in vector (list)
                nodes.sort_by(|a, b| {
                    b.is_folder.cmp(&a.is_folder).then(a.name.cmp(&b.name))
                });
                nodes
            })
            .unwrap_or_default();

        FileNode { name, path: relative_path, is_folder: true, children: Some(children) }
    } else {
        FileNode { name, path: relative_path, is_folder: false, children: None } // If file
    }
}

#[tauri::command]
pub fn get_file_tree(app: tauri::AppHandle) -> Result<FileNode, String> {
    let root = infinium_dir(&app)?;

    if !root.exists() {
      match fs::create_dir_all(&root) {
        Ok(()) => (),
        Err(e) => return Err(e.to_string()),
      }
    }

    Ok(build_tree(&root, &root))
}

// Read file
#[tauri::command]
pub fn read_file(app: tauri::AppHandle, file_name: String) -> Result<String, String> {
    let path = infinium_dir(&app)?.join(&file_name);

    fs::read_to_string(path).map_err(|e| e.to_string())
}

// Create file
#[tauri::command]
pub fn create_file(app: tauri::AppHandle, file_name: String) -> Result<(), String> {
    let path = infinium_dir(&app)?.join(format!("{}.md", file_name));

    fs::write(path, "").map_err(|e| e.to_string())
}