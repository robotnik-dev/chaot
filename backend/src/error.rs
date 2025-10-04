use crate::card::{Deserialize, Serialize};
use rocket::Request;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        let json = Json(ErrorResponse {
            status: 500,
            message: self.0.to_string(),
        });
        json.respond_to(request)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    status: usize,
    message: String,
}
