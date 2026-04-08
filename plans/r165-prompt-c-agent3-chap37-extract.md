# R165 Prompt C — Agent 3: Extract BSTSpecsAndLemmas from Chap37. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent3`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Extract shared spec fns and proof lemmas from Chap37's BST files into
`src/Chap37/BSTSpecsAndLemmas.rs`. ~13 duplicated proof functions across
19 files, 20711 lines. Large chapter but many files share the same BST
invariant proofs (ordered, contains, size).

## Files

`src/Chap37/` — 19 algorithm files:

- BSTPlain{St,Mt}Eph.rs
- BSTSplay{St,Mt}Eph.rs
- BSTAVL{St,Mt}Eph.rs
- BSTRB{St,Mt}Eph.rs
- BSTBBAlpha{St,Mt}Eph.rs
- BSTSet{Plain,Splay,AVL,RB,BBAlpha}MtEph.rs
- AVLTreeSeq.rs, AVLTreeSeq{St,Mt}Eph.rs, AVLTreeSeqStPer.rs

## Approach

Follow the pattern from `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files and all standards.** This is a big chapter. Focus on
   identifying the shared BST ordering and containment lemmas — the same
   `spec_is_bst_link`, `spec_contains_link`, size proofs appear everywhere.
2. **Identify shared specs and lemmas.** All BST variants (Plain, Splay, AVL,
   RB, BBAlpha) share the core BST invariant specs and proofs. Rotation lemmas
   may differ by balancing scheme but ordering proofs are identical.
3. **Create `BSTSpecsAndLemmas.rs`.** Sections 1, 6, 7 only. No types.
   Generic over `T: StT + Ord` or similar.
4. **Register in lib.rs** as first entry in Chap37's module block.
5. **Validate:** `scripts/validate.sh isolate Chap37`

## Report

Write `plans/agent3-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 3: extract BSTSpecsAndLemmas (−N lines)"`, then `git push`.
