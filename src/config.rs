/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
/// a bunch of other parameters going here, like API keys for external services
/// or flags enabling or disabling certain features or test modes of the API.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.example` in the repository root for details.
#[derive(clap::Parser, Debug)]
pub struct Config {
    /// The connection string for the PostgreSQL database.
    #[clap(long, env)]
    pub database_url: String,
    /// The max connection pool size for the database.
    #[clap(long, env, default_value = "50")]
    pub max_pool_size: u32,
    /// The url for the HTTP server to listen on.
    #[clap(long, env, default_value = "0.0.0.0:3000")]
    pub server_url: String,
}
