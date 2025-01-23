use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub value: Value,
    // TODO add params
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Value {
    #[default]
    Individual,
    Group,
    Org,
    Location,
    // TODO add x-names and IANA tokens
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("InvalidKind")]
    NoValue,
    #[error("InvalidKind")]
    InvalidValue,
}

pub fn validate(
    _params: Option<Vec<(String, Vec<String>)>>,
    value: Option<String>,
) -> Result<Property, Error> {
    let Some(value) = value else {
        return Err(Error::NoValue);
    };

    match value.as_str() {
        "individual" => Ok(Property {
            value: Value::Individual,
        }),
        "group" => Ok(Property {
            value: Value::Individual,
        }),
        "org" => Ok(Property {
            value: Value::Individual,
        }),
        "location" => Ok(Property {
            value: Value::Individual,
        }),
        _ => Err(Error::InvalidValue),
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KIND:{}",
            match self.value {
                Value::Individual => "individual",
                Value::Location => "location",
                Value::Group => "group",
                Value::Org => "org",
            }
        )
    }
}
