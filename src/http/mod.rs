// Utility modules.

use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

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

// TODO - investigate use of tower_http
// use tower_http::{
//     catch_panic::CatchPanicLayer, compression::CompressionLayer,
//     sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer, trace::TraceLayer,
// };

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<ApiContext>` to the parameters of
/// handler function.
#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db_pool: PgPool,
}
