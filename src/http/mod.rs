// Utility modules.
use crate::config::Config;
use axum::{http::header::AUTHORIZATION, Router};
use sqlx::PgPool;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

/// Defines a common error type to use for all request handlers.
mod error;

/// Contains definitions for the application-specific parameters to handler functions.
mod extractor;

/// A catch-all module for other common types in the API. Arguable, the `error` and `extractor`
/// modules could have been children of this one, but that's more of a subjective decision.
mod types;

/// Modules introducing the API routes. These are the actual handlers for the HTTP requests.
mod sensors;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<ApiContext>` to the parameters of
/// handler function.
#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db_pool: PgPool,
}

pub async fn serve(config: Config, db_pool: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(config),
        db_pool,
    };

    let app = api_router(api_context);

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server")
}

async fn api_router(api_context: ApiContext) -> Router {
    // This is the order that the modules were authored in
    Router::new()
        .merge(sensors::router())
        // Enables logging. Use `RUST_LOG=tower_http=debug` to see the logs.
        .layer((
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
            CompressionLayer::new(),
            TraceLayer::new_for_http().on_failure(()),
            TimeoutLayer::new(Duration::from_secs(30)),
            CatchPanicLayer::new(),
        ))
        .with_state(api_context)
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }
}
