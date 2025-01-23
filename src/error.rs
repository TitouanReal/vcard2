pub use crate::property::{fn_, kind, version};

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parser")]
    PropertyError(#[from] ical::property::PropertyError),

    #[error("Missing header for vcard")]
    MissingBegin,
    #[error("Missing header for vcard")]
    MissingVersion,
    #[error("Missing header for vcard")]
    MissingValueOfVersion,
    #[error("Missing header for vcard")]
    UnsupportedVersion(version::Value),
    #[error("Missing header for vcard")]
    UnknownVersion(String),
    #[error("Missing header for vcard")]
    MissingFn,
    #[error("Missing header for vcard")]
    SecondBegin,
    #[error("Missing header for vcard")]
    SecondVersion,
    #[error("Missing header for vcard")]
    SecondKind,
    #[error("Missing header for vcard")]
    InvalidPropertyName,
    #[error("Missing header for vcard")]
    InvalidParameter,
    #[error("Missing header for vcard")]
    PropertyAfterEnd,
    #[error("Missing header for vcard")]
    MissingEnd,

    #[error("NoVcardInString")]
    NoVcardInString,
    #[error("MultipleVCardsInString")]
    MultipleVCardsInString,

    // Properties
    #[error("version")]
    VersionError(#[from] version::Error),
    #[error("kind")]
    Kind(#[from] kind::Error),
    #[error("fn")]
    FNError(#[from] fn_::Error),
}
