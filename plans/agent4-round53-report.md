<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent4 Round 53 Report

## Summary

No net changes to source files this round. The Chap38 ClonePreservesView work was a
duplicate of agent2's assignment, and the StrReplace operations produced no persistent
changes (files match HEAD exactly). The Round 52 commit (`7f7a6983e`) already documents
why the ClonePreservesView approach for Chap38 was reverted: it cascaded into new holes
via `view_ord_consistent` / `obeys_cmp_spec` assumes that can't be added to `Clone::clone`
impls (Verus rule: impl methods cannot declare requires not in the trait).

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
| 2 | 38 | BSTParaMtEph.rs | 1 | 1 | 0 |
| 3 | 39 | BSTParaTreapMtEph.rs | 0 | 0 | 0 |

## What Was Investigated

### Phase 1: Chap38 clone bridge (duplicate assignment)

The prompt matched agent2's assignment. Investigated the ClonePreservesView approach:

- Added `+ ClonePreservesView` bounds to `expose_internal`, all free functions, impl blocks.
- Added `requires view_ord_consistent + obeys_cmp_spec` to `expose_internal` to prove
  ordering transfer from `k@ == node.key@` using `lemma_cmp_equal_congruent_right`.
- Hit Verus rule: `Clone::clone` impls cannot declare `requires` clauses (only inherited
  from the trait). Since `std::clone::Clone::clone` has no requires, we can't propagate
  `view_ord_consistent` into Clone bodies through the trait.
- StrReplace operations produced no net file changes (files match HEAD). This is consistent
  with the Round 52 commit which already tried and reverted this approach.

**Blocker**: The `assume(c == *x)` in `clone_elem` is a genuine structural hole. Closing it
requires bridging structural equality (`==`) from view equality (`@`). For `T: MtKey`, this
needs `view_ord_consistent`, but that precondition cannot be carried through `Clone::clone`
impls (Verus trait impl rule). The hole remains until Verus supports richer Clone trait specs
or until we find a way to prove `c == *x` without `view_ord_consistent`.

### Phase 2: Chap39 Send blocker (pre-closed)

`BSTParaTreapMtEph.rs` shows 0 holes — already closed by the Round 52 commit.

### Phase 3: Chap26 ETSPMtEph (not reached)

Not attempted this round.

## Verification

Full verification: **4476 verified, 0 errors** (validates clean; script hits 130s timeout
due to verification time, but the run completes successfully at ~115s direct).

## Techniques Used

- Investigated `lemma_cmp_equal_congruent_right` for ordering transfer via view equality.
- Confirmed Verus rule: impl methods cannot add requires beyond the trait declaration.
- Confirmed Round 52 revert was correct: ClonePreservesView cascade introduces unresolvable
  constraints at Clone impl boundaries.

## Remaining Holes

| # | Chap | File | Line | Hole | Blocker |
|---|:----:|---|:---:|---|---|
| 1 | 38 | BSTParaStEph.rs | 152 | `assume(c == *x)` | Clone impl requires rule |
| 2 | 38 | BSTParaMtEph.rs | 152 | `assume(c == *x)` | Clone impl requires rule |
