# Changelog

All notable changes to `cnice` are documented here. The project follows
Semantic Versioning.

## 0.1.1 - 2026-09-24

- Repository moved to github.com/corbet-foss/cnice; registry metadata points there.
- Released from a single tag through CI (crates.io and JSR trusted publishing).
- Add explicit Swiss French (`fr-ch`) and Swiss Italian (`it-ch`)
  salutation rows (same norms as `fr`/`it`); vectors pin both locales.

## 0.1.0 - 2026-09-18

- Replace the ported German-only matcher with the uniform salutation
  renderer: one code path for every locale in `tables/salutation.json`
  (de/de-ch/de-at/de-li, fr, it, rm, en/en-gb/en-us), driven entirely by
  table rows — no per-language branch. `Region`, `parse_region`,
  `region_uses_comma` and `de_salutation` are gone; parsers take an
  explicit locale, `salutation(locale, name)` renders through the row
  templates, `honorific_warning(location, locale, name)` replaces
  `de_honorific_warning`, and `is_supported(locale)` reports row coverage.
- Add Romansh (Rumantsch Grischun) rows; the `rm` salutation row and
  `Cordials salids` closing are marked native-pending (see Review below)
  and pin provisional bytes for determinism.
- Add `rm` closing row (`Cordials salids`, native-pending).

## Review

- `rm` salutation row and closing: provisional bytes awaiting native
  review (opening attested in Lia Rumantscha/BAKOM correspondence).
  Vectors prove determinism, not correctness.

### Initial scaffold (2026-09-17)

- New facade crate `cnice` over formulaic correspondence phrases:
  `cnice.greet` ports 100% of the `cgreet` public API (German salutations,
  regions, title handling, surname extraction, validation warnings) and
  `cnice.farewell` ports 100% of the `cfarewell` public API (closings,
  available locales) with identical names, semantics, and shared
  conformance vectors (74 greet + 57 farewell = 131 total).
- License this new release line under LGPL-3.0-only WITH LGPL-3.0-linking-exception across Cargo, npm, JSR,
  Python and Typst, with the complete LGPL and incorporated GPL notices.
- The `cgreet` and `cfarewell` repositories keep their original grants untouched; this facade does not relicense them.
