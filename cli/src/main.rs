use std::{
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
};

use api::server_state::ServerState;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{HeaderValue, Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use clap::Parser;

use leptos::{prelude::provide_context, server_fn::axum::register_explicit};
use leptos_axum::handle_server_fns_with_context;
use rust_embed::Embed;
use tokio::signal;

#[tokio::main]
async fn main() {
    // server fnの登録
    register_explicit::<api::GetRootDir>();
    register_explicit::<api::GetNavTree>();

    let args = Args::parse();
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, args.port);

    let root_dir = resolve_directory(Path::new(&args.dir)).expect("Invalid directory");

    let app = Router::new()
        .route("/", get(html_handler))
        .route("/assets/frontend.js", get(js_handler))
        .route("/assets/frontend_bg.wasm", get(wasm_handler))
        .route("/api/{*server_fn}", post(server_fn_handler))
        .with_state(ServerState::new(root_dir));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

#[derive(Parser, Debug)]
#[command( version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("."))]
    dir: String,

    #[arg(short, long, default_value_t = 5000)]
    port: u16,
}

#[allow(dead_code)]
#[derive(Embed)]
#[folder = ".."]
#[include = "target/site/pkg/*"]
#[include = "index.html"]
struct AppAssets;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // どちらかのシグナルを待つ
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn html_handler() -> impl IntoResponse {
    match AppAssets::get("index.html") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .body(Body::from(content.data))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".into())
            .unwrap(),
    }
}

async fn js_handler() -> impl IntoResponse {
    match AppAssets::get("target/site/pkg/frontend.js") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/javascript"),
            )
            .body(Body::from(content.data))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".into())
            .unwrap(),
    }
}

async fn wasm_handler() -> impl IntoResponse {
    match AppAssets::get("target/site/pkg/frontend_bg.wasm") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/wasm"),
            )
            .body(Body::from(content.data))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("404 Not Found".into())
            .unwrap(),
    }
}

async fn server_fn_handler(
    State(state): State<ServerState>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(move || provide_context(state.clone()), req).await
}

fn resolve_directory(dir: &Path) -> Result<PathBuf, std::io::Error> {
    let path = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        std::env::current_dir()?.join(dir)
    };

    if !path.exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", path.display()),
        ))
    } else if !path.is_dir() {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a directory: {}", path.display()),
        ))
    } else {
        Ok(path.canonicalize()?)
    }
}
