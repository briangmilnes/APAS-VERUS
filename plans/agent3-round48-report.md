# Agent 3 — Round 48 Report

## Summary

Closed 1 Chap47 hash table hole. Proved DoubleHash insert's `assume(false)` via
`lemma_spec_second_hash_value` (exposing the closed spec body = 1 from within its module)
plus `lemma_probe_mod_identity`, showing double hashing degenerates to linear probing at
the spec level and all m positions are visited. Added empties-tracking to DoubleHash resize.

Baseline: 38 holes, 4419 verified.
Result: 37 holes, 4422 verified.

## Holes Before/After

| # | Chap | File | Before | After | Notes |
|---|------|------|--------|-------|-------|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | 1 | 0 | assume(false) closed via proof-by-contradiction |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 1 | 1 | assume(false) remains; see analysis below |
| 3 | 47 | ParaHashTableStEph.rs | 2 | 2 | clone_elem assume + call_hash_fn external_body (irreducible) |

## Changes Made

### ParaHashTableStEph.rs (Chap47)
- Added `lemma_spec_second_hash_value` proof fn that exposes the body of the closed
  `spec_second_hash` (= 1) from within its module. This avoids needing `#[verifier::opaque]`
  (which veracity counts as a hole) while still making the value available to other modules.

### DoubleHashFlatHashTableStEph.rs (Chap47)
- Added `lemma_probe_mod_identity` proof fn (duplicated from LinProb) for modular arithmetic.
- Overrode `spec_has_insert_capacity`: exists an Empty slot in the table.
- Overrode `spec_resize_ok`: new_size > current_size.
- Replaced `assume(false)` with proof-by-contradiction:
  1. `lemma_spec_second_hash_value` gives `step == 1`.
  2. With step == 1, double hash probe `(h + d*1) % m` == `(h + d) % m` is linear probing.
  3. `lemma_probe_mod_identity` maps each position j to a probe attempt d.
  4. Loop invariant says all m probe positions are non-Empty.
  5. `spec_has_insert_capacity` says an Empty exists — contradiction.
- Added `pairs@.len() <= i` invariant to resize Phase 1.
- Added empties-tracking to resize Phase 3 (same pattern as LinProb):
  `lemma_all_empties_count`, `lemma_empties_positive_implies_exists_empty`,
  `lemma_one_slot_change_empties`.

## QuadProb Analysis

The quadratic probe sequence `(h + d²) % m` does NOT visit all m positions for m ≥ 3.
For example, d² mod 7 ∈ {0, 1, 2, 4} — only 4 of 7 positions. This means the loop CAN
exhaust all m attempts without finding an Empty, even if one exists at an unprobed position.

The `assume(false)` covers a genuine runtime possibility, not just a proof gap. To eliminate
it would require either:
1. A table-size constraint (m prime + load factor < 0.5 guarantees quadratic probing visits
   enough positions by the quadratic residue theorem).
2. A stronger `spec_has_insert_capacity` that says "for all h, the probe sequence has an
   Empty" — but maintaining this through resize requires number-theoretic lemmas about
   quadratic residues mod m.
3. Changing the probe formula to triangular numbers `(h + (d² + d)/2) % m` with power-of-2
   table sizes, which visits all m positions.

None of these is a small change. The `assume(false)` is a legitimate irreducible hole
without structural changes to the probing strategy or table-size management.

## Remaining Chap47 Holes (3)

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 47 | QuadProbFlatHashTableStEph.rs | assume(false) at fallthrough | Quadratic residue coverage; needs table-size constraints |
| 2 | 47 | ParaHashTableStEph.rs | assume() in clone_elem | Verus Clone bridge limitation (irreducible) |
| 3 | 47 | ParaHashTableStEph.rs | external_body on call_hash_fn | Verus Fn closure calling limitation (irreducible) |

## Techniques Used

- **Closed-to-lemma pattern**: Instead of making `spec_second_hash` opaque (which veracity
  counts as a hole), added a proof lemma in the same module that exposes the closed fn's
  body via its ensures clause. Other modules call the lemma to learn the value.
- **Proof by contradiction**: Same structure as R46 LinProb proof — loop invariant says all
  positions non-Empty, precondition says Empty exists, contradiction.
- **Empties-tracking for resize**: Counting spec + inductive lemmas to maintain
  `spec_has_insert_capacity` through the reinsertion loop.

## Verification

- 4422 verified, 0 errors
- 2613 RTT passed, 0 skipped
- 37 total project holes (was 38)
