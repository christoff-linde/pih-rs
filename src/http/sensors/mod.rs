use axum::routing::{get, post};
use axum::Router;

use crate::http::Result;
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::prelude::*;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::http::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/", get(root)).route(
        "/api/sensors",
        post(create_sensor_reading).get(list_sensor_readings),
    )
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_sensor_reading(
    ctx: State<ApiContext>,
    Json(payload): Json<SensorReading>,
) -> Result<Json<SensorReading>> {
    let payload: SensorReading = payload;
    let now_with_utc: DateTime<Local> = Local::now();

    let result = sqlx::query_as!(
        SensorReadingFromQuery,
        // language=PostgreSQL
        r#"WITH inserted_reading as (
            INSERT INTO sensor_data (time, sensor_id, temperature, humidity)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        )
        SELECT inserted_reading.*
        FROM inserted_reading
        "#,
        now_with_utc,
        payload.sensor_id,
        payload.temperature,
        payload.humidity,
    )
    .fetch_one(&ctx.db_pool)
    .await;

    match result {
        Ok(_) => {
            println!("ROW inserted");
        }
        Err(error) => {
            println!("ERROR inserting row: {}", error);
        }
    }

    Ok(Json(SensorReading {
        time: Some(now_with_utc),
        sensor_id: payload.sensor_id,
        temperature: payload.temperature,
        humidity: payload.humidity,
    }))
}

#[derive(Debug, Deserialize, Default)]
struct ListSensorReadingsQuery {
    sensor_id: Option<i32>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Serialize)]
struct MultipleSensorReadings {
    sensor_readings: Vec<SensorReading>,
}

async fn list_sensor_readings(
    ctx: State<ApiContext>,
    query: Query<ListSensorReadingsQuery>,
) -> Result<Json<MultipleSensorReadings>> {
    let sensor_readings: Vec<_> = sqlx::query_as!(
        SensorReadingFromQuery,
        r#"
            SELECT time, sensor_id, temperature, humidity
            FROM sensor_data
            WHERE ($1::TIMESTAMPTZ IS NULL OR time >= $1)
              AND ($2::TIMESTAMPTZ IS NULL OR time <= $2)
              AND ($3::INT IS NULL OR sensor_id = $3)
            ORDER BY time
            LIMIT $4
            OFFSET $5
        "#,
        query.start_time,
        query.end_time,
        query.sensor_id,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0),
    )
    .fetch(&ctx.db_pool)
    .map_ok(SensorReadingFromQuery::into_sensor_reading)
    .try_collect()
    .await?;

    Ok(Json(MultipleSensorReadings { sensor_readings }))
}

#[derive(Debug, Serialize, Deserialize)]
struct SensorReading {
    time: Option<DateTime<Local>>,
    sensor_id: i32,
    temperature: f64,
    humidity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SensorReadingFromQuery {
    time: DateTime<Local>,
    sensor_id: Option<i32>,
    temperature: Option<f64>,
    humidity: Option<f64>,
}

impl SensorReadingFromQuery {
    fn into_sensor_reading(self) -> SensorReading {
        SensorReading {
            time: Some(self.time),
            sensor_id: self.sensor_id.unwrap(),
            temperature: self.temperature.unwrap(),
            humidity: self.humidity.unwrap(),
        }
    }
}
