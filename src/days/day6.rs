use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ElfCountResponse {
    elf: usize,
    elf_on_a_shelf: usize,
    shelf_with_no_elf_on_it: usize,
}

pub async fn day6_both(body: String) -> impl IntoResponse {
    println!("Body: {}", body);

    let elf_matches = body.matches("elf").count();
    let elf_on_a_shelf_matches = body.matches("elf on a shelf").count();
    let shelf_with_no_elf_on_it_matches =
        body.matches("shelf").count() - body.matches("elf on a shelf").count();

    let elf_count_response = ElfCountResponse {
        elf: elf_matches - body.matches("shelf").count(),
        elf_on_a_shelf: elf_on_a_shelf_matches,
        shelf_with_no_elf_on_it: shelf_with_no_elf_on_it_matches,
    };
    println!("Response: {:?}", elf_count_response);

    Json(elf_count_response)
}
