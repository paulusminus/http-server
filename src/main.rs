use std::error::Error;

use crate::configuration::{compression, listen_address, logging};
use axum::{routing::get_service, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod configuration;
mod shutdown;

const USE_IPV6: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::default()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let www_root = std::env::var("WWW_ROOT")?;

    let files_service = ServeDir::new(www_root);
    let router = Router::new().fallback_service(get_service(files_service));
    let listener = TcpListener::bind(listen_address(USE_IPV6, port))
        .await
        .map_err(axum::Error::new)?;
    axum::serve(
        listener,
        router
            .layer(logging())
            .layer(compression())
            .into_make_service(),
    )
    .with_graceful_shutdown(shutdown::exit_on_signal())
    .await?;
    Ok(())
}
