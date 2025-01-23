use std::fs;

use vcard4::VCard;

#[test]
fn reading() {
    for path in fs::read_dir("tests/data/valid").unwrap() {
        let path = path.unwrap().path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "vcf") {
            let mut json_path = path.clone();
            json_path.set_extension("json");

            let vcard = fs::read_to_string(path).unwrap();
            let json = fs::read_to_string(json_path).unwrap();

            let left: VCard = vcard.parse().unwrap();
            println!("{}", serde_json::to_string(&left).unwrap());
            let right: VCard = serde_json::from_str(&json).unwrap();

            assert_eq!(left, right);
        }
    }
}
