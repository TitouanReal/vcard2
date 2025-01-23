use std::fmt;

use ical::parser::vcard::component::VcardContact;
use serde::{Deserialize, Serialize};

use super::parameter::pref;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Property {
    pub value: String,
    pub pref: Option<pref::Parameter>,
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

    let pref = if let Some(params) = params {
        match pref::extract(&params) {
            Ok(pref) => pref,
            Err(_) => panic!(),
        }
    } else {
        None
    };

    Ok(Property { value, pref })
}

pub fn extract(vcard: &VcardContact) -> Vec<Property> {
    let email_properties: Vec<ical::property::Property> = vcard
        .clone()
        .properties
        .into_iter()
        .filter(|property| property.name == "EMAIL")
        .collect();

    let mut emails = Vec::new();

    for property in email_properties {
        if let Some(value) = property.value {
            let pref = if let Some(params) = property.params {
                match pref::extract(&params) {
                    Ok(pref) => pref,
                    Err(_) => panic!(),
                }
            } else {
                None
            };
            emails.push(Property { value, pref });
        }
    }

    emails
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut property = String::from("EMAIL");

        if let Some(pref) = self.pref {
            property.push_str(&format!(";{}", pref));
        }

        property.push(':');
        property.push_str(&self.value);

        write!(f, "{}", property)
    }
}
