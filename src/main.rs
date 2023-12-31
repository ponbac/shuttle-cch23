use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use days::{day1_both, day4_part1, day4_part2, day6_both, day7_part1, day7_part2, day7_part3};

mod api_error;
mod days;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*numbers", get(day1_both))
        .route("/4/strength", post(day4_part1))
        .route("/4/contest", post(day4_part2))
        .route("/6", post(day6_both))
        .route("/7/decode", get(day7_part1))
        .route("/7/bake", get(day7_part2))
        .route("/7/crazy", get(day7_part3));

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
