use std::num::ParseIntError;

use card::Card;
pub use rocket::{Responder, get, launch, routes, serde::json::Json};

use crate::{index::Index, name::Name};

pub mod book;
pub mod card;
pub mod entry;
pub mod index;
pub mod name;
pub mod page;
pub mod pokeapi;
pub mod side;
pub mod csv_record;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;

#[derive(Debug, Responder)]
pub enum ApplicationError {
    #[response(status = 405, content_type = "json")]
    UnknownLanguage(String),

    #[response(status = 500, content_type = "json")]
    RequestError(String),

    #[response(status = 500, content_type = "json")]
    ConversionError(String),

    #[response(status = 405, content_type = "json")]
    InputError(String),
    
    #[response(status = 404, content_type = "json")]
    NotFound(String),
}

impl From<reqwest::Error> for ApplicationError {
    fn from(value: reqwest::Error) -> Self {
        ApplicationError::RequestError(value.to_string())
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(value: serde_json::Error) -> Self {
        ApplicationError::ConversionError(value.to_string())
    }
}

impl From<ParseIntError> for ApplicationError {
    fn from(value: ParseIntError) -> Self {
        ApplicationError::ConversionError(value.to_string())
    }
}

impl From<csv::Error> for ApplicationError {
    fn from(value: csv::Error) -> Self {
        ApplicationError::ConversionError(value.to_string())
    }
}

#[get("/<id>")]
async fn card_by_id(id: usize) -> Result<Json<Card>, ApplicationError> {
    let index = Index::try_from(id)?;
    let card = Card::try_from_index(index).await?;
    Ok(Json(card))
}

#[get("/<name>/<lan>")]
async fn card_by_name(name: &str, lan: &str) -> Result<Json<Card>, ApplicationError> {
    let name = Name::try_new(name, lan)?;
    let card = Card::try_from_name(name).await?;
    Ok(Json(card))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1/", routes![card_by_id, card_by_name])
}
