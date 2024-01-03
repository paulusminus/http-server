use crate::configuration::{compression, listen_address, logging};
use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

mod configuration;
mod shutdown;

const USE_IPV6: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(fmt::layer())
        .init();

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let www_root = std::env::var("WWW_ROOT")?;

    let service = ServiceBuilder::new()
        .layer(compression())
        .layer(logging())
        .service(ServeDir::new(www_root));

    let router = Router::new().fallback_service(service);
    let listener = TcpListener::bind(listen_address(USE_IPV6, port))
        .await
        .map_err(axum::Error::new)?;
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::exit_on_signal())
        .await?;
    Ok(())
}
