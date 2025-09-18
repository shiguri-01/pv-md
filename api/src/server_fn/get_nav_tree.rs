use std::path::{Path, PathBuf};

use leptos::{
    prelude::{ServerFnError, use_context},
    server,
};
use serde::{Deserialize, Serialize};

use crate::utils::normalize_path;

#[server(GetNavTree, "/api", endpoint = "get_nav_tree")]
pub async fn get_nav_tree() -> Result<Vec<NavTreeDto>, ServerFnError> {
    let root_dir = match use_context::<crate::server_state::ServerState>() {
        Some(state) => state.root_dir().to_path_buf(),
        None => {
            return Err(ServerFnError::ServerError(
                "Failed to get root directory".to_string(),
            ));
        }
    };
    let files = scan_markdown_files(&root_dir);
    let tree = build_nav_tree(&files);
    Ok(tree.into_iter().map(Into::into).collect())
}

#[derive(Debug, Clone)]
pub enum NavTree {
    File {
        name: String,
        path: PathBuf,
    },
    Dir {
        name: String,
        path: PathBuf,
        children: Vec<NavTree>,
    },
}

impl NavTree {
    fn new_file(name: String, path: PathBuf) -> Self {
        NavTree::File { name, path }
    }
    fn new_dir(name: String, path: PathBuf) -> Self {
        NavTree::Dir {
            name,
            path,
            children: Vec::new(),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            NavTree::File { name, .. } => name,
            NavTree::Dir { name, .. } => name,
        }
    }
    pub fn path(&self) -> &PathBuf {
        match self {
            NavTree::File { path, .. } => path,
            NavTree::Dir { path, .. } => path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavTreeDto {
    File {
        name: String,
        path: String,
    },
    Dir {
        name: String,
        path: String,
        children: Vec<NavTreeDto>,
    },
}

impl From<NavTree> for NavTreeDto {
    fn from(value: NavTree) -> Self {
        match value {
            NavTree::File { name, path } => NavTreeDto::File {
                name,
                path: normalize_path(&path),
            },
            NavTree::Dir {
                name,
                path,
                children,
            } => NavTreeDto::Dir {
                name,
                path: normalize_path(&path),
                children: children.into_iter().map(Into::into).collect(),
            },
        }
    }
}

fn scan_markdown_files(dir: &Path) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .filter(|e| e.path().is_file())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                ext == "md"
            } else {
                false
            }
        })
        .map(|e| e.path().strip_prefix(dir).unwrap().to_path_buf())
        .collect()
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn build_nav_tree(relative_paths: &[PathBuf]) -> Vec<NavTree> {
    // すべてのノードの親となる、一時的なルートノード
    let mut root = NavTree::new_dir("root".to_string(), PathBuf::new());

    for path in relative_paths {
        let mut current_node = &mut root;
        let components: Vec<_> = path.components().collect();

        if components.is_empty() {
            continue;
        }

        let is_file = path.extension().is_some();
        let dir_components = if is_file {
            &components[..components.len() - 1]
        } else {
            &components[..]
        };
        let file_component = if is_file { &components.last() } else { &None };

        let mut current_path = PathBuf::new();

        // ディレクトリのノードを辿るか作成する
        for component in dir_components {
            let name = component.as_os_str().to_string_lossy().to_string();
            current_path.push(component.as_os_str());

            // 現在のノードから同じ名前のディレクトリを探す
            let children = match current_node {
                NavTree::Dir { children, .. } => children,

                // ファイルは見つからないはず
                NavTree::File { .. } => unreachable!(),
            };

            let child_dir_index = match children
                .iter()
                .position(|child| child.name() == name && matches!(child, NavTree::Dir { .. }))
            {
                Some(index) => index, // 既存のディレクトリを使用
                None => {
                    // ディレクトリが存在しない場合は新たに作成
                    let new_dir = NavTree::new_dir(name.clone(), current_path.clone());
                    children.push(new_dir);
                    children.len() - 1 // 新しいディレクトリのインデックス
                }
            };

            // 次の階層のノードに移動
            current_node = &mut children[child_dir_index];
        }

        // ファイルのノードを追加
        if let Some(file_component) = file_component
            && let NavTree::Dir { children, .. } = current_node
        {
            let file_name = file_component.as_os_str().to_string_lossy().into_owned();
            let file_path = path.clone();
            children.push(NavTree::new_file(file_name, file_path));
        }
    }

    // 一時的なルートノードの子ノードを返す
    if let NavTree::Dir { children, .. } = root {
        children
    } else {
        vec![]
    }
}
