# Agent 1 Round 59 Report

## Summary

Proved both `assume(false)` holes in `QuadProbFlatHashTableStEph.rs`, closing
Chap47's QuadProb module to 0 holes. Assessed ETSPMtEph.rs external_body as
structural (float arithmetic boundary).

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 47 | QuadProbFlatHashTableStEph.rs | 2 | 0 | -2 |
| 2 | 26 | ETSPMtEph.rs | 1 | 1 | 0 |

**Net: -2 holes.**

## Targets

### Target 1: `lemma_triangular_injective` (Chap47, line 142) — PROVED

Triangular numbers T(i) = i*(i+1)/2 are injective mod 2^k for 0 <= i < j < 2^k.

**Technique — 2-adic parity argument:**
1. If T(i) % m == T(j) % m then (j-i)*(j+i+1) % 2m == 0 where m = 2^k.
2. Exactly one of (j-i) and (j+i+1) is odd (their sum is 2j+1, odd).
3. New helper `lemma_odd_factor_pow2`: if a is odd and 2^n | a*b, then 2^n | b.
   Proved by induction on n using `mul_mod_noop` and parity decomposition.
4. The even factor must be divisible by 2^(k+1), but both factors are bounded
   strictly below 2^(k+1). Contradiction.

Key Z3 assist: `by (nonlinear_arith)` for multiplication associativity/commutativity
chains and the final `a + b == 2*j + 1` derivation.

### Target 2: `lemma_empty_slot_reachable` (Chap47, line 160) — PROVED

Probe exhaustion is unreachable: if all m triangular probe positions are non-Empty,
but an Empty slot exists, contradiction.

**Technique — pigeonhole via set cardinality:**
1. Build probe sequence `Seq::new(m, |d| spec_tri_probe(h, d, m))`.
2. `no_duplicates()` from Target 1 + `lemma_mod_add_cancel` (new helper proving
   adding a constant preserves mod inequality).
3. `unique_seq_to_set` gives `probes.to_set().len() == m`.
4. All probes are in `[0, m)`, so `probes.to_set() ⊆ set_int_range(0, m)`.
5. `lemma_int_range(0, m)` gives `|set_int_range(0, m)| == m`.
6. `lemma_subset_equality` gives `probes.to_set() == set_int_range(0, m)`.
7. The Empty slot `s` is in `set_int_range(0, m)` hence in the probe set, but
   all probes are non-Empty. Contradiction.

**Supporting changes:**
- Added `spec_has_insert_capacity` override requiring exists-Empty slot
  (matching LinProb/DoubleHash pattern).
- Strengthened `spec_resize_ok` to require `new_size > table.current_size`.
- Added empties-tracking helpers (`spec_count_empties`, `lemma_all_empties_count`,
  `lemma_empties_positive_implies_exists_empty`, `lemma_one_slot_change_empties`)
  to prove `spec_has_insert_capacity` holds during resize reinsertion loop.
- Updated resize loop invariant with `spec_count_empties` tracking.
- Updated insert exhaustion proof to extract exists-Empty witness from
  `spec_has_insert_capacity` precondition.
- Fixed two `==>` warnings in assert-forall (changed to `implies`).

### Target 3: `ETSPMtEph.rs:612` external_body — STRUCTURAL, LEFT ALONE

`point_distance` computes Euclidean distance using `f64` subtraction, multiplication,
and `sqrt()`. The spec function `spec_point_distance` is `uninterp` (no body). Verus
has no floating-point arithmetic axioms. This is a float-arithmetic boundary identical
to the Chap56-59 graph algorithm pattern. Not algorithmic logic.

## Verification

- `scripts/validate.sh`: 4495 verified, 1 error (pre-existing Chap43/OrderedSetStPer.rs:910)
- `scripts/rtt.sh`: 2610 tests passed, 0 skipped
- `scripts/holes.sh src/Chap47/QuadProbFlatHashTableStEph.rs`: 0 holes, 9 clean proof fns

## Chapters Closed

Chap47 QuadProbFlatHashTableStEph.rs: **CLOSED** (0 holes, 9 clean proof functions).
