# R166 Prompt A — Agent 1: Lift repeated proof patterns to lemmas in Chap37 Plain+Splay. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Find assert and proof block patterns that repeat across functions within each
file, and across the St/Mt variant pair. Extract them into proof lemmas.
Lemmas shared by both variants go in `BSTSpecsAndLemmas.rs` (already exists).
Lemmas specific to one file stay in that file.

## Your files (Chap37 Plain + Splay only)

- `src/Chap37/BSTPlainStEph.rs` — 4 fns ≥50 proof lines
- `src/Chap37/BSTPlainMtEph.rs` — 4 fns ≥50 proof lines
- `src/Chap37/BSTSplayStEph.rs` — 3 fns, 1722 proof lines (splay is 694 alone)
- `src/Chap37/BSTSplayMtEph.rs` — 3 fns, 902 proof lines
- `src/Chap37/BSTSpecsAndLemmas.rs` — shared specs/lemmas (add to this)
- `src/Chap37/BSTSetPlainMtEph.rs` — thin wrapper, check for duplication
- `src/Chap37/BSTSetSplayMtEph.rs` — thin wrapper, check for duplication

Do NOT modify AVL, RB, or BBAlpha files — another agent handles those.

## Approach

**Step 1 — Read all your files and all standards.**

**Step 2 — Within each file, find repeated assert/proof patterns across functions.**
For example, in BSTPlainStEph.rs, do `insert_node`, `delete_node`, and
`delete_min_node` all prove the same BST ordering invariant after modification?
If yes, extract a `lemma_bst_ordering_after_modify` or similar.

**Step 3 — Across St/Mt pairs, find identical proof structures.**
BSTPlainStEph and BSTPlainMtEph implement the same algorithm. Their proofs
differ only in type bounds (StT vs MtKey). If a lemma can be generic enough
to serve both, put it in `BSTSpecsAndLemmas.rs`.

**Step 4 — For splay specifically:** R163 already extracted `lemma_zig_child_ordering`
and `lemma_zag_child_ordering`. Look for MORE repeated patterns in the remaining
~1400 combined splay proof lines. Each rotation case (zig-zig, zig-zag, zag-zig,
zag-zag) likely has shared element-preservation and size-preservation proofs.

**Step 5 — Extract and validate.** After each lemma extraction:
```bash
scripts/validate.sh isolate Chap37
```

## Report

Write `plans/agent1-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 1: lift proof patterns Chap37 Plain+Splay (−N lines)"`, then `git push`.
