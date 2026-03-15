# Chap38 Spec Audit — Round 17

## BSTParaStEph.rs

| # | Function | Old ensures | New ensures | Status |
|---|----------|-------------|-------------|--------|
| 1 | `reduce` | (none) | `empty ==> result@ == base@` | external_body |
| 2 | `filter` | `subset_of, finite` | No change | Partial — closure limitation |
| 3 | `collect_in_order` | `len` only | No change | Partial — content not specified |
| 4 | `in_order` | `len` only | No change | Partial — content not specified |

### Notes

**filter**: The textbook says `result = {k in T | f(k)}`. The correct spec would be
`forall|k| filtered@.contains(k@) <==> (self@.contains(k@) && predicate(k))`. However,
Verus closure limitations prevent specifying predicate semantics in ensures — the closure's
`ensures` is not constrained by the `Fn(&T) -> bool` trait bound. The `subset_of` spec is
the strongest practical specification given these constraints.

**reduce**: The textbook computes `fold(tree_elements, base, op)` using tree structure.
A full spec would require a recursive spec function over the tree decomposition, but
`ParamBST`'s view is `Set<T::V>` which loses tree structure. The empty-tree base case
`self@.len() == 0 ==> result@ == base@` is the strongest spec achievable without
introducing a structural spec function.

**in_order/collect_in_order**: Length is proven. Content ordering (sorted + bijective with
set elements) requires relating `Set<T::V>` to a sorted `Seq<T::V>`, which is a separate
proof obligation.

All other functions (expose, join_mid, join_m, split, insert, delete, find, min_key,
join_pair, union, intersect, difference) have strong textbook-aligned specs.
