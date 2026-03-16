# Agent 2 Round 26 Report

## Mission

Replace `external_body` on `delete` in Chap40 BSTKeyValueStEph.rs with a rotation-based
implementation. Secondary: prove delete in BSTReducedStEph.rs and BSTSizeStEph.rs, prove
find/insert_link in Chap39 BSTTreapStEph.rs.

## Results

| # | Chap | File | Holes Before | Holes After | Notes |
|---|------|------|-------------|-------------|-------|
| 1 | 40 | BSTKeyValueStEph.rs | 1 external_body | 1 assume | Rotation-based delete implemented |

## Verification

- 4111 verified, 0 errors
- 217 total holes (project-wide)

## What Was Done

Replaced the `external_body` on `delete_link` in `src/Chap40/BSTKeyValueStEph.rs` with a
full rotation-based delete implementation:

- **Less/Greater cases**: Recurse into the appropriate subtree. Content proved via
  `Map::remove` distribution + `K::antisymmetric`. Ordering proved via
  `lemma_ordered_assemble_kv`.
- **Equal/leaf case**: Remove the node (link is already None from `take()`).
- **Equal/rotate_right case**: Rotate right, recurse into right subtree (where old root
  now lives). All proof obligations verify, including `lemma_ordered_assemble_kv`.
- **Equal/rotate_left case**: Rotate left, recurse into left subtree. Content and count
  verify. Ordering has 1 `assume` due to flaky Z3 conjunction (see below).

Added helper lemmas:
- `lemma_strict_lt_transitive` and `lemma_strict_gt_transitive` for key ordering proofs.
- `lemma_node_key_in_link` for root key membership.
- `lemma_content_left_in_link` and `lemma_content_right_in_link` for subtree content.
- Rotation ensures include root key change guarantees via `TotalOrder::le`.

## Remaining Hole: Flaky Z3 Conjunction

The rotate_left Equal case has 1 `assume(spec_ordered_link(link))`. Every sub-assertion
verifies individually:

```
spec_ordered_link(node.left) ✔
spec_ordered_link(node.right) ✔
forall |k| left ordering ✔  (le(k, root) ✔, k != root ✔)
forall |k| right ordering ✔
```

But the conjunction fails. Verus confirms: "NOTE: Verus failed to prove an assertion even
though all of its sub-assertions succeeded."

The symmetric rotate_right case uses identical proof structure and verifies cleanly with
`lemma_ordered_assemble_kv(link)`.

Approaches tried (all failed):
1. `lemma_ordered_assemble_kv` (match-based) -- flaky conjunction
2. `lemma_ordered_assemble_kv_flat` (flat preconditions) -- flaky conjunction
3. Combined proof function with fresh Z3 context -- call-site flaky conjunction
4. Direct `assert(spec_ordered_link(link)) by { reveal_with_fuel }` -- flaky conjunction
5. Assert all preconditions individually before call -- all ✔, call fails
6. Reorder assertions (unchanged side first) -- still fails
7. `rlimit(20)` -- made MORE things fail (3 errors)
8. Helper function with mutual recursion -- tuple decreases polluted Z3 globally
9. Rotate-then-retry approach -- complex decreases polluted Z3 globally
10. `#[verifier::spinoff_prover]` -- broke rotate_right too (was benefiting from neighbor quantifiers)
11. Even the inner `le(k, rot.key) && k != rot.key` conjunction is flaky (both ✔, && fails)

## Not Attempted

- **Task #4**: Prove delete in BSTReducedStEph.rs and BSTSizeStEph.rs (same rotation
  pattern, would hit the same flaky Z3 issue on rotate_left).
- **Tasks #1/#2**: Prove find/insert_link in Chap39 BSTTreapStEph.rs (insert_link blocked
  by missing `obeys_partial_cmp_spec_properties`).
- **Bonus**: Replace `requires true` in BSTTreapMtEph.rs.

## Techniques

- Rotation-based BST deletion (rotate target down until leaf, then remove)
- `K::antisymmetric` to prove key exclusion from unchanged subtrees
- Ghost variables to capture pre-mutation content for post-mutation bridging
- `Map::remove` distribution over `union_prefer_right` + `insert`
