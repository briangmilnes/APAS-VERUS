# Agent 4 — R102 Report: Prove MtPer num_edges

## Objective

Remove 2 assumes from `AdjTableGraphMtPer::num_edges` in `src/Chap52/AdjTableGraphMtPer.rs`.

## Result: Both assumes eliminated

| # | Chap | File | Assume | Status |
|---|------|------|--------|--------|
| 1 | 52 | AdjTableGraphMtPer.rs | L228: `assume(count + neighbors@.len() <= self.spec_num_edges())` | Proved |
| 2 | 52 | AdjTableGraphMtPer.rs | L240: `assume(count == self.spec_num_edges())` | Proved |

## Holes before/after

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 52 | AdjTableGraphMtPer.rs | 3 | 1 | -2 |

Remaining hole: `delete_vertex` L441 `assume(updated.spec_adjtablegraphmtper_wf())` — rwlock predicate issue, unrelated to num_edges.

## Technique

**Key insight:** The original code iterated over `domain().to_seq()`, but `OrderedTableMtPer::domain()` has weak ensures (`self@.dom().finite()` only — no `domain@ == self@.dom()`). This made connecting the loop sum to `spec_sum_adj_sizes` impossible.

**Solution:** Rewrote num_edges to use `first_key()` + `find()` + `delete()` in a while loop:

1. Clone `self.adj` into `remaining`
2. While `remaining.size() > 0`:
   - `first_key()` gives a key k with `remaining@.dom().contains(k@)` (strong ensures)
   - `find(&k)` gives `neighbors@` with `remaining@[k@] == neighbors@` (R102 fix)
   - Add `neighbors.size()` to count
   - `delete(&k)` removes k from remaining
3. Loop invariant: `count + spec_sum_adj_sizes(remaining@) == self.spec_num_edges()`

**Why it works:** `first_key` has strong enough ensures to identify a key in the domain. `find` (fixed in R102) gives the value. `delete` gives `remaining@ == old@.remove(k@)`. The local copy of `lemma_sum_adj_remove` decomposes the recursive sum at any key, connecting each iteration's addition to the spec.

**Added proof infrastructure:**
- `lemma_sum_adj_remove` — local copy (standalone rule forbids StEph imports), proves `spec_sum_adj_sizes(m) == m[k].len() + spec_sum_adj_sizes(m.remove(k))`

**Auxiliary invariants:**
- `remaining@.dom().subset_of(self.spec_adj().dom())` — enables graph closure for neighbor finiteness
- `forall|k| remaining@.dom().contains(k) ==> remaining@[k] == self.spec_adj()[k]` — connects remaining values to original map

## Verification

```
scripts/validate.sh isolate Chap52
verification results:: 2820 verified, 0 errors
```

## Steps used: 3 (of 20 budget)
