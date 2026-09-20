//! Deterministic locale-correct salutations for formal correspondence.
//!
//! One matcher for every language: each locale row in
//! `tables/salutation.json` carries its honorifics, academic titles,
//! name templates, comma rule and formal fallback; the code below only
//! resolves the locale, matches normalized tokens and fills templates.
//! There is deliberately no per-language branch — a new language is a new
//! table row, never new code. `tests/vectors/*.json` is the executable
//! contract every language port runs. The Typst module in `typst/greet.typ`
//! reads the same canonical table.
//!
//! Rules are sourced from the national correspondence norms recorded per
//! row (`source_norms`). Rows marked `"review": "native-pending"` pin
//! provisional bytes for determinism; their correctness still awaits
//! native review (see `tables/README.md`). Same input always yields the
//! same output: no models, no I/O.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

/// Canonical display form and gender group of one honorific token.
#[derive(serde::Deserialize)]
struct Honorific {
    group: String,
    display: String,
}

/// Salutation rules of one locale: token tables plus the templates the
/// generic matcher fills. Template slots are `{honorific}`, `{titles}`
/// and `{surname}`; empty parts collapse to single spaces.
#[derive(serde::Deserialize)]
struct SalutationEntry {
    honorifics: HashMap<String, Honorific>,
    titles: HashMap<String, String>,
    sole_titles: Vec<String>,
    filler: HashSet<String>,
    named: HashMap<String, String>,
    formal: String,
    comma: bool,
    /// Provenance metadata for readers and reviewers; enforced by the
    /// schema test, not read at runtime.
    #[allow(dead_code)]
    #[serde(default)]
    review: Option<String>,
    /// Provenance metadata for readers and reviewers; enforced by the
    /// schema test, not read at runtime.
    #[allow(dead_code)]
    source_norms: String,
}

#[derive(serde::Deserialize)]
struct SalutationFile {
    locales: HashMap<String, SalutationEntry>,
    fallback: String,
}

static SALUTATION: LazyLock<SalutationFile> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/salutation.json"))
        .expect("tables/salutation.json is valid")
});

/// Normalize one whitespace-separated token for table lookup: strip
/// leading/trailing `.`, lowercase ASCII A–Z only. Must stay in sync with
/// `tables/README.md` — every port implements this exact normalization.
fn norm(token: &str) -> String {
    token.trim_matches('.').to_ascii_lowercase()
}

fn base_language(code: &str) -> &str {
    code.split('-').next().unwrap_or(code)
}

/// Resolve a lowercase table key: case-insensitive exact code, base
/// language, then English fallback. All stored locale IDs are lowercase.
fn resolve_key(locale: &str) -> &'static str {
    let lower = locale.to_ascii_lowercase();
    SALUTATION
        .locales
        .get_key_value(lower.as_str())
        .or_else(|| SALUTATION.locales.get_key_value(base_language(&lower)))
        .map_or(SALUTATION.fallback.as_str(), |(key, _)| key.as_str())
}

/// Whether a locale code has a salutation row: present directly or through
/// its (lowercased) base language. Consumers use this predicate instead of
/// any language list in code.
#[must_use]
pub fn is_supported(locale: &str) -> bool {
    SALUTATION.locales.contains_key(locale)
        || SALUTATION
            .locales
            .contains_key(base_language(&locale.to_ascii_lowercase()))
}

/// Last whitespace-separated token of a recipient name.
/// "Dr. Jane Doe" -> "Doe"; single-token and hyphenated names survive;
/// empty/whitespace yields "".
#[must_use]
pub fn salutation_last_name(name: &str) -> &str {
    name.split_whitespace().next_back().unwrap_or("")
}

/// Canonical display honorific of a recipient name for the locale, or ""
/// when the first token is unparsable. Abbreviations outside the table are
/// rejected; display forms are canonicalized (never accusative, never
/// abbreviated beyond the table form). No gender is ever inferred: without
/// an explicit honorific the caller falls back to the formal template.
#[must_use]
pub fn salutation_honorific(locale: &str, name: &str) -> &'static str {
    let entry = &SALUTATION.locales[resolve_key(locale)];
    let first = name.split_whitespace().next().unwrap_or("");
    entry
        .honorifics
        .get(norm(first).as_str())
        .map_or("", |honorific| honorific.display.as_str())
}

/// Academic titles preserved in the salutation, as display forms.
/// Protocol keeps only the highest title, so a sole title (e.g.
/// Professor) suppresses every other title.
#[must_use]
pub fn salutation_titles(locale: &str, name: &str) -> Vec<&'static str> {
    let entry = &SALUTATION.locales[resolve_key(locale)];
    let mut kept: Vec<&'static str> = Vec::new();
    for token in name.split_whitespace() {
        let Some(title) = entry.titles.get(norm(token).as_str()).map(String::as_str) else {
            continue;
        };
        if !kept.contains(&title) {
            kept.push(title);
        }
    }
    for title in &kept {
        if entry.sole_titles.iter().any(|sole| sole.as_str() == *title) {
            return vec![*title];
        }
    }
    kept
}

