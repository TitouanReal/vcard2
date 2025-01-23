use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Property {
    value: String,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("InvalidKind")]
    NoValue,
}

pub fn validate(
    _params: Option<Vec<(String, Vec<String>)>>,
    value: Option<String>,
) -> Result<Property, Error> {
    match value {
        Some(value) => Ok(Property { value }),
        None => Err(Error::NoValue),
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FN:{}", self.value)
    }
}
