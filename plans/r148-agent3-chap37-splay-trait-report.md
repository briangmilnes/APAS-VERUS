# R148/R148b Agent 3 — Traitify BSTSplay in Chap37

## Summary

Moved free functions into trait methods across BSTSplay files in Chap37.
Created `BSTSplayNodeFns` (Node), `BSTSplayLinkFns` (Link<T>), and
`BSTSplayMtNodeFns` (MtEph Node), `BSTSetSplayMtEphHelperFns` (Set).

## Method Counts

### BSTSplayStEph.rs

| # | Chap | File | Function | Self Type | Status |
|---|------|------|----------|-----------|--------|
| 1 | 37 | BSTSplayStEph.rs | update | &mut Node | method |
| 2 | 37 | BSTSplayStEph.rs | size_link | &Link | method |
| 3 | 37 | BSTSplayStEph.rs | height_link | &Link | method |
| 4 | 37 | BSTSplayStEph.rs | find_link | &Link | method |
| 5 | 37 | BSTSplayStEph.rs | min_link | &Link | method |
| 6 | 37 | BSTSplayStEph.rs | max_link | &Link | method |
| 7 | 37 | BSTSplayStEph.rs | in_order_collect | &Link | method |
| 8 | 37 | BSTSplayStEph.rs | pre_order_collect | &Link | method |
| 9 | 37 | BSTSplayStEph.rs | splay | Box<Node> | assoc fn |
| 10 | 37 | BSTSplayStEph.rs | bst_insert | &mut Link | assoc fn |
| 11 | 37 | BSTSplayStEph.rs | insert_link | &mut Link | assoc fn |
| 12 | 37 | BSTSplayStEph.rs | new_node | T | assoc fn |

**8 methods, 4 associated functions.**

### BSTSplayMtEph.rs — 19 functions in BSTSplayMtNodeFns, all associated functions

MtEph Link method conversion not yet applied (same pattern as StEph, pending).

### BSTSetSplayMtEph.rs — 4 functions

`copy_set` is `&self` method. `values_vec`, `rebuild_from_vec`, `build_from_vec` are
associated functions.

## What Blocks Splay/bst_insert/insert_link from Becoming Methods

**splay (Box<Node<T>>):** When moved to `impl BSTSplayBoxFns for Box<Node<T>>`,
the splay proof's 900+ lines of assertions fail. The ensures uses `spec_is_bst_link(&Some(self))`
but Verus can't unfold `as_link()` (trait spec fn) through Box<Node<T>> trait dispatch
for the recursive postconditions. The SMT sensitivity noted in CLAUDE.md is real — the
proof is fragile.

**bst_insert/insert_link (&mut Link<T>):** When converted to `&mut self` methods on
Link<T>, the proof blocks referencing `old(self)` through trait dispatch fail. The verifier
can't properly resolve `spec_contains_link(old(self), x)` through the `BSTSplayLinkFns`
trait's spec function wrappers for `&mut self` postconditions. The `&self` methods work
because they don't need `old(self)`.

## Verification

- Isolate Chap37: 1946 verified, 0 errors
- RTT: 3690 passed, 0 skipped

## Key Technique: Spec Wrapper Functions

Implementing traits on `Link<T>` = `Option<Box<Node<T>>>` requires spec wrapper
functions in the trait because the trait's `Self` is abstract and Verus can't match
`&Self` to `&Link<T>` for free spec functions. Pattern:

```rust
pub trait BSTSplayLinkFns<T: TotalOrder + Clone> {
    spec fn link_is_bst(&self) -> bool;
    spec fn link_contains(&self, value: T) -> bool;
    // ...
    fn find_link<'a>(&'a self, target: &T) -> (found: Option<&'a T>)
        requires self.link_is_bst();
}

impl<T: TotalOrder + Clone> BSTSplayLinkFns<T> for Link<T> {
    open spec fn link_is_bst(&self) -> bool { spec_is_bst_link(self) }
    open spec fn link_contains(&self, value: T) -> bool { spec_contains_link(self, value) }
}
```

Callers bridging between the wrapper trait and the free spec fns need explicit assertions:
```rust
assert(self.root.link_contains(x) == spec_contains_link(&self.root, x));
```
