"""JSON command-line bridge for callers in any programming language."""
import argparse
import base64
import dataclasses
import json
import sys
from cnice import farewell, greet

FUNCTIONS = {}
for _module in (greet, farewell):
    _namespace = _module.__name__.split(".")[-1]
    for _name in _module.__all__:
        FUNCTIONS[_name] = getattr(_module, _name)
        FUNCTIONS[f"{_namespace}.{_name}"] = getattr(_module, _name)


def _json_default(value):
    if dataclasses.is_dataclass(value):
        return dataclasses.asdict(value)
    if isinstance(value, bytes):
        return base64.b64encode(value).decode("ascii")
    raise TypeError(f"Cannot serialize {type(value).__name__}")


def _reject_constant(value):
    raise ValueError(f"{value} is not a JSON number")


def main():
    parser = argparse.ArgumentParser(description="Call cnice functions with JSON arguments.")
    parser.add_argument("function", choices=sorted(FUNCTIONS))
    parser.add_argument("arguments", nargs="?", default="[]", help="JSON array of positional arguments or object of keyword arguments; '-' reads UTF-8 stdin")
    args = parser.parse_args()
    try:
        values = json.loads(sys.stdin.buffer.read().decode("utf-8") if args.arguments == "-" else args.arguments, parse_constant=_reject_constant)
        if not isinstance(values, (list, dict)):
            raise ValueError("arguments must be a JSON array or object")
        # Image operations accept the JSON object returned by normalize.
        if args.function in ("exceeds_limits", "scale_to_fit", "signature_size"):
            raw = values[0] if isinstance(values, list) else values["image"]
            if isinstance(raw, dict):
                raw = dict(raw)
                raw["bytes"] = base64.b64decode(raw["bytes"], validate=True)
                from cnice import greet as _greet

                image = _greet.DecodedImage(**raw)
                if isinstance(values, list):
                    values[0] = image
                else:
                    values["image"] = image
        fn = FUNCTIONS[args.function]
        result = fn(*values) if isinstance(values, list) else fn(**values)
        print(json.dumps(result, default=_json_default, ensure_ascii=True, allow_nan=False))
    except (ValueError, TypeError, KeyError, IndexError, AttributeError, OverflowError) as error:
        parser.error(str(error))


if __name__ == "__main__":
    main()
