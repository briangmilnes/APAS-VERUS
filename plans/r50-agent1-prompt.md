# Agent 1 — Round 50

## Primary Target: Chap43 holes (5 real holes across 4 files)

| # | Chap | File | Line | Type | Description |
|---|------|------|------|------|-------------|
| 1 | 43 | AugOrderedTableMtEph.rs | 672 | external_body | Prove the body |
| 2 | 43 | AugOrderedTableStPer.rs | 117 | proof_fn_with_holes | lemma_reducer_clone_total — closure assume |
| 3 | 43 | OrderedSetStEph.rs | 1134 | assume (algorithmic) | filter assume with exists quantifier |
| 4 | 43 | OrderedSetStPer.rs | 1031 | assume (algorithmic) | filter assume with exists quantifier (same pattern as #3) |
| 5 | 43 | OrderedTableMtPer.rs | 321 | assume (algorithmic) | assume(len < usize::MAX) — capacity bounds |

Also fix `OrderedSetStPer.rs:1157` fn_missing_requires on `from_sorted_elements`.

## Secondary Target: Chap57 + Chap59 int graph file warnings (3 fn_missing_requires)

These are verified graph algorithms (Dijkstra, Johnson APSP) with integer weights. They are
real, substantial algorithm implementations — not scaffolding. The 3 warnings are trivial:
add real requires clauses (not `requires true`, not tautologies).

| # | Chap | File | Line | Warning | Fix |
|---|------|------|------|---------|-----|
| 1 | 57 | DijkstraStEphU64.rs | 104 | fn_missing_requires | pq_entry_new — read the function, add real precondition |
| 2 | 59 | JohnsonStEphI64.rs | 73 | fn_missing_requires | adjust_distance — read the function, add real precondition |
| 3 | 59 | JohnsonStEphI64.rs | 89 | fn_missing_requires | reweight_edge — read the function, add real precondition |

## Standards to Read First

1. `src/standards/using_closures_standard.rs` — for lemma_reducer_clone closure assume
2. `src/standards/partial_eq_eq_clone_standard.rs` — for Clone bridge pattern
3. `src/standards/capacity_bounds_standard.rs` — for the len < usize::MAX assume

## Approach

1. **Start with Chap57+59 warnings** — quick wins, 15 minutes. Read each function body,
   understand what it assumes about its inputs, write the real requires clause.
2. **OrderedTableMtPer.rs:321** — capacity bounds assume. You did capacity bounds work in
   R48. Apply the same pattern: propagate the bound from callers.
3. **OrderedSetStEph.rs:1134 + OrderedSetStPer.rs:1031** — paired filter assumes with
   exists quantifier. These are the same proof obligation in two files. Prove it once,
   replicate. The assume says the filter result matches a spec-level filter with an
   existential witness.
4. **AugOrderedTableStPer.rs lemma_reducer_clone_total** — closure assume for Clone.
   Read the closures standard. The assume says the cloned closure preserves requires.
5. **AugOrderedTableMtEph.rs:672** — external_body. Read the function, understand what
   it does, write the verified body.
6. **Validate after each fix**: `scripts/validate.sh`.

## Constraints

- Do NOT modify Example files (Example41_3.rs — those 4 holes are skipped per CLAUDE.md)
- Do NOT add new assumes, external_body, or accept()
- Do NOT weaken ensures clauses
- Do NOT add `requires true` or tautological requires
- Each StEph/StPer/MtEph file is standalone — no cross-imports

## Success Criteria

- 3 fn_missing_requires warnings fixed (Chap57+59)
- Net hole reduction in Chap43 (target: -2 or more from the 5 holes)
- 0 verification errors, 0 RTT failures, 0 PTT failures
