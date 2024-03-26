use std::error::Error;
use std::net::SocketAddr;

use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use tokio::select;
use tokio::{
    net::{TcpListener, TcpStream},
    task::JoinSet,
};
use tracing::{error, info};

use crate::{
    configuration::listen_address,
    service::{file_server, FileServerService},
    shutdown::exit_on_signal,
};

mod configuration;
mod service;
mod shutdown;

const USE_IPV6: bool = true;

async fn handle_connection(
    stream: TcpStream,
    address: SocketAddr,
    hyper_service: FileServerService,
) {
    if let Err(error) = Builder::new(TokioExecutor::new())
        .serve_connection(TokioIo::new(stream), hyper_service)
        .await
    {
        error!("error handling connection {address}: {error}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")?.parse::<u16>()?;
    let www_root = std::env::var("WWW_ROOT")?;
    let hyper_service = file_server(www_root);

    let listen_addr = listen_address(USE_IPV6, port);
    let tcp_listener = TcpListener::bind(&listen_addr).await?;
    info!("listening on http://{}", &listen_addr);

    let mut join_set = JoinSet::new();
    loop {
        select! {
            accept = tcp_listener.accept() =>
                match accept {
                    Ok((stream, address)) => {
                        join_set.spawn(handle_connection(stream, address, hyper_service.clone()));
                    }
                    Err(error) => {
                        error!("failed to accept connection: {}", error)
                    }
                },
            _ = exit_on_signal() => break
        }
    }


    Ok(
        join_set.shutdown().await
    )
}
