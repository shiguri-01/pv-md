use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use clap::Parser;
use rust_embed::Embed;
use tokio::signal;

#[allow(dead_code)]
#[derive(Embed)]
#[folder = ".."]
#[include = "target/site/pkg/*"]
#[include = "index.html"]
struct AppAssets;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, args.port);

    let app = Router::new()
        .route("/", get(html_handler))
        .route("/assets/frontend.js", get(js_handler))
        .route("/assets/frontend_bg.wasm", get(wasm_handler));

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
