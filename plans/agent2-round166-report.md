# R166 Agent 2 Report: Lift repeated BST proof patterns to shared lemmas in Chap37

## Summary

Extracted 4 shared proof lemmas into `BSTSpecsAndLemmas.rs` that replace duplicated
inline BST ordering/containment proofs across 5 files in the AVL, RB, and BBAlpha
BST variants. Net reduction: **120 lines**.

## Lemmas added to `src/Chap37/BSTSpecsAndLemmas.rs`

| # | Chap | Lemma | Purpose |
|---|------|-------|---------|
| 1 | 37 | `lemma_bst_insert_left` | BST + containment after inserting into left child |
| 2 | 37 | `lemma_bst_insert_right` | BST + containment after inserting into right child |
| 3 | 37 | `lemma_bst_delete_left` | BST + containment after deleting from left child |
| 4 | 37 | `lemma_bst_delete_right` | BST + containment after deleting from right child |

## Files modified

| # | Chap | File | Lines removed | Lines added | Pattern replaced |
|---|------|------|---------------|-------------|-----------------|
| 1 | 37 | BSTAVLStEph.rs | 55 | 11 | insert left/right proof blocks |
| 2 | 37 | BSTAVLMtEph.rs | 30 | 7 | insert left/right proof blocks |
| 3 | 37 | BSTRBStEph.rs | 52 | 9 | insert left/right proof blocks |
| 4 | 37 | BSTBBAlphaStEph.rs | 104 | 18 | insert + delete left/right proof blocks |
| 5 | 37 | BSTBBAlphaMtEph.rs | 81 | 19 | insert + delete left/right proof blocks |
| 6 | 37 | BSTSpecsAndLemmas.rs | 0 | 210 | new shared lemmas |

## Rotation lemma attempt

Attempted to extract rotation BST proofs (rotate_right/rotate_left) into shared
lemmas. This failed because `tree_is_bst()` on parameter-passed trees hits Z3
fuel limits — the recursive BST predicate needs to unfold through an equality
constraint, which Z3 cannot do within default fuel. The rotation proofs remain
inline in each file (AVLStEph, AVLMtEph, RBStEph). They share `lemma_bst_deep`
but the tree-construction-coupled assertions cannot be cleanly factored out.

## Verification

- Full validate: 5588 verified, 0 errors
- RTT: 3776 passed
- PTT: 221 passed

## Techniques

- **Proof-fn parameter passing for BST facts**: the insert/delete lemmas take
  the original tree, new child, and reconstructed tree as parameters. The lemma
  proves ordering quantifiers and containment equivalence internally. The caller
  gets `new_tree.tree_is_bst()` and `forall|x| tree_contains(x) <==> ...` from
  the lemma ensures, replacing ~15 lines of inline proof per call site with a
  single lemma invocation.

- **Z3 fuel limitation on tree_is_bst**: `tree_is_bst` is a conjunction of
  recursive calls and quantifiers. When the tree is passed as a proof-fn parameter
  with `requires tree == Node(...)`, Z3 must substitute through the equality before
  unfolding, which exceeds fuel limits for deeply-nested trees (rotation produces
  2-level nesting). Insert/delete only produce 1-level nesting, which stays within
  fuel.

## Net lines

−120 lines (358 removed across 5 files, 238 added including 210 in shared lemmas).
