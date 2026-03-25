# R76 Agent 5 — BSTSetSplayMtEph BTreeSet rewrite (13 holes)

## Objective

Eliminate up to 13 holes in `src/Chap37/BSTSetSplayMtEph.rs` by rewriting it to remove the
`std::collections::BTreeSet` dependency, following the exact pattern you used on
BSTSetAVLMtEph in R75 (and that Agent 1 replicated on BSTSetRBMtEph in R76).

## Baseline

- 4814 verified, 0 errors, 0 warnings
- BSTSetSplayMtEph.rs: 13 holes (13 external_body)
- Root causes: `values_vec` (line ~143), `from_sorted_iter` (line ~164), `iter` (line ~379)
- 10 downstream functions blocked by these root causes

## Current holes

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 143 | values_vec | external_body | ROOT (iter) |
| 2 | 152 | copy_set | external_body | from_sorted_iter |
| 3 | 208 | delete | external_body | filter |
| 4 | 223 | union | external_body | join_m |
| 5 | 254 | intersection | external_body | join_m |
| 6 | 282 | difference | external_body | join_m |
| 7 | 309 | split | external_body | ROOT (iter) |
| 8 | 326 | join_pair | external_body | ROOT (iter) |
| 9 | 341 | join_m | external_body | ROOT (iter) |
| 10 | 357 | filter | external_body | ROOT (iter) |
| 11 | 367 | reduce | external_body | ROOT (iter) |
| 12 | 379 | iter | external_body | ROOT |
| 13 | 476 | IntoIterator owned | external_body | iter |

## Reference implementations

**Read `src/Chap37/BSTSetAVLMtEph.rs` and `src/Chap37/BSTSetRBMtEph.rs` first.** Both
have already been rewritten from the same BTreeSet pattern. Your job is to replicate that
approach for the Splay variant. Key changes:

1. **Remove `use std::collections::BTreeSet`** — add `#[cfg(verus_keep_ghost)] use crate::vstdplus::feq::feq::obeys_feq_clone;`
2. **Replace `from_sorted_iter` with `rebuild_from_vec` + `build_from_vec`** — takes `Vec<T>`,
   uses explicit while loop with `insert()`.
3. **Replace `values_vec`** — remove external_body, use `tree.in_order()` + explicit `while`
   loop with `nth()`. Add `assume(obeys_feq_clone::<T>())`.
4. **Rewrite `copy_set`** — remove external_body, just `build_from_vec(values_vec(&set.tree))`.
5. **Rewrite `delete`, `split`** — remove external_body, explicit while loops.
6. **Rewrite `join_pair`, `join_m`** — remove external_body, sequential insert loops (remove
   ParaPair if present).
7. **Keep `union`/`intersection`/`difference` as external_body** — recursive ParaPair closures.
8. **Keep `filter`/`reduce` as external_body** — FnMut closure requires not provable. But
   write verified loop bodies inside.
9. **`iter()`** — remove external_body, use `values_vec`.
10. **`iter_in_order()`** — add `assume(obeys_feq_clone::<T>())` if not already present.
11. **`IntoIterator` impls** — match AVL/RB pattern: remove `external`/`external_body`, add
    `requires`/`ensures`, delegate to `self.iter()`.

## Key difference from AVL/RB

- BSTSetSplayMtEph wraps `BSTSplayMtEph<T>` with `+ 'static` bounds on many functions.
- Check what `BSTSplayMtEph` exposes — confirm `in_order()`, `insert()`, `new()` are available
  and what their specs are.

## Expected outcome

~8-10 holes eliminated. Remaining ~3-5 holes will be structural: `obeys_feq_clone` assumes,
filter/reduce external_body, union/intersection/difference external_body.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent5/ready`.

## Report

Write `plans/agent5-round76-report.md` with holes before/after (table with Chap column).
