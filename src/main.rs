use std::error::Error;

use crate::configuration::{compression, listen_address, logging};
use axum::Router;
use configuration::Website;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod configuration;
mod shutdown;

const PORT: u16 = 3060;
const USE_IPV6: bool = true;
const WEBSITES: &[(&str, &str)] = &[("/", "paulmin-nl"), ("/lipl-book", "lipl-book"), ("/css", "picocss/pico-1.5.10/css")];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let lipl_router = lipl_storage_server::router_from_environment().await?.route_layer(ValidateRequestHeaderLayer::basic("paul", "CumGranoSalis"));
    let router = Website::new(WEBSITES).router().merge(lipl_router);
    serve(router, USE_IPV6, PORT).await?;
    Ok(())
}

async fn serve(app: Router, use_ipv6: bool, port: u16) -> Result<(), axum::Error> {
    axum::Server::bind(&listen_address(use_ipv6, port))
        .serve(
            app.layer(logging())
                .layer(compression())
                .into_make_service(),
        )
        .with_graceful_shutdown(shutdown::exit_on_signal())
        .await
        .map_err(axum::Error::new)
}
