use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f32, net::SocketAddr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // `POST / update-sensor` goes to `update_sensor`
        .route("/update-sensor", post(update_sensor));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn update_sensor(Json(payload): Json<UpdateSensor>) -> (StatusCode, Json<SensorData>) {
    let utc: DateTime<Utc> = Utc::now();
    let sensor_data = DataObj {
        temperature: payload.temperature,
        humidity: payload.humidity,
    };
    let sensor_response = SensorData {
        timestamp: utc,
        sensor_id: payload.sensor_id,
        data: sensor_data,
    };

    (StatusCode::CREATED, Json(sensor_response))
}

#[derive(Deserialize)]
struct UpdateSensor {
    sensor_id: String,
    temperature: f32,
    humidity: f32,
}

#[derive(Serialize)]
struct SensorData {
    timestamp: DateTime<Utc>,
    sensor_id: String,
    data: DataObj,
}

#[derive(Serialize)]
struct DataObj {
    temperature: f32,
    humidity: f32,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
