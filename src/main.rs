use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

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
        .route("/update-sensor", get(update_sensor));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

async fn update_sensor(payload: Query<UpdateSensor>) -> Json<SensorData> {
    let payload: UpdateSensor = payload.0;
    let now_with_utc: DateTime<Local> = Local::now();
    let sensor_response = SensorData {
        timestamp: now_with_utc,
        sensor_id: payload.sensor_id,
        temperature: payload.temperature,
        humidity: payload.humidity,
    };

    let file_name_prefix = Local::now().format("%d%m%Y");
    let file_path = format!("./data/home_data_{:}.csv", file_name_prefix);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    writer.serialize(&sensor_response).unwrap();
    writer.flush().unwrap();

    Json(sensor_response)
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

#[derive(Deserialize, Debug)]
struct UpdateSensor {
    sensor_id: String,
    temperature: f32,
    humidity: f32,
}

#[derive(Debug, Serialize)]
struct SensorData {
    timestamp: DateTime<Local>,
    sensor_id: String,
    temperature: f32,
    humidity: f32,
}

#[derive(Debug, Serialize)]
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
