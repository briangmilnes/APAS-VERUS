# Agent 1 — Round 84 Report

## Objective

Decompose UnionFindStEph wf into named sub-predicates and prove `union_merge` + `union`.

## Result

**5168 verified, 0 errors** (up from 5138 at R83 start).
`union_merge` and `union` retain `external_body`, but all proof components now verify separately.

## What Was Done

### Wf sub-predicate decomposition

Closed 10 quantified sub-predicates to prevent Z3 from instantiating them in exec functions:

| # | Sub-predicate | Status |
|---|---|---|
| 1 | `spec_roots_idempotent` | closed |
| 2 | `spec_parent_closed` | closed |
| 3 | `spec_roots_in_dom` | closed |
| 4 | `spec_elements_forward` | closed |
| 5 | `spec_elements_backward` | closed |
| 6 | `spec_elements_distinct` | closed |
| 7 | `spec_self_parent_is_root` | closed |
| 8 | `spec_parent_preserves_root` | closed |
| 9 | `spec_rank_increases` | closed |
| 10 | `spec_rank_bounded` | closed |

Four non-quantified predicates remain open: `spec_key_model`, `spec_feq_full`, `spec_parent_rank_same_dom`, `spec_roots_parent_same_dom`.

### New proof infrastructure (all verified)

| # | Function | Purpose |
|---|---|---|
| 1 | `UnionMergeInfo` | Ghost struct: winner/loser/winner_val from exec step |
| 2 | `lemma_decompose_wf` | Monolithic wf → 14 opaque sub-predicates |
| 3 | `lemma_assemble_wf` | 14 opaque sub-predicates → monolithic wf |
| 4 | `union_merge_exec` | Exec mutations only — no wf in scope. Returns `Ghost<UnionMergeInfo>` |
| 5 | `lemma_establish_union_pre` | Assembles `spec_union_lemma_pre` from individual facts |
| 6 | `lemma_union_wf_roots_closed` | Wraps `lemma_union_wf_roots` with closed interface |
| 7 | `lemma_union_wf_frame` | Frame: elements predicates transfer when unchanged |
| 8 | `spec_union_lemma_pre` | Now `closed` to keep quantifiers out of union_merge |

Existing sub-lemmas updated with `reveal` calls:
- `lemma_union_wf_parent`: +`reveal(spec_union_lemma_pre, spec_parent_closed, spec_self_parent_is_root, spec_parent_preserves_root)`
- `lemma_union_wf_ordering`: +`reveal(spec_union_lemma_pre, spec_rank_increases, spec_rank_bounded, spec_roots_idempotent)`

### Why union_merge and union still need external_body

**Root cause: 13+13 quantifier explosion.**

`spec_unionfindsteph_wf()` is an `open spec fn` with 13 quantified conjuncts. Any function that has wf in both pre-state and post-state gives Z3 26+ quantifiers interacting with all proof terms.

- `union_merge`: pre-state wf (from requires) + post-state wf (from ensures) = 26 quantifiers. Z3 peaked at 9.6 GB with rlimit(50), OOM'd at 17+ GB with rlimit(80+).
- `union`: pre-state wf (from find's ensures) + post-state wf (from union_merge's ensures) = 26 quantifiers. Same issue.

**The exec/proof split reduced Z3 from 18 GB to 9.6 GB** by isolating HashMap/Map axioms from quantified wf predicates. But 9.6 GB still exceeds the rlimit budget.

**Fundamental fix needed:** Make `spec_unionfindsteph_wf` itself `closed`, and refactor `find_root_loop` to use point-wise proof lemma calls instead of open quantifiers in its loop invariant. This eliminates all 26 quantifiers from every function's Z3 context. Estimated: 4-6 more iterations.

### Holes table

| # | Chap | File | Hole Type | Notes |
|---|---|---|---|---|
| 1 | 65 | UnionFindStEph.rs | admit | Rank overflow (2^rank theorem, left as instructed) |
| 2 | 65 | UnionFindStEph.rs | external_body | union_merge (Z3 26-quantifier limit) |
| 3 | 65 | UnionFindStEph.rs | external_body | union (blocked by union_merge) |

### Verification counts

- **Before (R83):** 5138 verified, 0 errors
- **After (R84):** 5168 verified, 0 errors (+30 from new proof infrastructure)
- **Chap65 clean proof functions:** 12 (was ~0 with external_body on everything)
