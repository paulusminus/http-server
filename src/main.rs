use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod configuration;
mod shutdown;

#[cfg(windows)]
use shutdown::exit_on_signal_windows as exit_on_signal;

#[cfg(unix)]
use shutdown::exit_on_signal_unix as exit_on_signal;

use crate::configuration::{compression, logging, PORT};

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    serve(two_serve_dirs(), PORT).await
}

fn two_serve_dirs() -> Router {
    let serve_dir_from_assets = ServeDir::new("paulmin-nl");
    let serve_dir_from_dist = ServeDir::new("lipl-book");

    Router::new()
        .nest_service("/", serve_dir_from_assets)
        .nest_service("/lipl-book", serve_dir_from_dist)
}

async fn serve(app: Router, port: u16) -> Result<(), axum::Error> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app.layer(logging())
                .layer(compression())
                .into_make_service(),
        )
        .with_graceful_shutdown(exit_on_signal())
        .await
        .map_err(axum::Error::new)
}
