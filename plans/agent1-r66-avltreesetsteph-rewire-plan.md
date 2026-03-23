# R66 Agent 1: AVLTreeSetStEph Backing Store Rewire — Detailed Plan

## Goal

Change `AVLTreeSetStEph<T>` backing store from `AVLTreeSeqStEphS<T>` (Ch37 flat sorted
array) to `ParamBST<T>` (Ch38 parametric BST). Write recursive defaults for 7 renamed
functions: find, insert, delete, filter, intersection, union, difference.

## Critical Design Issue: `obeys_cmp_spec` Gap

**Problem.** `ParamBST<T>` operations (find, insert, delete, split, union, intersect,
difference, filter) all require `vstd::laws_cmp::obeys_cmp_spec::<T>()` and
`view_ord_consistent::<T>()` in their `requires` clauses. These are unprovable for
generic `T: Ord` — Verus only has broadcast proofs for concrete numeric types (u64, etc.).

**Current AVLTreeSetStEphTrait signatures cannot change** (task constraint). They require
only `self.spec_avltreesetsteph_wf()`. So `obeys_cmp_spec` and `view_ord_consistent`
must come from somewhere to satisfy BSTParaStEph's requires.

**Three options:**

| # | Approach | Holes location | Count | Severity |
|---|----------|---------------|-------|----------|
| A | Include cmp in wf | empty(), singleton(), from_seq() | 3 | Low: "can't prove type axiom" |
| B | Exclude cmp from wf | find, insert, delete, filter, intersection, union, difference, + _iter variants | 14+ | High: no function body verifies |
| C | Add external_body trigger fn | None (trigger fn is the hole) | 1 | Low: localized trust assumption |

**Recommendation: Option A.** Include `obeys_cmp_spec::<T>()` and
`view_ord_consistent::<T>()` in the wf predicate. This means:

- `empty()`, `singleton()`, `from_seq()` will have verification failures — they cannot
  prove these type-level axioms for generic T. These are 3 irreducible holes.
- All other functions (find, insert, delete, filter, intersection, union, difference,
  all _iter variants, size, to_seq, clone, eq) verify cleanly because wf provides cmp.
- For concrete types (u64, etc.), callers satisfy wf because `broadcast use
  vstd::laws_cmp::group_laws_cmp` fires automatically.
- Parallel: `obeys_feq_full::<T>()` is already in BSTParaStEph's wf and has the same
  limitation — it uses `obeys_feq_full_trigger` (an existing external_body) to bridge.

**Alternative (Option C):** Create `obeys_cmp_spec_trigger::<T>()` in vstdplus analogous
to `obeys_feq_full_trigger::<T>()` — an `external_body` proof fn that returns
`b: bool` with `ensures b ==> obeys_cmp_spec::<T>()`. This is a 1-line trust assumption
that mirrors the existing feq pattern and would give 0 holes. But the task says "Do NOT
add external_body" — this needs user approval.

## New Struct and View

```rust
pub struct AVLTreeSetStEph<T: StT + Ord> {
    pub tree: ParamBST<T>,
}

impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
    type V = Set<<T as View>::V>;
    open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
}
```

ParamBST already views as `Set<T::V>`, so no conversion needed.

## New wf Predicate

```rust
open spec fn spec_avltreesetsteph_wf(&self) -> bool {
    self.tree.spec_bstparasteph_wf()     // self@.finite() && obeys_feq_full::<T>()
    && self@.len() < usize::MAX as nat   // capacity for insert
    && vstd::laws_cmp::obeys_cmp_spec::<T>()
    && view_ord_consistent::<T>()
}
```

Why `self@.len() < usize::MAX`: BSTParaStEph::insert requires `old(self)@.len() <
usize::MAX`, but AVLTreeSetStEphTrait::insert requires `old(self)@.len() + 1 < usize::MAX`.
The trait's stronger bound covers both. Including len in wf lets delete maintain it
automatically (size decreases). insert's trait requires `len + 1 < usize::MAX` which is
stronger than wf, so the new wf is re-established after insert.

## Function Implementation Strategy

### Defaults (recursive via BSTParaStEph)

All 7 default functions delegate directly to ParamBST methods:

