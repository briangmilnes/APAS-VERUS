# R67 Agent 4: OrderedSetStEph Rewrite for ParamBST Backing

## Goal

Uncomment and rewrite `OrderedSetStEph` to work with AVLTreeSetStEph's new backing store
(`ParamBST<T>` instead of `AVLTreeSeqStEphS<T>`). The old code reaches two levels deep
into `self.base_set.elements` for indexed sequence operations. With the BST backing, all
algorithms must be rewritten using tree operations (split, min_key, expose, in_order).

## Current State

- `OrderedSetStEph` is **commented out** in `src/lib.rs` (line 545).
- The file exists at `src/Chap43/OrderedSetStEph.rs` with the old implementation.
- It wraps `AVLTreeSetStEph<T>` and accesses `self.base_set.elements` extensively.
- AVLTreeSetStEph now has `pub tree: ParamBST<T>` (no `.elements` field).

## Struct

```rust
pub struct OrderedSetStEph<T: StT + Ord + TotalOrder> {
    pub base_set: AVLTreeSetStEph<T>,
}
```

The struct stays the same — it wraps AVLTreeSetStEph. But all code that accessed
`self.base_set.elements` must now use `self.base_set.tree` (a `ParamBST<T>`) or
the AVLTreeSetStEph trait methods.

## Operations to Rewrite

The trait has 17 exec methods (9 defaults + 8 `_iter` variants). The `_iter` variants
contain the actual implementations; defaults delegate to them. The defaults that are
just `self.foo_iter(...)` don't need changes — only the `_iter` bodies do.

### Available ParamBST operations

From `src/Chap38/BSTParaStEph.rs`, `ParamBSTTrait`:
- `new()`, `singleton(key)` — constructors
- `expose()` → `Exposed::Leaf` or `Exposed::Node(left, key, right)` — decompose
  - ensures: left elements < key < right elements, disjoint, finite
- `join_mid(exposed)` — rebalance (note: unbalanced for ParamBST, balanced for treap)
- `size()` — O(1) cached
- `is_empty()` — O(1)
- `find(key)` → `Option<T>` — O(log n) search
- `insert(key)` — O(log n) BST insert
- `delete(key)` — O(log n) BST delete
- `split(key)` → `(left, found, right)` — partition by key
  - ensures: left < key < right, disjoint, union = self minus key
- `min_key()` → `Option<T>` — minimum element
- `join_pair(other)` — merge two trees where all left < all right
- `union(other)`, `intersect(other)`, `difference(other)` — set operations
- `filter(pred)` — filter by predicate
- `in_order()` → `ArraySeqStPerS<T>` — sorted sequence

### Algorithm rewrites

| # | Method | Old approach (indexed seq) | New approach (tree ops) |
|---|--------|--------------------------|------------------------|
| 1 | `first_iter` | Scan seq for min via loop | `self.base_set.tree.min_key()` |
| 2 | `last_iter` | Scan seq for max via loop | Walk right via expose, or `in_order()` last element |
| 3 | `previous_iter` | Binary search in sorted seq | `split(k)` → `left.min_key()` gives max of {x < k}... actually need max_key. Alternative: `split(k)` → walk right spine of left tree |
| 4 | `next_iter` | Binary search in sorted seq | `split(k)` → `right.min_key()` gives min of {x > k} |
| 5 | `split_iter` | Partition seq by index | `self.base_set.tree.split(k)` → wrap both sides |
| 6 | `get_range_iter` | Scan seq, collect in range | `split(k1)` → right; `split(k2)` on right → left is the range |
| 7 | `rank_iter` | Count elements < k in seq | `split(k)` → `left.size()` |
| 8 | `split_rank_iter` | Split seq at index i | `in_order()` → take first i, build two sets |
| 9 | `select` | Index into sorted seq | `in_order()` → `nth(i)` |

**Note on `last` and `previous`**: ParamBST has `min_key` but no `max_key`. Options:
- Write a local `max_key` helper that walks right via `expose` (simple recursive descent)
- Use `in_order()` and take last element (O(n) but correct)
- Add `max_key` to the local module as a free function

A local recursive `max_key` via expose is cleanest — it's O(log n) for balanced trees:
```
fn max_key(tree: &ParamBST<T>) -> Option<T> {
    match tree.expose() {
        Leaf => None,
        Node(_, k, right) => if right.is_empty() { Some(k) } else { max_key(&right) }
    }
}
```

### Set operations (delegations)

These delegate directly to AVLTreeSetStEph and don't access `.elements`:
- `size`, `empty`, `singleton`, `find`, `insert`, `delete`
- `filter`, `intersection`, `union`, `difference`
- `from_seq`

Check each one — if it only calls `self.base_set.foo()`, it should work unchanged.
If it accesses `self.base_set.elements`, rewrite it.

## Steps

1. **Read** the current OrderedSetStEph.rs thoroughly.
2. **Read** AVLTreeSetStEph.rs to understand the new API.
3. **Read** BSTParaStEph.rs for ParamBST's trait interface and ensures.
4. **Uncomment** `pub mod OrderedSetStEph;` in lib.rs.
5. **Rewrite** all functions that access `.elements`.
6. **Add** axiom requires (`obeys_cmp_spec`, `view_ord_consistent`) where needed —
   follow AVLTreeSetStEph's pattern.
7. **Validate** — target 0 errors.
8. **Run RTT and PTT**.
9. **Run** `scripts/holes.sh src/Chap43/` — target 0 new holes.

## Constraints

- Do NOT modify BSTParaStEph.rs or AVLTreeSetStEph.rs.
- Do NOT add `assume`, `accept`, or `external_body` on algorithmic logic.
- Do NOT uncomment OrderedSetStPer, OrderedSetMtEph, or OrderedTableMtPer.
- The `_iter` variants keep their iterative style where possible (they iterate over
  the tree structure rather than a flat sequence now).
- Defaults delegate to `_iter` variants (same as current pattern).
- Run validate, rtt, ptt sequentially.
