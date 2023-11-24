use tokio::signal::unix::{signal, SignalKind};

#[allow(dead_code)]
#[inline]
pub async fn exit_on_signal_windows() {
    match tokio::signal::ctrl_c().await {
        Ok(_) => {
            message::exit_on_signal_int();
        }
        Err(error) => {
            message::error_on_receiving_signal(error);
        }
    };
}

#[inline]
pub async fn exit_on_signal_unix() {
    let mut wait_on_term_stream = signal(SignalKind::terminate()).unwrap();
    let mut wait_on_term_int = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        on_int = wait_on_term_int.recv() => {
            if on_int.is_some() {
                message::exit_on_signal_int();
            }
        }
        on_term = wait_on_term_stream.recv() => {
            if on_term.is_some() {
                message::exit_on_signal_term();
            }
        }
    }
}

mod message {
    pub fn exit_on_signal_int() {
        tracing::info!("Exiting on recieved signal SIGINT");
    }

    pub fn exit_on_signal_term() {
        tracing::info!("Exiting on recieved signal SIGTERM");
    }

    pub fn error_on_receiving_signal<E: std::error::Error>(error: E) {
        tracing::error!("Error {error} receiving signal");
    }
}