/// Surname for the salutation: last significant token after dropping the
/// honorific, academic titles, and post-nominal grades. The raw token is
/// preserved (never normalized for display).
#[must_use]
pub fn salutation_surname<'a>(locale: &str, name: &'a str) -> &'a str {
    let entry = &SALUTATION.locales[resolve_key(locale)];
    name.split_whitespace()
        .rfind(|token| !entry.filler.contains(norm(token).as_str()))
        .unwrap_or("")
}

fn punct(entry: &SalutationEntry) -> &'static str {
    if entry.comma { "," } else { "" }
}

/// Locale-correct salutation through the uniform matcher. Without a
/// parsable honorific or surname it falls back to the locale's formal
/// template so the letter stays formally safe.
#[must_use]
pub fn salutation(locale: &str, name: &str) -> String {
    let entry = &SALUTATION.locales[resolve_key(locale)];
    let punct = punct(entry);
    let first = name.split_whitespace().next().unwrap_or("");
    let honorific = entry.honorifics.get(norm(first).as_str());
    let surname = salutation_surname(locale, name);
    let (Some(honorific), true) = (honorific, !surname.is_empty()) else {
        return format!("{}{punct}", entry.formal);
    };
    let titles = salutation_titles(locale, name).join(" ");
    let template = entry
        .named
        .get(honorific.group.as_str())
        .map_or(entry.formal.as_str(), String::as_str);
    let rendered = template
        .replace("{honorific}", honorific.display.as_str())
        .replace("{titles}", titles.as_str())
        .replace("{surname}", surname);
    let collapsed = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    format!("{collapsed}{punct}")
}

/// Non-blocking advisory when the recipient name is missing: the letter
/// still renders with the formal salutation, but a tailored opportunity
/// should name a person. Returns `None` when a last name is available.
#[must_use]
pub fn recipient_salutation_warning(location: &str, name: &str) -> Option<String> {
    if salutation_last_name(name).is_empty() {
        Some(format!(
            "{location}: job.cl_recipient.name is empty; using formal salutation (provide a name for tailored opportunities)"
        ))
    } else {
        None
    }
}

/// Non-blocking advisory when the recipient name carries no parsable
/// honorific for the locale: the letter falls back to the formal
/// salutation, so a human should supply the full address form. Returns
/// `None` for empty names (already covered by
/// [`recipient_salutation_warning`]) and for complete names.
#[must_use]
pub fn honorific_warning(location: &str, locale: &str, name: &str) -> Option<String> {
    if salutation_last_name(name).is_empty() {
        return None;
    }
    if salutation_honorific(locale, name).is_empty() || salutation_surname(locale, name).is_empty()
    {
        Some(format!(
            "{location}: job.cl_recipient.name has no parsable honorific for {locale}; using formal salutation (provide an explicit honorific for tailored opportunities)"
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_schema() {
        assert!(!SALUTATION.locales.is_empty(), "table must not be empty");
        assert!(
            SALUTATION
                .locales
                .contains_key(SALUTATION.fallback.as_str()),
            "fallback must be a known locale"
        );
        for (key, entry) in &SALUTATION.locales {
            assert_eq!(
                key,
                &key.to_ascii_lowercase(),
                "locale ID must be lowercase: {key:?}"
            );
            for token in entry.honorifics.keys() {
                assert_eq!(
                    token,
                    &norm(token),
                    "honorific key not normalized: {token:?}"
                );
            }
            for honorific in entry.honorifics.values() {
                assert!(
                    honorific.group == "m" || honorific.group == "f",
                    "honorific group must be m/f: {key:?}"
                );
                assert!(
                    !honorific.display.is_empty(),
                    "honorific display must not be empty"
                );
            }
            for token in entry.titles.keys() {
                assert_eq!(token, &norm(token), "title key not normalized: {token:?}");
            }
            for display in entry.titles.values() {
                assert!(!display.is_empty(), "title display form must not be empty");
            }
            for sole in &entry.sole_titles {
                assert!(
                    entry.titles.values().any(|display| display == sole),
                    "sole title must be a known display form: {key:?} {sole:?}"
                );
            }
            for token in &entry.filler {
                assert_eq!(
                    token,
                    &norm(token),
                    "filler token not normalized: {token:?}"
                );
            }
            for group in ["m", "f"] {
                let template = entry
                    .named
                    .get(group)
                    .unwrap_or_else(|| panic!("locale needs a {group} named template: {key:?}"));
                assert!(
                    template.contains("{surname}"),
                    "named template must place the surname: {key:?}"
                );
            }
            assert!(
                !entry.formal.is_empty(),
                "formal fallback must not be empty"
            );
            if let Some(review) = &entry.review {
                assert_eq!(review, "native-pending", "unknown review state: {key:?}");
            }
            assert!(
                !entry.source_norms.is_empty(),
                "source norms must be cited: {key:?}"
            );
        }
    }

    #[test]
    fn supported_resolution() {
        assert!(is_supported("de-ch"));
        assert!(is_supported("DE-CH"));
        assert!(is_supported("fr-be"));
        assert!(!is_supported("xx"));
        assert_eq!(resolve_key("xx"), "en");
        assert_eq!(resolve_key("de-li"), "de-li");
    }
}
