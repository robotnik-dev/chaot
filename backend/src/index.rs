use std::fmt::Display;

use anyhow::anyhow;

use crate::{
    card::{Deserialize, Serialize},
    error::Result,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Index(pub usize);

impl Index {
    pub fn try_new(index: usize) -> Result<Self> {
        if index == 0 {
            Err(anyhow!("Provided Card ID: {index} can't be lower than 1").into())
        } else {
            Ok(Self(index))
        }
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}
