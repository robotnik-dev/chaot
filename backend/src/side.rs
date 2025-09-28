use crate::{
    CARDS_PER_PAGE,
    card::{Deserialize, Serialize},
    index::Index,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Side {
    A,
    B,
}

impl From<&Index> for Side {
    fn from(value: &Index) -> Self {
        let rest = (value.0 as f32 / CARDS_PER_PAGE as f32).fract();
        if rest > 0.5 || rest == 0.0 {
            Self::B
        } else {
            Self::A
        }
    }
}
