# R165 Prompt B — Agent 2: Extract BSTTreapSpecsAndLemmas from Chap39. AFK.

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

Extract shared spec fns and proof lemmas from Chap39's BST treap files into
`src/Chap39/BSTTreapSpecsAndLemmas.rs`. ~31 duplicated proof functions
across 4 files, 7234 lines.

## Files

`src/Chap39/`:

- BSTTreapStEph.rs
- BSTTreapMtEph.rs
- BSTParaTreapMtEph.rs
- BSTSetTreapMtEph.rs

## Approach

Follow the pattern from `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files and all standards.**
2. **Identify shared specs and lemmas.** BST treap operations share ordering
   lemmas, priority invariants, split/join correctness proofs. St and Mt
   versions likely have identical proof structure with different type bounds.
3. **Create `BSTTreapSpecsAndLemmas.rs`.** Sections 1, 6, 7 only. No types.
4. **Register in lib.rs** as first entry in Chap39's module block.
5. **Variant files import** via `pub use`.
6. **Validate:** `scripts/validate.sh isolate Chap39`

## Report

Write `plans/agent2-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 2: extract BSTTreapSpecsAndLemmas (−N lines)"`, then `git push`.
