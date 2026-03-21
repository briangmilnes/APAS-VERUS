<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 50 Report

## Summary

Completed verification of `src/Chap38/BSTParaMtEph.rs`. The prior round (R49) had
already refactored `Arc<RwLock>` → `RwLock` and proven `expose_internal`,
`intersect_inner`, and `difference_inner`. This round fixed remaining verification
errors and proved `filter_inner`.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 38 | BSTParaMtEph.rs | 5 | 3 | −2 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |

**Before holes in BSTParaMtEph.rs:**
- `filter_inner` external_body
- `expose_internal` fn_missing_requires
- `clone_elem` assume (standard clone bridge)
- `lemma_cmp_order_axioms` assume x2 (Ord consistency, permitted)

**After holes in BSTParaMtEph.rs:**
- `clone_elem` assume (standard clone bridge — accepted pattern)
- `lemma_cmp_order_axioms` assume x2 (permitted per R50 constraints)

## Work Done

### Verification Errors Fixed (3)

| # | Chap | File | Line | Error | Fix |
|---|:----:|---|:---:|---|---|
| 1 | 38 | BSTParaMtEph.rs | 380 | Precondition on `expose_internal` | Added `use_type_invariant(self)` to `expose` |
| 2 | 38 | BSTParaMtEph.rs | 1003 | Size bound in `union_inner` | Added `a@.len() + b@.len() <= usize::MAX` requires to `union_inner` and public `union`/`join_pair` trait methods; added proof hints for recursive bounds |
| 3 | 38 | BSTParaMtEph.rs | 1419 | Precondition in `filter_parallel` | Added `use_type_invariant(tree)` before calling `filter_inner` |

### Trigger Warnings Fixed (4)

All in `split_inner` — added `#[trigger]` to auto-selected terms on lines 757, 792, 808, 843.

### filter_inner Proven

Removed `#[verifier::external_body]` and wrote a sequential recursive implementation. Key design decisions:

- **Why sequential**: `spec_fn(T::V) -> bool` = `FnSpec<(T::V,), bool>` has `PhantomData<(T::V,)>` and does not implement `Send`. Parallel closures via `ParaPair!` require `Send`, so capturing `spec_pred` in named closures is impossible. The sequential approach borrows `predicate: &Arc<F>` for both recursive calls.
- **Proof structure**: Mirror of `BSTParaStEph::filter_inner` with MtKey bounds and `use_type_invariant` calls.
- **Size bound for `join_mid`**: Added `left@.len() + right@.len() < usize::MAX as nat` to `expose_internal` ensures (provable from the lock predicate's `size: usize` field).
- **assert-forall with implies**: Used `implies` (not `==>`) in `assert forall` bodies so the antecedent is assumed, enabling case-split proofs.
- **Ordering transitivity**: Used `lemma_cmp_antisymmetry(o, key)` + `lemma_cmp_transitivity(s, key, o)` to prove `s < o` for elements from opposite subtrees.

### expose_internal Enhanced

Added `left@.len() + right@.len() < usize::MAX as nat` to the `Exposed::Node` case of `expose_internal` ensures. This bound is already established inside the function body from the lock predicate's `(*node).left@.len() + (*node).right@.len() < usize::MAX as nat` invariant. Exposing it in ensures allows callers (`filter_inner`, `split_inner` recursion) to use the bound directly.

## Verification Counts

| Stage | Result |
|---|---|
| Verified functions | 4448 (↑ from 4447) |
| Errors | 0 |
| RTT | 2611/2611 pass |
| PTT | 147/147 pass |

## Remaining Holes in BSTParaMtEph.rs

| # | Line | Type | Description | Status |
|---|:---:|---|---|---|
| 1 | 152 | `assume()` | `clone_elem` clone bridge | Accepted pattern |
| 2 | 229 | `assume()` | `lemma_cmp_order_axioms`: `obeys_cmp_spec` | Permitted R50 |
| 3 | 230 | `assume()` | `lemma_cmp_order_axioms`: `view_ord_consistent` | Permitted R50 |

The `lemma_cmp_order_axioms` assumes would be closable if `MtKey` required
`obeys_cmp_spec` and `view_ord_consistent` directly (then they'd be derivable
from the trait bounds). This is a known gap tracked separately.

## Chapters Closed

None newly closed (BSTParaMtEph.rs still has 3 holes). However, all algorithmic
`external_body` holes are now gone from `BSTParaMtEph.rs`.

## Files Changed

- `src/Chap38/BSTParaMtEph.rs` — main work
- `src/Chap38/analyses/veracity-review-verus-proof-holes.log` — regenerated
- `tests/Chap38/TestBSTParaMtEph.rs` — no intentional changes (diff is from prior session)
