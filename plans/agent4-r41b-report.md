# Agent 4 — Round 41b Report

## Baseline
- Main at `1ef1d437`, branch `agent4/ready`
- 4276 verified, 0 errors, 187 holes, 30 clean chapters, 2612 RTT pass

## Results
- 4276 verified, 0 errors, **184 holes (-3)**, 30 clean chapters, 2612 RTT pass
- Chap47: 9 → 6 holes (-3), 16 warnings (unchanged), 4 clean files (unchanged)

## Changes

### Part A+C: spec_second_hash + DoubleHash WF Bridge Closure (-3 holes)

Added `pub closed spec fn spec_second_hash<Key>(key: Key, table_size: nat) -> nat`
to ParaHashTableStEph.rs as an uninterpreted function (body hidden from SMT). The
`external_body` `compute_second_hash` now ensures `step as nat == spec_second_hash(*key,
table_size as nat)`, linking runtime to spec.

Rewrote `spec_doublehashflathashsteph_wf` probe chain from existential (`exists |s: int,
n: int| s >= 1 && ...`) to concrete (`let s = spec_second_hash(k, m as nat) as int;
s >= 1 && exists |n: int| ...`). This makes the step deterministic per key/table_size
pair.

With concrete step in wf, the 3 assume blocks in insert/lookup/delete became provable:
wf gives `s = spec_second_hash(key, m)`, `second_hash` gives `step = spec_second_hash(key,
m)`, so `s == step`. Replaced assumes with `step as nat == spec_second_hash(key, m as nat)`
in loop invariants. Z3 instantiates wf for `k = key` and unifies the concrete step.

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Added `spec_second_hash` spec fn |
| 2 | 47 | ParaHashTableStEph.rs | Strengthened `compute_second_hash` ensures |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | Strengthened `second_hash` ensures |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | Rewrote wf to use `spec_second_hash` |
| 5 | 47 | DoubleHashFlatHashTableStEph.rs | Removed insert assume (-1 hole) |
| 6 | 47 | DoubleHashFlatHashTableStEph.rs | Removed lookup assume (-1 hole) |
| 7 | 47 | DoubleHashFlatHashTableStEph.rs | Removed delete assume (-1 hole) |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | Updated 3 wf maintenance proofs |

### Part B: call_hash_fn Spec (No Change Needed)

`call_hash_fn` already has strong spec: `ensures index < table_size, index as nat ==
(spec_hash@)(*key) % (table_size as nat)`. No strengthening needed.

### Part D: StructChainedHashTable resize (Deferred)

Not attempted this round. Requires proving linked list chain traversal collects
all entries via `spec_chain_to_map`. Estimated 50-80 lines of proof.

## Remaining Chap47 Holes (6)

| # | Chap | File | Hole | Status |
|---|------|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | assume(c == *x) in clone_elem | Irreducible: clone bridge |
| 2 | 47 | ParaHashTableStEph.rs | external_body call_hash_fn | Irreducible: opaque Fn closure |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | assume(false) insert | Unreachable with load < 1 |
| 4 | 47 | LinProbFlatHashTableStEph.rs | assume(false) insert | Unreachable with load < 1 |
| 5 | 47 | QuadProbFlatHashTableStEph.rs | assume(false) insert | Reachable at runtime |
| 6 | 47 | StructChainedHashTable.rs | external_body resize | Provable but complex |

## Technique

**Closed spec fn as uninterpreted function**: `closed spec fn` hides its body from SMT,
making it behave like an uninterpreted function. The `external_body` runtime function
links to it via ensures (`step == spec_second_hash(key, m)`). The wf predicate uses the
same spec fn, eliminating the existential. Z3 sees `spec_second_hash(key, m)` in both
the wf and the runtime postcondition, unifying them automatically.

This pattern applies whenever an `external_body` function's result is used in an
invariant that previously required an existential witness. Instead of `exists |s|`,
define a `closed spec fn` that maps inputs to the abstract result, link the external_body
to it, and use the spec fn directly in the invariant.
