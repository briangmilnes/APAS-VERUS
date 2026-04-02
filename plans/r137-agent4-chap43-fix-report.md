# R137 Agent 4 — Fix OrderedTable in_order bugs: O(n) → O(lg n)

## Summary

Fixed 2 of 10 OrderedTable `in_order()` bugs, plus built the infrastructure for
the remaining 8. Added `max_key` to BSTParaStEph, created `TotalOrderBridge` trait,
and added `reveal_param_bst_backings` to expose the BST type invariant.

## Infrastructure Added

### 1. `max_key` on ParamBST (BSTParaStEph.rs)

Mirror of `min_key`: traverses right branches to find the maximum element.
APAS: "last need only to traverse right branches." O(lg n).

### 2. `TotalOrderBridge` trait (vstdplus/total_order.rs)

New trait bridging `Ord::cmp_spec` to `TotalOrder::le`:

```rust
pub trait TotalOrderBridge: TotalOrder + Ord {
    proof fn cmp_spec_less_implies_le(a: Self, b: Self)
        requires a.cmp_spec(&b) == Ordering::Less
        ensures TotalOrder::le(a, b);
    proof fn cmp_spec_greater_implies_le(a: Self, b: Self)
        requires a.cmp_spec(&b) == Ordering::Greater
        ensures TotalOrder::le(b, a);
}
```

Implemented for: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
(empty proof body — Z3 proves from concrete definitions), plus String (assumed).

**Why needed:** BST operations (min_key, max_key, split, expose) have `cmp_spec`-based
ensures. OrderedTable postconditions use `TotalOrder::le`. Without the bridge, O(lg n)
implementations can't transfer BST ordering guarantees to TotalOrder postconditions.
O(n) implementations avoided this by doing element-by-element TotalOrder::cmp comparisons.

### 3. `reveal_param_bst_backings` (BSTParaStEph.rs)

Exec fn that exposes the BST type invariant:
```rust
pub fn reveal_param_bst_backings<T: StT + Ord>(tree: &ParamBST<T>)
    ensures forall|v: T::V| tree@.contains(v) ==> exists|t: T| tree@.contains(t@) && t@ == v
```

**Why needed:** BST quantifiers (min_key, expose) quantify over `T` values with trigger
`tree@.contains(t@)`. To instantiate these quantifiers from view-level containment
(`tree@.contains((kv, vv))`), Z3 needs existential witnesses for backing `T` values.
The BST type invariant provides these but isn't visible outside the module.
This fn makes the invariant's backing-witness property public.

### 4. `TotalOrderBridge` propagation

Changed `where K: TotalOrder` to `where K: TotalOrderBridge` on all TotalOrder-using
functions across Chap43 files (OrderedTableStEph, OrderedTableStPer, AugOrderedTableStEph,
AugOrderedTableStPer, AugOrderedTableMtEph, OrderedTableMtEph, OrderedTableMtPer,
OrderedSetStEph, OrderedSetStPer, OrderedSetMtEph). Also fixed one cascade into
Chap52/AdjTableGraphMtPer.rs (num_edges).

## Functions Fixed (2 of 10)

| # | Chap | File | Function | Old | New |
|---|------|------|----------|-----|-----|
| 1 | 43 | OrderedTableStEph.rs | `first_key_iter` | O(n): in_order + linear scan | O(lg n): `tree.min_key()` + bridge proof |
| 2 | 43 | OrderedTableStEph.rs | `last_key_iter` | O(n): in_order + linear scan | O(lg n): `tree.max_key()` + bridge proof |

## Functions Remaining (8 of 10, in BOTH StEph and StPer)

| # | Function | Approach | Difficulty |
|---|----------|----------|------------|
| 1 | `next_key_iter` | Recursive BST descent with TotalOrder::cmp | Hard — recursive helper + bridge proof |
| 2 | `previous_key_iter` | Recursive BST descent with TotalOrder::cmp | Hard — mirror of next_key_iter |
| 3 | `split_key_iter` | Recursive key-only split using expose + join_mid | Hard — full recursive split with domain proofs |
| 4 | `get_key_range_iter` | Two key-only splits | Medium — depends on split_key_iter |
| 5 | `rank_key_iter` | Recursive size-based descent with TotalOrder::cmp | Hard — filter set equivalence proof |
| 6 | `select_key` | Recursive size-based descent | Hard — filter set rank proof |
| 7 | `split_rank_key_iter` | Size-based pair-at-rank + BST split | Medium — depends on size-based select |

Each remaining function requires a recursive free function helper that walks the BST
using `expose()`, performs O(lg n) work, and proves postconditions using the bridge +
BST set decomposition. The proof patterns are established by first_key_iter/last_key_iter.

## Validation

- 5498 verified, 0 errors (full validation)
- 3583 RTTs passing
- Chap43 isolate: 2648 verified, 0 errors

## Techniques Used

- **BST type invariant exposure**: `reveal_param_bst_backings` makes the backing-element
  existential public, enabling callers to instantiate BST quantifiers from view-level facts.
- **TotalOrderBridge trait**: Proof fn connecting `cmp_spec` to `TotalOrder::le`, implemented
  per-type. Empty bodies for integers (Z3 handles), assumes for String.
- **Quantifier witness chain**: In the proof, `lemma_map_contains_pair_in_set` → choose vv →
  `reveal_param_bst_backings` → choose tp (backing Pair) → instantiate min_key/max_key forall
  → `spec_pair_key_determines_order` → `cmp_spec_less_implies_le`.
