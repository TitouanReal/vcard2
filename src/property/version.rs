use std::fmt;

use ical::parser::vcard::component::VcardContact;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Property {
    value: Value,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Value {
    V2_1,
    V3_0,
    V4_0,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("NoVersionProperty")]
    NoVersionProperty,
    #[error("MultipleVersionProperty")]
    MultipleVersionProperty,
    #[error("VersionPropertyHasNoValue")]
    VersionPropertyHasNoValue,
    #[error("UnsupportedVersion")]
    UnsupportedVersion(Value),
    #[error("UnknownVersion")]
    UnknownVersion(String),
}

pub fn extract(vcard: &VcardContact) -> Result<Property, Error> {
    let versions: Vec<ical::property::Property> = vcard
        .clone()
        .properties
        .into_iter()
        .filter(|property| property.name == "VERSION")
        .collect();

    match versions.len() {
        0 => Err(Error::NoVersionProperty),
        2.. => Err(Error::MultipleVersionProperty),
        1 => match versions[0].clone().value {
            None => Err(Error::VersionPropertyHasNoValue),
            Some(value) => match value.as_str() {
                "2.1" => Err(Error::UnsupportedVersion(Value::V2_1)),
                "3.0" => Err(Error::UnsupportedVersion(Value::V3_0)),
                "4.0" => Ok(Property { value: Value::V4_0 }),
                other => Err(Error::UnknownVersion(other.to_owned())),
            },
        },
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VERSION:{}",
            match self.value {
                Value::V2_1 => "2.1".to_owned(),
                Value::V3_0 => "3.0".to_owned(),
                Value::V4_0 => "4.0".to_owned(),
            }
        )
    }
}
