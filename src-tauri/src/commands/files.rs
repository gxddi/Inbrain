use std::fs;
use std::path::PathBuf;

const NOTEBOOK_NAME: &str = "Infinium";

// Gives javascript way to read rust struct as json
#[derive(serde::Serialize)]
pub struct FileNode {
    name: String,
    path: String,
    is_folder: bool,
    children: Option<Vec<FileNode>>,
}

// Get path to user's Notebook folder
fn get_notebook_dir() -> Result<PathBuf, String> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| "Could not resulove home directory".to_string())?;

    Ok(home_dir.join(NOTEBOOK_NAME))
}

impl FileNode {
    fn init_root() -> Self {
        let root = get_notebook_dir().unwrap();
        let children = fs::read_dir(&root)
            .map(|entries| {
                let mut nodes: Vec<FileNode> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| Self::build_tree(&e.path()))
                    .collect();
                nodes.sort_by(|a, b| b.is_folder.cmp(&a.is_folder).then(a.name.cmp(&b.name)));
                nodes
            })
            .unwrap_or_default();

        Self {
            name: String::from("root"),
            path: root.to_string_lossy().to_string(),
            is_folder: true,
            children: Some(children),
        }
    }

    // Build file tree from path
    fn build_tree(path: &PathBuf) -> FileNode {
        let root = get_notebook_dir().unwrap();

        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let relative_path = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            let children = fs::read_dir(path)
                .map(|entries| {
                    let mut nodes: Vec<FileNode> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| Self::build_tree(&e.path()))
                        .collect();
                    nodes.sort_by(|a, b| b.is_folder.cmp(&a.is_folder).then(a.name.cmp(&b.name)));
                    nodes
                })
                .unwrap_or_default();

            FileNode {
                name,
                path: relative_path,
                is_folder: true,
                children: Some(children),
            }
        } else {
            FileNode {
                name,
                path: relative_path,
                is_folder: false,
                children: None,
            }
        }
    }

    fn touch_file(file_name: String) -> Result<(), String> {
        let path = get_notebook_dir()?.join(format!("{}.md", file_name));

        fs::write(path, "").map_err(|e| e.to_string())
    }

    fn read_file(file_name: String) -> Result<String, String> {
        let path = get_notebook_dir()?.join(&file_name);

        fs::read_to_string(path).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn get_root_node() -> Result<FileNode, String> {
    let root = get_notebook_dir()?;

    if !root.exists() {
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    }

    Ok(FileNode::init_root())
}

#[tauri::command]
pub fn read_file(file_name: String) -> Result<String, String> {
    FileNode::read_file(file_name)
}

#[tauri::command]
pub fn create_file(file_name: String) -> Result<(), String> {
    FileNode::touch_file(file_name)
}
