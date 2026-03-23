# Fold `.finite()` into `spec_*_wf()` — Plan

## Problem

`.dom().finite()` (for Map-backed types) and `.finite()` (for Set-backed types)
is scattered across ~150 ensures clauses in Chap41–43. Some modules correctly
include finite in their wf predicate; others don't, forcing every function to
restate it. The gap cascades: if OrderedTable's wf is missing finite,
AugOrderedTable must restate it too.

## Audit Results

| # | Chap | File                      | View  | WF has finite? | ensures count | Status         |
|---|------|---------------------------|-------|----------------|---------------|----------------|
| 1 |   41 | AVLTreeSetStEph.rs        | Set   | YES            |  0            | OK             |
| 2 |   41 | AVLTreeSetStPer.rs        | Set   | YES            |  0            | OK             |
| 3 |   41 | AVLTreeSetMtEph.rs        | Set   | YES            |  1            | redundant      |
| 4 |   41 | AVLTreeSetMtPer.rs        | Set   | YES            |  8            | redundant      |
| 5 |   41 | ArraySetStEph.rs          | Set   | YES            |  2            | redundant      |
| 6 |   41 | ArraySetEnumMtEph.rs      | Set   | NO             |  3            | **fix wf**     |
| 7 |   42 | TableStEph.rs             | Map   | NO             |  2            | **fix wf**     |
| 8 |   42 | TableStPer.rs             | Map   | NO             |  2            | **fix wf**     |
| 9 |   42 | TableMtEph.rs             | Map   | NO             |  2            | **fix wf**     |
|10 |   43 | OrderedSetStEph.rs        | Set   | inherited      |  4            | redundant      |
|11 |   43 | OrderedSetStPer.rs        | Set   | inherited      | 22            | redundant      |
|12 |   43 | OrderedSetMtEph.rs        | Set   | YES            | 14            | redundant      |
|13 |   43 | OrderedTableStEph.rs      | Map   | NO             | 30+           | **fix wf**     |
|14 |   43 | OrderedTableStPer.rs      | Map   | NO             |  5+           | **fix wf**     |
|15 |   43 | OrderedTableMtEph.rs      | Map   | YES            | 21            | redundant      |
|16 |   43 | OrderedTableMtPer.rs      | Map   | YES            | 12            | redundant      |
|17 |   43 | AugOrderedTableStEph.rs   | Map   | cascades       | 19            | cascades       |
|18 |   43 | AugOrderedTableStPer.rs   | Map   | cascades       | 14            | cascades       |
|19 |   43 | AugOrderedTableMtEph.rs   | Map   | YES            | 47            | redundant      |

**Total redundant ensures to remove: ~200+**

## Dependency Cascade

```
Layer 0 — base types (fix these first)
  Ch42 TableStEph         add self@.dom().finite() to wf
  Ch42 TableStPer         add self@.dom().finite() to wf
  Ch42 TableMtEph         add self@.dom().finite() to wf (or check if already there)
  Ch41 ArraySetEnumMtEph  add self@.finite() to wf

Layer 1 — types that wrap Layer 0 (fix or inherit)
  Ch43 OrderedTableStEph  add self@.dom().finite() to wf
  Ch43 OrderedTableStPer  add self@.dom().finite() to wf

Layer 2 — types that wrap Layer 1 (cascade — no wf change needed)
  Ch43 AugOrderedTableStEph    wf delegates to OrderedTable wf → gets finite for free
  Ch43 AugOrderedTableStPer    same

Layer 3 — Mt wrappers (already have finite in wf, just cleanup ensures)
  All MtEph/MtPer files
```

## Execution Plan

### Phase 1: Add finite to wf predicates (6 files)

These files need `finite()` added to their `spec_*_wf()` predicate, then prove
it holds in every function that ensures wf. The proof typically requires calling
`lemma_entries_to_map_finite` or similar.

| # | File                          | Add to wf                    |
|---|-------------------------------|------------------------------|
| 1 | Ch42/TableStEph.rs            | `self@.dom().finite()`       |
| 2 | Ch42/TableStPer.rs            | `self@.dom().finite()`       |
| 3 | Ch42/TableMtEph.rs            | `self@.dom().finite()`       |
| 4 | Ch41/ArraySetEnumMtEph.rs     | `self@.finite()`             |
| 5 | Ch43/OrderedTableStEph.rs     | `self@.dom().finite()`       |
| 6 | Ch43/OrderedTableStPer.rs     | `self@.dom().finite()`       |

**Risk**: Medium. Every function that ensures `spec_*_wf()` now implicitly
ensures finite. Functions must prove finite holds for their output. Most already
do (they have `ensures ..., self@.dom().finite()` separately), so the proof
work is the same — we're just moving WHERE it's stated.

**Validation**: `scripts/validate.sh` after each file. The verified count should
stay at 4489+ with 0 errors. If a function can't prove finite, it means the
current ensures was lying (claiming finite without proof), and we've found a bug.

### Phase 2: Remove redundant ensures (all 19 files)

For every file in the table above, remove `.finite()` / `.dom().finite()` from
ensures clauses where wf already guarantees it. Rules:

1. If `ensures ..., self.spec_*_wf()` is present, remove `self@.dom().finite()`
   or `self@.finite()` from the same ensures.
2. If the function returns a NEW object (e.g., `split.0`, `range`), and that
   object also has `ensures ..., split.0.spec_*_wf()`, remove the redundant
   finite on that too.
3. Keep `.finite()` in ensures ONLY when:
   - The function does NOT ensure wf on the returned/modified object.
   - The type is not the module's own type (e.g., returning a bare `Set` or `Map`
     that has no wf predicate).

**Validation**: `scripts/validate.sh` after each file. Callers that relied on the
ensures `finite()` must now derive it from the ensures `wf()`. Since wf is `open
spec fn`, Verus unfolds it and sees the finite conjunct.

### Phase 3: Update traits

For the trait declarations (the `spec fn spec_*_wf(&self) -> bool;` line), no
change is needed — the trait declares the signature, the impl provides the body.
But doc comments on the trait's wf spec should note that wf implies finite.

### Phase 4: Validate cascade

After Phase 1+2, verify:
- AugOrderedTable files: their wf delegates to OrderedTable wf, so they get
  finite for free. Remove redundant ensures.
- Mt files: same cascade check.
- `scripts/validate.sh` + `scripts/rtt.sh` + `scripts/ptt.sh` all clean.

## NOT Touched

- AVLTreeSetStEph/StPer (Ch41) — already correct.
- ArraySetStEph (Ch41) — already has finite in wf.
- OrderedSetStEph/StPer (Ch43) — inherits finite from AVLTreeSetStEph wf
  transitively. The ensures are redundant but harmless; clean up in Phase 2.

## Ordering Constraint

**Phase 1 MUST complete before Phase 2.** Removing ensures before adding to wf
would break callers that depend on the ensures clause for finite.

Layer 0 files (Ch42 Tables) must be fixed before Layer 1 (Ch43 OrderedTables),
because OrderedTable's wf proof may need to invoke TableStEph's wf to get finite.
Actually, OrderedTable wraps AVLTreeSeqStEph (not TableStEph), so the layers are
independent. But it's cleaner to go bottom-up.

## Agent Assignment

This is mechanical refactoring. One agent, one round. Estimated: ~45 minutes for
Phase 1 (6 files × validate), ~30 minutes for Phase 2 (remove ensures, validate).
