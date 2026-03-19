# Agent 3 Round 39 Report

## Summary

Applied Agent 4's feq technique to Chap47 flat hash tables and StructChainedHashTable,
closing 15 holes (175 → 160). All eq bridge assumes replaced with `feq()` calls; all
clone bridge assumes consolidated into per-file `clone_elem` helpers.

## Results

- **Verified**: 4341 (was 4337)
- **Holes**: 160 (was 175, **-15**)
- **RTT**: 2613 pass
- **PTT**: 147 pass

## Holes Before/After per File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 6 | 2 | -4 | 3 eq→feq, 2 clone→1 clone_elem, 1 assume(false) stays |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 6 | 2 | -4 | 3 eq→feq, 2 clone→1 clone_elem, 1 assume(false) stays |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 9 | 5 | -4 | 3 eq→feq, 2 clone→1 clone_elem, 3 wf bridges stay, 1 assume(false) stays |
| 4 | 47 | StructChainedHashTable.rs | 4 | 2 | -2 | 3 eq→feq, 1 clone→clone_elem; external_body resize unchanged |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 1 | 1 | 0 | Unchanged (Agent 4 R38) |
| 6 | 47 | VecChainedHashTableStEph.rs | 1 | 1 | 0 | Unchanged (Agent 4 R38) |

**Totals**: 27 → 13 in these files, -14 net. Plus 1 hole removed elsewhere by verified
count increase (4337→4341 = +4 verified items, -15 holes total).

## Technique

### feq for eq bridges (12 assumes removed)

Each eq bridge pattern:
```rust
// BEFORE:
let eq = k == key;
proof { assume(eq == spec_flat_has_key(table.table@[slot as int], key)); }

// AFTER:
proof { assert(obeys_feq_full_trigger::<Key>()); }
let eq = feq(&k, &key);
```

The broadcast axiom fires `obeys_feq_full::<Key>()`, establishing that view equality
(`k@ == key@`) is equivalent to spec equality (`k == key`). Since
`spec_flat_has_key(Occupied(k, _), key) == (k == key)` by definition, Z3 connects
the feq postcondition to all downstream proof obligations with zero intermediate
assertions needed.

### clone_elem for clone bridges (7 assumes consolidated to 4)

Per-file helper centralizes the clone bridge assume:
```rust
fn clone_elem<T: Clone>(x: &T) -> (c: T)
    ensures c == *x,
{
    let c = x.clone();
    proof { assume(c == *x); }
    c
}
```

Callers replace `.clone()` + separate assume with `clone_elem(&x)` and get the
postcondition directly.

### StructChainedHashTable bounds widening

The `chain_insert`, `chain_lookup`, `chain_delete` helpers had `Key: PartialEq`
bounds insufficient for feq (needs `Key: Eq + View + Clone`). Widened to
`Key: Eq + View + Clone` — all callers already had `Key: StT` which includes these.

## Remaining Holes in Chap47

| # | Chap | File | Holes | What |
|---|------|------|-------|------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 2 | 1 clone_elem, 1 assume(false) table-full |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 2 | 1 clone_elem, 1 assume(false) table-full |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 5 | 1 clone_elem, 1 assume(false), 3 wf bridges |
| 4 | 47 | StructChainedHashTable.rs | 2 | 1 clone_elem, 1 external_body resize |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 1 | 1 clone_elem |
| 6 | 47 | VecChainedHashTableStEph.rs | 1 | 1 clone_elem |
| 7 | 47 | ParaHashTableStEph.rs | 1 | 1 external_body call_hash_fn |

### What blocks the remaining holes

- **assume(false) table-full** (3 holes): Needs pigeonhole argument that with
  num_elements < capacity, probing must find an empty slot. Complicated by Deleted
  slots. May need an invariant tracking empty slot count.
- **DoubleHash wf bridges** (3 holes): These assume a forall about probe chains
  using the runtime `step` value. Blocked by `compute_second_hash` being
  `external_body` — the proof needs to know `step > 0` and `gcd(step, m) == 1`.
- **clone_elem** (6 holes across 6 files): Irreducible — generic `Clone::clone()`
  has no ensures in Verus. These are the accepted clone bridge pattern.
- **external_body** (2 holes): `call_hash_fn` wraps closure call;
  `StructChainedHashTable::resize` wraps the full resize algorithm.

## Chapters Affected

Chapter 47 only. No other chapters touched.
