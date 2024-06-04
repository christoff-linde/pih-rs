use axum::routing::{get, post};
use axum::Router;

use crate::http::Result;
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::http::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", get(root))
        .route("/api/sensors", get(create_reading))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_reading(
    ctx: State<ApiContext>,
    query: Query<SensorReading>,
) -> Result<Json<SensorReading>> {
    let now_with_utc: DateTime<Local> = Local::now();

    dbg!(&query);

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
        query.sensor_id,
        query.temperature,
        query.humidity,
    )
    .fetch_one(&ctx.db_pool)
    .await;

    match result {
        Ok(_) => {
            tracing::debug!("Successfully inserted row: {:?}", &result);
        }
        Err(error) => {
            tracing::error!("Error inserting row: {:?}", error);
        }
    }

    Ok(Json(SensorReading {
        time: Some(now_with_utc),
        sensor_id: query.sensor_id,
        temperature: query.temperature,
        humidity: query.humidity,
    }))
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
            tracing::debug!("Successfully inserted row: {:?}", &result);
        }
        Err(error) => {
            tracing::error!("Error inserting row: {:?}", error);
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