| # | Function | BSTParaStEph call | Notes |
|---|----------|------------------|-------|
| 1 | find | `self.tree.find(x).is_some()` | find returns Option<T>, need .is_some() for B |
| 2 | insert | `self.tree.insert(x)` | Mutates in place |
| 3 | delete | `self.tree.delete(x)` | Mutates in place |
| 4 | filter | `Self { tree: self.tree.filter(f, Ghost(spec_pred)) }` | Returns new ParamBST |
| 5 | intersection | `Self { tree: self.tree.intersect(other_tree_ref) }` | Need &other.tree |
| 6 | union | `Self { tree: self.tree.union(other_tree_ref) }` | Need &other.tree |
| 7 | difference | `Self { tree: self.tree.difference(other_tree_ref) }` | Need &other.tree |

### _iter variants

All 7 _iter variants delegate to the default (same BSTParaStEph call):

```rust
fn find_iter(&self, x: &T) -> (found: B) { self.find(x) }
fn insert_iter(&mut self, x: T) { self.insert(x) }
// ... etc.
```

Per the task: "If you cannot write truly iterative BST traversal for an `_iter` variant,
have it delegate to the same BSTParaStEph method as the default."

### Non-renamed functions

| Function | Implementation |
|----------|---------------|
| size | `self.tree.size()` |
| empty | `Self { tree: ParamBST::new() }` + proof for wf |
| singleton | `Self { tree: ParamBST::singleton(x) }` + proof for wf |
| from_seq | Loop: iterate seq via nth, clone, insert into self |
| to_seq | collect_in_order → Vec → AVLTreeSeqStEphS::from_vec |

### TotalOrder trait

With BST backing, sorted is always true by construction:

```rust
open spec fn spec_elements_sorted(&self) -> bool { true }
open spec fn spec_values_seq(&self) -> Seq<T> { Seq::empty() }
```

All `_sorted` methods delegate to non-sorted counterparts (ensures trivially includes
`self.spec_elements_sorted()` which is `true`).

### PartialEq (eq_clone_workaround)

```rust
fn eq(&self, other: &Self) -> (equal: bool) {
    proof { assume(self.spec_avltreesetsteph_wf()); assume(other.spec_avltreesetsteph_wf()); }
    let equal = self.size() == other.size() && self.difference(other).size() == 0;
    proof { assume(equal == (self@ == other@)); }
    equal
}
```

Same assume pattern as current code (eq_clone_workaround — classified as warnings, not holes).

### Clone

```rust
fn clone(&self) -> (cloned: Self) {
    AVLTreeSetStEph { tree: self.tree.clone() }
}
```

ParamBST::clone ensures `cloned@ == self@`.

### Debug / Display (outside verus!)

