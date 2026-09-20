//! Combined conformance suite: every `tests/vectors/*.json` vector runs
//! against `cnice::greet` (uniform salutation renderer, one matcher for
//! every language) or `cnice::farewell` (locale closings).

use cnice::{farewell, greet};

fn run_vector(file: &std::path::Path, vector: &serde_json::Value) {
    let name = vector["name"].as_str().unwrap_or("<unnamed>");
    let context = format!("{} :: {name}", file.display());
    let input = vector["input"].as_str().unwrap_or_default();
    let actual: serde_json::Value = match vector["fn"].as_str().unwrap_or("") {
        // cnice.greet: uniform matcher, locale always explicit
        "salutation" => {
            let locale = vector["locale"]
                .as_str()
                .expect("salutation vector needs locale");
            serde_json::Value::String(greet::salutation(locale, input))
        }
        "salutation_last_name" => {
            serde_json::Value::String(greet::salutation_last_name(input).to_owned())
        }
        "salutation_honorific" => {
            let locale = vector["locale"]
                .as_str()
                .expect("honorific vector needs locale");
            serde_json::Value::String(greet::salutation_honorific(locale, input).to_owned())
        }
        "salutation_surname" => {
            let locale = vector["locale"]
                .as_str()
                .expect("surname vector needs locale");
            serde_json::Value::String(greet::salutation_surname(locale, input).to_owned())
        }
        "salutation_titles" => {
            let locale = vector["locale"]
                .as_str()
                .expect("titles vector needs locale");
            serde_json::Value::Array(
                greet::salutation_titles(locale, input)
                    .iter()
                    .map(|title| serde_json::Value::String((*title).to_owned()))
                    .collect(),
            )
        }
        "recipient_salutation_warning" => {
            let location = vector["location"]
                .as_str()
                .expect("warning vector needs location");
            greet::recipient_salutation_warning(location, input)
                .map_or(serde_json::Value::Null, serde_json::Value::String)
        }
        "honorific_warning" => {
            let location = vector["location"]
                .as_str()
                .expect("warning vector needs location");
            let locale = vector["locale"]
                .as_str()
                .expect("warning vector needs locale");
            greet::honorific_warning(location, locale, input)
                .map_or(serde_json::Value::Null, serde_json::Value::String)
        }
        "is_supported" => {
            let locale = vector["locale"]
                .as_str()
                .expect("is_supported vector needs locale");
            serde_json::Value::Bool(greet::is_supported(locale))
        }
        // cnice.farewell (was cfarewell)
        "available_locales" => serde_json::to_value(farewell::available_locales()).unwrap(),
        "closing" => {
            let locale = vector["locale"]
                .as_str()
                .expect("closing vector needs locale");
            let override_closing = vector.get("override").and_then(serde_json::Value::as_str);
            serde_json::Value::String(farewell::closing(locale, override_closing).to_owned())
        }
        other => panic!("{context}: unknown fn {other:?}"),
    };
    let expected = vector
        .get("expected")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    assert_eq!(actual, expected, "{context}");
}

#[test]
fn conformance_vectors() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .expect("tests/vectors exists")
        .map(|entry| entry.expect("readable entry").path())
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no vector files in tests/vectors");
    let mut count = 0;
    for file in &files {
        let raw = std::fs::read_to_string(file).expect("vector file is readable");
        let vectors: Vec<serde_json::Value> =
            serde_json::from_str(&raw).expect("vector file is valid JSON");
        for vector in &vectors {
            run_vector(file, vector);
            count += 1;
        }
    }
    // greet salutation + parsers + warnings + policy, farewell closings.
    assert_eq!(count, 189, "combined vector suite changed size");
}
