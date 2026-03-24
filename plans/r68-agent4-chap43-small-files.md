# R68 Agent 4: Chap43 Small File Hole Burndown

## Goal

Close all 11 holes across 4 small Chap43 files:
- OrderedSetStEph.rs: 5 holes
- AugOrderedTableStEph.rs: 2 holes
- AugOrderedTableStPer.rs: 2 holes
- OrderedTableMtEph.rs: 2 holes

## File 1: OrderedSetStEph.rs (5 holes)

**File**: `src/Chap43/OrderedSetStEph.rs`

| # | Line | Type | Description |
|---|------|------|-------------|
| 1 | 575 | assume | `assume(result@.to_set() =~= self@)` in to_seq — collect_in_order set membership |
| 2 | 576 | assume | `assume(forall\|i\| 0 <= i < result@.len() ==> self@.contains(result@[i]))` |
| 3 | 767 | assume | `assume(k1_clone@ == k1@)` — clone view gap in get_range_iter |
| 4 | 783 | assume | `assume(k2_clone@ == k2@)` — clone view gap in get_range_iter |
| 5 | 949 | unsafe | Iterator next() `unsafe { &*ptr }` — standard pattern |

**Strategy**:

Holes 1-2 (collect_in_order): ParamBST's `collect_in_order` currently only ensures
`out@.len()`, not membership. Check if ParamBST's `in_order` method has stronger ensures.
If `in_order` ensures `result@.to_set() =~= self@` or similar, use `in_order` instead
of `collect_in_order`. If neither has membership ensures, these assumes may need to stay
unless you can write a local proof that walks the result.

Read `src/Chap38/BSTParaStEph.rs` — check `collect_in_order` and `in_order` ensures
carefully.

Holes 3-4 (clone view): The clone workaround pattern — `k.clone()` should preserve view
but Verus can't prove it for generic T. Check if `obeys_feq_clone::<T>()` is in the
requires. If so, you might be able to use the `feq_clone_ensures` lemma from
`src/vstdplus/feq/feq.rs` to prove `k_clone@ == k@`. Read the feq standard.

Hole 5 (unsafe): Standard iterator pattern. Leave as-is.

## File 2: AugOrderedTableStEph.rs (2 holes)

**File**: `src/Chap43/AugOrderedTableStEph.rs`

| # | Line | Type | Description |
|---|------|------|-------------|
| 1 | 796 | assume | `assume(self.base_table.tree@.finite())` in reduce_val |
| 2 | 807 | assume | `assume(self.base_table.tree@.finite())` in reduce_range |

**Strategy**: ParamBST's type invariant guarantees finite sets, but it's private (not
exposed in ensures). Options:

1. Check if `spec_bstparasteph_wf()` implies `self@.finite()` — if so, assert from wf.
   The OrderedTable wf includes `self.tree.spec_bstparasteph_wf()`. Read BSTParaStEph's
   wf definition to see if it includes or implies finiteness.

2. Check if `self.tree@.len() < usize::MAX as nat` (which IS in OrderedTable's wf)
   implies `self.tree@.finite()` — a set with bounded nat length is finite.

3. Use `vstd::set_lib::lemma_len_subset` or similar to derive finiteness from the
   len bound.

4. If OrderedTable's `spec_orderedtablesteph_wf()` or the AugOrderedTable's wf includes
   `self@.dom().finite()`, extract it from there.

## File 3: AugOrderedTableStPer.rs (2 holes)

**File**: `src/Chap43/AugOrderedTableStPer.rs`

Same pattern as AugOrderedTableStEph — `assume(self.base_table.tree@.finite())` in
reduce_val and reduce_range. Same fix strategy.

## File 4: OrderedTableMtEph.rs (2 holes)

**File**: `src/Chap43/OrderedTableMtEph.rs`

| # | Line | Type | Description |
|---|------|------|-------------|
| 1 | 767 | assume | `assume(obeys_cmp_spec::<Pair<K, V>>())` in from_sorted_entries |
| 2 | 768 | assume | `assume(view_ord_consistent::<Pair<K, V>>())` in from_sorted_entries |

**Strategy**: `from_sorted_entries` is a constructor — no `&self`, no wf in scope. The
trait requires for `from_sorted_entries` may not include these axioms. Check:

1. What does the OrderedTableMtEph trait's `from_sorted_entries` requires clause say?
2. Can you add `obeys_cmp_spec` and `view_ord_consistent` to the requires?
3. Do callers already satisfy these? (Check callers of `from_sorted_entries`.)

If the trait doesn't have these in requires, adding them is the clean fix — but check
that all callers can provide them.

## Approach

1. **Read all 4 files** + BSTParaStEph.rs (especially wf and collect_in_order/in_order ensures)
2. **Read** `src/vstdplus/feq/feq.rs` for clone view proof patterns
3. **Start with AugOrderedTable files** (2+2 = 4 holes, likely same fix for both)
4. **Then OrderedSetStEph** (5 holes, mixed difficulty)
5. **Then OrderedTableMtEph** (2 holes)
6. **Validate** after each file

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStEph.rs, OrderedTableStPer.rs.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Iterator unsafe (hole 5) stays — standard pattern.
- Run validate, rtt, ptt sequentially.
