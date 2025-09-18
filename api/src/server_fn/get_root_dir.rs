use leptos::{
    prelude::{ServerFnError, use_context},
    server,
};

use crate::{server_state::ServerState, utils::normalize_path};

#[server(GetRootDir, "/api", endpoint = "get_root_dir")]
pub async fn get_root_dir() -> Result<String, ServerFnError> {
    match use_context::<ServerState>() {
        Some(state) => Ok(normalize_path(state.root_dir())),
        None => Err(ServerFnError::ServerError(
            "Failed to convert path to string".to_string(),
        )),
    }
}
