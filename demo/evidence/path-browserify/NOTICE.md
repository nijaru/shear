# path-browserify demonstration evidence

Source: https://github.com/browserify/path-browserify at `872fec31a8bac7b9b43be0e54ef3037e0202c5fb`, package 1.0.1.

`index.patch` records Shear's two checked local redundant-alternative removals in `index.js`. No upstream tests, public interfaces, or dependencies were edited. The source describes itself as Node.js v8.11.1 POSIX path code transpiled with Babel.

Retained notices:
- `LICENSE`: MIT, copyright 2013 James Halliday.
- `NODE-NOTICE.txt`: original Node/Joyent copyright and permission header.

`package-lock.json` was generated for this demonstration, not supplied by upstream. It pins the existing development dependency graph for `npm ci --package-lock=true --ignore-scripts`. Dependencies retain their own licenses; their source packages are not included here.

The unchanged default suite passed 263 TAP assertions with the same 11 Windows suite skips before/after. The separately invoked parse/format suite passed 247 assertions with the same four Windows suite skips. This is POSIX evidence, not Windows validation or general semantic equivalence. See `docs/MIXED_EXAMPLE.md` for exact identities and reproduction. No upstream endorsement is implied.
