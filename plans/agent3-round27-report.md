# Agent 3 — Round 27 Report

## Summary

Wired per-module wf specs into all 3 flat hash table files via `spec_impl_wf` overrides.
Proved LinProb lookup using the real linear-probing wf spec (`spec_linprobflathashsteph_wf`),
replacing the R26 proof that relied on `spec_hashtable_wf` (wrong for open addressing).
Reverted QuadProb and DoubleHash lookups to `external_body` because R26 proofs used the
wrong wf and wrapping arithmetic overflow blocks reproving them with per-module wf.

Analyzed insert proof feasibility and found two design blockers affecting all 3 files.

## Verification

- 4106 verified, 0 errors
- 2613 RTT pass, 0 failures
- 147 PTT pass, 0 failures

## Holes Before/After (Chap47 flat hash tables)

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|:------:|:-----:|:-----:|-------|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 | Added spec_impl_wf (spec fn, no new holes) |
| 2 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 | 0 | lookup reproved with correct per-module wf |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 | 0 | spec_impl_wf added; lookup reverted to external_body |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 | 0 | spec_impl_wf added; lookup reverted to external_body |

Total Chap47: 37 before, 37 after (net 0 change).

The hole count is unchanged numerically, but the structural improvements are significant:

1. **spec_impl_wf wired**: All 3 flat hash tables now override `spec_impl_wf` to return
   their per-module wf spec, enabling future proofs to use the correct invariant.

2. **LinProb lookup is correctly proved**: R26's proof used `spec_hashtable_wf` which
   assumes each key resides at its hash slot — valid for chained tables but vacuously
   true for open addressing (where collisions displace keys). R27's proof uses
   `spec_linprobflathashsteph_wf` which encodes the real linear probe chain invariant.

3. **QuadProb/DoubleHash honestly reported**: R26's "proofs" were technically valid but
   vacuous (the wf they assumed is never satisfied for a real open-addressed table with
   collisions). R27 reverted them to `external_body` and added the correct `spec_impl_wf`.

## Techniques Used

1. **vstd modular arithmetic lemmas**: Used `lemma_small_mod(x, m)` (proves `x % m == x`
   when `x < m`) and `lemma_mod_add_multiples_vanish(b, m)` (proves `(m + b) % m == b % m`)
   from `vstd::arithmetic::div_mod` to prove slot computation.

2. **lemma_probe_mod_identity**: New helper proof function proving
   `(h + (j - h + m) % m) % m == j` for `0 <= h, j < m`. Uses vstd lemmas internally.
   Needed to connect spec-level probe positions back to array indices.

3. **Incremental probe-chain tracking**: Loop invariants track two properties over all
   attempts `d` from 0 to current attempt:
   - `!spec_flat_has_key(table@[(h+d)%m], key)` — key not at probe position d
   - `!(table@[(h+d)%m] is Empty)` — probe position d is not empty
   These connect to the wf's probe-chain integrity clause to derive contradictions.

4. **Multi-trigger contradiction**: The no-duplicate-keys quantifier uses two triggers.
   To instantiate it, the proof asserts `spec_flat_has_key` at both slot i and slot j,
   then derives a contradiction.

## Insert Proof Analysis (Blocked — Design Issues)

Two design issues prevent proving insert on any flat hash table:

### 1. Deleted-entry duplicate bug

Current insert code exits at the first non-Occupied slot (Empty OR Deleted):
```rust
} else {  // matches both Empty and Deleted
    table.table.set(slot, FlatEntry::Occupied(key, value));
    ...
    return;
}
```

If key K exists at slot `(h+3)%m` and slot `(h+1)%m` is Deleted, insert places K at
`(h+1)%m` — creating a duplicate that violates the no-duplicate-keys wf invariant.

**Fix**: Two-pass insert (first scan for existing key, then insert at first available),
or single-pass with deferred insert slot.

### 2. Missing load factor precondition

The trait's insert requires only `num_elements < usize::MAX`. If the table is full (all
slots Occupied or Deleted), the loop exhausts without inserting, violating `ensures
table@ == old(table)@.insert(key, value)`.

**Fix**: Add precondition like `num_elements < current_size` or `exists_empty_slot(table)`.

Both fixes require design changes to `ParaHashTableStEphTrait` and/or the insert implementations.

## Remaining Holes (Flat Hash Tables)

Per file:
- **insert**: `external_body` — blocked by design issues above
- **delete**: `external_body` — requires insert proof pattern + tombstone reasoning
- **resize**: `external_body` — rehashing proof, complex but not design-blocked
- **lookup** (QuadProb/DoubleHash only): `external_body` — `wrapping_mul` overflow blocks
  relating exec slot to spec probe position

**LinProb lookup**: Proved (1 eq bridge assume in PartialEq::eq pattern).

## Files Modified

- `src/Chap47/ParaHashTableStEph.rs` — Added `spec_impl_wf` to trait (previous session)
- `src/Chap47/LinProbFlatHashTableStEph.rs` — Added `spec_impl_wf` override, `lemma_probe_mod_identity`, reproved lookup with per-module wf
- `src/Chap47/QuadProbFlatHashTableStEph.rs` — Added `spec_impl_wf` override, reverted lookup to external_body
- `src/Chap47/DoubleHashFlatHashTableStEph.rs` — Added `spec_impl_wf` override, reverted lookup to external_body
