/**
 * Deterministic locale-correct salutations for formal correspondence.
 *
 * Pure TypeScript port of cnice::greet: zero dependencies, zero Node APIs,
 * synchronous, no I/O — runs in browsers, edge runtimes, and Node alike.
 * One matcher for every language: each locale row in `tables/salutation.json`
 * at the repository root carries its honorifics, academic titles, name
 * templates, comma rule and formal fallback; the code below only resolves the
 * locale, matches normalized tokens and fills templates. There is deliberately
 * no per-language branch — a new language is a new table row, never new code.
 * `tests/vectors/*.json` is the shared conformance suite. A change here
 * without a matching vector is a bug.
 */
import { SALUTATION_TABLE } from './generated/greet-tables.ts';

/**
 * Whitespace split matching Rust `split_whitespace` (Unicode White_Space)
 * exactly — JavaScript `\s` differs on U+0085 and U+FEFF, so the set is
 * explicit. See `tables/greet/README.md`.
 */
const WS = new Set([
    '\t',
    '\n',
    '\x0B',
    '\x0C',
    '\r',
    ' ',
    '\u0085',
    '\u00A0',
    '\u1680',
    '\u2000',
    '\u2001',
    '\u2002',
    '\u2003',
    '\u2004',
    '\u2005',
    '\u2006',
    '\u2007',
    '\u2008',
    '\u2009',
    '\u200A',
    '\u2028',
    '\u2029',
    '\u202F',
    '\u205F',
    '\u3000',
]);

function splitWhitespace(text: string): string[] {
    const tokens: string[] = [];
    let current = '';
    for (const char of text) {
        if (WS.has(char)) {
            if (current !== '') {
                tokens.push(current);
                current = '';
            }
        } else {
            current += char;
        }
    }
    if (current !== '') tokens.push(current);
    return tokens;
}

/**
 * Normalize one token for table lookup: strip leading/trailing `.`,
 * lowercase ASCII A–Z only (never Unicode-aware lowercasing).
 */
function norm(token: string): string {
    return token.replace(/^\.+|\.+$/g, '').replace(/[A-Z]/g, (c) => c.toLowerCase());
}

/** Lowercase ASCII A–Z only, matching Rust `to_ascii_lowercase`. */
function asciiLower(text: string): string {
    return text.replace(/[A-Z]/g, (c) => c.toLowerCase());
}

function baseLanguage(code: string): string {
    return code.split('-')[0] ?? code;
}

const table = SALUTATION_TABLE;

const fillerByLocale = new Map<string, Set<string>>();
for (const [key, entry] of Object.entries(table.locales)) {
    fillerByLocale.set(key, new Set(entry.filler));
}

/**
 * Resolve a lowercase table key: case-insensitive exact code, base language,
 * then English fallback. All stored locale IDs are lowercase.
 */
function resolveKey(locale: string): string {
    const lower = asciiLower(locale);
    if (Object.hasOwn(table.locales, lower)) return lower;
    const base = baseLanguage(lower);
    return Object.hasOwn(table.locales, base) ? base : table.fallback;
}

/**
 * Whether a locale code has a salutation row: present directly or through its
 * (lowercased) base language. Consumers use this predicate instead of any
 * language list in code.
 */
export function isSupported(locale: string): boolean {
    const lower = asciiLower(locale);
    return Object.hasOwn(table.locales, locale) || Object.hasOwn(table.locales, baseLanguage(lower));
}

/**
 * Last whitespace-separated token of a recipient name.
 * "Dr. Jane Doe" -> "Doe"; single-token and hyphenated names survive;
 * empty/whitespace yields "".
 */
export function salutationLastName(name: string): string {
    const tokens = splitWhitespace(name);
    return tokens.length > 0 ? tokens[tokens.length - 1] : '';
}

/**
 * Canonical display honorific of a recipient name for the locale, or "" when
 * the first token is unparsable. Abbreviations outside the table are rejected;
 * display forms are canonicalized (never accusative, never abbreviated beyond
 * the table form). No gender is ever inferred: without an explicit honorific
 * the caller falls back to the formal template.
 */
export function salutationHonorific(locale: string, name: string): string {
    const entry = table.locales[resolveKey(locale)];
    const first = splitWhitespace(name)[0] ?? '';
    const key = norm(first);
    return Object.hasOwn(entry.honorifics, key) ? entry.honorifics[key].display : '';
}

/**
 * Academic titles preserved in the salutation, as display forms. Protocol
 * keeps only the highest title, so a sole title (e.g. Professor) suppresses
 * every other title.
 */
export function salutationTitles(locale: string, name: string): string[] {
    const entry = table.locales[resolveKey(locale)];
    const kept: string[] = [];
    for (const token of splitWhitespace(name)) {
        const key = norm(token);
        const title = Object.hasOwn(entry.titles, key) ? entry.titles[key] : undefined;
        if (title !== undefined && !kept.includes(title)) kept.push(title);
    }
    for (const title of kept) {
        if (entry.sole_titles.includes(title)) return [title];
    }
    return kept;
}

/**
 * Surname for the salutation: last significant token after dropping the
 * honorific, academic titles, and post-nominal grades. The raw token is
 * preserved (never normalized for display).
 */
export function salutationSurname(locale: string, name: string): string {
    const filler = fillerByLocale.get(resolveKey(locale)) ?? new Set<string>();
    const tokens = splitWhitespace(name);
    for (let index = tokens.length - 1; index >= 0; index--) {
        if (!filler.has(norm(tokens[index]))) return tokens[index];
    }
    return '';
}

/**
 * Locale-correct salutation through the uniform matcher. Without a parsable
 * honorific or surname it falls back to the locale's formal template so the
 * letter stays formally safe.
 */
export function salutation(locale: string, name: string): string {
    const key = resolveKey(locale);
    const entry = table.locales[key];
    const punct = entry.comma ? ',' : '';
    const first = splitWhitespace(name)[0] ?? '';
    const hkey = norm(first);
    const honorific = Object.hasOwn(entry.honorifics, hkey) ? entry.honorifics[hkey] : undefined;
    const surname = salutationSurname(locale, name);
    if (honorific === undefined || surname === '') return `${entry.formal}${punct}`;
    const titles = salutationTitles(locale, name).join(' ');
    const template = entry.named[honorific.group] ?? entry.formal;
    const rendered = template
        .split('{honorific}')
        .join(honorific.display)
        .split('{titles}')
        .join(titles)
        .split('{surname}')
        .join(surname);
    const collapsed = splitWhitespace(rendered).join(' ');
    return `${collapsed}${punct}`;
}

/**
 * Non-blocking advisory when the recipient name is missing: the letter still
 * renders with the formal salutation, but a tailored opportunity should name
 * a person. Returns `null` when a last name is available.
 */
export function recipientSalutationWarning(location: string, name: string): string | null {
    if (salutationLastName(name) === '') {
        return `${location}: job.cl_recipient.name is empty; using formal salutation (provide a name for tailored opportunities)`;
    }
    return null;
}

/**
 * Non-blocking advisory when the recipient name carries no parsable honorific
 * for the locale: the letter falls back to the formal salutation, so a human
 * should supply the full address form. Returns `null` for empty names (already
 * covered by `recipientSalutationWarning`) and for complete names.
 */
export function honorificWarning(location: string, locale: string, name: string): string | null {
    if (salutationLastName(name) === '') return null;
    if (salutationHonorific(locale, name) === '' || salutationSurname(locale, name) === '') {
        return `${location}: job.cl_recipient.name has no parsable honorific for ${locale}; using formal salutation (provide an explicit honorific for tailored opportunities)`;
    }
    return null;
}
