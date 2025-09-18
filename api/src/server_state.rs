use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ServerState {
    root_dir: PathBuf,
}

impl ServerState {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}
