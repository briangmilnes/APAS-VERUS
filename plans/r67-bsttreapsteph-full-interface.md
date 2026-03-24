# R67 Agent 1: BSTTreapStEph Full Interface

## Goal

Create `BSTTreapStEph` with the full parametric BST interface — expose, join_mid, split,
join_pair, union, intersect, difference, filter, reduce, in_order, collect_in_order — so
that AVLTreeSetStEph and OrderedTableStEph have a **balanced** St-bounds backing store.

Currently BSTTreapStEph (Chap39) only has find/insert/delete. The full interface exists
in BSTParaTreapMtEph (also Chap39) but requires `MtKey` bounds (Send + Sync + 'static).
The St version needs `StT + Ord` bounds.

## Source Material

**Reference implementation**: `src/Chap39/BSTParaTreapMtEph.rs`
- `ParamTreapTrait` (line ~1853): 18 methods — new, expose, join_mid, size, is_empty,
  insert, delete, find, split, join_pair, union, intersect, difference, filter, reduce,
  in_order, collect_in_order, singleton
- Internal functions: expose_internal, join_with_priority, split_inner, join_pair_inner,
  union_inner, intersect_inner, difference_inner, filter_inner, reduce_inner,
  collect_in_order (free fn), singleton_inner

**Existing St file**: `src/Chap39/BSTTreapStEph.rs`
- Currently has: find, insert, delete (trait `BSTTreapStEphTrait`)
- Node: `{ key: T, priority: u64, size: usize, left: Link<T>, right: Link<T> }`
- Bounds: `T: StT + Ord + IsLtTransitive`

## Approach

**Extend** the existing `BSTTreapStEph.rs` file — do not create a new file. Add the
missing operations to the existing type and trait. The node structure already has
`priority`, `size`, `left`, `right` — everything needed for balanced treap operations.

### Bound Changes

The Mt version uses `MtKey` which includes `Clone + Send + Sync + 'static`. For St:
- Replace `MtKey` → `StT + Ord`
- Replace `ClonePreservesView` requirement → use `obeys_feq_clone::<T>()` in requires
  (the St pattern from BSTParaStEph.rs)
- Drop `+ 'static` bounds
- Drop `Arc<F>` wrappers on closure parameters — use `&F` or `F` directly
- `Pred<T>` (Mt closure trait) → `PredSt<T>` (St closure trait)
- `Fn(T, T) -> T + Send + Sync + 'static` → `Fn(T, T) -> T`

### Key Translation Pattern

For each internal function in BSTParaTreapMtEph:
1. Copy the function signature
2. Replace `MtKey + ClonePreservesView + 'static` → `StT + Ord`
3. Replace `ParamTreap<T>` → `BSTTreapStEph<T>` (or `Link<T>` for internal fns)
4. Replace `Arc<F>` → `&F`
5. Replace `Pred<T>` → `PredSt<T>`
6. Keep all requires/ensures/decreases/proof blocks — the algorithmic logic is identical
7. The clone bridge uses `clone_elem` (which exists in the file or needs adding)

### View

The Mt version views as `Set<T::V>`. The St version should do the same for the new
trait methods (the existing `BSTTreapStEphTrait` uses a different spec style with
`spec_in_order`, `spec_contains`, etc.).

**Design choice**: Add a new trait `ParamBSTTreapStEphTrait` with the parametric
interface (matching the style of `ParamBSTTrait` from Ch38), keeping the existing
`BSTTreapStEphTrait` for backward compatibility. The new trait uses `View<V = Set<T::V>>`.

Alternatively, if `BSTTreapStEphTrait` can be extended cleanly, add the methods there.
Use your judgment — the goal is the full interface available on `BSTTreapStEph<T>`.

### Priority Generation

Mt version uses `rand::random::<i64>()` for priorities. St version should use
`SeededRng` from `src/vstdplus/seeded_rng.rs` or a similar verified random source.
Check how the existing `BSTTreapStEph` handles priorities in `insert` — it takes
`priority: u64` as a parameter. Follow that pattern for new operations (the caller
provides priority, or generate internally).

Actually — look at how `join_with_priority` works in the Mt version. It takes an
explicit priority parameter. For `insert`, the Mt version generates a random priority
internally. For St, check what the existing insert does and follow that pattern.

### Operations to Add

These do NOT exist in BSTTreapStEph and must be added:

1. **expose** — decompose into (left, key, right) or Leaf
2. **join_mid** — balanced reassembly (the balancing happens here via priority comparison + rotations)
3. **split** — partition by key into (less, found, greater)
4. **join_pair** — merge two disjoint trees where all keys in left < all keys in right
5. **union** — set union via split + recursive merge
6. **intersect** — set intersection
7. **difference** — set difference
8. **filter** — keep elements matching predicate
9. **reduce** — fold with associative operator
10. **in_order** — inorder traversal to ArraySeqStPerS
11. **collect_in_order** — accumulate into Vec
12. **singleton** — single-element tree
13. **min_key** — if BSTParaStEph has it and it's useful

### What NOT to Change

- Do NOT modify BSTParaTreapMtEph.rs
- Do NOT modify BSTParaStEph.rs
- Do NOT modify AVLTreeSetStEph.rs or OrderedTableStEph.rs (that's a separate task)
- Do NOT add `assume`, `accept`, or `external_body` on algorithmic logic
- Do NOT add `admit()`

### Verification Target

- 0 errors, 0 new holes
- All new methods verify with full specs (requires + ensures)
- `scripts/validate.sh` clean
- `scripts/rtt.sh` passes
- `scripts/ptt.sh` passes

## Definition of Done

1. `BSTTreapStEph<T>` has the full parametric interface (18 methods)
2. All methods have correct requires/ensures matching the Mt version's contracts
3. `join_mid` performs priority-based rotations (this is where balancing happens)
4. `scripts/validate.sh` — 0 errors
5. `scripts/rtt.sh` — all pass
6. `scripts/ptt.sh` — all pass
7. `scripts/holes.sh src/Chap39/` — 0 new holes
