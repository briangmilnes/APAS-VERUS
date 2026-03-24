# R67 Agent 2: AVLTreeSetStPer Backing Store Rewire

## Goal

Rewire `AVLTreeSetStPer<T>` from `AVLTreeSeqStPerS<T>` (Ch37 flat sorted array) to
`ParamBST<T>` (Ch38 parametric BST). This mirrors agent1's R66 rewire of AVLTreeSetStEph.

## Reference Implementation

**Read first**: `src/Chap41/AVLTreeSetStEph.rs` — the completed StEph rewire. Your StPer
version should follow the same patterns: struct field, view, wf, delegation to ParamBST
methods, `_iter` variants delegating to defaults.

**Current file**: `src/Chap41/AVLTreeSetStPer.rs`

**Key differences between StEph and StPer**:
- StPer is a persistent (clone-based) data structure — operations return new values
  rather than mutating in place. Where StEph has `fn insert(&mut self, x: T)`, StPer
  has `fn insert(&self, x: T) -> (updated: Self)`.
- StPer's `delete` and `insert` return new trees (functional style).
- Otherwise the trait interface is nearly identical.

## Current Structure

```rust
pub struct AVLTreeSetStPer<T: StT + Ord> {
    pub elements: AVLTreeSeqStPerS<T>,
}
```

wf: `self.elements.spec_avltreeseqstper_wf() && self.elements@.no_duplicates() && self@.finite() && obeys_feq_full::<T>()`

20 trait methods including `_iter` variants from R65.

## Steps

### Step 1: Change struct field

```rust
pub struct AVLTreeSetStPer<T: StT + Ord> {
    pub tree: ParamBST<T>,
}
```

Add `#[verifier::reject_recursive_types(T)]` (required by ParamBST).

### Step 2: Update View

View should become `self.tree@` (ParamBST views as `Set<T::V>`).

### Step 3: Update wf

Follow AVLTreeSetStEph's pattern — wf should be:
```rust
self.tree.spec_bstparasteph_wf()
&& self@.len() < usize::MAX as nat
```

**Do NOT include `obeys_cmp_spec` or `view_ord_consistent` in wf.** Add them as explicit
`requires` on methods that delegate to BST operations needing them (find, insert, delete,
filter, intersection, union, difference, and their _iter variants). This is the lesson
from R66 — type axioms in wf break constructors.

### Step 4: Rewrite method bodies

Each method delegates to the corresponding ParamBST method. For persistent operations
that return `Self`, wrap the result: `AVLTreeSetStPer { tree: result_tree }`.

**Methods that delegate to axiom-requiring BST methods** (need explicit requires):
- find, insert, delete, filter, intersection, union, difference
- find_iter, insert_iter, delete_iter, filter_iter, intersection_iter, union_iter, difference_iter
- from_seq (calls insert in loop)

**Methods that don't need axioms**:
- size (tree.size), to_seq (tree.in_order), empty (ParamBST::new), singleton (ParamBST::singleton)

### Step 5: Update PartialEq::eq

Follow AVLTreeSetStEph pattern — add assumes for wf and axioms in the eq body.

### Step 6: Update imports

Add: `use crate::Chap38::BSTParaStEph::BSTParaStEph::*;`
Remove or keep AVLTreeSeqStPer import only if needed for `to_seq` return type or `from_seq` parameter.

### Step 7: Check callers

- `AVLTreeSetMtPer` (Chap41) — wraps StPer in RwLock. Check if it compiles after the
  field change. Update any `.elements` references to `.tree`.
- `OrderedTableStPer` (Chap43) — wraps `AVLTreeSetStPer<Pair<K, V>>`. References
  `self.base_set.elements` extensively. **Do NOT fix OrderedTableStPer** — that's another
  agent's task. If it breaks, comment it out in lib.rs with a note.
- `OrderedSetStPer` (Chap43) — already commented out. Leave it.

### Step 8: TotalOrder trait

Follow AVLTreeSetStEph's pattern: `spec_elements_sorted` returns `true` (BST is sorted
by construction). `_sorted` variants delegate to non-sorted counterparts.

### Step 9: Verify

- `scripts/validate.sh` — 0 errors
- `scripts/rtt.sh` — all pass
- `scripts/ptt.sh` — all pass
- `scripts/holes.sh src/Chap41/` — 0 holes

## Constraints

- Do NOT modify BSTParaStEph.rs or AVLTreeSetStEph.rs.
- Do NOT add `assume`, `accept`, or `external_body` on algorithmic logic.
- Do NOT fix OrderedTableStPer or OrderedSetStPer — comment them out if they break.
- The `_iter` variants should delegate to their default counterparts (same as StEph).
