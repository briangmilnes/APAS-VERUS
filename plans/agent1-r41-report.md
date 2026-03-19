# Agent 1 — Round 41 Report

## Summary

Verified: 4281 → 4298 (+17). Errors: 0. RTT: 2613/2613 passed.
Chap43 OrderedTableStEph: 18 → 11 holes (−7 external_body removed).
Chap43 OrderedTableMtEph: 1 → 0 holes (−1 external_body removed).

## Work Completed

### Part A: OrderedTableStEph.rs proofs (7 external_body removed)

| # | Chap | File | Function | Status | Technique |
|---|------|------|----------|--------|-----------|
| 1 | 43 | OrderedTableStEph.rs | reduce | Proven | Direct iteration, no map proof needed |
| 2 | 43 | OrderedTableStEph.rs | from_sorted_entries | Proven | Recursive split, spec_keys_no_dups propagation |
| 3 | 43 | OrderedTableStEph.rs | map | Proven | clone_plus whole pair, lemma_cloned_view_eq |
| 4 | 43 | OrderedTableStEph.rs | delete | Proven | Deterministic position mapping (found_at) |
| 5 | 43 | OrderedTableStEph.rs | difference | Proven | Ghost kept sequence, key_in_other helper |
| 6 | 43 | OrderedTableStEph.rs | restrict | Proven | Ghost kept sequence, ArraySetStEph::find |
| 7 | 43 | OrderedTableStEph.rs | subtract | Proven | Ghost kept sequence, ArraySetStEph::find |

### Part B: RTT fix

- Root cause: insert pushed updated entries at END of Vec, destroying key order.
  After R39 restructure to AVLTreeSeqStEphS backing, this broke non-commutative reduce.
- Fix: Modified insert to push replacement in-place during scan.
- Unignored `test_string_concatenation_multithreaded` in TestAugOrderedTableMtEph.rs.
- Fixed StEph test assertion from `"HelloWorldBeautiful "` to `"HelloBeautiful World"`.

### Part C: MtEph from_sorted_entries (1 external_body removed)

- Removed external_body, calls StEph's proven from_sorted_entries + from_st wrapper.
- Used full qualified path to avoid name shadowing.
- gated `spec_keys_no_dups` import behind verus compilation context.

### Helper function added

- `key_in_other<K, V>(seq, k) -> bool`: Scans AVL sequence for key existence using only
  `obeys_view_eq::<K>()`. No value cloning needed. Used by difference.

## Remaining Holes in OrderedTableStEph.rs (11)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 43 | OrderedTableStEph.rs | avl_seq_length | Intentional wrapper (no wf req) |
| 2 | 43 | OrderedTableStEph.rs | avl_seq_nth | Intentional wrapper (no wf req) |
| 3 | 43 | OrderedTableStEph.rs | insert | usize::MAX edge (new key → len+1) |
| 4 | 43 | OrderedTableStEph.rs | domain | feq_clone vs feq_full mismatch |
| 5 | 43 | OrderedTableStEph.rs | tabulate | usize::MAX edge (keys.len ≤ max) |
| 6 | 43 | OrderedTableStEph.rs | filter | Z3 completeness invariant |
| 7 | 43 | OrderedTableStEph.rs | intersection | Closure value-correctness existential |
| 8 | 43 | OrderedTableStEph.rs | union | Closure value-correctness existential |
| 9 | 43 | OrderedTableStEph.rs | get_key_range | Complex TotalOrder slicing |
| 10 | 43 | OrderedTableStEph.rs | split_key | Complex TotalOrder partitioning |
| 11 | 43 | OrderedTableStEph.rs | rank_key | TotalOrder counting |
| 12 | 43 | OrderedTableStEph.rs | select_key | TotalOrder indexed access |
| 13 | 43 | OrderedTableStEph.rs | next (iterator) | Iterator::next can't have requires |

Note: items 1-2 are intentional convenience wrappers. Item 13 is a Verus limitation.

## Key Proof Patterns

1. **Ghost kept sequence**: Track source indices of kept entries as `ghost mut kept: Seq<int>`.
   Invariants: strictly increasing, entries match source, completeness via explicit
   existential witness (`assert(kept[new_idx] == q)` after push).

2. **Completeness existential maintenance**: Z3 loses existential witnesses across loop
   iterations. Fix: explicit `choose` witness from old sequence + `assert(kept[witness] == q)`
   for extended sequence. Critical for difference/restrict/subtract.

3. **key_in_other helper**: Avoids calling `find()` which requires `obeys_feq_full::<V>()`.
   Only needs `obeys_view_eq::<K>()` for key comparison.

4. **Clone whole pair**: For map, clone entire `Pair<K,V>` via `clone_plus` + `lemma_cloned_view_eq`
   instead of cloning K and V separately. Avoids needing `obeys_feq_full::<K>()` independently.

## Blockers Identified

- **usize::MAX boundary**: `from_vec` requires `values@.len() < usize::MAX` but some
  functions (insert for new keys, tabulate) can't prove strict inequality from their
  requires clauses. Would need trait spec changes or a Verus axiom about Vec capacity.

- **Closure value-correctness existentials**: Intersection/union ensures require
  `exists|v1: V, v2: V, r: V| ... f.ensures((&v1, &v2), r) ...`. Z3 cannot maintain
  these exec-level existential witnesses across loop iterations. Same class of issue
  as filter's completeness invariant.

- **feq_clone vs feq_full**: domain's requires has `obeys_feq_clone::<K>()` but
  ArraySetStEph::insert needs `obeys_feq_full::<K>()` (via wf). Would need trait spec
  change or alternative construction path.
