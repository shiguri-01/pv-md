use std::path::Path;

pub fn normalize_path(path: &Path) -> String {
    let mut path_str = path.to_string_lossy().to_string();

    // WindowsのVerbatim Pathプレフィックスを取り除く
    if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
        path_str = stripped.to_string();
    }

    path_str.replace('\\', "/")
}
