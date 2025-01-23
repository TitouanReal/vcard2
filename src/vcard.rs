use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use vec1::Vec1;

use crate::{property::*, unvalidated_vcard::UnvalidatedVCard, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct VCard {
    // General
    pub kind: kind::Property,

    // Identification
    pub fns: Vec1<fn_::Property>,

    // Communications
    pub tels: Vec<tel::Property>,
    pub emails: Vec<email::Property>,
    // Explanatory
    // VERSION: version::Property,
}

impl FromStr for VCard {
    type Err = Error;

    fn from_str(vcard: &str) -> Result<Self, Self::Err> {
        use Error::*;

        let vcard_props: UnvalidatedVCard = vcard.parse()?;

        let kind = match vcard_props.kind.len() {
            0 => kind::Property::default(),
            1 => vcard_props.kind[0].clone(),
            _ => return Err(SecondKind),
        };

        let Ok(fns) = Vec1::try_from_vec(vcard_props.fns) else {
            return Err(MissingFn);
        };

        Ok(VCard {
            kind,
            fns,
            tels: vcard_props.tels,
            emails: vcard_props.emails,
        })
    }
}

impl fmt::Display for VCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vcard = String::from("BEGIN:VCARD\r\n");

        vcard.push_str("VERSION:4.0\r\n");

        vcard.push_str(&format!("{}\r\n", self.kind));

        for fn_ in &self.fns {
            vcard.push_str(&format!("{}\r\n", fn_));
        }

        for tel in &self.tels {
            vcard.push_str(&format!("{}\r\n", tel));
        }

        for email in &self.emails {
            vcard.push_str(&format!("{}\r\n", email));
        }

        vcard.push_str("END:VCARD");

        write!(f, "{}", vcard)
    }
}
