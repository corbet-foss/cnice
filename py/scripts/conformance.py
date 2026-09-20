"""Run canonical vectors against either the source port or an installed wheel."""
import importlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
if "--installed" not in sys.argv:
    sys.path.insert(0, str(ROOT / "py"))
greet = importlib.import_module("cnice.greet")
farewell = importlib.import_module("cnice.farewell")

ARGUMENTS = {
    "salutation": ["locale", "input"],
    "salutation_last_name": ["input"], "salutation_honorific": ["locale", "input"],
    "salutation_titles": ["locale", "input"], "salutation_surname": ["locale", "input"],
    "recipient_salutation_warning": ["location", "input"],
    "honorific_warning": ["location", "locale", "input"],
    "is_supported": ["locale"],
    "closing": ["locale", "override"], "available_locales": [],
}

FAREWELL_FNS = set(farewell.__all__)

files = sorted((ROOT / "tests/vectors").glob("*.json"))
assert files, "No conformance vectors"
count = 0
for file in files:
    for vector in json.loads(file.read_text(encoding="utf-8")):
        name = vector["fn"]
        values = dict(vector)
        if name in FAREWELL_FNS:
            fn = getattr(farewell, name)
        else:
            fn = getattr(greet, name)
        actual = fn(*(values.get(key) for key in ARGUMENTS[name]))
        if isinstance(actual, tuple):
            actual = list(actual)
        if actual != vector["expected"]:
            raise AssertionError(f"{file.name} :: {vector['name']}: {actual!r} != {vector['expected']!r}")
        count += 1
print(f"Python cnice: {count} vectors passed across {len(files)} files")
