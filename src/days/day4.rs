use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

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

pub struct GigadeerStats {
    fastest: Gigadeer,
    tallest: Gigadeer,
    magician: Gigadeer,
    consumer: Gigadeer,
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
        fastest: fastest.clone(),
        tallest: tallest.clone(),
        magician: magician.clone(),
        consumer: consumer.clone(),
    }))
}

impl Serialize for GigadeerStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GigadeerStats", 4)?;
        state.serialize_field(
            "fastest",
            &format!(
                "Speeding past the finish line with a strength of {} is {}",
                self.fastest.strength, self.fastest.name
            ),
        )?;
        state.serialize_field(
            "tallest",
            &format!(
                "{} is standing tall with his {} cm wide antlers",
                self.tallest.name, self.tallest.antler_width
            ),
        )?;
        state.serialize_field(
            "magician",
            &format!(
                "{} could blast you away with a snow magic power of {}",
                self.magician.name, self.magician.snow_magic_power
            ),
        )?;
        state.serialize_field(
            "consumer",
            &format!(
                "{} ate lots of candies, but also some {}",
                self.consumer.name, self.consumer.favorite_food
            ),
        )?;
        state.end()
    }
}
