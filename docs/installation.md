# Installation and distribution

The main branch ships 0.1.0 under LGPL-3.0-only WITH LGPL-3.0-linking-exception, published to registries. The existing releases documented below keep their original
license grants; this change does not replace their artifacts.

This guide describes version 0.1.0. Check the linked registry or release
for availability; a source manifest alone does not establish publication. `cnice` exposes two namespaces — `greet` (German salutations, was `cgreet`) and `farewell` (locale-specific valedictions, was `cfarewell`) — in every language port.

## JavaScript and Rust

Use Cargo for Rust and npm, pnpm, Yarn, or Bun for JavaScript. These JavaScript
package managers share the npm registry; each consumes the same package.
Deno can use `npm:@corbet-foss/cnice`. The browser export bundles runtime
dependencies and needs no import map. The declared Rust minimum is 1.94. Release checks use the worker's current stable compiler; a separate minimum-version check is required to
verify that lower bound.

## Python and the command line

The pure Python package requires Python 3.10+ and is published on
[PyPI](https://pypi.org/project/cnice/0.1.0/). Install this release with pip
or uv in your Python environment:

```sh
python -m pip install cnice==0.1.0
```

```sh
uv pip install cnice==0.1.0
```

For a uv project, `uv add cnice==0.1.0` adds the package to your dependencies.

After installation, functions can be called from Python or through either CLI
entrypoint:

```sh
cnice --help
python -m cnice --help
```

The CLI takes a function name and a JSON array of positional arguments or an
object of keyword arguments. Use `-` to read arguments from stdin. It writes
JSON to stdout; errors use stderr and a nonzero exit status.

For an isolated CLI environment, use `pipx install cnice==0.1.0` or
`uv tool install cnice==0.1.0`.

The wheel contains no native extensions and is platform independent. Release
evidence records the Python version and operating system actually exercised.
Verified wheels and source distributions are also attached to the
[GitHub release](https://github.com/corbet-foss/cnice/releases/tag/v0.1.0).

## JSR

Version 0.1.0 is published as
[`@corbet-foss/cnice`](https://jsr.io/@corbet-foss/cnice@0.1.0):

```sh
deno add jsr:@corbet-foss/cnice@0.1.0
```

## Typst

Version 0.1.0 has not yet been submitted to
[Typst Universe](https://typst.app/universe/package/cnice). Once published, import it directly
in the Typst web app or a local Typst document:

```typst
#import "@preview/cnice:0.1.0": *
```

For a local installation, download `cnice-0.1.0-typst.tar.gz` from the matching GitHub release and
extract its contents into `typst/packages/local/cnice/0.1.0` under your
[Typst data directory](https://github.com/typst/packages#local-packages):

| System | Data directory |
| --- | --- |
| Linux | `$XDG_DATA_HOME`, or `~/.local/share` |
| macOS | `~/Library/Application Support` |
| Windows | `%APPDATA%` |

```typst
#import "@local/cnice:0.1.0": *
```

The archive includes its manifest, tables, source, and licenses. CI compiles
an example against a fresh installation of the actual archive with Typst 0.15.
For the Typst web app, upload the extracted files and import the entrypoint
listed in `typst.toml` by its relative path.

## System package managers

These are language libraries, with a portable Python CLI. Homebrew, APT, RPM,
WinGet, Chocolatey, and Scoop are not additional registries for importing a
Rust crate or JavaScript module. Use their Python or Node runtime and the
language package manager above. No native system-package listing is claimed.
