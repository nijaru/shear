const assert = require('node:assert/strict');
const { resolve } = require('node:path');
if (process.argv.length !== 4) {
  throw new Error('Usage: node demo/compare_path.cjs ORIGINAL_INDEX CANDIDATE_INDEX (trusted modules only)');
}
const before = require(resolve(process.argv[2]));
const after = require(resolve(process.argv[3]));
let seed = 42;
const alphabet = [...'/ab. é\\💫'];
const paths = ['', '/', '.', '..', 'a', '/a/b/', '../a', '//a//b', 'a.txt', '\0'];
while (paths.length < 128) {
  seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
  const length = seed % 30;
  let path = '';
  for (let i = 0; i < length; i++) {
    seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
    path += alphabet[seed % alphabet.length];
  }
  paths.push(path);
}
let comparisons = 0;
function observe(api, name, args) {
  try { return JSON.stringify({value: api[name](...args)}); }
  catch (error) { return JSON.stringify({error: error.name, message: error.message}); }
}
function check(name, args) {
  assert.equal(observe(after, name, args), observe(before, name, args), `${name}: ${JSON.stringify(args)}`);
  comparisons++;
}
for (const p of paths) {
  for (const name of ['normalize', 'isAbsolute', 'dirname', 'basename', 'extname', 'parse']) check(name, [p]);
  check('format', [before.parse(p)]);
  for (const q of paths) {
    for (const name of ['relative', 'basename', 'join', 'resolve']) check(name, [p, q]);
  }
}
for (const value of [undefined, null, false, {}, [], 42]) {
  for (const name of ['normalize', 'isAbsolute', 'dirname', 'basename', 'extname', 'parse', 'format', 'relative', 'join', 'resolve']) check(name, [value]);
}
console.log(JSON.stringify({seed: 42, paths: paths.length, comparisons, matched: true}));
