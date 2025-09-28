use crate::{
    CARDS_PER_BOOK, CARDS_PER_PAGE,
    card::{Deserialize, Serialize},
    index::Index,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Page(pub usize);

impl Page {
    /// Calculates the absolut page number counting from 0
    pub fn absolut(index: &Index) -> Self {
        Self((index.0 as f32 / CARDS_PER_PAGE as f32).ceil() as usize)
    }

    /// Takes into the maximum cards per book into account and calculates the page relative to each book
    pub fn relative(index: &Index) -> Self {
        let pages = (CARDS_PER_BOOK / CARDS_PER_PAGE) as u16;
        let page = (index.0 as f32 / CARDS_PER_PAGE as f32).ceil() as u16;
        let remainder = page % pages;
        if remainder == 0 {
            Self(CARDS_PER_PAGE)
        } else {
            Self(remainder as usize)
        }
    }
}
