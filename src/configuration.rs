use std::net::{IpAddr, SocketAddr};

use axum::Router;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    compression::CompressionLayer,
    services::ServeDir,
    trace::{
        DefaultMakeSpan, DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest,
        DefaultOnResponse, TraceLayer,
    },
};
use tracing::Level;

pub struct Website<'a> {
    serve_dirs: &'a [(&'static str, &'static str)],
}

impl<'a> Website<'a> {
    pub fn new(serve_dirs: &'a [(&'static str, &'static str)]) -> Website<'a> {
        Website { serve_dirs }
    }

    pub fn router(&self) -> Router {
        let mut router = Router::new();
        for (path, serve_dir) in self.serve_dirs {
            let p = path;
            router = router.nest_service(p, ServeDir::new(serve_dir));
        }
        router
    }
}

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
