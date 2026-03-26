# R84 Agent 1 — UnionFind wf decomposition + prove union_merge/union, STEP 20

## Objective

Refactor UnionFindStEph.rs so that internal functions use individual named
sub-predicates instead of the monolithic `spec_unionfindsteph_wf()`. Then prove
`union_merge` and `union`.

## WHY this refactoring is necessary

`spec_unionfindsteph_wf()` is a conjunction of 13 quantified predicates. Any
function with wf in both requires and ensures causes Z3 to instantiate ALL 13
quantifiers over ALL proof terms. `union_merge` modifies parent + rank + roots
simultaneously, tripling the term space. Z3 peaks at 17 GB and crashes.

The fix: each internal function lists only the 4-6 sub-predicates it actually
needs. Z3 never sees the full conjunction. Memory drops from 17 GB to ~3-4 GB.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh`.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

This is a refactoring task. Do NOT start by removing external_body and validating.
You will burn rlimit budget and get nowhere.

Instead:

### Step 1: Read and map dependencies (no edits, no validation)

Read `src/Chap65/UnionFindStEph.rs` fully. For each function, determine which
of the 13 named sub-predicates (lines 76-146) it actually uses. Build a table:

```
| Function          | Needs from requires        | Proves in ensures          |
|-------------------|---------------------------|---------------------------|
| find_root_loop    | parent_closed, rank_inc,  | (none — immutable)        |
|                   | parent_preserves_root,    |                           |
|                   | self_parent_is_root       |                           |
| insert            | all (calls lemma)         | all (lemma proves full)   |
| union_merge_lt    | parent_closed, roots_idem,| parent_closed, roots_idem,|
|                   | self_parent_is_root, ...  | self_parent_is_root, ...  |
```

### Step 2: Plan the signature changes (no edits, no validation)

For each function, write the new requires/ensures using sub-predicate names.
The 13 sub-predicates are:

```
spec_key_model::<V>()              // global, not per-instance
spec_feq_full::<V>()               // global, not per-instance
spec_parent_rank_same_dom(uf)
spec_roots_parent_same_dom(uf)
spec_roots_idempotent(uf)
spec_parent_closed(uf)
spec_roots_in_dom(uf)
spec_elements_forward(uf)
spec_elements_backward(uf)
spec_elements_distinct(uf)
spec_self_parent_is_root(uf)
spec_parent_preserves_root(uf)
spec_rank_increases(uf)
spec_rank_bounded(uf)
```

### Step 3: Edit ALL signatures at once, then validate

Change all internal functions (find_root_loop, lemma_root_is_self_parent,
lemma_insert_preserves_wf, lemma_non_root_rank_lt_root, union_merge, the 3
sub-lemmas) to use decomposed predicates.

Keep the TRAIT signatures using `spec_unionfindsteph_wf()` — external callers
(Kruskal) still use the monolithic wf. The trait impl's function bodies can
assert the individual sub-predicates from wf.

### Step 4: Remove external_body from union_merge and union

With decomposed predicates, each function should verify under rlimit(80) and
~3-4 GB Z3 instead of 17 GB.

## The admit for rank overflow

Line 718 has `admit()` for `rank_u + 1 <= usize::MAX`. Leave it. This needs a
2^rank theorem that's a separate proof task.

## Important

- Do NOT delete the monolithic `spec_unionfindsteph_wf()` — it's used by the trait
  and by Kruskal. Keep it as the public API. Decompose only the internals.
- Do NOT modify KrustalStEph.rs or PrimStEph.rs.
- Do NOT add new `assume` or `accept` (the existing admit for rank overflow stays).
- Do NOT weaken ensures clauses.

## STEP 20

## Report

Write `plans/agent1-round84-report.md`.
