use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

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
        .route("/1/*numbers", get(day1_part2));

    Ok(router.into())
}

async fn day1_part2(Path(numbers): Path<String>) -> impl IntoResponse {
    let xor_sum: u32 = numbers
        .split('/')
        .map(|s| s.parse::<u32>().unwrap())
        .fold(0, |acc, n| acc ^ n);

    format!("{}", xor_sum.pow(3))
}
