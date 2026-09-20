/**
 * cnice facade: locale-correct salutations (`greet`) and locale-specific
 * valedictions (`farewell`). Pure TypeScript, zero
 * dependencies, zero Node APIs — runs in browsers, edge runtimes, and Node
 * alike. Behavior is defined by `tables/salutation.json` and
 * `tables/farewell/*.json` at the repository root; `tests/vectors/*.json`
 * is the shared conformance suite.
 */
export * as greet from './greet.ts';
export * as farewell from './farewell.ts';
