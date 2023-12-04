use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*numbers", get(day1_part2))
        .route("/4/strength", post(day4_part1));

    Ok(router.into())
}

async fn day1_part2(Path(numbers): Path<String>) -> impl IntoResponse {
    println!("numbers: {}", numbers);

    let xor_sum: i64 = numbers
        .split('/')
        .map(|s| s.parse::<i64>().unwrap())
        .fold(0, |acc, n| acc ^ n);

    format!("{}", xor_sum.pow(3))
}

#[derive(Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
}

async fn day4_part1(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    format!("{}", reindeers.iter().map(|r| r.strength).sum::<u32>())
}