Use `collect_in_order` into a Vec, then iterate:

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut v: Vec<T> = Vec::new();
    self.tree.collect_in_order(&mut v);
    write!(f, "{{")?;
    for i in 0..v.len() {
        if i > 0 { write!(f, ", ")?; }
        write!(f, "{:?}", v[i])?;
    }
    write!(f, "}}")
}
```

## Proof Helpers

| Helper | Status | Reason |
|--------|--------|--------|
| `lemma_wf_implies_len_bound` | KEEP | Used in from_seq for capacity proof |
| `spec_inorder_values` | KEEP | Public spec fn, may have external callers |
| `spec_seq_sorted` | KEEP | Used by TotalOrder trait |
| `lemma_inorder_values_maps_to_views` | KEEP | Public, harmless |
| `lemma_empty_set_is_sorted` | SIMPLIFY | Body references .elements; simplify to empty body (ensures is now `true`) |
| `lemma_push_sorted` | KEEP | Standalone, no struct dependency |
| `lemma_subseq_sorted` | KEEP | Standalone, no struct dependency |

## Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | AVLTreeSetStEph.rs | Complete rewrite: struct, view, wf, all impls |
| 2 | 41 | AVLTreeSetMtEph.rs | Fix 2 proof lines (342, 362): `common_st.elements@` → `common_st.tree@` or `common_st@` |
| 3 | — | lib.rs | Comment out 5 Chap43 modules |

### lib.rs Changes (5 modules commented out)

```
// R66: commented out — depend on AVLTreeSetStEph.elements field (removed in backing store rewire)
// pub mod OrderedSetStEph;
// pub mod OrderedSetStPer;
// pub mod OrderedSetMtEph;
// pub mod OrderedTableMtPer;
// pub mod Example43_1;
```

These access `self.base_set.elements` (OrderedSet*) or depend transitively on those modules.

### AVLTreeSetMtEph.rs Fixes

Line 342: `vstd::seq_lib::seq_to_set_is_finite(common_st.elements@);`
→ Remove or replace with `assert(common_st@.finite());` (wf guarantees finite via BSTParaStEph wf).

Line 362: `vstd::seq_lib::seq_to_set_is_finite(remaining_st.elements@);`
→ Same fix.

## Expected Verification Outcome

### Clean (no holes)

- find, find_iter, insert, insert_iter, delete, delete_iter
- filter, filter_iter, intersection, intersection_iter
- union, union_iter, difference, difference_iter
- size, to_seq, clone
- All TotalOrder _sorted methods (trivially true)
- Default, PartialEq (workaround assumes — classified as warnings)

### Holes (3 — irreducible without Option C)

| # | Function | Why |
|---|----------|-----|
| 1 | empty() | Cannot prove obeys_cmp_spec::<T>() for generic T |
| 2 | singleton() | Same |
| 3 | from_seq() | Same (calls empty/insert internally) |

These would be eliminated by Option C (obeys_cmp_spec_trigger in vstdplus).

### Possible additional holes

- `to_seq`: Converting BSTParaStEph → Vec → AVLTreeSeqStEphS needs proof that
  `seq@.to_set() =~= self@`. This proof depends on what `collect_in_order` ensures.
  Currently `collect_in_order` only ensures length, not element membership. May need
  `in_order()` instead (which ensures element membership) plus a Vec conversion loop
  with feq proof work. If the proof gets complex, this could become a partial hole.

## Imports

```rust
use std::fmt;
use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::cmp::PartialEqSpecImpl;

use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;  // for from_seq/to_seq types
use crate::Chap38::BSTParaStEph::BSTParaStEph::*;         // NEW: backing store
#[cfg(verus_keep_ghost)]
use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
use crate::Types::Types::*;
use crate::vstdplus::total_order::total_order::TotalOrder;
```

Broadcast use adds `group_laws_cmp`:

```rust
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    vstd::laws_cmp::group_laws_cmp,
};
```

## File Size Estimate

Old file: 2288 lines (heavy iterative loop bodies with 50+ line proof blocks each).
New file: ~400–500 lines (thin delegation layer over ParamBST).

## Execution Order

1. Write new AVLTreeSetStEph.rs (complete rewrite)
2. Fix AVLTreeSetMtEph.rs (2 proof lines)
3. Comment out 5 modules in lib.rs
4. `scripts/validate.sh` — fix errors
5. `scripts/rtt.sh` — fix test failures
6. `scripts/ptt.sh` — fix proof test failures
7. Commit + push

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| obeys_cmp_spec holes in empty/singleton/from_seq | Certain | Option C (user approval needed) or accept 3 holes |
| to_seq proof complexity | Medium | Use in_order() + Vec clone loop; worst case: partial hole |
| filter PredSt vs Fn(&T)->bool mismatch | Low | PredSt<T> = Fn(&T)->B where B=bool; compatible with BSTParaStEph's Fn(&T)->bool |
| intersection/union/difference accessing other.tree | Low | &other.tree gives &ParamBST<T>; trait methods take &Self so other.tree is accessible |
| MtEph fix insufficient | Low | Only 2 lines; may need assert(wf) instead of seq_to_set_is_finite |
| RTT failures from changed struct field | Medium | Tests construct via trait (empty/singleton), not struct literal; should be OK |

## Decision Needed

**Option A vs Option C for obeys_cmp_spec.** Option A (cmp in wf, 3 holes) is the no-new-
external_body path. Option C (trigger fn, 0 holes) requires adding one external_body proof
fn to vstdplus — same pattern as existing `obeys_feq_full_trigger`. User approval needed
for Option C since the task says "Do NOT add external_body."
