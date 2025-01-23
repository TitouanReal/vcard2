use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter(u8);

pub enum Error {
    InvalidRange,
    NotANumber,
    MultiplePrefParams,
    NoValue,
}

impl TryFrom<u8> for Parameter {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100 {
            Ok(Self(value))
        } else {
            Err(Self::Error::InvalidRange)
        }
    }
}

impl TryFrom<&str> for Parameter {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = value.parse() {
            if value <= 100 {
                Ok(Self(value))
            } else {
                Err(Self::Error::InvalidRange)
            }
        } else {
            Err(Error::NotANumber)
        }
    }
}

pub fn extract(params: &[(String, Vec<String>)]) -> Result<Option<Parameter>, Error> {
    let pref_params: Vec<(String, Vec<String>)> = params
        .to_owned()
        .clone()
        .into_iter()
        .filter(|param| param.0 == "PREF")
        .collect();
    match pref_params.len() {
        0 => Ok(None),
        1 => match pref_params[0].1.len() {
            0 => Err(Error::NoValue),
            1 => match pref_params[0].1[0].as_str().try_into() {
                Ok(pref) => Ok(Some(pref)),
                Err(e) => Err(e),
            },
            2.. => Err(Error::MultiplePrefParams),
        },
        2.. => Err(Error::MultiplePrefParams),
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PREF={}", self.0)
    }
}
