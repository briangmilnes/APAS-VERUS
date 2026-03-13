# Agent 2 — Round 7: Chap37 BST MtEph + Chap38 Parallel BST

## Mission

Fix requires_true warnings in all BST*MtEph and BSTSet*MtEph files (Chap37), then prove
ext_body holes in Chap38 parallel BST files.

**Your success metric is holes eliminated.** Every hole you started with that still exists
when you finish is a failure. Do not write "deferred" or "hard" or "would require" — those
are words for quitters. Read the error, read vstd, write intermediate lemmas, decompose the
obligation, try a different approach. If you genuinely exhaust every idea on a specific hole,
say exactly what you tried and where you got stuck — but that should be the exception, not
the pattern. You were put on this task to prove things, not to catalog reasons you didn't.

## Your Files (ONLY touch these)

**Chap37 BST MtEph (10 files, ~2 holes each = ~20 holes):**
1. `src/Chap37/BSTAVLMtEph.rs`
2. `src/Chap37/BSTPlainMtEph.rs`
3. `src/Chap37/BSTRBMtEph.rs`
4. `src/Chap37/BSTBBAlphaMtEph.rs`
5. `src/Chap37/BSTSplayMtEph.rs`
6. `src/Chap37/BSTSetAVLMtEph.rs`
7. `src/Chap37/BSTSetBBAlphaMtEph.rs`
8. `src/Chap37/BSTSetPlainMtEph.rs`
9. `src/Chap37/BSTSetRBMtEph.rs`
10. `src/Chap37/BSTSetSplayMtEph.rs`

**Chap38 (2 files, 27 holes):**
11. `src/Chap38/BSTParaStEph.rs` — 8 holes (ext_body)
12. `src/Chap38/BSTParaMtEph.rs` — 19 holes (ext_body)

**DO NOT touch AVLTreeSeq*, BSTSplayStEph, BSTAVLStEph, BSTPlainStEph, BSTRBStEph,
BSTBBAlphaStEph, or any other StEph files. Those are Agent 1's or already clean.**

## Execution Order

1. Read `src/standards/*.rs` (all 15 files).
2. Run `scripts/holes.sh src/Chap37/` and `scripts/holes.sh src/Chap38/`.
3. **BSTAVLMtEph.rs**: Fix `requires true` — add real preconditions (spec_wf, bounds).
   This is the template. All BST*MtEph have the same pattern.
4. Replicate to BSTPlainMtEph, BSTRBMtEph, BSTBBAlphaMtEph, BSTSplayMtEph (4 files).
5. **BSTSetAVLMtEph.rs**: Same requires_true pattern but for set operations.
   Template for the other 4 BSTSet*MtEph files.
6. Replicate to BSTSetBBAlphaMtEph, BSTSetPlainMtEph, BSTSetRBMtEph, BSTSetSplayMtEph.
7. **BSTParaStEph.rs** (8 holes): Remove ext_body on split/join/union/intersection.
   These are algorithmic — write real proofs.
8. **BSTParaMtEph.rs** (19 holes): Same operations, parallel variant. Prove through
   lock-boundary + delegation pattern.
9. After each file: `scripts/validate.sh`. Fix all errors before moving on.
10. When done: `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.

## Rules

- Read CLAUDE.md fully.
- Never add assume(), accept(), admit(), or external_body.
- Never sequentialize Mt files.
- Search vstd before writing new lemmas.
- Commit to `agent2/ready` branch when done.

## Attitude

Prove big or go home. You are not here to assess difficulty — you are here to eliminate
holes. The requires_true warnings are mechanical — no excuses for leaving those. The Chap38
ext_body removals are real proof work — that's why you exist. A round where you fix the
warnings but punt on Chap38 is a half-finished round. Do the work. Every ext_body on
algorithmic logic is a wrapper hiding a proof you can write. Write it.
