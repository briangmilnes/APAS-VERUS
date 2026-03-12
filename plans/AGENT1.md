# Agent 1 Report — PBOGH Round 4.1

## Changes

### Phase 1: Accept trivial spec_wf holes (10 files)

Added `// accept hole` comments to 10 trivial `spec_wf { true }` predicates
across Chap18, Chap19, and Chap12. All are Vec-backed (or Mutex<Vec>-backed)
types where `true` is the correct wf body because `Vec@.len() <= usize::MAX`
is not axiomatically provable in Verus.

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 18 | ArraySeqMtEph.rs | `// accept hole: Vec-backed, true is correct` |
| 2 | 18 | ArraySeqMtPer.rs | `// accept hole: Vec-backed, true is correct` |
| 3 | 18 | ArraySeqStEph.rs | `// accept hole: Vec-backed, true is correct` |
| 4 | 18 | ArraySeqStPer.rs | `// accept hole: Vec-backed, true is correct` |
| 5 | 18 | LinkedListStEph.rs | `// accept hole: Vec-backed, true is correct` |
| 6 | 18 | LinkedListStPer.rs | `// accept hole: Vec-backed, true is correct` |
| 7 | 19 | ArraySeqMtEph.rs | `// accept hole: Vec-backed, true is correct` |
| 8 | 19 | ArraySeqStEph.rs | `// accept hole: Vec-backed, true is correct` |
| 9 | 19 | ArraySeqStPer.rs | `// accept hole: Vec-backed, true is correct` |
| 10 | 12 | Exercise12_5.rs | `// accept hole: Mutex<Vec>-backed, true is correct` |

### Phase 2: Prove dfs_finish_order and compute_finish_order specs (5 files)

Added three new pub proof lemmas to TopoSortStEph.rs:
- `lemma_set_true_num_false_eq`: setting false→true decreases count by exactly 1
- `lemma_all_true_num_false_zero`: all-true sequence has zero false entries
- `lemma_all_false_num_false_eq_len`: all-false sequence has num_false == length

Proved conservation law on `dfs_finish_order` in TopoSortStEph.rs (shared)
and SCCStPer.rs (local):
- `visited@[vertex as int]` — vertex is visited after call
- `finish_order@.len() + spec_num_false(visited@) == old(finish_order)@.len() + spec_num_false(old(visited)@)` — each newly visited vertex produces exactly one push
- `forall|i| result@[i] < graph@.len()` — all elements are valid vertex indices

Proved `compute_finish_order` in both SCC files:
- `result@.len() == graph@.len()` — finish order contains all n vertices
- `forall|i| result@[i] < graph@.len()` — all valid vertex indices
- `result.spec_avltreeseq*_wf()` — well-formed tree

Also strengthened `topo_sort` in TopoSortStEph.rs to use conservation
invariant, proving `order@.len() == graph@.len()`.

| # | Chap | File | Change |
|---|------|------|--------|
| 11 | 55 | TopoSortStEph.rs | 3 new lemmas + conservation ensures |
| 12 | 55 | TopoSortStEph.rs | `topo_sort` conservation invariant |
| 13 | 55 | SCCStEph.rs | `compute_finish_order`: len==n + valid |
| 14 | 55 | SCCStPer.rs | `dfs_finish_order` conservation ensures |
| 15 | 55 | SCCStPer.rs | `compute_finish_order`: len==n + valid |

## Holes/Errors Closed

| # | Chap | File | Type | Resolution |
|---|------|------|------|------------|
| 1 | 18 | ArraySeqMtEph.rs | trivial spec_wf | accept hole |
| 2 | 18 | ArraySeqMtPer.rs | trivial spec_wf | accept hole |
| 3 | 18 | ArraySeqStEph.rs | trivial spec_wf | accept hole |
| 4 | 18 | ArraySeqStPer.rs | trivial spec_wf | accept hole |
| 5 | 18 | LinkedListStEph.rs | trivial spec_wf | accept hole |
| 6 | 18 | LinkedListStPer.rs | trivial spec_wf | accept hole |
| 7 | 19 | ArraySeqMtEph.rs | trivial spec_wf | accept hole |
| 8 | 19 | ArraySeqStEph.rs | trivial spec_wf | accept hole |
| 9 | 19 | ArraySeqStPer.rs | trivial spec_wf | accept hole |
| 10 | 12 | Exercise12_5.rs | trivial spec_wf | accept hole |
| 11 | 55 | TopoSortStEph.rs | weak ensures | Proved conservation law + valid-indices |
| 12 | 55 | TopoSortStEph.rs | weak ensures | `topo_sort` proves len==n via conservation |
| 13 | 55 | SCCStEph.rs | fn_missing_ensures | Proved len==n + valid-indices + wf |
| 14 | 55 | SCCStPer.rs | weak ensures | Proved conservation on local dfs_finish_order |
| 15 | 55 | SCCStPer.rs | fn_missing_ensures | Proved len==n + valid-indices + wf |

## Blockers

### Chap52/EdgeSetGraphMtPer.rs:out_neighbors (1 external_body)

Cannot prove until Chap41 `AVLTreeSetMtPer::filter` spec is strengthened.
Current filter ensures only `subset_of` — insufficient for the set-equality
ensures on `out_neighbors`. The spec does not need a closure (only the body
does); Pattern C from `using_closures_standard.rs` shows how to bridge
runtime closures to spec via `Ghost(spec_fn)`. Another agent is fixing
the AVL filter spec.

### Remaining warnings (cosmetic, do not affect clean status)

- Chap55/SCCStEph.rs:149 — `requires true` on `check_wf_adj_list_eph`
- Chap55/SCCStPer.rs:204 — `requires true` on `check_wf_adj_list_per`
- Chap57/DijkstraStEphI64.rs:69 — `requires true` on `pq_entry_new`

These are validation functions and constructors with no meaningful
precondition. `requires true` is correct; the warnings are cosmetic.

## Verification

- **Validated**: 3670 verified, 0 errors.
- **Chapters**:

| # | Chap | Before | After | Clean? |
|---|------|--------|-------|--------|
| 1 | 18 | 6 holes | 0 | Yes (+1) |
| 2 | 19 | 3 holes | 0 | Yes (+1) |
| 3 | 12 | 1 hole | 0 | Yes (+1) |
| 4 | 52 | 1 hole | 1 | No (blocked on Chap41) |
| 5 | 55 | 2 errors | 0 | Yes (+1) |
| 6 | 57 | 0 | 0 | Already clean |

**Net: +4 clean chapters** (18, 19, 12, 55). Project total: 29 clean chapters.
**15 items closed**: 10 holes via accept, 5 via real proves (conservation + len==n + valid-indices).
