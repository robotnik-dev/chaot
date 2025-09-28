use crate::{
    ApplicationError,
    card::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Name {
    En(String),
    De(String),
}

impl Name {
    pub fn try_new(name: &str, lan: &str) -> Result<Self, ApplicationError> {
        match lan {
            "de" => Ok(Self::De(name.to_string())),
            "en" => Ok(Self::En(name.to_string())),
            _ => Err(ApplicationError::UnknownLanguage(lan.to_string())),
        }
    }
}
