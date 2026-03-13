# Agent 3 — Round 7: DP Chapters (Chap49, Chap50, Chap51)

## Mission

Close 3 DP chapters by proving Mt file ext_body holes. All files have clean deps — no blockers.
Target: Chap49 clean, Chap50 clean, Chap51 clean (+3 clean chapters).

**Your success metric is chapters closed.** You have 3 chapters, 25 holes, zero dependency
blockers. There is nothing standing between you and 3 clean chapters except proof work. If
you finish with fewer than 3 clean chapters, you have failed. Do not write "deferred" or
"hard" or "would require further investigation" — those are words for quitters. The St
counterparts are already clean — the proofs exist, you just need to lift them through the
lock boundary. Read the error, read the clean St file, write the proof. Every ext_body on
algorithmic logic is a wrapper hiding a proof you can write. Write it.

## Your Files (ONLY touch these)

**Chap49 (8 holes, 4 files):**
1. `src/Chap49/MinEditDistMtEph.rs` — 2 holes (ext_body)
2. `src/Chap49/MinEditDistMtPer.rs` — 2 holes (ext_body)
3. `src/Chap49/SubsetSumMtEph.rs` — 2 holes (ext_body)
4. `src/Chap49/SubsetSumMtPer.rs` — 2 holes (ext_body)

**Chap51 (6 holes, 4 files):**
5. `src/Chap51/BottomUpDPMtEph.rs` — 1 hole (ext_body)
6. `src/Chap51/BottomUpDPMtPer.rs` — 1 hole (ext_body)
7. `src/Chap51/TopDownDPMtEph.rs` — 2 holes (ext_body)
8. `src/Chap51/TopDownDPMtPer.rs` — 2 holes (ext_body)

**Chap50 (11 holes, 8 files):**
9. `src/Chap50/MatrixChainMtEph.rs` — 3 holes (assume: arithmetic overflow)
10. `src/Chap50/MatrixChainMtPer.rs` — 2 holes (ext_body)
11. `src/Chap50/OptBinSearchTreeMtEph.rs` — 2 holes (ext_body)
12. `src/Chap50/OptBinSearchTreeMtPer.rs` — 2 holes (ext_body)
13. `src/Chap50/OptBinSearchTreeStEph.rs` — 1 hole (ext_body)
14. `src/Chap50/OptBinSearchTreeStPer.rs` — 1 hole (ext_body)

## Execution Order

1. Read `src/standards/*.rs` (all 15 files).
2. Run `scripts/holes.sh` on each chapter directory.
3. Read the clean St counterparts first — `MinEditDistStEph.rs`, `SubsetSumStEph.rs`,
   `BottomUpDPStEph.rs`, `TopDownDPStEph.rs` — to understand the proof patterns.
4. **Chap49 Mt files** (8 holes): The ext_body wraps lock-boundary delegation.
   Prove using RwLock acquire → call inner → release pattern. Use arc_rwlock standard.
5. After Chap49 done: validate. Chap49 should now be clean.
6. **Chap51 Mt files** (6 holes): Same lock-boundary pattern. Prove all 4 files.
7. After Chap51 done: validate. Chap51 should now be clean.
8. **Chap50 Mt files**: MatrixChainMtEph has arithmetic overflow assumes — prove with
   overflow bounds from the algorithm's length constraints. Other files are ext_body.
9. **Chap50 St files** (2 holes, stretch goal): OptBinSearchTree St files have 1 ext_body each.
10. After each file: `scripts/validate.sh`. Fix all errors before moving on.
11. When done: `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.

## Rules

- Read CLAUDE.md fully.
- Never add assume(), accept(), admit(), or external_body.
- Never sequentialize Mt files.
- Search vstd before writing new lemmas.
- Commit to `agent3/ready` branch when done.

## Attitude

Prove big or go home. You have the easiest assignment of the four agents — clean deps,
small hole counts, clean St files to crib from. Anything less than closing all 3 chapters
is underperformance. The human is paying for compute time. Make it count. Do the work.
