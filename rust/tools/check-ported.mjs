// Replays the recorded cases of the ported domain functions (rust/units/ported.txt) against
// the Rust build of rust/units/domain-rs, function by function (ADR-011).
// `rust-anything diff` checks every recorded function at once, ported or not; this one says
// which ported function passes and stops on any mismatch.
// Usage: node rust/tools/check-ported.mjs   (after `rust-anything build rust/units/domain-rs`)
import { createRequire } from 'node:module'
import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { isDeepStrictEqual } from 'node:util'

const here = dirname(fileURLToPath(import.meta.url))
const unit = join(here, '..', 'units', 'domain-rs')
const require = createRequire(import.meta.url)
const rust = require(join(unit, 'index.cjs'))

const ported = readFileSync(join(here, '..', 'units', 'ported.txt'), 'utf8')
  .split('\n')
  .map(line => line.replace(/#.*/, '').trim())
  .filter(Boolean)
const wanted = new Set(ported)
const results = new Map(ported.map(name => [name, { pass: 0, fail: 0, skip: 0, first: null }]))

// rust-anything writes NaN, ±Infinity and -0 as {"$num": "..."}, undefined as {"$undefined": true}.
const decode = value => {
  if (Array.isArray(value)) return value.map(decode)
  if (value && typeof value === 'object') {
    const keys = Object.keys(value)
    if (keys.length === 1 && keys[0] === '$num') return Number(value.$num)
    if (keys.length === 1 && keys[0] === '$undefined') return undefined
    return Object.fromEntries(keys.map(k => [k, decode(value[k])]))
  }
  return value
}

for (const line of readFileSync(join(unit, 'cases.jsonl'), 'utf8').split('\n')) {
  if (!line) continue
  const c = decode(JSON.parse(line))
  if (!wanted.has(c.fn)) continue
  const r = results.get(c.fn)
  let got
  try {
    got = { ok: rust[c.fn](...c.args) }
  } catch (e) {
    got = { error: String(e && e.message) }
  }
  if ('error' in got && got.error.includes('SKIP: input not of the typed shape')) {
    r.skip++
    continue
  }
  const same = 'ok' in c ? 'ok' in got && isDeepStrictEqual(got.ok, c.ok) : 'error' in got
  if (same) r.pass++
  else {
    r.fail++
    r.first ??= { args: c.args, expected: c.ok ?? { error: c.error }, got }
  }
}

let failed = false
for (const [name, r] of results) {
  const total = r.pass + r.fail
  console.log(`${r.fail === 0 && total > 0 ? 'ok  ' : 'FAIL'} ${name}: ${r.pass}/${total}${r.skip ? ` (${r.skip} skipped: not of the typed shape)` : ''}`)
  if (total === 0) failed = true
  if (r.fail) {
    failed = true
    console.log('     first mismatch:', JSON.stringify(r.first).slice(0, 400))
  }
}
process.exit(failed ? 1 : 0)
