<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 51 Report

## Summary

Closed 2 of 4 holes in Chap38. The 2 `assume(obeys_cmp_spec)` + `assume(view_ord_consistent)`
in `BSTParaMtEph.rs` are closed. The 2 `assume(c == *x)` clone bridge holes remain blocked.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|---------:|--------:|------:|
| 1 | 38 | BSTParaMtEph.rs | 3 | 1 | -2 |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 | 0 |
| — | — | **Total** | **4** | **2** | **-2** |

## Verification Count

- Before: 4465 verified, 0 errors (pre-change baseline)
- After: 4465 verified, 0 errors

## Holes Closed

| # | Chap | File | Line | Hole Type | Resolution |
|---|:----:|---|-----:|-----------|-----------|
| 1 | 38 | BSTParaMtEph.rs | 229 | `assume(obeys_cmp_spec)` | Removed; added `requires obeys_cmp_spec` |
| 2 | 38 | BSTParaMtEph.rs | 230 | `assume(view_ord_consistent)` | Removed; added `requires view_ord_consistent` |

### Technique

`lemma_cmp_order_axioms<T: MtKey>()` previously `assume`d both ordering axioms. Following
the pattern established in `BSTParaStEph.rs`, the function now takes them as `requires` and
re-states them as `ensures`. The two axioms were then propagated upward through:

- Public trait methods: `insert`, `delete`, `find`, `split`, `join_pair`, `join_pair_inner`,
  `union`, `intersect`, `difference`, `filter`
- Internal free helpers: `split_inner`, `find_recursive`, `union_inner`, `intersect_inner`,
  `difference_inner`, `filter_inner`, `filter_parallel`

Callers were updated in the trait definitions only; impl bodies inherit `requires` from the
trait automatically in Verus.

## Remaining Holes

| # | Chap | File | Line | Hole Type | Blocker |
|---|:----:|---|-----:|-----------|---------|
| 1 | 38 | BSTParaMtEph.rs | 152 | `assume(c == *x)` | No generic `assume_specification` for `T::clone` in vstd |
| 2 | 38 | BSTParaStEph.rs | 152 | `assume(c == *x)` | Same blocker |

### Clone Bridge Blocker

`clone_elem<T: Clone>` needs to prove `c == *x` after `let c = x.clone()`. This requires
either:
1. A generic `assume_specification` for `T::clone` asserting `cloned(*self, result)` — not
   in vstd (only primitive types have it).
2. Adding `ClonePreservesView` as a bound in `MtKey` in `Concurrency.rs` and adding a
   structural equality bridge via `feq` — requires modifying files outside Chap38.

Both paths are blocked by the "DO NOT touch files outside Chap38" constraint.

## Chapters Closed

None. Chap38 has 2 remaining holes and is still holed.
