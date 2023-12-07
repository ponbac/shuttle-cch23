use std::{cmp, collections::HashMap};

use axum::{response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    flour: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
}

pub async fn day7_part1(jar: CookieJar) -> impl IntoResponse {
    let encoded_recipe = jar
        .get("recipe")
        .unwrap_or_else(|| panic!("No recipe found in cookie jar!"))
        .value()
        .to_string();

    println!("Encoded recipe: {}", encoded_recipe);

    let recipe = general_purpose::STANDARD
        .decode(encoded_recipe.as_bytes())
        .unwrap()
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>();

    println!("Decoded recipe: {}", recipe);

    let recipe: Recipe = serde_json::from_str(&recipe).unwrap();

    Json(recipe)
}

#[derive(Debug, Serialize, Deserialize)]
struct SetOfIngredients {
    flour: usize,
    sugar: usize,
    butter: usize,
    #[serde(rename = "baking powder")]
    baking_powder: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CookingRequest {
    recipe: SetOfIngredients,
    pantry: SetOfIngredients,
}

#[derive(Debug, Serialize, Deserialize)]
struct CookingResponse {
    cookies: usize,
    pantry: SetOfIngredients,
}

pub async fn day7_part2(jar: CookieJar) -> impl IntoResponse {
    let encoded_recipe = jar
        .get("recipe")
        .unwrap_or_else(|| panic!("No recipe found in cookie jar!"))
        .value()
        .to_string();

    println!("Encoded recipe: {}", encoded_recipe);

    let recipe = general_purpose::STANDARD
        .decode(encoded_recipe.as_bytes())
        .unwrap()
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>();

    println!("Decoded recipe: {}", recipe);

    let request: CookingRequest = serde_json::from_str(&recipe).unwrap();
    let recipe = request.recipe;
    let mut pantry = request.pantry;

    let n_cookies = cmp::min(
        cmp::min(
            cmp::min(
                cmp::min(pantry.flour / recipe.flour, pantry.sugar / recipe.sugar),
                pantry.butter / recipe.butter,
            ),
            pantry.baking_powder / recipe.baking_powder,
        ),
        pantry.chocolate_chips / recipe.chocolate_chips,
    );

    pantry.flour -= n_cookies * recipe.flour;
    pantry.sugar -= n_cookies * recipe.sugar;
    pantry.butter -= n_cookies * recipe.butter;
    pantry.baking_powder -= n_cookies * recipe.baking_powder;
    pantry.chocolate_chips -= n_cookies * recipe.chocolate_chips;

    Json(CookingResponse {
        cookies: n_cookies,
        pantry,
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct CrazyRequest {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrazyResponse {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

pub async fn day7_part3(jar: CookieJar) -> impl IntoResponse {
    let encoded_recipe = jar
        .get("recipe")
        .unwrap_or_else(|| panic!("No recipe found in cookie jar!"))
        .value()
        .to_string();
    println!("Encoded recipe: {}", encoded_recipe);

    let recipe = general_purpose::STANDARD
        .decode(encoded_recipe.as_bytes())
        .unwrap()
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>();
    println!("Decoded recipe: {}", recipe);

    let request: CrazyRequest = serde_json::from_str(&recipe).unwrap();

    let mut n_cookies = 0;
    let mut pantry = request.pantry.clone();

    while request.recipe.iter().all(|(ingredient, amount)| {
        pantry
            .get(ingredient)
            .map_or(false, |&available| available >= *amount)
    }) {
        n_cookies += 1;

        for (ingredient, amount) in request.recipe.iter() {
            pantry
                .entry(ingredient.clone())
                .and_modify(|e| *e -= amount);
        }
    }

    Json(CrazyResponse {
        cookies: n_cookies,
        pantry,
    })
}
