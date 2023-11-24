use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), axum::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    serve(two_serve_dirs(), 3005).await
}

fn two_serve_dirs() -> Router {
    // you can also have two `ServeDir`s nested at different paths
    let serve_dir_from_assets = ServeDir::new("www.paulmin.nl");
    let serve_dir_from_dist = ServeDir::new("lipl-book");

    Router::new()
        .nest_service("/", serve_dir_from_assets)
        .nest_service("/lipl-book", serve_dir_from_dist)
}

async fn serve(app: Router, port: u16) -> Result<(), axum::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .map_err(axum::Error::new)
}
