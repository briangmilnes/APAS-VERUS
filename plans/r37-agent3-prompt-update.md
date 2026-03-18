# R37 Agent 3 Update: OrderedSet rank/select

## What You Did

Proved split in OrderedSetStEph and OrderedSetStPer. Good work on the feq
provenance loop technique.

## New Target: rank and select (4 holes)

You deferred these citing "connecting sorted-sequence indexing to
Set::filter cardinality through TotalOrder + existential quantifiers."
Take another crack at them.

### OrderedSetStEph.rs (2 holes)

| # | Line | Function |
|---|------|----------|
| 1 | 885 | rank |
| 2 | 908 | select |

### OrderedSetStPer.rs (2 holes)

| # | Line | Function |
|---|------|----------|
| 1 | 806 | rank |
| 2 | 824 | select |

### Approach

Both rank and select delegate to `self.base_set` (AVLTreeSetStEph/StPer).
Read the base implementations first:

1. Check `src/Chap41/AVLTreeSetStEph.rs` — does it have `rank` and `select`
   methods? What do their ensures say?

2. If the base method ensures match the OrderedSet trait ensures, remove
   external_body and delegate directly.

3. If the ensures don't match, you need a bridging proof. The OrderedSet
   trait spec talks about `self@.filter(|k'| TotalOrder::lt(k', k@)).len()`
   for rank and sorted sequence indexing for select. The base AVLTree
   implementation may use different internal representations.

4. For **rank**: The key insight is that `rank(k)` counts elements less
   than k. In a set backed by an AVL tree, this is the number of elements
   whose view is less than k's view under TotalOrder. Try:
   - Call base_set.rank(k)
   - Bridge the base ensures to the OrderedSet ensures using TotalOrder
     lemmas (reflexive, transitive, antisymmetric, total)
   - The filter cardinality should equal the count of elements < k

5. For **select**: select(i) returns the i-th smallest element. The base
   AVL tree may return elements in-order. Bridge using sorted sequence
   properties.

### If still blocked

If the base AVLTreeSet doesn't have rank/select with strong enough ensures,
report exactly what's missing. That tells us what Chap41 work is needed
before Chap43 can finish.

## Rules

- assume() only. NEVER accept().
- Do NOT touch OrderedTableStEph/StPer (agent 2) or Chap47/57 (agent 4).
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent3-round37-report.md`.
- Commit, push to `agent3/ready`.
