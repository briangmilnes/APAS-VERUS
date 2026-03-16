# Agent 2 — R28: Chap40 fn_missing_requires + Delete Proofs

## State

Main at latest commit. 4114 verified, 0 errors. You are Agent 2.

## Assignment

Two tasks in Chap40 only:

### Task 1: Fix 4 fn_missing_requires (mechanical)

| File | Line | Function | What to add |
|------|------|----------|-------------|
| BSTKeyValueStEph.rs | 565 | clone_link | `requires spec_ordered_link(link)` or equivalent BST invariant on the link |
| BSTKeyValueStEph.rs | 685 | compare_kv_links | `requires spec_ordered_link(...)` on both input links |
| BSTReducedStEph.rs | 546 | clone_link | Same pattern — BST invariant on link |
| BSTReducedStEph.rs | 646 | compare_reduced_links | BST invariant on both links |
| BSTSizeStEph.rs | 538 | compare_links | BST invariant on both links |
| BSTSizeStEph.rs | 1178 | clone_link | BST invariant on link |

For each: read the function body, understand what it needs, add the real `requires`.
**Do NOT add `requires true`.**

### Task 2: Prove delete in BSTReducedStEph.rs and BSTSizeStEph.rs (2 external_body)

Agent 2 already proved delete_link in BSTKeyValueStEph.rs in R26. Now do the same for:

- `BSTReducedStEph.rs:690` — `external_body` on delete. Follow the same rotation-based
  pattern from BSTKeyValueStEph.rs delete_link.
- `BSTSizeStEph.rs:575` — `external_body` on delete. Same pattern.

These are structurally identical to BSTKeyValueStEph delete — they traverse to find the
key, replace with in-order successor/predecessor, and rebalance. The proof pattern is the
same: show the BST invariant is maintained through rotations.

### Task 3: Prove the assume in BSTKeyValueStEph.rs:1276

```
assume(spec_ordered_link(link));
```

This assume says the link is ordered after a delete operation. If you proved delete
correctly in R26, this should follow from the delete postcondition. Wire it up.

## Rules

- Do NOT touch files outside Chap40.
- Do NOT use `verifier::spinoff_prover`.
- Do NOT add `requires true`.
- Run `scripts/validate.sh` after changes. 0 errors required.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- Write report to `plans/agent2-round28-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent2/ready`.
