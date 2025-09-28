use crate::{
    CARDS_PER_BOOK,
    card::{Deserialize, Serialize},
    index::Index,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Book(pub usize);

impl From<&Index> for Book {
    fn from(value: &Index) -> Self {
        Book((value.0 as f32 / CARDS_PER_BOOK as f32).ceil() as usize)
    }
}
