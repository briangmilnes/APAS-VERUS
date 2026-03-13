# Round 7 Agent Work Plan — Hole Reduction

## Context

434 holes across 15 holed chapters (31 clean). 3780 verified, 0 errors, 2600 RTT, 147 PTT.
Goal: reduce holes, close clean-deps chapters, advance the Chap37 dependency chain.

**Hole breakdown:** 126 assume (29%), 304 external_body (70%), 2 external (0%), 2 trivial_spec_wf (0%).

**Dependency chain:**
```
Chap37 → Chap41 → Chap42 → Chap43 → Chap52, Chap53
Chap37 → Chap45 → Chap57, Chap65
```

**Clean-deps holed chapters** (workable without unblocking): Chap26 (4), Chap37 (40), Chap38 (27), Chap49 (8), Chap50 (11), Chap51 (6), Chap66 (3).

**Internal-only deps** (blocked only by own chapter): Chap39 (37), Chap47 (45).

## Agent 1: Chap37 AVLTreeSeq — Critical Path (40 holes)

**40 holes. Target: -20. High priority — unblocks 8 downstream chapters.**

| # | File | Holes | Type | Deps |
|---|------|-------|------|------|
| 1 | AVLTreeSeq.rs | 3 | assume (nat_max) | clean |
| 2 | AVLTreeSeqStEph.rs | 8 | assume + ext_body | clean |
| 3 | AVLTreeSeqStPer.rs | 14 | ext_body + assume | clean |
| 4 | AVLTreeSeqMtPer.rs | 13 | ext_body | clean |
| 5 | BSTSplayStEph.rs | 2 | trivial_wf + assume | clean |

Phase 1: AVLTreeSeq.rs (3 holes) — prove nat_max assumes, get foundation clean.
Phase 2: AVLTreeSeqStEph.rs (8 holes) — prove insert/delete/find assumes, remove ext_body wrappers.
Phase 3: AVLTreeSeqStPer.rs (14 holes) — replicate StEph proofs to StPer.
Phase 4: BSTSplayStEph.rs (2 holes) — trivial_spec_wf body + remaining assume.
Phase 5: AVLTreeSeqMtPer.rs (13 holes) — lock-boundary ext_body if time.

Do NOT touch BSTAVLMtEph, BSTPlainMtEph, BSTRBMtEph, BSTBBAlphaMtEph, BSTSplayMtEph, or any BSTSet*MtEph files — those are Agent 2's.

**Clean chapter impact:** If all 40 holes closed, Chap37 goes clean (+1) AND unblocks Chap41, 42, 43, 45, 52, 53, 55, 57, 65.

## Agent 2: Chap37 BST MtEph Lock-Boundary (remaining MtEph files)

**Holes in BSTAVLMtEph + BSTPlainMtEph + BSTRBMtEph + BSTBBAlphaMtEph + BSTSplayMtEph + 5 BSTSet*MtEph. Also Chap38 (27 holes). Target: -25.**

Note: Round 6 partially worked these. Count remaining holes from current state.

| # | File | Holes | Type | Deps |
|---|------|-------|------|------|
| 1 | Chap37/BSTAVLMtEph.rs | ~2 | requires_true warning | clean |
| 2 | Chap37/BSTPlainMtEph.rs | ~2 | requires_true warning | clean |
| 3 | Chap37/BSTRBMtEph.rs | ~2 | requires_true warning | clean |
| 4 | Chap37/BSTBBAlphaMtEph.rs | ~2 | requires_true warning | clean |
| 5 | Chap37/BSTSplayMtEph.rs | ~2 | requires_true warning | clean |
| 6 | Chap37/BSTSetAVLMtEph.rs | ~2 | requires_true warning | clean |
| 7 | Chap37/BSTSetBBAlphaMtEph.rs | ~2 | requires_true warning | clean |
| 8 | Chap37/BSTSetPlainMtEph.rs | ~2 | requires_true warning | clean |
| 9 | Chap37/BSTSetRBMtEph.rs | ~2 | requires_true warning | clean |
| 10 | Chap37/BSTSetSplayMtEph.rs | ~2 | requires_true warning | clean |
| 11 | Chap38/BSTParaMtEph.rs | 19 | ext_body | clean |
| 12 | Chap38/BSTParaStEph.rs | 8 | ext_body | clean |

Phase 1: Fix all BST*MtEph requires_true warnings — add real preconditions.
Phase 2: Fix all BSTSet*MtEph requires_true warnings — same pattern.
Phase 3: Chap38/BSTParaStEph.rs (8 holes) — remove ext_body on algorithmic logic.
Phase 4: Chap38/BSTParaMtEph.rs (19 holes) — remove ext_body, prove parallel BST ops.

**Clean chapter impact:** Chap38 could go clean (+1) if all ext_body removed.

## Agent 3: DP Chapters — Chap49 + Chap50 + Chap51 (25 holes)

**25 holes total, all clean deps. Target: -15.**

| # | File | Holes | Type | Deps |
|---|------|-------|------|------|
| 1 | Chap49/MinEditDistMtEph.rs | 2 | ext_body | clean |
| 2 | Chap49/MinEditDistMtPer.rs | 2 | ext_body | clean |
| 3 | Chap49/SubsetSumMtEph.rs | 2 | ext_body | clean |
| 4 | Chap49/SubsetSumMtPer.rs | 2 | ext_body | clean |
| 5 | Chap50/MatrixChainMtEph.rs | 3 | assume (arithmetic overflow) | clean |
| 6 | Chap50/MatrixChainMtPer.rs | 2 | ext_body | clean |
| 7 | Chap50/OptBinSearchTreeMtEph.rs | 2 | ext_body | clean |
| 8 | Chap50/OptBinSearchTreeMtPer.rs | 2 | ext_body | clean |
| 9 | Chap50/OptBinSearchTreeStEph.rs | 1 | ext_body | clean |
| 10 | Chap50/OptBinSearchTreeStPer.rs | 1 | ext_body | clean |
| 11 | Chap51/BottomUpDPMtEph.rs | 1 | ext_body | clean |
| 12 | Chap51/BottomUpDPMtPer.rs | 1 | ext_body | clean |
| 13 | Chap51/TopDownDPMtEph.rs | 2 | ext_body | clean |
| 14 | Chap51/TopDownDPMtPer.rs | 2 | ext_body | clean |

