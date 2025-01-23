use std::fmt;

use ical::parser::vcard::component::VcardContact;
use serde::{Deserialize, Serialize};

use super::parameter;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub value: String,
    pub pref: Option<parameter::pref::Parameter>,
    pub types: Vec<parameter::tel_type::Parameter>,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("InvalidKind")]
    NoValue,
}

pub fn validate(
    params: Option<Vec<(String, Vec<String>)>>,
    value: Option<String>,
) -> Result<Property, Error> {
    let Some(value) = value else {
        return Err(Error::NoValue);
    };

    let pref = if let Some(params) = &params {
        match parameter::pref::extract(params) {
            Ok(pref) => pref,
            Err(_) => panic!(),
        }
    } else {
        None
    };

    let types = if let Some(params) = &params {
        match parameter::tel_type::extract(params) {
            Ok(types) => types,
            Err(_) => panic!(),
        }
    } else {
        Vec::new()
    };

    Ok(Property { value, pref, types })
}

pub fn extract(vcard: &VcardContact) -> Vec<Property> {
    let tel_properties: Vec<(String, Option<_>)> = vcard
        .clone()
        .properties
        .into_iter()
        .filter(|property| property.name == "TEL")
        .filter_map(|property| property.value.map(|value| (value, property.params)))
        .collect();

    let mut tels = Vec::new();

    for (value, params) in tel_properties {
        let pref = if let Some(params) = &params {
            match parameter::pref::extract(params) {
                Ok(pref) => pref,
                Err(_) => panic!(),
            }
        } else {
            None
        };

        let types = if let Some(params) = &params {
            match parameter::tel_type::extract(params) {
                Ok(types) => types,
                Err(_) => panic!(),
            }
        } else {
            Vec::new()
        };

        tels.push(Property { value, pref, types });
    }

    tels
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut property = String::from("TEL");

        if let Some(pref) = self.pref {
            property.push_str(&format!(";{pref}"));
        }

        for type_ in &self.types {
            property.push_str(&format!(";{type_}"));
        }

        property.push(':');
        property.push_str(&self.value);

        write!(f, "{}", property)
    }
}
