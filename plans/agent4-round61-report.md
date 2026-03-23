# Agent 4 — Round 61 Report

## Summary

Investigated Chap26 `point_distance` float arithmetic hole, ran 5x stability validation
(all clean), regenerated all analysis files, and produced daily proof table.

## Target 1: Chap26 ETSPMtEph.rs `point_distance` — structural assessment

The `point_distance` external_body at `src/Chap26/ETSPMtEph.rs:613` is irreducible with
current axioms.

**Why it can't be proved:**
- `spec_point_distance` is fully uninterpreted — no structural definition connecting to
  `x`, `y` coordinates
- The body computes `(dx*dx + dy*dy).sqrt()` using f64 subtraction, multiplication, and sqrt
- `vstdplus/float.rs` provides: total order axioms, `f64_add_spec` (commutative, identity,
  monotone), `f64_sub_spec` (declared, no axioms). That's all.
- **Missing**: `f64_mul_spec`, `f64_sqrt_spec`, non-negativity of `x*x`, non-negativity
  of sqrt, subtraction axioms

**Can we prove partial properties?**
- `result >= 0.0` — No. Requires sqrt non-negativity + `a*a >= 0` axioms, neither exists.
- Symmetry (`d(a,b) == d(b,a)`) — No. `spec_point_distance` is uninterpreted; can't reason
  about its structure.
- Identity (`d(a,a) == 0`) — No. Same reason.
- Triangle inequality — No. Same reason, plus needs addition monotonicity for sqrt.

**Could axioms help?** We *could* add broadcast axioms about `spec_point_distance` itself
(non-negativity, symmetry, identity of indiscernibles), but these would be axioms, not proofs.
They'd help callers but wouldn't eliminate the `external_body`. No callers currently need
these properties — the tour proof is structural (cycle/point membership), not distance-based.

**Verdict**: Correct pattern. No change possible without new float arithmetic axioms.

## Target 2: 5x Stability Validation

| # | Verified | Errors | Elapsed |
|---|----------|--------|---------|
| 1 | 4496     | 0      | 72s     |
| 2 | 4496     | 0      | 69s     |
| 3 | 4496     | 0      | 70s     |
| 4 | 4496     | 0      | 70s     |
| 5 | 4496     | 0      | 69s     |

No flakes detected.

## Target 3: Analysis Regeneration

All four analysis scripts run; 174 files updated:
- `scripts/all-holes-by-chap.sh` — proof holes per chapter
- `scripts/all-style-by-chap.sh` — style warnings per chapter
- `scripts/all-fn-impls-by-chap.sh` — function inventories per chapter
- `scripts/chapter-cleanliness-status.sh` — chapter cleanliness status

## Target 4: Daily Proof Table

| # | Round | Holes Start | Holes End | Delta | Clean Chaps | Dirty Chaps | Verified |
|---|-------|-------------|-----------|-------|-------------|-------------|----------|
| 1 | R59   | 24          | 18        | -6    | 41          | 5           | 4496     |
| 2 | R60   | 18          | 12        | -6    | 41          | 5           | 4496     |

Sources: R59 merge commit `16ec7888a` ("24→18"), R60 merge commit `5f79b7263` ("24→12"),
chapter-cleanliness-status.log at each merge point.

## Remaining Holes (12)

| # | Chap | File | Holes | Nature |
|---|------|------|-------|--------|
| 1 | 26   | ETSPMtEph.rs | 1 | f64 arithmetic (sqrt, mul) — structural |
| 2 | 43   | OrderedSetStPer.rs | 1 | internal dependency |
| 3 | 43   | OrderedSetStEph.rs | 1 | internal dependency |
| 4 | 45   | BinaryHeapPQStEph.rs | 1 | heap property |
| 5 | 47   | ParaHashTableStEph.rs | 2 | hash table wf |
| 6 | 53   | various | 6 | capacity/graph search |

## Validation

- **Validate**: 5/5 clean (4496 verified, 0 errors)
- **RTT**: 2610 passed, 0 skipped
- **PTT**: 147 passed, 0 skipped
