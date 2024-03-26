use hyper_util::service::TowerToHyperService;
use tower::ServiceBuilder;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::compression::Compression;
use tower_http::services::ServeDir;
use tower_http::trace::Trace;

use crate::configuration::{compression, logging};

pub type FileServerService =
    TowerToHyperService<Trace<Compression<ServeDir>, SharedClassifier<ServerErrorsAsFailures>>>;

pub fn file_server<S: AsRef<str>>(www_root: S) -> FileServerService {
    TowerToHyperService::new(
        ServiceBuilder::new()
            .layer(logging())
            .layer(compression())
            .service(ServeDir::new(www_root.as_ref())),
    )
}
