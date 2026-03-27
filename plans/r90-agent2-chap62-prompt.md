# R90 Agent 2 — StarPartitionMtEph + add r_inv to star_contract_mt, STEP 20

## Objective

Two tasks in Chap62:
1. Prove `parallel_star_partition` in StarPartitionMtEph.rs (1 hole)
2. Add `r_inv` ghost predicate to `star_contract_mt` in StarContractionMtEph.rs
   (unblocks Chap64 SpanTreeMtEph)

## Task 1: parallel_star_partition

### The Problem (from R89 agent4 report)

The proof body has a value-level vs view-level `no_duplicates` gap. `SetStEph::to_seq()`
ensures `seq@.no_duplicates()` at the VALUE level (`i != j => seq[i] != seq[j]`).
But the proof tries to derive contradictions from VIEW-level equality
(`vertices_vec@[i]@ == vertices_vec@[j]@`), which doesn't follow.

### Fix Strategy

Write a bridge lemma that for types satisfying `StT + Hash + Eq`, if a sequence
comes from `SetStEph::to_seq()`, then view-level no_duplicates holds too. The
argument: set membership is view-based (`contains` uses `@`), `to_seq` produces
elements from the set, and the set can't contain two elements with the same view.

Alternatively: rewrite the 6 loop invariants to work with value-level equality
instead of view-level. Where the proof says `if jjv == jv2` (comparing views),
change to comparing values directly.

Read `src/Chap62/StarPartitionStEph.rs` — the proved StEph version. See how it
handles the no_duplicates issue.

## Task 2: Add r_inv to star_contract_mt

### The Problem (from R89 agent3 SpanTree report)

`star_contract_mt` lacks the `r_inv` ghost predicate that `star_contract` (StEph)
has. The StEph version takes `Ghost(r_inv): Ghost<spec_fn(R) -> bool>` and ensures
`r_inv(result)`. This lets callers propagate arbitrary result invariants (like
`spec_setsteph_wf()`) through the recursion.

### Fix

Read `src/Chap62/StarContractionStEph.rs` and see how `star_contract` uses `r_inv`.
Add the same parameter to `star_contract_mt` and `star_contract_mt_fuel` (if it
exists). Thread it through the recursive calls and ensure `r_inv(result)` in the
ensures.

This is a signature change — update all callers:
- `src/Chap63/ConnectivityMtEph.rs` — `count_components_hof`, `connected_components_hof`
- `src/Chap64/SpanTreeMtEph.rs` — `spanning_tree_star_contraction_mt`

For ConnectivityMtEph callers, pass `Ghost(|_r| true)` as r_inv (they don't need
result invariants). For SpanTreeMtEph, pass `Ghost(|r: SetStEph<Edge<V>>| r.spec_setsteph_wf())`.

## Read first

- `src/Chap62/StarPartitionStEph.rs` — **proved StEph, your reference**
- `src/Chap62/StarPartitionMtEph.rs` — your file (proof body inside external_body)
- `src/Chap62/StarContractionStEph.rs` — **proved StEph with r_inv pattern**
- `src/Chap62/StarContractionMtEph.rs` — your file (add r_inv here)

## Isolation

```bash
scripts/validate.sh isolate Chap64
```

(Pulls in Chap62 + 63 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Prioritize Task 2 (r_inv) first — it's likely easier and unblocks Chap64.
- For Task 1, if the bridge lemma approach is too hard, try the value-level
  rewrite approach.
- Do NOT add assume or accept.
- You may modify ConnectivityMtEph.rs and SpanTreeMtEph.rs to update caller
  signatures for the r_inv parameter.

## STEP 20

## Report

Write `plans/agent2-r90-chap62-report.md`.
