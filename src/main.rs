use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{fs::OpenOptions, time::Duration};

use sqlx::postgres::{PgPool, PgPoolOptions};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await
        .expect("cannot connect to database");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        ////.route("/", get(root))
        // .route(
        //     "/",
        //     get(using_connection_pool_extractor).post(using_connection_extractor),
        // )
        .route("/", get(root))
        // `POST / update-sensor` goes to `update_sensor`
        .route("/update-sensor", get(update_sensor))
        .route("/sensor-data", get(get_sensor_data))
        .with_state(pool);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error` response
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn root() -> &'static str {
    "Hello, World!"
}

// async fn update_sensor(payload: Query<UpdateSensor>) -> Json<SensorData> {
//     let payload: UpdateSensor = payload.0;
//     let now_with_utc: DateTime<Local> = Local::now();
//     let sensor_response = SensorData {
//         timestamp: now_with_utc,
//         sensor_id: payload.sensor_id,
//         temperature: payload.temperature,
//         humidity: payload.humidity,
//     };
//
//     let file_name_prefix = Local::now().format("%d%m%Y");
//     let file_path = format!("./data/home_data_{:}.csv", file_name_prefix);
//
//     let file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .append(true)
//         .open(file_path)
//         .unwrap();
//
//     let mut writer = csv::WriterBuilder::new()
//         .has_headers(false)
//         .from_writer(file);
//     writer.serialize(&sensor_response).unwrap();
//     writer.flush().unwrap();
//
//     Json(sensor_response)
// }

async fn update_sensor(payload: Query<UpdateSensor>, State(pool): State<PgPool>) {
    let payload: UpdateSensor = payload.0;
    let now_with_utc: DateTime<Local> = Local::now();

    dbg!(&payload);

    // let date_str = "2020-04-12T22:10:57+02:00";
    // convert the string into DateTime<FixedOffset>
    // let datetime = DateTime::parse_from_rfc3339(&payload.timestamp).unwrap();
    // convert the string into DateTime<Utc> or other timezone
    // let datetime_utc = datetime.with_timezone(&Local);

    // let yeet = DateTime::parse_from_rfc3339(&payload.timestamp).unwrap_or_else(|e| panic!("Failed to parse"));
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
