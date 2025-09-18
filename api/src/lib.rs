use std::path::Path;

use leptos::{
    prelude::{ServerFnError, use_context},
    server,
};

use crate::server_state::ServerState;
pub mod server_state;

#[server(GetRootDir, "/api", endpoint = "get_root_dir")]
pub async fn get_root_dir() -> Result<String, ServerFnError> {
    match use_context::<ServerState>() {
        Some(state) => Ok(normalize_path(state.root_dir())),
        None => Err(ServerFnError::ServerError(
            "Failed to convert path to string".to_string(),
        )),
    }
}

fn normalize_path(path: &Path) -> String {
    let mut path_str = path.to_string_lossy().to_string();

    // WindowsのVerbatim Pathプレフィックスを取り除く
    if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
        path_str = stripped.to_string();
    }

    path_str.replace('\\', "/")
}
