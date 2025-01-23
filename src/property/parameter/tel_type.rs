use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Parameter {
    Work,
    Home,
    Text,
    Voice,
    Fax,
    Cell,
    Video,
    Pager,
    TextPhone,
}

pub enum Error {
    Invalid(String),
}

impl TryFrom<&str> for Parameter {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "work" => Ok(Self::Work),
            "home" => Ok(Self::Home),
            "text" => Ok(Self::Text),
            "voice" => Ok(Self::Voice),
            "fax" => Ok(Self::Fax),
            "cell" => Ok(Self::Cell),
            "video" => Ok(Self::Video),
            "pager" => Ok(Self::Pager),
            "textphone" => Ok(Self::TextPhone),
            other => Err(Error::Invalid(other.to_owned())),
        }
    }
}

pub fn extract(params: &[(String, Vec<String>)]) -> Result<Vec<Parameter>, Error> {
    params
        .iter()
        .filter(|param| param.0 == "TYPE")
        .flat_map(|param| param.1.to_owned())
        .map(|s| s.as_str().try_into())
        .collect()
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TYPE={}", format!("{self:?}").to_lowercase())
    }
}
