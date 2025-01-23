use std::{io::BufReader, str::FromStr};

use crate::{property::*, Error};

pub struct UnvalidatedVCard {
    // General
    pub kind: Vec<kind::Property>,

    // Identification
    pub fns: Vec<fn_::Property>,

    // Communications
    pub tels: Vec<tel::Property>,
    pub emails: Vec<email::Property>,
    // Explanatory
    // VERSION: version::Property,
}

impl FromStr for UnvalidatedVCard {
    type Err = Error;

    fn from_str(vcard: &str) -> Result<Self, Self::Err> {
        use Error::*;

        let buf = BufReader::new(vcard.as_bytes());
        let mut properties = ical::PropertyParser::new(ical::LineReader::new(buf));

        match properties.next() {
            None => Err(MissingBegin),
            Some(Err(e)) => Err(PropertyError(e)),
            Some(Ok(property)) => {
                if property.name == "BEGIN"
                    && property.value.map(|s| &s == "VCARD").unwrap_or_default()
                {
                    if property.params.is_some() {
                        Err(InvalidParameter)
                    } else {
                        Ok(())
                    }
                } else {
                    Err(MissingBegin)
                }
            }
        }?;

        match properties.next() {
            None => Err(MissingVersion),
            Some(Err(e)) => Err(PropertyError(e)),
            Some(Ok(property)) => {
                if property.name == "VERSION" {
                    if let Some(value) = property.value {
                        match value.as_str() {
                            "4.0" => Ok(()),
                            "3.0" => Err(UnsupportedVersion(version::Value::V3_0)),
                            "2.1" => Err(UnsupportedVersion(version::Value::V2_1)),
                            _ => Err(UnknownVersion(value)),
                        }
                    } else {
                        Err(MissingValueOfVersion)
                    }
                } else {
                    Err(MissingVersion)
                }
            }
        }?;

        // General
        let mut kind = Vec::new();

        // Identification
        let mut fns = Vec::new();

        // Communications
        let mut tels = Vec::new();
        let mut emails = Vec::new();

        let mut encountered_end = false;

        for property_result in properties {
            if encountered_end {
                return Err(PropertyAfterEnd);
            }

            let property = match property_result {
                Ok(property) => property,
                Err(e) => return Err(PropertyError(e)),
            };

            match property.name.as_str() {
                "BEGIN" => return Err(SecondBegin),
                "VERSION" => return Err(SecondVersion),
                "KIND" => kind.push(kind::validate(property.params, property.value)?),
                "FN" => fns.push(fn_::validate(property.params, property.value)?),
                "TEL" => tels.push(tel::validate(property.params, property.value).unwrap()),
                "EMAIL" => emails.push(email::validate(property.params, property.value).unwrap()),
                "END" => encountered_end = true,
                _ => return Err(InvalidPropertyName),
            }
        }

        if !encountered_end {
            return Err(MissingEnd);
        }

        Ok(Self {
            kind,
            fns,
            tels,
            emails,
        })
    }
}
