use std::fs;

use vcard4::{property::version, Error, VCard};

#[test]
fn missing_begin() {
    let content = fs::read_to_string("tests/data/invalid/missing_begin.vcf").unwrap();

    let vcard: Result<VCard, _> = content.parse();

    assert_eq!(vcard, Err(Error::MissingBegin));
}

#[test]
fn missing_end() {
    let content = fs::read_to_string("tests/data/invalid/missing_end.vcf").unwrap();

    let vcard: Result<VCard, _> = content.parse();

    assert_eq!(vcard, Err(Error::MissingEnd));
}

#[test]
fn missing_fn() {
    let content = fs::read_to_string("tests/data/invalid/missing_fn.vcf").unwrap();

    let vcard: Result<VCard, _> = content.parse();

    assert_eq!(vcard, Err(Error::MissingFn));
}

#[test]
fn missing_version() {
    let content = fs::read_to_string("tests/data/invalid/missing_version.vcf").unwrap();

    let vcard: Result<VCard, _> = content.parse();

    assert_eq!(vcard, Err(Error::MissingVersion));
}

#[test]
fn version_3() {
    let content = fs::read_to_string("tests/data/invalid/version_3.vcf").unwrap();

    let vcard: Result<VCard, _> = content.parse();

    assert_eq!(vcard, Err(Error::UnsupportedVersion(version::Value::V3_0)));
}
