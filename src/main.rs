use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
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

    sqlx::migrate!().run(&pool).await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST / update-sensor` goes to `update_sensor`
        .route("/update-sensor", get(update_sensor))
        .route("/sensor-data", get(get_sensor_data))
        .with_state(pool);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
