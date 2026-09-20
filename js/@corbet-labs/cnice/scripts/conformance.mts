/**
 * TypeScript side of the cross-language conformance gate: runs every
 * `tests/vectors/*.json` vector through `src/index.ts` (the `greet` and
 * `farewell` namespaces) and compares with `expected` exactly. Exits
 * non-zero with the first mismatch.
 *
 * Run from the package root:
 *   bun ./scripts/conformance.mts
 */
import { readdirSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join, resolve } from 'node:path';
import { farewell, greet } from '../src/index.js';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../../../..');

interface Vector {
    name: string;
    fn: string;
    input: string;
    location?: string;
    locale: string;
    override?: string;
    expected: unknown;
}

function checkEqual(actual: unknown, expected: unknown, what: string): void {
    const got = JSON.stringify(actual) ?? 'undefined';
    const want = JSON.stringify(expected) ?? 'undefined';
    if (got !== want) throw new Error(`${what}: ${got} vs ${want}`);
}

function runVector(file: string, vector: Vector): void {
    const what = `${file} :: ${vector.name}`;
    let actual: unknown;
    switch (vector.fn) {
        case 'is_supported':
            actual = greet.isSupported(vector.locale);
            break;
        case 'salutation_last_name':
            actual = greet.salutationLastName(vector.input);
            break;
        case 'salutation_honorific':
            actual = greet.salutationHonorific(vector.locale, vector.input);
            break;
        case 'salutation_surname':
            actual = greet.salutationSurname(vector.locale, vector.input);
            break;
        case 'salutation_titles':
            actual = greet.salutationTitles(vector.locale, vector.input);
            break;
        case 'salutation':
            actual = greet.salutation(vector.locale, vector.input);
            break;
        case 'recipient_salutation_warning':
            if (vector.location === undefined) throw new Error(`${what}: missing location`);
            actual = greet.recipientSalutationWarning(vector.location, vector.input);
            break;
        case 'honorific_warning':
            if (vector.location === undefined) throw new Error(`${what}: missing location`);
            actual = greet.honorificWarning(vector.location, vector.locale, vector.input);
            break;
        case 'available_locales':
            actual = farewell.availableLocales();
            break;
        case 'closing':
            actual = farewell.closing(vector.locale, vector.override);
            break;
        default:
            throw new Error(`${what}: unknown fn ${vector.fn}`);
    }
    checkEqual(actual, vector.expected, what);
}

const dir = join(ROOT, 'tests/vectors');
const files = readdirSync(dir)
    .filter((file) => file.endsWith('.json'))
    .sort();
if (files.length === 0) throw new Error('no vector files in tests/vectors');
let count = 0;
for (const file of files) {
    const vectors = JSON.parse(readFileSync(join(dir, file), 'utf8')) as Vector[];
    for (const vector of vectors) {
        runVector(file, vector);
        count += 1;
    }
}
console.log(`TS conformance green: ${count} vectors across ${files.length} files`);
