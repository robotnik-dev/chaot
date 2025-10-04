use card::Card;
pub use rocket::{Responder, get, launch, routes, serde::json::Json};

use crate::{error::Result, index::Index, name::Name};

pub mod book;
pub mod card;
pub mod csv_record;
pub mod entry;
pub mod error;
pub mod index;
pub mod name;
pub mod page;
pub mod pokeapi;
pub mod side;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;

#[get("/id/<id>")]
pub async fn card_by_id(id: usize) -> Result<Json<Card>> {
    let index = Index::try_new(id)?;
    let card = Card::try_from_index(index).await?;
    Ok(Json(card))
}

#[get("/name/<name>")]
pub async fn card_by_name(name: &str) -> Result<Json<Card>> {
    let name = Name::new(name);
    let card = Card::try_from_name(name).await?;
    Ok(Json(card))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1/", routes![card_by_id, card_by_name])
}
