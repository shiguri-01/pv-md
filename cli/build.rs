use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../frontend/src/lib.rs");
    println!("cargo:rerun-if-changed=../frontend/Cargo.toml");
    println!("cargo:rerun-if-changed=../index.html");

    if env::var("SKIP_WASM_BUILD").is_ok() {
        println!("Skipping WASM build due to SKIP_WASM_BUILD");
        return;
    }

    let is_release = env::var("PROFILE").map(|p| p == "release").unwrap_or(false);

    let workspace_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .expect("cli has a parent directory")
        .to_path_buf();
    let out_dir = workspace_root.join("target").join("site").join("pkg");

    // cargoの競合を防ぐために専用のディレクトリを作成
    let frontend_target_dir = workspace_root.join("target").join("frontend_build");

    // 1. frontendをwasm32用にビルド
    let mut build_args = vec![
        "build",
        "-p",
        "frontend",
        "--target",
        "wasm32-unknown-unknown",
        "--target-dir",
        frontend_target_dir.to_str().unwrap(),
    ];
    if is_release {
        build_args.push("--release");
    }
    let status = Command::new("cargo")
        .current_dir(&workspace_root)
        .args(&build_args)
        .status()
        .expect("failed to run cargo build for frontend");
    assert!(status.success(), "frontend build failed");

    // 2. wasm-bindgenでjs, wasmを生成
    let target_dir = frontend_target_dir
        .join("wasm32-unknown-unknown")
        .join(if is_release { "release" } else { "debug" });
    let wasm_in = target_dir.join("frontend.wasm");
    assert!(
        wasm_in.exists(),
        "compiled wasm not found: {}",
        wasm_in.display()
    );

    std::fs::create_dir_all(&out_dir).expect("failed to create site/pkg dir");
    let status = Command::new("wasm-bindgen")
        .args([
            wasm_in.to_string_lossy().as_ref(),
            "--target",
            "web",
            "--out-dir",
        ])
        .arg(&out_dir)
        .status()
        .expect("failed to run wasm-bindgen");
    assert!(status.success(), "wasm-bindgen failed");
}
