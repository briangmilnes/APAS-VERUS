# R148 Agent 3 — Traitify BSTSplay in Chap37

## Summary

Moved free functions into trait impls across three BSTSplay files in Chap37.
Created `BSTSplayNodeFns` (StEph), `BSTSplayMtNodeFns` (MtEph), and
`BSTSetSplayMtEphHelperFns` (BSTSetSplayMtEph) traits.

## Changes by File

| # | Chap | File | Functions Moved | Trait Created |
|---|------|------|-----------------|---------------|
| 1 | 37 | BSTSplayStEph.rs | 12 | `BSTSplayNodeFns<T>` |
| 2 | 37 | BSTSplayMtEph.rs | 19 | `BSTSplayMtNodeFns<T>` |
| 3 | 37 | BSTSetSplayMtEph.rs | 4 | `BSTSetSplayMtEphHelperFns<T>` |

## BSTSplayStEph.rs (12 functions)

Created `BSTSplayNodeFns<T: TotalOrder + Clone>` trait on `Node<T>` with:
- 4 spec accessor functions (`spec_key`, `spec_left`, `spec_right`, `spec_node_size`)
- `update(&mut self)` — converted to method with spec accessor ensures
- 11 associated functions: `new_node`, `size_link`, `height_link`, `splay`,
  `bst_insert`, `insert_link`, `find_link`, `min_link`, `max_link`,
  `in_order_collect`, `pre_order_collect`

All `// veracity: no_requires` annotations preserved. No new requires added.
Call sites updated with `Self::` (internal) and `Node::<T>::` (from BSTSplayStEphTrait impl).

## BSTSplayMtEph.rs (19 functions)

Created `BSTSplayMtNodeFns<T: StTInMtT + Ord + TotalOrder>` trait on `Node<T>` with:
- Core 11 functions (same as StEph with Mt type bounds)
- Mt-specific: `in_order_parallel`, `pre_order_parallel`, `clone_link`,
  `build_balanced`, `filter_parallel`, `reduce_parallel`, `height_rec`,
  `compute_link_spec_size`

**Note:** `update` kept as associated function `update(node: &mut Node<T>)` rather than
`&mut self` method. The MtEph `update` has a complex conditional ensures involving
`link_spec_size` that Verus can't verify through spec accessor indirection. Direct field
references in the ensures work; spec accessor wrappers don't unfold properly for the
conditional clause. StEph's simpler ensures works with spec accessors.

## BSTSetSplayMtEph.rs (4 functions)

Created `BSTSetSplayMtEphHelperFns<T>` trait on `BSTSetSplayMtEph<T>` with supertrait
`BSTSetSplayMtEphTrait<T>`:
- `values_vec` — associated function
- `rebuild_from_vec` — associated function
- `build_from_vec` — associated function returning `Self`
- `copy_set(&self)` — converted to method (was `copy_set(set: &BSTSetSplayMtEph<T>)`)

## Verification

- Isolate Chap37: 1946 verified, 0 errors
- RTT: 3690 passed, 0 skipped
- No SMT flakiness encountered. Splay function verified cleanly as trait method.

## Techniques

- Generic params removed from impl functions (T comes from trait)
- `requires`/`ensures` moved to trait declarations; `decreases` kept in impl
- Internal calls: `Self::method(...)` prefix
- External calls: `Node::<T>::method(...)` prefix
- `update` method calls via Box auto-deref: `root.update()` (StEph only)
- `update` as associated fn with deref coercion: `Self::update(&mut root)` (MtEph)
