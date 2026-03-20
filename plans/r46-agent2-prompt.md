# R46 Agent 2: Chap59 + Chap41 + Chap43 (14 holes)

## Assignment

Close remaining holes in Johnson APSP, AVLTreeSet, and OrderedTable/OrderedSet.

## Baseline

69 holes total. 4396 verified. Your chapters: Chap59 (2), Chap41 (6), Chap43 (6).

## Target Holes

### Chap59 — Johnson APSP (2 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 59 | JohnsonMtEphI64.rs | johnson_apsp_mt | 88 | external_body | Mt delegation |
| 2 | 59 | JohnsonStEphI64.rs | johnson_apsp | 329 | assume | Graph size bound |

Also fix warnings:
- JohnsonStEphI64.rs: `adjust_distance` fn_missing_requires (line 73)
- JohnsonStEphI64.rs: `reweight_edge` fn_missing_requires (line 89)

The Mt version likely delegates. The St assume is about result graph size after
BellmanFord — may need a lemma.

### Chap41 — AVLTreeSet (6 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | intersection | 1085 | assume | usize::MAX bound |
| 2 | 41 | AVLTreeSetStEph.rs | difference | 1352 | assume | usize::MAX bound |
| 3-6 | 41 | Example41_3.rs | 4 functions | — | external_body | SKIP (Example file) |

Only 2 real holes. The usize::MAX assumes guard against Vec overflow during
intersection/difference. These may need a requires clause bounding input sizes,
or a proof that the result is smaller than either input.

### Chap43 — OrderedTable + OrderedSet (6 holes)

| # | Chap | File | Function | Line | Type | Notes |
|---|------|------|----------|------|------|-------|
| 1 | 43 | AugOrderedTableMtEph.rs | — | 671 | external_body | Check function |
| 2 | 43 | AugOrderedTableStPer.rs | lemma_reducer_clone_total | 117 | proof_fn_with_holes | Closure totality |
| 3 | 43 | AugOrderedTableStPer.rs | — | 124 | assume | Closure requires |
| 4 | 43 | OrderedSetStEph.rs | — | 1117 | assume | Filter + TotalOrder |
| 5 | 43 | OrderedSetStPer.rs | — | 1031 | assume | Filter + TotalOrder |
| 6 | 43 | OrderedTableStPer.rs | — | 2798-2815 | external_body (x2) | Check functions |

Read the files to identify exact functions. The filter assumes in OrderedSet
may need a lemma about filter preserving the TotalOrder property.

Also fix fn_missing_requires warnings:
- OrderedSetStEph.rs: `from_sorted_elements` (line 1385)
- OrderedSetStPer.rs: `from_sorted_elements` (line 1157)

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT convert assume() to accept().

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap59/ src/Chap41/ src/Chap43/`.
Write your report to `plans/agent2-round46-report.md`.
