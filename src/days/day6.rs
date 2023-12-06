use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ElfCountResponse {
    elf: usize,
    elf_on_a_shelf: usize,
    shelf_with_no_elf_on_it: usize,
}

// The number of shelves that don't have an elf on it. That is, the number of strings "shelf" that are not preceded by the string "elf on a ". Put this count in the field "shelf with no elf on it".
pub async fn day6_both(body: String) -> impl IntoResponse {
    Json(ElfCountResponse {
        elf: body.matches("elf").count(),
        elf_on_a_shelf: body.matches("elf on a shelf").count(),
        shelf_with_no_elf_on_it: body.matches("shelf").count()
            - body.matches("elf on a shelf").count(),
    })
}
