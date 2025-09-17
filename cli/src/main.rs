use std::net::{Ipv4Addr, SocketAddrV4};

use axum::Router;
use clap::Parser;
use tokio::signal;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, args.port);

    let app = Router::new().route("/", axum::routing::get(hello_world));

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

async fn hello_world() -> String {
    String::from("Hello, world!")
}

async fn shutdown_signal(){
    let ctrl_c = async{
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
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
