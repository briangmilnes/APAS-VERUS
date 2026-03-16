# Agent 3 — R29: Chap37 Remaining fn_missing_requires + Chap39 fn_missing_requires

## State

Main at latest commit. 4118 verified, 0 errors. You are Agent 3.

## Assignment

Two chapters: finish the Chap37 fn_missing_requires that Agent 1 left behind, plus
do Chap39.

### Chap37 Remaining (22 warnings)

Agent 1 fixed 31 of 53 in R28. The remaining 22 are:

**BSTSplayStEph.rs (5)** — Agent 1 claimed "any requires on helpers destabilizes splay
SMT proof." This is suspect. Try adding requires anyway — if verification fails, leave
them and document what specifically breaks.

**BSTSet*MtEph.rs (14 across 5 files)** — These are `values_vec`, `rebuild_from_vec`,
`from_sorted_iter` wrapper functions:

| File | Count |
|------|-------|
| BSTSetSplayMtEph.rs | 3 |
| BSTSetPlainMtEph.rs | 3 |
| BSTSetBBAlphaMtEph.rs | 3 |
| BSTSetAVLMtEph.rs | 3 |
| BSTSetRBMtEph.rs | 2 |

Pattern for these:
- `values_vec`: requires `self.spec_<module>_wf()`.
- `rebuild_from_vec`: read the function body. May need sorted input or length bounds.
- `from_sorted_iter`: may genuinely have no precondition on the iterator. If so, just
  make sure the ensures doesn't depend on a missing requires.

**AVLTreeSeq files (3)** — 1 each in AVLTreeSeqStPer, AVLTreeSeqStEph, AVLTreeSeqMtPer.
These are `push_left_iter` and `inorder_collect` — iterator/traversal helpers.

### Chap39 fn_missing_requires (5 warnings)

All in BSTTreapMtEph.rs (5 warnings). This file has treap helpers that need BST ordering
invariant and/or priority heap invariant. Read each function, add real requires.

## Rules

- Do NOT touch files outside Chap37 and Chap39.
- Do NOT add `requires true`.
- Run `scripts/validate.sh` after changes. 0 errors required.
- If adding requires to BSTSplayStEph.rs genuinely breaks verification, leave those
  and document exactly which functions and what error you get.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- Write report to `plans/agent3-round29-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent3/ready`.
