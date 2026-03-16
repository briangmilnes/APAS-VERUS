# Agent 2 — Round 25: Chap40 Insert/Delete with Real Ordering Proofs

## Mission

You proved find/contains/get in R24 by defining `spec_ordered_link` and adding it to wf.
Now prove the hard part: **insert and delete** with real ordering preservation through
rotations and tree restructuring. 5 holes remain — all insert or delete.

The user's feedback: "The real invariants should be on these and proofs should be attempted."
Don't give up at the first SMT difficulty. Push through.

## Current State (5 holes)

| # | Chap | File | Holes | Functions |
|---|------|------|:-----:|-----------|
| 1 | 40 | BSTSizeStEph.rs | 1 | delete |
| 2 | 40 | BSTKeyValueStEph.rs | 2 | insert, delete |
| 3 | 40 | BSTReducedStEph.rs | 2 | insert, delete |

## What You Proved in R24

- `spec_ordered_link` in all 3 files (BST ordering invariant)
- `spec_ordered_link` added to each file's wf predicate
- find + contains in BSTSizeStEph (2 holes closed)
- find + contains + get in BSTKeyValueStEph (3 holes closed)
- find + contains + get in BSTReducedStEph (3 holes closed)
- Total: 8 holes closed, 13 → 5

Your R24 report noted: insert in BSTKeyValue/BSTReduced hit SMT trigger issues with
`take()`-modified structure references during rotation. Delete in all 3 files needs
proving that filtered sorted traversal produces an ordered tree.

## Part 1: Prove Insert (BSTKeyValueStEph + BSTReducedStEph)

BSTSizeStEph::insert was proved in R23. The same approach should work for the other two.

### Insert Strategy

The insert body does:
1. Compare key with node.key
2. Recurse into left or right subtree
3. After recursive insert, call `fix_up` (rotations for balancing)

**Ordering preservation through insert**:
- If `key < node.key`, insert into left subtree. All keys in left subtree were `< node.key`.
  After insert, all keys in left subtree are still `< node.key` (new key is `< node.key`).
  Right subtree unchanged. Ordering preserved.
- Mirror for `key > node.key`.
- If `key == node.key`, update value in place. No structural change. Ordering trivially preserved.

**Ordering preservation through rotations**:
You already proved content preservation through rotations in R23 (pre-move ghost capture).
Ordering preservation follows from: if the BST property holds before rotation, and content
is preserved, and the structural transformation maintains the left-right ordering, then
BST property holds after.

Key proof technique from Agent 1's splay proof (R23):
- `reveal_with_fuel(spec_is_bst_link, 2)` or `reveal_with_fuel(spec_ordered_link, 2)` to
  expose the ordering constraints two levels deep
- Ghost capture before `take()`: `let ghost orig_left = root.left;`
- After rotation, assert the left/right subtree contents haven't changed relative to the
  root key

### SMT Trigger Hints

Your R24 report mentioned trigger issues with `take()`-modified structures. Try:
1. `reveal_with_fuel(spec_ordered_link, 3)` — expose more structure
2. Explicit `assert` statements at each step: assert ordering holds for left, then right,
   then the node itself
3. Break the proof into smaller lemmas if the SMT solver times out on the combined assertion
4. Use `reveal(obeys_cmp_ord)` and `reveal(obeys_partial_cmp_spec_properties)` as you did
   for find

## Part 2: Prove Delete (All 3 Files)

Delete is harder. The typical BST delete:
1. Find the node with the target key
2. If leaf: remove it
3. If one child: replace with child
4. If two children: find min of right subtree (or max of left), swap, delete the swapped node

**Ordering preservation through delete**:
- Removing a node doesn't add any keys, so existing ordering relationships are preserved.
- The tricky part is the two-children case: the replacement node (min of right subtree)
  is > all keys in left subtree and < all remaining keys in right subtree.

Check how BSTSizeStEph's delete works. If it uses a filter+rebuild approach instead of
in-place removal, the proof strategy is different:
- Collect all elements except the target key (filter)
- Build a new tree from the filtered collection
- The filtered collection is sorted (removing one element from a sorted sequence preserves
  sortedness)
- Building from a sorted sequence produces an ordered tree

### Reference: Clean BST Files

Look at how these clean files handle delete:
- `src/Chap37/BSTPlainStEph.rs` — simplest BST, may have delete proved
- `src/Chap37/BSTAVLStEph.rs` — AVL tree delete with rebalancing

## Part 3: Replace `requires true` in Chap40 (Bonus)

Your R24 work added `requires true` to some functions in BSTKeyValueStEph.rs (2),
BSTReducedStEph.rs (2), BSTSizeStEph.rs (2). Replace these with the real `spec_ordered_link`
predicates that you now have. These functions should require ordering on their Link
arguments.

## Important

- You MAY strengthen wf predicates and add requires to functions.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`. If a function needs no precondition, omit requires entirely.
- `scripts/validate.sh` after each file — 0 errors.
- If insert proves but delete doesn't, commit the insert wins. Partial progress is real.

## Deliverables

- Insert proved in BSTKeyValueStEph and BSTReducedStEph (2 holes)
- Delete proved where possible (up to 3 holes)
- `requires true` replaced with real specs (6 instances)
- `plans/agent2-round25-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
