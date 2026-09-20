//! Deterministic locale-correct valedictions for formal correspondence.
//!
//! `cnice::farewell` (previously the `cfarewell` crate): the valediction per locale
//! lives as data in `tables/farewell/closing.json` (see
//! `tables/farewell/README.md` for the resolution rule); `tests/vectors/*.json` is
//! the executable contract every language port runs. The Typst module in
//! `typst/` is generated from the same table.
//!
//! Same input always yields the same output: no models, no I/O.

use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(serde::Deserialize)]
struct ClosingTable {
    locales: HashMap<String, String>,
    fallback: String,
}

static TABLE: LazyLock<ClosingTable> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/farewell/closing.json"))
        .expect("tables/farewell/closing.json is valid")
});

fn base_language(locale: &str) -> &str {
    locale.split('-').next().unwrap_or(locale)
}

/// Resolve a lowercase table key: case-insensitive exact code, base language,
/// then English fallback. All stored and returned locale IDs are lowercase.
fn resolve_key(locale: &str) -> &'static str {
    let lower = locale.to_ascii_lowercase();
    TABLE
        .locales
        .get_key_value(lower.as_str())
        .or_else(|| TABLE.locales.get_key_value(base_language(&lower)))
        .map_or(TABLE.fallback.as_str(), |(key, _)| key.as_str())
}

/// Valediction for a BCP 47 locale. Unknown locales fall back through the
/// base language to English; an explicit override always wins (used for
/// per-workspace closing choices).
#[must_use]
pub fn closing<'a>(locale: &str, override_closing: Option<&'a str>) -> &'a str {
    override_closing.unwrap_or_else(|| {
        TABLE
            .locales
            .get(resolve_key(locale))
            .map_or("", String::as_str)
    })
}

/// BCP 47 locale codes with a valediction entry, sorted.
#[must_use]
pub fn available_locales() -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = TABLE.locales.keys().map(String::as_str).collect();
    codes.sort_unstable();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_schema() {
        assert!(!TABLE.locales.is_empty(), "table must not be empty");
        assert!(
            TABLE.locales.contains_key(TABLE.fallback.as_str()),
            "fallback must be a known locale"
        );
        for key in TABLE.locales.keys() {
            let base = base_language(key);
            assert!(!base.is_empty(), "locale key must not be empty");
            assert_eq!(
                key,
                &key.to_ascii_lowercase(),
                "locale ID must be lowercase: {key:?}"
            );
        }
        for value in TABLE.locales.values() {
            assert!(!value.is_empty(), "valediction must not be empty");
        }
    }
}
