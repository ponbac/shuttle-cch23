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
        .route("/4/strength", post(day4_part1))
        .route("/4/contest", post(day4_part2));

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

#[derive(Serialize, Deserialize, Clone)]
struct Gigadeer {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
struct GigadeerStats {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn day4_part2(Json(gigadeers): Json<Vec<Gigadeer>>) -> impl IntoResponse {
    let fastest = gigadeers
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();
    let tallest = gigadeers
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .unwrap();
    let magician = gigadeers
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .unwrap();
    let consumer = gigadeers
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .unwrap();

    Json(GigadeerStats {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}
