"""Locale-correct salutations through the uniform matcher.

One matcher for every language: each locale row in tables/salutation.json
carries its honorifics, academic titles, name templates, comma rule and
formal fallback; the code below only resolves the locale, matches
normalized tokens and fills templates.
"""

import re

from ._tables import TABLES

_SALUTATION = TABLES["salutation"]
_LOCALES = _SALUTATION["locales"]
_FALLBACK = _SALUTATION["fallback"]
_WS = re.compile(r"[\u0009-\u000d\u0020\u0085\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000]+")
_ASCII_LOWER = str.maketrans("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz")

__all__ = ["salutation", "salutation_honorific", "salutation_titles",
           "salutation_surname", "salutation_last_name",
           "recipient_salutation_warning", "honorific_warning", "is_supported"]


def _tokens(name: str) -> list[str]:
    return [token for token in _WS.split(name) if token]


def _norm(token: str) -> str:
    return token.strip(".").translate(_ASCII_LOWER)


def _ascii_lower(value: str) -> str:
    return value.translate(_ASCII_LOWER)


def _base_language(code: str) -> str:
    return code.split("-", 1)[0]


def _resolve_key(locale: str) -> str:
    lower = _ascii_lower(locale)
    if lower in _LOCALES:
        return lower
    base = _base_language(lower)
    if base in _LOCALES:
        return base
    return _FALLBACK


def is_supported(locale: str) -> bool:
    """Whether a locale has a salutation row directly or via its base language."""
    return locale in _LOCALES or _base_language(_ascii_lower(locale)) in _LOCALES


def salutation_last_name(name: str) -> str:
    """Last whitespace-separated token, or an empty string."""
    tokens = _tokens(name)
    return tokens[-1] if tokens else ""


def salutation_honorific(locale: str, name: str) -> str:
    """Canonical display honorific for the locale, or "" when unparsable."""
    entry = _LOCALES[_resolve_key(locale)]
    tokens = _tokens(name)
    first = tokens[0] if tokens else ""
    honorific = entry["honorifics"].get(_norm(first))
    return honorific["display"] if honorific is not None else ""


def salutation_titles(locale: str, name: str) -> list[str]:
    """Recognized academic titles as display forms, with sole-title suppression."""
    entry = _LOCALES[_resolve_key(locale)]
    kept: list[str] = []
    for token in _tokens(name):
        title = entry["titles"].get(_norm(token))
        if title is not None and title not in kept:
            kept.append(title)
    for title in kept:
        if title in entry["sole_titles"]:
            return [title]
    return kept


def salutation_surname(locale: str, name: str) -> str:
    """Last significant token after dropping filler; raw form preserved."""
    entry = _LOCALES[_resolve_key(locale)]
    filler = set(entry["filler"])
    for token in reversed(_tokens(name)):
        if _norm(token) not in filler:
            return token
    return ""


def salutation(locale: str, name: str) -> str:
    """Locale-correct salutation, falling back to the formal template when incomplete."""
    entry = _LOCALES[_resolve_key(locale)]
    punct = "," if entry["comma"] else ""
    tokens = _tokens(name)
    first = tokens[0] if tokens else ""
    honorific = entry["honorifics"].get(_norm(first))
    surname = salutation_surname(locale, name)
    if honorific is None or not surname:
        return entry["formal"] + punct
    titles = " ".join(salutation_titles(locale, name))
    template = entry["named"].get(honorific["group"], entry["formal"])
    rendered = template.replace("{honorific}", honorific["display"]).replace("{titles}", titles).replace("{surname}", surname)
    collapsed = " ".join(_tokens(rendered))
    return collapsed + punct


def recipient_salutation_warning(location: str, name: str) -> str | None:
    """Advisory for an empty recipient name."""
    if salutation_last_name(name) == "":
        return f"{location}: job.cl_recipient.name is empty; using formal salutation (provide a name for tailored opportunities)"
    return None


def honorific_warning(location: str, locale: str, name: str) -> str | None:
    """Advisory for a nonempty name without a parsable honorific for the locale."""
    if salutation_last_name(name) == "":
        return None
    if not salutation_honorific(locale, name) or not salutation_surname(locale, name):
        return f"{location}: job.cl_recipient.name has no parsable honorific for {locale}; using formal salutation (provide an explicit honorific for tailored opportunities)"
    return None
