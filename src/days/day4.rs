use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}

pub async fn day4_part1(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    format!("{}", reindeers.iter().map(|r| r.strength).sum::<u32>())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gigadeer {
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
pub struct GigadeerStats {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn day4_part2(
    Json(gigadeers): Json<Vec<Gigadeer>>,
) -> Result<Json<GigadeerStats>, ApiError> {
    if gigadeers.is_empty() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "No gigadeers provided",
        ));
    }

    let find_max_by = |f: fn(&Gigadeer) -> i32| gigadeers.iter().max_by_key(|g| f(g)).unwrap();

    let fastest = find_max_by(|g| g.speed as i32);
    let tallest = find_max_by(|g| g.height);
    let magician = find_max_by(|g| g.snow_magic_power);
    let consumer = find_max_by(|g| g.candies_eaten_yesterday);

    Ok(Json(GigadeerStats {
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
    }))
}
