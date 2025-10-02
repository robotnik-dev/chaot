use std::num::ParseIntError;

use card::Card;
use rocket::serde::Serialize;
pub use rocket::{Responder, get, launch, routes, serde::json::Json};

use crate::{index::Index, name::Name};

pub mod book;
pub mod card;
pub mod csv_record;
pub mod entry;
pub mod index;
pub mod name;
pub mod page;
pub mod pokeapi;
pub mod side;

pub const BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon/";
pub const LANGUAGE_URL: &str = "https://raw.githubusercontent.com/PokeAPI/pokeapi/refs/heads/master/data/v2/csv/pokemon_species_names.csv";
pub const CARDS_PER_BOOK: usize = 576;
pub const CARDS_PER_PAGE: usize = 24;

#[derive(Debug, Clone, Serialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    //TODO: display type of error
    error: String,
}

impl From<&str> for ErrorResponse {
    fn from(value: &str) -> Self {
        Self {
            error: value.to_string(),
        }
    }
}

#[derive(Debug, Responder)]
pub enum ApplicationError {
    #[response(status = 405, content_type = "json")]
    UnknownLanguage(Json<ErrorResponse>),

    #[response(status = 500, content_type = "json")]
    RequestError(Json<ErrorResponse>),

    #[response(status = 500, content_type = "json")]
    ConversionError(Json<ErrorResponse>),

    #[response(status = 405, content_type = "json")]
    InputError(Json<ErrorResponse>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),
}

impl From<reqwest::Error> for ApplicationError {
    fn from(value: reqwest::Error) -> Self {
        ApplicationError::RequestError(Json(ErrorResponse::from(value.to_string().as_str())))
    }
}

impl From<serde_json::Error> for ApplicationError {
    fn from(value: serde_json::Error) -> Self {
        ApplicationError::ConversionError(Json(ErrorResponse::from(value.to_string().as_str())))
    }
}

impl From<ParseIntError> for ApplicationError {
    fn from(value: ParseIntError) -> Self {
        ApplicationError::ConversionError(Json(ErrorResponse::from(value.to_string().as_str())))
    }
}

impl From<csv::Error> for ApplicationError {
    fn from(value: csv::Error) -> Self {
        ApplicationError::ConversionError(Json(ErrorResponse::from(value.to_string().as_str())))
    }
}

#[get("/id/<id>")]
async fn card_by_id(id: usize) -> Result<Json<Card>, ApplicationError> {
    let index = Index::try_from(id)?;
    let card = Card::try_from_index(index).await?;
    Ok(Json(card))
}

#[get("/name/<name>")]
async fn card_by_name(name: &str) -> Result<Json<Card>, ApplicationError> {
    let name = Name::new(name);
    let card = Card::try_from_name(name).await?;
    Ok(Json(card))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1/", routes![card_by_id, card_by_name])
}
