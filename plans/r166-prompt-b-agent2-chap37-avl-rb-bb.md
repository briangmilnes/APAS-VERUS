# R166 Prompt B — Agent 2: Lift repeated proof patterns to lemmas in Chap37 AVL+RB+BBAlpha. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent2`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Find assert and proof block patterns that repeat across functions within each
file, and across St/Mt variant pairs. Extract them into proof lemmas.
Shared lemmas go in `BSTSpecsAndLemmas.rs` (already exists).

## Your files (Chap37 AVL + RB + BBAlpha only)

- `src/Chap37/BSTAVLStEph.rs` — 5 fns ≥50 proof lines (808 total)
- `src/Chap37/BSTAVLMtEph.rs` — rebalance, insert, rotate
- `src/Chap37/BSTRBStEph.rs` — insert_node (76 lines)
- `src/Chap37/BSTRBMtEph.rs` — insert_link (101 lines), rotates
- `src/Chap37/BSTBBAlphaStEph.rs` — delete_node (106), insert (81)
- `src/Chap37/BSTBBAlphaMtEph.rs` — delete_node (112), delete_min (78)
- `src/Chap37/AVLTreeSeq.rs` — shared AVL tree sequence type
- `src/Chap37/AVLTreeSeqStEph.rs`, `AVLTreeSeqMtPer.rs`, `AVLTreeSeqStPer.rs`
- `src/Chap37/BSTSpecsAndLemmas.rs` — shared specs/lemmas (add to this)
- `src/Chap37/BSTSetAVLMtEph.rs`, `BSTSetRBMtEph.rs`, `BSTSetBBAlphaMtEph.rs` — thin wrappers

Do NOT modify Plain or Splay files — another agent handles those.

## Approach

**Step 1 — Read all your files and all standards.**

**Step 2 — Rotation lemmas.** AVL, RB, and BBAlpha all do rotations. The proofs
that rotations preserve BST ordering and element containment should be
structurally identical. If `rotate_left` and `rotate_right` proofs are
duplicated across AVL/RB/BBAlpha, extract shared rotation lemmas into
`BSTSpecsAndLemmas.rs`.

**Step 3 — Rebalance patterns.** AVL's `rebalance` (193/177 lines) is the biggest
function. It has multiple cases (left-heavy, right-heavy, balanced). Each
case likely re-proves ordering after rotation. Extract the common rebalance
invariant proof.

**Step 4 — Within-file patterns.** `insert_node` and `delete_node` in each
variant prove the same post-conditions. Extract shared insert/delete
postcondition lemmas.

**Step 5 — Validate:** `scripts/validate.sh isolate Chap37`

## Report

Write `plans/agent2-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 2: lift proof patterns Chap37 AVL+RB+BBAlpha (−N lines)"`, then `git push`.
