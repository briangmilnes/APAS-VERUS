# R66 Agent 1: AVLTreeSetStEph — Backing Store Rewire + Recursive Defaults

Read these files first:
- `src/standards/iterative_vs_recursive_standard.rs` — the naming pattern
- `src/Chap38/BSTParaStEph.rs` — the target backing store (full trait + impl)
- `src/Chap41/AVLTreeSetStEph.rs` — the file you're changing
- `plans/iterative-vs-recursive-plan.md` — context (you're doing Tier 0b + Tier 1 for this file)

## Goal

Change AVLTreeSetStEph's backing store from AVLTreeSeqStEph (flat sorted array) to
BSTParaStEph (Ch38 parametric BST). Then write recursive defaults for the 7 renamed
functions. The `_iter` variants become the alternative; the defaults become recursive
delegations to BSTParaStEph.

## Why BSTParaStEph

BSTParaStEph (Ch38) has verified recursive: expose, split, join_mid, join_m, join_pair,
find, insert, delete, union, intersect, difference, filter, min_key, in_order, reduce.
Its View is `Set<T::V>` — same as AVLTreeSetStEph. This is the textbook's parametric
BST interface (CS 38.11).

BSTTreapStEph (Ch39) only has find/insert/delete — NOT split/join/union/etc. Don't
use it.

## Step 1: Change the struct and View

Current:
```rust
pub struct AVLTreeSetStEph<T: StT + Ord> {
    pub elements: AVLTreeSeqStEphS<T>,
}
impl View for AVLTreeSetStEph<T> {
    type V = Set<<T as View>::V>;
    open spec fn view(&self) -> Set<<T as View>::V> { self.elements@.to_set() }
}
```

Target:
```rust
pub struct AVLTreeSetStEph<T: StT + Ord> {
    pub tree: BSTParaStEph<T>,
}
impl View for AVLTreeSetStEph<T> {
    type V = Set<<T as View>::V>;
    open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
}
```

BSTParaStEph already views as `Set<T::V>`. The View becomes a trivial delegation.

## Step 2: Update wf

Current wf checks AVLTreeSeqStEph wf + no_duplicates + finite + feq.

New wf:
```rust
open spec fn spec_avltreesetsteph_wf(&self) -> bool {
    self.tree.spec_bstparasteph_wf()
    && self@.finite()
    && obeys_feq_full::<T>()
}
```

BSTParaStEph's wf guarantees BST ordering and size consistency. We add finite
(folding it into wf per the finite-in-wf plan) and feq.

## Step 3: Rewrite function bodies

The 7 renamed functions (find, insert, delete, filter, intersection, union, difference)
currently have `_iter` bodies that iterate over the flat array. The defaults are one-line
delegations to `_iter`.

**Flip it:** The defaults become recursive delegations to `self.tree`'s BSTParaStEph
methods. The `_iter` variants keep the old iterative logic BUT rewritten for the BST
backing store (or, pragmatically, also delegate to BSTParaStEph since the old iterative
bodies won't work on a BST without a flat array).

If you cannot write truly iterative BST traversal for an `_iter` variant, have it
delegate to the same BSTParaStEph method as the default. Both prove trivially because
the specs are identical. The naming convention is preserved; truly iterative BST
algorithms are a future task.

**Other functions** (size, empty, singleton, from_seq, to_seq, select, is_empty, etc.)
also need rewriting to use BSTParaStEph's API. Most are straightforward delegations.

Check BSTParaStEph's trait requirements carefully: many methods require
`vstd::laws_cmp::obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()`. These
need to be in AVLTreeSetStEph's requires clauses or wf predicate.

## Step 4: Spec helpers

You may need to remove or rewrite spec helpers that reference `self.elements` (the old
AVLTreeSeqStEph field). Replace with equivalent specs on `self.tree`. For example,
`spec_elements_sorted` is no longer needed — BSTParaStEph's BST property gives sorted
in-order traversal.

You may need `spec_sorted` for OrderedSet compatibility. Check what OrderedSetStEph
calls on AVLTreeSetStEph and make sure those specs still work.

## Step 5: Update callers

Check `src/Chap43/OrderedSetStEph.rs` and `OrderedSetStPer.rs` — they wrap
AVLTreeSetStEph. If they reference `self.base_set.elements` directly, those accesses
break. They should only use trait methods, which will still work. But check.

Also check `tests/` and `rust_verify_test/` for any tests that construct
AVLTreeSetStEph with `elements:` field syntax.

## Step 6: Imports

Add `use crate::Chap38::BSTParaStEph::*;` to AVLTreeSetStEph.rs.
Remove unused AVLTreeSeqStEph imports if no longer needed.

## Validation

`scripts/validate.sh` after completing the rewrite. The verified count should be
close to 4476 (current baseline). Some functions may need additional proof work
where BSTParaStEph's ensures don't exactly match AVLTreeSetStEph's ensures.
Fix any failures before considering the file done.

Run `scripts/rtt.sh` and `scripts/ptt.sh` after validate is clean.

## Constraints

- Do NOT change BSTParaStEph.rs. It is a dependency, not your file.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT change the AVLTreeSetStEphTrait signatures or specs (requires/ensures).
  The trait is the contract. Only the struct, View, wf, and impl bodies change.
- Do NOT touch AVLTreeSetStPer.rs — that's a follow-up.
- Commit: `R66: AVLTreeSetStEph backing store rewire to BSTParaStEph + recursive defaults`
- Push to `agent1/ready`.

DOT. AFK.
