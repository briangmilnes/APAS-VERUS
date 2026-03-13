# Agent 1 — Round 9 Report

## Summary

Chap37: 15 → 7 holes (8 eliminated). Target was ≤ 5; fell 2 short.
Chap45: 4 → 4 holes (0 eliminated). Target was 0; all holes are structural.
Verification: 3929 verified (+9 from 3920), 0 errors.
RTT: 2600 passed. PTT: 147 passed.

## Holes Before/After by File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 37 | AVLTreeSeq.rs | 2 | 1 | -1 | Proved clone; next needs feq |
| 2 | 37 | AVLTreeSeqStEph.rs | 4 | 0 | -4 | All 4 external_body proved |
| 3 | 37 | AVLTreeSeqStPer.rs | 4 | 2 | -2 | Proved iter+build; 2 clone assumes remain |
| 4 | 37 | AVLTreeSeqMtPer.rs | 3 | 3 | 0 | Thread boundaries + clone bridge |
| 5 | 37 | BSTSplayStEph.rs | 2 | 1 | -1 | Proved clone; trivial_wf remains |
| 6 | 37 | BST*MtEph (7 files) | 0 | 0 | 0 | requires_true warnings removed |
| 7 | 45 | BinaryHeapPQ.rs | 2 | 2 | 0 | Weak delete_min spec blocks proof |
| 8 | 45 | BalancedTreePQ.rs | 1 | 1 | 0 | Sorted invariant not in spec |
| 9 | 45 | Example45_2.rs | 1 | 1 | 0 | External impl (demo code) |

## Techniques Used

1. **Recursive Box clone pattern**: Replace `external_body` on `Clone for Node<T>` with
   explicit `match &self.left { None => None, Some(boxed) => Some(Box::new((&**boxed).clone())) }`
   and `ensures true, decreases *self`. Applied to AVLTreeSeq.rs, AVLTreeSeqStEph.rs,
   BSTSplayStEph.rs.

2. **Iterative-to-recursive conversion**: Converted `push_left_iter` while loops to
   recursive functions with `decreases *link` for Verus termination proof. Applied to
   AVLTreeSeqStEph.rs and AVLTreeSeqStPer.rs.

3. **as_deref() elimination**: Replaced `Option<Arc<T>>::as_deref()` with
   `match &node { None => None, Some(arc) => Some(&**arc) }` for Verus compatibility.
   Applied to AVLTreeSeqStPer.rs iterator functions.

4. **slice_subrange for sub-slicing**: Used `vstd::slice::slice_subrange(a, i, j)` instead
   of `&a[i..j]` (range indexing unsupported in Verus exec code). Applied to
   AVLTreeSeqStPer.rs `build_balanced_from_slice`.

5. **requires_true removal**: Mechanical removal of `requires true,` from 11+ files across
   BST*MtEph, BSTSplayStEph, and AVLTreeSeq* modules.

## Remaining 7 Holes (Chap37) — What Blocks Them

| # | File | Hole Type | What Blocks It |
|---|------|-----------|----------------|
| 1 | AVLTreeSeq.rs | external_body next() | Strong spec needs feq bridge for generic clone |
| 2 | AVLTreeSeqStPer.rs | assume (set_rec) | obeys_feq_clone bridge for Arc cloning |
| 3 | AVLTreeSeqStPer.rs | assume (build_balanced) | clone bridge: val@ == a@[mid]@ |
| 4 | AVLTreeSeqMtPer.rs | assume (set_rec) | Same as StPer #2 |
| 5 | AVLTreeSeqMtPer.rs | external_body (build_balanced) | &[T] lifetime vs 'static thread boundary |
| 6 | AVLTreeSeqMtPer.rs | external_body (subseq_copy) | Thread spawn boundary with Mutex slots |
| 7 | BSTSplayStEph.rs | trivial_wf | Needs tree_is_bst spec + proof on all splay ops |

## Remaining 4 Holes (Chap45)

| # | File | Hole Type | What Blocks It |
|---|------|-----------|----------------|
| 1 | BinaryHeapPQ.rs | assume (sorted) | delete_min spec lacks minimality guarantee |
| 2 | BinaryHeapPQ.rs | external (impl) | Closure in external impl block |
| 3 | BalancedTreePQ.rs | external_body (contains) | Sorted invariant not captured in spec |
| 4 | Example45_2.rs | external (impl) | Demo code with String operations |

## Commit

Hash: (see git log after commit)
Branch: agent1/ready
