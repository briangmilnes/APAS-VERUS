# Agent1 Round 32 Report

## Assignment

Prove external_body holes in Chap37 and Chap38 (5 targets).

## Verification

- 4117 verified, 0 errors
- 2613 RTTs passed
- 147 PTTs passed
- Total holes: 188 (was 189)

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 37 | AVLTreeSeq.rs | 1 | 1 | 0 | next() irreducible |
| 2 | 37 | AVLTreeSeqMtPer.rs | 2 | 2 | 0 | 'static closure boundary |
| 3 | 38 | BSTParaStEph.rs | 5 | 4 | -1 | clone proved |
| 4 | 38 | BSTParaMtEph.rs | 9 | 9 | 0 | not assigned |

## What Was Proved

**Task 2b: BSTParaStEph.rs ParamBST::clone** (line 1538)

Removed external_body from `Clone::clone`. The old impl acquired RwLock, cloned inner Option, rebuilt RwLock. The new impl uses expose/join_mid decomposition:

1. `expose()` decomposes BST into `Exposed::Node(left, key, right)` or `Exposed::Leaf`.
2. `join_mid()` reconstructs a new BST from exposed parts.
3. expose's ensures exactly satisfy join_mid's requires, so the proof is structural.

Key technique: **Lexicographic decreases for mutual recursion.** clone calls expose, expose calls clone on children. Termination proof:
- `expose`: `decreases self@.len(), 0nat`
- `clone`: `decreases self@.len(), 1nat`

Added `vstd::set_lib::lemma_set_disjoint_lens` in expose to prove child sizes are strictly smaller than parent.

## What Was Not Proved (and Why)

**Task 1a: AVLTreeSeq.rs next()** — IRREDUCIBLE.
Iterator::next() from std cannot have `requires` in Verus (trait method impls cannot declare requires not in the trait). `type_invariant` conflicts with pub fields needed for `open spec fn view()`. No workaround exists without modifying the iterator standard.

**Tasks 1b/1c: AVLTreeSeqMtPer.rs build_balanced_from_slice / subseq_copy** — IRREDUCIBLE.
`para_pair` requires `FnOnce() -> A + Send + 'static` closures. The inner closures capture `&[T]` slices with non-'static lifetimes. This is a structural limitation of the thread boundary.

**Task 2a: BSTParaStEph.rs expose assume** (line 472) — IRREDUCIBLE.
The assume bridges `T::clone()` view preservation and `cmp_spec` ordering transfer. Proving it requires `obeys_feq_clone` + `view_ord_consistent` in the function's requires, but expose is called from clone which is a trait method with no room for extra requires. The assume is the eq/clone workaround pattern applied to ordering.

## Techniques Used

- Expose/join_mid decomposition pattern for BST clone
- Lexicographic decreases (`self@.len(), 0nat` / `self@.len(), 1nat`) for mutual recursion
- `vstd::set_lib::lemma_set_disjoint_lens` for set size reasoning
- `use_type_invariant(self)` to access ParamBST's structural invariants