Phase 1: Chap49 Mt files (8 holes, 4 files) — all ext_body on lock-boundary. Prove them to close Chap49 (+1 clean chapter).
Phase 2: Chap51 Mt files (6 holes, 4 files) — same pattern. Close Chap51 (+1 clean chapter).
Phase 3: Chap50 Mt files (10 holes) — ext_body + arithmetic assumes. Close Chap50 (+1 clean chapter).
Phase 4: Chap50 St files (1 hole each, OptBinSearchTree) — stretch goal.

**Clean chapter impact:** Chap49 (+1), Chap51 (+1), Chap50 (+1) = up to +3 clean chapters.

## Agent 4: Chap41 + Chap45 + Chap26 + Chap66 (25 holes, clean deps files only)

**Focus on files with clean deps to close chapters. 25 holes. Target: -12.**

| # | File | Holes | Type | Deps |
|---|------|-------|------|------|
| 1 | Chap41/ArraySetStEph.rs | 9 | assume (set semantics) | clean |
| 2 | Chap41/ArraySetEnumMtEph.rs | 5 | assume (finite) | clean |
| 3 | Chap45/BinaryHeapPQ.rs | 2 | assume (multiset sorted) | clean |
| 4 | Chap45/LeftistHeapPQ.rs | 1 | fn_missing_requires | clean |
| 5 | Chap26/ETSPStEph.rs | 2 | ext_body | clean |
| 6 | Chap26/ETSPMtEph.rs | 2 | ext_body | clean |
| 7 | Chap66/BoruvkaStEph.rs | 3 | ext_body | clean |
| 8 | Chap45/HeapsortExample.rs | ~5 | fn_missing_spec + mixed | blocks Chap45 |

Phase 1: Chap66/BoruvkaStEph.rs (3 holes) — ext_body removal. Close Chap66 (+1 clean chapter).
Phase 2: Chap26/ETSPStEph.rs + ETSPMtEph.rs (4 holes) — ext_body removal. Close Chap26 (+1 clean chapter).
Phase 3: Chap41/ArraySetStEph.rs (9 holes) — set semantics assumes: clone/view bridge, finite set proofs. This is real proof work.
Phase 4: Chap41/ArraySetEnumMtEph.rs (5 holes) — finite set assumes.
Phase 5: Chap45/BinaryHeapPQ + LeftistHeapPQ (3 holes) — multiset + missing requires.

Do NOT touch Chap41/AVLTreeSet* files (those depend on Chap37 being clean first).

**Clean chapter impact:** Chap66 (+1), Chap26 (+1). ArraySetStEph progress helps unblock Chap42/43 chain.

## Summary

| # | Agent | Chapters | Holes In | Target | Work Type |
|---|-------|----------|----------|--------|-----------|
| 1 | Agent 1 | 37 (AVLTreeSeq) | 40 | -20 | AVLTreeSeq proofs — critical path |
| 2 | Agent 2 | 37 (BST MtEph), 38 | 47 | -25 | MtEph requires_true + parallel BST ext_body |
| 3 | Agent 3 | 49, 50, 51 | 25 | -15 | DP Mt ext_body removal |
| 4 | Agent 4 | 41, 45, 26, 66 | 25 | -12 | Set semantics + near-clean chapters |

**Projected:** -72 holes (434 → ~362). **+3 to +6 clean chapters** (34-37 total).

**Agent expectation:** Prove big or go home. Every agent is measured by holes eliminated,
not by difficulty assessments written. Writing "deferred" or "too hard" without exhausting
every approach is failure. The proofs are hard — that's the job. Read the error, search vstd,
write intermediate lemmas, try a different decomposition. Agents that return with most holes
still open and a report full of excuses are wasting compute time and creating cleanup work
for the human.

**No file overlap between agents.** Agent 1 and Agent 2 share Chap37 but work on disjoint files (explicit partition above). Merge in any order.

## File Partition (Chap37)

To avoid the Round 6 agent3/agent4 overlap problem:

- **Agent 1 owns:** AVLTreeSeq.rs, AVLTreeSeqStEph.rs, AVLTreeSeqStPer.rs, AVLTreeSeqMtPer.rs, BSTSplayStEph.rs
- **Agent 2 owns:** BSTAVLMtEph.rs, BSTPlainMtEph.rs, BSTRBMtEph.rs, BSTBBAlphaMtEph.rs, BSTSplayMtEph.rs, BSTSetAVLMtEph.rs, BSTSetBBAlphaMtEph.rs, BSTSetPlainMtEph.rs, BSTSetRBMtEph.rs, BSTSetSplayMtEph.rs
- **Neither agent touches:** BSTAVLStEph.rs, BSTBBAlphaStEph.rs, BSTPlainStEph.rs, BSTRBStEph.rs (already clean)

## Verification

After each agent: `scripts/validate.sh` (0 errors), `scripts/rtt.sh`, `scripts/ptt.sh`.
After all merges: `scripts/all-holes-by-chap.sh`, `scripts/chapter-cleanliness-status.sh`.
