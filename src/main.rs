use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sqlx::postgres::{PgPool, PgPoolOptions};

use pih_rs::config::Config;

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
        .expect("cannot connect to database");

    // This embeds database migrations in the application binary so we can ensure
    // the database schema is up to date when the application starts.
    sqlx::migrate!().run(&db_pool).await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST / update-sensor` goes to `update_sensor`
        .route("/update-sensor", get(update_sensor))
        .route("/sensor-data", get(get_sensor_data))
        .with_state(db_pool);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(&config.server_url)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .context("error running server")
}

// TODO - move to error module
struct AppError(anyhow::Error);

// TODO - move to error module
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}

// TODO - move to error module
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn update_sensor(payload: Query<UpdateSensor>, State(pool): State<PgPool>) {
    let payload: UpdateSensor = payload.0;
    let now_with_utc: DateTime<Local> = Local::now();

    dbg!(&payload);

    let sensor_data = SensorData {
        timestamp: now_with_utc,
        sensor_id: payload.sensor_id,
        temperature: payload.temperature,
        humidity: payload.humidity,
    };

    dbg!(&sensor_data);

    let result = sqlx::query("INSERT INTO sensor_data VALUES ($1, $2, $3, $4)")
        .bind(sensor_data.timestamp)
        .bind(sensor_data.sensor_id)
        .bind(sensor_data.temperature)
        .bind(sensor_data.humidity)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => println!("ROW inserted"),
        Err(error) => println!("ERROR inserting row: {}", error),
    }
}

async fn get_sensor_data() -> Json<Vec<SensorData>> {
    let file_path = "./data/home_data_28112023.csv";
    let file = OpenOptions::new().read(true).open(file_path).unwrap();

    let mut rdr = csv::Reader::from_reader(file);
    let sensor_data: Result<Vec<SensorData>, csv::Error> = rdr.deserialize().collect();

    match sensor_data {
        Ok(data) => Json(data),
        Err(err) => {
            // Handle error, log, or return appropriate response
            println!("Error reading CSV: {}", err);
            Json(vec![])
        }
    }
}

//fn write_to_csv(sensor_data: &SensorData) -> Result<(), Box<dyn Error>> {
//    let utc: DateTime<Utc> = Utc::now();
//   let mut file = OpenOptions::new()
//        .write(true)
//        .create(true)
//        .append(true)
//        .open("./data/home_data_18112023.csv")
//        .unwrap();
//
//    let mut writer = csv::WriterBuilder::new().from_writer(file);
//
//    Ok(())
// }

#[derive(Serialize, Deserialize, Debug)]
struct UpdateSensor {
    timestamp: Option<String>,
    sensor_id: i32,
    temperature: f32,
    humidity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SensorData {
    timestamp: DateTime<Local>,
    sensor_id: i32,
    temperature: f32,
    humidity: f32,
}

#[derive(Debug, Serialize)]
struct DataObj {
    temperature: f32,
    humidity: f32,
}
