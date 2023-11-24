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
