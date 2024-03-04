use anyhow::Context;

use clap::Parser;

use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::postgres::PgPoolOptions;

use pih_rs::config::Config;
use pih_rs::http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events.
                "pih_rs=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // set up a connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(config.max_pool_size)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .context("cannot connect to database")?;

    // This embeds database migrations in the application binary so we can ensure
    // the database schema is up to date when the application starts.
    sqlx::migrate!().run(&db_pool).await.unwrap();

    http::serve(config, db_pool).await?;

    Ok(())
}
