use std::net::SocketAddr;

use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    compression::CompressionLayer,
    trace::{
        DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
        DefaultOnResponse, TraceLayer,
    },
};
use tracing::Level;

pub const PORT: u16 = 3001;
pub const USE_IPV6: bool = true;

#[inline]
pub fn logging() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_request(DefaultOnRequest::default())
        .on_body_chunk(DefaultOnBodyChunk::default())
        .on_eos(DefaultOnEos::default())
        .on_failure(DefaultOnFailure::default())
}

#[inline]
pub fn compression() -> CompressionLayer {
    CompressionLayer::new().br(true).gzip(true)
}

fn ipv6_listen_address() -> SocketAddr {
    ([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], PORT).into()
}

fn ipv4_listen_address() -> SocketAddr {
    ([0,0,0,0], PORT).into()
}

pub fn listen_address() -> SocketAddr {
    if USE_IPV6 {
        ipv6_listen_address()
    }
    else {
        ipv4_listen_address()
    }
}