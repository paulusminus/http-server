use crate::configuration::{compression, listen_address, logging};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
    service::TowerToHyperService,
};
use std::error::Error;
use tokio::{net::TcpListener, task::JoinSet};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::{error, info};

mod configuration;

const USE_IPV6: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let www_root = std::env::var("WWW_ROOT")?;

    let tower_service = ServiceBuilder::new()
        .layer(compression())
        .layer(logging())
        .service(ServeDir::new(www_root));

    let hyper_service = TowerToHyperService::new(tower_service);

    let listen_addr = listen_address(USE_IPV6, port);
    let tcp_listener = TcpListener::bind(&listen_addr).await?;
    info!("listening on http://{}", &listen_addr);

    let mut join_set = JoinSet::new();
    loop {
        let (stream, addr) = match tcp_listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                error!("failed to accept connection: {e}");
                continue;
            }
        };

        let service = hyper_service.clone();
        let serve_connection = async move {
            info!("handling a request from {addr}");

            let result = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service)
                .await;

            if let Err(e) = result {
                error!("error serving {addr}: {e}");
            }

            info!("handled a request from {addr}");
        };

        join_set.spawn(serve_connection);
    }
}
