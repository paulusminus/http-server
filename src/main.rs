use std::net::SocketAddr;
use std::{convert::Infallible, error::Error};

use hyper::{body::Incoming, Request, Response};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
    service::TowerToHyperService,
};
use tokio::select;
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinSet,
};
use tower::{util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::{
    classify::{NeverClassifyEos, ServerErrorsFailureClass},
    compression::CompressionBody,
    services::{fs::ServeFileSystemResponseBody, ServeDir},
    trace,
};
use tracing::{error, info};

use crate::{
    configuration::{compression, listen_address, logging},
    shutdown::exit_on_signal,
};

mod configuration;
mod shutdown;

type HyperService = TowerToHyperService<
    BoxCloneService<
        Request<Incoming>,
        Response<
            trace::ResponseBody<
                CompressionBody<ServeFileSystemResponseBody>,
                NeverClassifyEos<ServerErrorsFailureClass>,
            >,
        >,
        Infallible,
    >,
>;

const USE_IPV6: bool = true;

async fn handle_connection(stream: TcpStream, address: SocketAddr, hyper_service: HyperService) {
    info!("start handling connection from {address}");

    match Builder::new(TokioExecutor::new())
        .serve_connection(TokioIo::new(stream), hyper_service)
        .await
    {
        Ok(_) => info!("finished handling connection from {address}"),
        Err(error) => error!("error handling connection {address}: {error}"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let www_root = std::env::var("WWW_ROOT")?;

    let tower_service = ServiceBuilder::new()
        .layer(logging())
        .layer(compression())
        .service(ServeDir::new(www_root))
        .boxed_clone();

    let hyper_service = TowerToHyperService::new(tower_service);

    let listen_addr = listen_address(USE_IPV6, port);
    let tcp_listener = TcpListener::bind(&listen_addr).await?;
    info!("listening on http://{}", &listen_addr);

    let mut join_set = JoinSet::new();
    loop {
        select! {
            accept = tcp_listener.accept() => {
                match accept {
                    Ok((stream, address)) => {
                        join_set.spawn(handle_connection(stream, address, hyper_service.clone()));
                    }
                    Err(error) => {
                        error!("failed to accept connection: {}", error);
                    }
                };
            }
            _ = exit_on_signal() => break
        }
    }
    Ok(())
}
