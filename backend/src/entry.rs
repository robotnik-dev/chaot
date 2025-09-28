use crate::{
    CARDS_PER_PAGE,
    card::{Deserialize, Serialize},
    index::Index,
    page::Page,
    side::Side,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Entry(pub usize);

impl Entry {
    pub fn new(index: &Index, page_absolut: &Page, side: &Side) -> Self {
        let max_card_no = CARDS_PER_PAGE * page_absolut.0;
        let midpoint = max_card_no - (CARDS_PER_PAGE / 2);
        match side {
            Side::A => {
                if page_absolut.0 == 1 {
                    Self(index.0)
                } else {
                    Self((CARDS_PER_PAGE / 2) - (midpoint % index.0))
                }
            }
            Side::B => Self(index.0 - midpoint),
        }
    }
}
