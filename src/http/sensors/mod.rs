use axum::routing::{get, post};
use axum::Router;
use itertools::Itertools;
use uuid::Uuid;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

// use crate::http::extractor::{AuthUser, MaybeAuthUser};
// use crate::http::profiles::Profile;
// use crate::http::types::Timestamptz;
use crate::http::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(root))
        // `POST / update-sensor` goes to `update_sensor`
        .route("/update-sensor", get(update_sensor))
        .route("/sensor-data", get(get_sensor_data))
    // .route("/sensors", get(sensors::list))
    // .route("/sensors/:id", get(sensors::get))
    // .route("/sensors", post(sensors::create))
    // .route("/sensors/:id", delete(sensors::delete))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn update_sensor(
    ctx: State<ApiContext>,
    Json(mut payload): Json<UpdateSensor>,
    // payload: Query<UpdateSensor>,
    // State(pool): State<PgPool>,
) -> Result<Json<SensorData>, E> {
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
        .execute(&ctx.db_pool)
        .await;

    // match result {
    //     Ok(_) => {
    //         println!("ROW inserted");
    //         Ok(Json(sensor_data))
    //     }
    //     Err(error) => {
    //         println!("ERROR inserting row: {}", error);
    //         Err(E::from(error))
    //     }
    // }
    Ok(Json(sensor_data))
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
