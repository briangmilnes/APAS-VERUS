# R148 Agent 2 — Traitify BSTRB in Chap37

## Summary

Converted free functions in three Chap37 RB tree files to trait methods, following the trait-impl pattern.

## Changes

### BSTRBStEph.rs (7 functions)

Created `BSTRBNodeFns<T: TotalOrder>` trait with supertraits `BSTSpecFns<T> + BalBinTreeTrait<T>` for `BalBinTree<T>`. Moved 7 free functions to trait methods:

| # | Chap | File | Function | self type |
|---|------|------|----------|-----------|
| 1 | 37 | BSTRBStEph.rs | rotate_right | self (consuming) |
| 2 | 37 | BSTRBStEph.rs | rotate_left | self (consuming) |
| 3 | 37 | BSTRBStEph.rs | insert_node | self (consuming) |
| 4 | 37 | BSTRBStEph.rs | contains_node | &self |
| 5 | 37 | BSTRBStEph.rs | find_node | &self |
| 6 | 37 | BSTRBStEph.rs | min_node | &self |
| 7 | 37 | BSTRBStEph.rs | max_node | &self |

Call sites in `BSTRBStEphTrait` impl updated (e.g., `insert_node(self.root, value)` → `self.root.insert_node(value)`).

### BSTRBMtEph.rs (18 functions → trait, 3 stay free)

Created `BSTRBMtNodeFns<T>` trait for `Link<T>` (= `Option<Box<Node<T>>>`). Since `Link<T>` is a type alias for a foreign type, the trait declares 5 spec methods to express contracts:

- `spec_bst(self)` → `spec_is_bst_link(self)`
- `spec_size(self)` → `link_spec_size(self)`
- `spec_contains(self, target)` → `link_contains(self, target)`
- `spec_height(self)` → `link_height(self)`
- `spec_is_empty(self)` → `self is None`

18 Link-first-arg functions moved to `impl BSTRBMtNodeFns<T> for Link<T>`:

| # | Chap | File | Function | self type |
|---|------|------|----------|-----------|
| 1 | 37 | BSTRBMtEph.rs | is_red | &self |
| 2 | 37 | BSTRBMtEph.rs | size_link | &self |
| 3 | 37 | BSTRBMtEph.rs | rotate_left | &mut self |
| 4 | 37 | BSTRBMtEph.rs | rotate_right | &mut self |
| 5 | 37 | BSTRBMtEph.rs | flip_colors | &mut self |
| 6 | 37 | BSTRBMtEph.rs | fix_up | &mut self |
| 7 | 37 | BSTRBMtEph.rs | insert_link | &mut self |
| 8 | 37 | BSTRBMtEph.rs | find_link | &self |
| 9 | 37 | BSTRBMtEph.rs | min_link | &self |
| 10 | 37 | BSTRBMtEph.rs | max_link | &self |
| 11 | 37 | BSTRBMtEph.rs | in_order_collect | &self |
| 12 | 37 | BSTRBMtEph.rs | pre_order_collect | &self |
| 13 | 37 | BSTRBMtEph.rs | in_order_parallel | &self |
| 14 | 37 | BSTRBMtEph.rs | pre_order_parallel | &self |
| 15 | 37 | BSTRBMtEph.rs | filter_parallel | &self |
| 16 | 37 | BSTRBMtEph.rs | reduce_parallel | &self |
| 17 | 37 | BSTRBMtEph.rs | height_rec | &self |
| 18 | 37 | BSTRBMtEph.rs | compute_link_spec_size | &self |

3 functions remain free (first arg is not `Link<T>`):
- `new_node(key: T)` — constructor
- `update(node: &mut Node<T>)` — operates on `Node<T>`
- `build_balanced(values: &[T])` — constructor from slice

Bridge assertions added in proof blocks where trait ensures (expressed via `spec_contains`) needed unfolding to `link_contains` for the SMT solver.

### BSTSetRBMtEph.rs (1 function)

Moved `copy_set` to `BSTSetRBMtEphTrait` as `fn copy_set(&self) -> Self`. Updated 4 call sites.

## Verification

| Metric | Count |
|--------|-------|
| Verified | 5702 |
| Errors | 0 |
| RTT | 3690 passed |
| PTT | 221 passed |

## Techniques

- Supertraits (`BSTSpecFns + BalBinTreeTrait`) for StEph trait to access spec methods on `BalBinTree<T>`.
- Spec method indirection for MtEph trait (5 open spec fns bridging trait to free functions) since `Link<T>` is a type alias for a foreign type.
- Bridge assertions (`assert(x.spec_contains(z) ==> y.spec_contains(z))`) to help Z3 connect trait ensures (spec methods) to free function assertions (link_contains).
- `*old(self)` for body ghost code in `&mut self` methods (old(self) gives `&mut Self` reference in body, needs deref).
- `old(self)` without `*` in requires/ensures (auto-deref for `&mut self` methods in spec context).
