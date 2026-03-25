# Agent 4 — Round 76 Report

## Summary

- **Holes**: 17 → 16 (−1)
- **Verified**: 4794 → 4798 (+4)
- **RTT**: 2619 passed
- **PTT**: 157 passed
- **Clean chapters**: 42/46 (no change)

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTRBMtEph.rs | 3 | 3 | 0 |
| 2 | 37 | BSTSplayMtEph.rs | 5 | 5 | 0 |
| 3 | 64 | SpanTreeStEph.rs | 2 | 1 | −1 |
| 4 | 64 | TSPApproxStEph.rs | 2 | 2 | 0 |
| 5 | 65 | UnionFindStEph.rs | 5 | 5 | 0 |

## Changes Made

### SpanTreeStEph.rs (Chap64) — 1 hole eliminated

- **Proved `verify_spanning_tree`**: Removed `external_body`. Added `graph_edges.spec_setsteph_wf()` and `valid_key_type_Edge::<V>()` to the for-loop invariant. The function verifies spanning tree edge count against `|V| - 1`.

### feq.rs (vstdplus) — Infrastructure improvement

- **Added `strictly_cloned` broadcast axioms**: `axiom_strictly_cloned_implies_eq` and `axiom_strictly_cloned_implies_eq_owned`. In the current Verus version, `Clone::clone` produces `strictly_cloned` (not `cloned`), so the existing `axiom_cloned_implies_eq_*` broadcasts don't fire. The new axioms are sound (same trust model as the `cloned` versions) and added to `group_feq_axioms`.

### UnionFindStEph.rs (Chap65) — Proof infrastructure

- **Added `lemma_three_clones_eq`**: Proof lemma that establishes 3 clone equalities in isolation from large wf predicates. This works around Z3 solver budget exhaustion when proving clone equalities in the presence of many quantified invariants.

## What Blocked Further Progress

### UnionFindStEph `insert` (Chap65)
Clone equality solved via `lemma_three_clones_eq` + `strictly_cloned` broadcasts. However, proving the 14-conjunct wf invariant preservation exceeds Z3 rlimit even at `rlimit(60)`. The function body is provably correct (all individual assertions verify in isolation with `spinoff_prover`) but the combined proof exceeds solver budget. Would need function splitting or wf predicate restructuring.

### BST height assumes (Chap37)
Both BSTRBMtEph and BSTSplayMtEph height functions have two structural Mt assumes: (1) `link_height < usize::MAX` — provable lemma gives `<=` not strict `<`; (2) ghost shadow matching locked data — fundamental Mt pattern without Verus enforcement mechanism.

### SpanTreeStEph `spanning_tree_star_contraction` (Chap64)
Uses closures passed to `star_contract`. The `star_contract` API requires `forall|inputs| expand.requires(inputs)` but the expand closure has specific wf preconditions (`spec_setsteph_wf`) that aren't universally satisfiable. Needs closure interface redesign.

### TSPApproxStEph `euler_tour`/`euler_tour_dfs` (Chap64)
Uses `HashSetWithViewPlus<(V, V)>` for visited edge tracking. `obeys_key_model::<(V, V)>()` is uninterpreted with no broadcast for tuples. Replacing `external_body` with `assume` for the key model would be same hole count.

### BSTSplayMtEph `clone` (Chap37)
Recursive `Clone` on `Node<T>` with `Box<Node<T>>` — Verus limitation with recursive clone cycles. Not attempted this round per R75 findings.

## Techniques Used

- Broadcast axiom addition for `strictly_cloned` (infrastructure)
- Proof lemma isolation from large wf contexts (solver budget workaround)
- Loop invariant propagation for `spec_setsteph_wf`
