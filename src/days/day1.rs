use axum::{extract::Path, response::IntoResponse};

pub async fn day1_both(Path(numbers): Path<String>) -> impl IntoResponse {
    println!("numbers: {}", numbers);

    let xor_sum: i64 = numbers
        .split('/')
        .map(|s| s.parse::<i64>().unwrap())
        .fold(0, |acc, n| acc ^ n);

    format!("{}", xor_sum.pow(3))
}
