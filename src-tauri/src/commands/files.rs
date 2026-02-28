use std::fs;
use std::path::PathBuf;

const NOTEBOOK_NAME: &str = "Infinium";

// Gives javascript way to read rust struct as json
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileNode {
    name: String,
    path: String,
    parent: Option<Box<FileNode>>,
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
            parent: None,
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

        let parent = path
            .parent()
            .and_then(|p| {
                let rel = p.strip_prefix(&root).ok()?;
                Some(Box::new(FileNode {
                    name: p
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| NOTEBOOK_NAME.to_string()),
                    path: rel.to_string_lossy().to_string(),
                    parent: None,
                    is_folder: true,
                    children: None,
                }))
            });

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
                parent,
                is_folder: true,
                children: Some(children),
            }
        } else {
            FileNode {
                name,
                path: relative_path,
                parent,
                is_folder: false,
                children: None,
            }
        }
    }

    fn touch_file(&self) -> Result<(), String> {
        let path = get_notebook_dir()?.join(&self.path);

        fs::write(path, "").map_err(|e| e.to_string())
    }

    fn read_file(&self) -> Result<String, String> {
        let path = get_notebook_dir()?.join(&self.path);

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
pub fn read_file(node: FileNode) -> Result<String, String> {
    node.read_file()
}

#[tauri::command]
pub fn create_file(file_name: String, folder_path: String) -> Result<(), String> {
    let rel_path = if folder_path.is_empty() {
        PathBuf::from(format!("{}.md", file_name))
    } else {
        PathBuf::from(&folder_path).join(format!("{}.md", file_name))
    };

    let node = FileNode {
        name: file_name,
        path: rel_path.to_string_lossy().to_string(),
        parent: None,
        is_folder: false,
        children: None,
    };
    node.touch_file()
}
