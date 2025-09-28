use crate::{
    ApplicationError,
    card::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Index(pub usize);

impl TryFrom<usize> for Index {
    type Error = ApplicationError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(ApplicationError::InputError(
                "Index should not be lower than 1".into(),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
