# Agent 2 Round 6 — BST MtEph + AVLTreeSeq (Chap37)

## Mode: AFK — execute relentlessly

Read CLAUDE.md and `src/standards/*.rs` before starting. Do the proof work.
Run `scripts/validate.sh` after each file. Fix errors before moving on.
When done, commit all changes, push to `agent2/ready`, then stop.

## Assignment

**62 holes: 31 assume, 30 ext_body, 1 trivial_wf. Target: -25.**

| # | File | Holes | Type |
|---|------|-------|------|
| 1 | BSTAVLMtEph.rs | 5 | assume (lock-boundary) |
| 2 | BSTPlainMtEph.rs | 5 | assume (lock-boundary) |
| 3 | BSTRBMtEph.rs | 5 | assume (lock-boundary) |
| 4 | BSTBBAlphaMtEph.rs | 5 | assume (lock-boundary) |
| 5 | BSTSplayMtEph.rs | 5 | assume (lock-boundary) |
| 6 | BSTSplayStEph.rs | 2 | 1 assume + 1 trivial_wf |
| 7 | AVLTreeSeq.rs | 3 | mixed |
| 8 | AVLTreeSeqStEph.rs | 7 | 2 assume + 5 ext_body |
| 9 | AVLTreeSeqStPer.rs | 13 | mixed |
| 10 | AVLTreeSeqMtPer.rs | 12 | ext_body |

All files are in `src/Chap37/`.

## Strategy

### Phase 1: BST*MtEph lock-boundary assumes (25 assumes, mechanical)

All 5 BST*MtEph files have identical structure: RwLock wrapper around a St BST.
Each has 5 assumes for insert, delete, find, size, is_empty — all lock-boundary bridges.

1. Read BSTAVLMtEph.rs. Understand the struct, View, lock predicate, ghost shadow.
2. For each assume: determine if it bridges ghost↔lock (accept) or is provable (prove).
3. Solve BSTAVLMtEph as template.
4. Replicate to BSTPlainMtEph, BSTRBMtEph, BSTBBAlphaMtEph, BSTSplayMtEph.

### Phase 2: BSTSplayStEph (2 holes)

- trivial_wf: Splay trees have no global invariant. `true` may be correct, or there
  may be a BST ordering invariant to express. Investigate.
- assume: likely clone or eq bridge. Investigate and prove or accept per standard.

### Phase 3: AVLTreeSeq proves (if time permits)

These are real proof work, not mechanical lock-boundary:
- AVLTreeSeq.rs (3 holes): insert_at_link nat_max, iterator, clone.
- AVLTreeSeqStEph.rs (7 holes): 2 assume (height overflow, next_key) + 5 ext_body.
- AVLTreeSeqStPer.rs (13 holes): mixed assumes and ext_body.
- AVLTreeSeqMtPer.rs (12 holes): ext_body delegating to St.

Priority: Phase 1 first (25 holes, mechanical, high ROI). Phase 3 only if Phase 1
completes cleanly.

## Rules

- Do NOT convert `assume` to `accept` wholesale. Read each assume, understand it,
  try to prove it first.
- Do NOT add new assumes or external_body.
- Do NOT modify files outside Chap37.
- Run `scripts/validate.sh` after each file change.

## Prior Work

Agent 2's full history is in `plans/AGENT2.md`. Key context:
- Chap37: 37→62 holes in current scan (some files gained holes from wf cascade).
- fn_missing_spec: 73 fixed across 15 files in earlier rounds.
- Chap37 wf size bound added to AVLTreeSeqStEph.rs (Round 5).
- compare_trees and clone_link proved in Round 5.
- BSTTreapMtEph (Chap39) Arc removed — now plain RwLock + ghost shadow. Not your chapter.

## Baseline

3771 verified, 0 errors. Chap37: 62 holes.

## Deliverable

When done: commit, push to `agent2/ready`, update `plans/AGENT2.md` with results.
