// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-foss/cnice'));
verify(createRequire(import.meta.url)('@corbet-foss/cnice'));
verify(await import('@corbet-foss/cnice/browser'));
console.log('cnice: installed ESM, CommonJS, and browser exports passed');
