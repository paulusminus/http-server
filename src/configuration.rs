use std::net::{IpAddr, SocketAddr};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    compression::CompressionLayer,
    trace::{
        DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
        DefaultOnResponse, TraceLayer,
    },
};
use tracing::Level;

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

fn ipv6_all() -> IpAddr {
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
}

fn ipv4_all() -> IpAddr {
    [0, 0, 0, 0].into()
}

pub fn listen_address(use_ipv6: bool, port: u16) -> SocketAddr {
    if use_ipv6 {
        (ipv6_all(), port).into()
    } else {
        (ipv4_all(), port).into()
    }
}
