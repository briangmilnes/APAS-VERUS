<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 50 Report

## Commits

| # | Hash | Description |
|---|---|---|
| 1 | `ee277f878` | Chap47 feq broadcast + obeys_feq_clone on lookup/delete/resize |
| 2 | `7e8764a1b` | Chap65 verified selection sort + prim_mst spec strengthening |

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:---:|---|:---:|:---:|:---:|
| 1 | 47 | ParaHashTableStEph.rs | 3 | 2 | -1 |
| 2 | 65 | KruskalStEph.rs | 1 | 0 | -1 |
| 3 | 65 | PrimStEph.rs | 1 | 1 | 0 |

**Total holes removed: 2** (1 assume → fixed via feq propagation approach; 1 external_body → replaced with verified sort)

## Warnings Before / After

| # | Chap | File | Warn Before | Warn After | Notes |
|---|:---:|---|:---:|:---:|---|
| 1 | 47 | ParaHashTableStEph.rs | 8 | 8 | wf naming refactor user-owned |
| 2 | 65 | KruskalStEph.rs | 0 | 1 | fn_missing_requires on sort (no real precondition; false positive) |
| 3 | 65 | PrimStEph.rs | 1 | 1 | pq_entry_new fn_missing_requires (no real precondition) |

## Work Done

### Chap47 ParaHashTableStEph.rs

- Added `broadcast use group_feq_axioms` to the module.
- Added `requires obeys_feq_clone::<Key>()` and `requires obeys_feq_clone::<Value>()` to `lookup`, `delete`, and `resize` trait methods. Callers must now supply the clone-view guarantee (satisfied since `Key: StT` and `Value: StT` imply `Eq`).
- The `clone_elem` assume hole (line 123) was investigated. Propagating `T: Eq + Clone` to `clone_elem` caused cascading `E0277` errors throughout `LinkedListChainedHashTableStEph`, `VecChainedHashTableStEph`, and `StructChainedHashTable`. Those helpers have `T: Clone` only and are incompatible with the `Eq` requirement for `obeys_feq_clone`. The assume remains; the architectural fix requires the user's `spec_impl_wf` refactor.
- The `call_hash_fn` external_body (line 501) was assessed: hashing a value is not verifiable through Rust's `Hash` trait with current vstd specs. Hole remains.
- The 8 `fn_missing_wf` warnings are syntactic: the code uses `Self::spec_impl_wf(table)` (a trait method that delegates to `spec_hashtable_wf`) rather than the pattern veracity expects. User is performing the wf naming/abstraction refactor.

### Chap65 KruskalStEph.rs

- Replaced `external_body sort_edges_by_weight` (using `Vec::sort_by` with no vstd spec) with a **verified selection sort**.
- Imports added: `use std::cmp::Ordering` and `use crate::vstdplus::pervasives_plus::pervasives_plus::vec_swap`.
- Algorithm: outer loop selects minimum of `[i..n)` by weight, swaps it to position `i`.
- Loop invariants:
  - Outer: sorted prefix `[0..i)`, prefix ≤ suffix, all elements from original.
  - Inner: `min_idx` tracks minimum in `[i..j)`.
- Proof hints: `WrappedF64::transitive` calls for transitivity in the less-than branch and in the post-swap outer invariant re-establishment.
- Verified with `broadcast use group_float_finite_total_order`.

### Chap65 PrimStEph.rs

- Added `requires spec_labgraphview_wf(graph@), obeys_key_model::<V>(), valid_key_type_LabEdge::<V, WrappedF64>()` and `ensures result.spec_setsteph_wf()` to `prim_mst` external_body free function.
- Added `ensures result.spec_setsteph_wf()` to `PrimStEphTrait::prim_mst` method.
- The `prim_mst` body uses complex control flow (nested while + for loops, priority queue, visited set) with no obvious termination argument for the outer while loop. Full algorithmic correctness proof is deferred. External_body retained.
- `pq_entry_new` fn_missing_requires: genuinely no precondition (struct constructor). Cannot add `requires true` or `// veracity: no_requires`. Warning remains.

## Verification

| Suite | Result |
|---|---|
| `scripts/validate.sh` | 4465 verified, 0 errors |
| `scripts/rtt.sh` | 2611 tests, 0 failed |
| `scripts/ptt.sh Chap65` | 0 tests (no PTTs for Chap65) |

## Remaining Holes

| # | Chap | File | Line | Hole | Blocks |
|---|:---:|---|:---:|---|---|
| 1 | 47 | ParaHashTableStEph.rs | 123 | assume(c == *x) clone bridge | `T: Clone` not `Eq`; needs user wf refactor |
| 2 | 47 | ParaHashTableStEph.rs | 501 | external_body call_hash_fn | Rust Hash trait not verifiable via vstd |
| 3 | 65 | PrimStEph.rs | 96 | external_body prim_mst | Full Prim correctness proof + termination arg |
