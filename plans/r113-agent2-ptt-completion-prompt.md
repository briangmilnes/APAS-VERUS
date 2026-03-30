# R113 Agent 2 — PTT pattern completion + new PTTs. AFK. PBOGH.

## Context

We have 214 PTTs across ~58 Prove files. Most follow the 6-pattern iterator
standard from `src/standards/iterator_ptt_standard.rs`. Some files are
incomplete (fewer than 4 patterns) and one collection has no PTT at all.

## Task 1: Complete incomplete PTTs

These Prove files have fewer than 4 test patterns. Read each one, read the
source module's iterator infrastructure, and add the missing patterns.

| # | Chap | File | Current | Target | Notes |
|---|------|------|---------|--------|-------|
| 1 | 05 | ProveKleeneStPer.rs | 3 | 4+ | Check if IntoIterator exists |
| 2 | 05 | ProveSetMtEph.rs | 3 | 4+ | Check borrow-into pattern |
| 3 | 18 | ProveArraySeqStPer.rs | 2 | 4-6 | Add borrow-into + consume if available |
| 4 | 18 | ProveLinkedListStEph.rs | 2 | 4-6 | Add borrow-into + consume |
| 5 | 18 | ProveLinkedListStPer.rs | 2 | 4-6 | Add borrow-into + consume |
| 6 | 37 | ProveAVLTreeSeq.rs | 2 | 4+ | Check available patterns |
| 7 | 43 | ProveOrderedSetStPer.rs | 2 | 4+ | Check IntoIterator |
| 8 | 54 | ProveBFSMtEph.rs | 2 | 4+ | Check available patterns |
| 9 | 54 | ProveBFSStEph.rs | 2 | 4+ | Check available patterns |

Three AugOrderedTable files have 0 patterns (skipped in R110 due to Clone
on reducer fn items). Leave those as-is.

## Task 2: Write PTT for BSTSetTreapMtEph

`src/Chap39/BSTSetTreapMtEph.rs` has `iter()` but no Prove file. Write
`rust_verify_test/tests/Chap39/ProveBSTSetTreapMtEph.rs` following the
standard patterns. Read the existing Chap37 BSTSet PTTs for reference
(e.g., `ProveBSTSetAVLMtEph.rs`).

## How to write each PTT

Read `src/standards/iterator_ptt_standard.rs` for the template. The 4
borrow patterns are always required:
- loop-borrow-iter
- loop-borrow-into
- for-borrow-iter
- for-borrow-into

The 2 consume patterns (loop-consume, for-consume) only if `IntoIterator
for Self` exists.

Read the source module before writing — get struct names, iterator types,
ghost iterator types, and constructors right.

## Steps

1. Read the iterator standard.
2. For each incomplete file: read it, read the source module, add missing patterns.
3. For BSTSetTreapMtEph: create new Prove file.
4. Run `cargo check -p rust_verify_test` after each batch.
5. Run `scripts/ptt.sh` when all changes are done.
6. Fix any failures. Iterate until clean.
7. Run `scripts/rtt.sh` to confirm no regressions.
8. Commit.

## Rules

- Do NOT run `scripts/validate.sh`.
- Read each source module before writing its PTT.
- Follow the exact pattern from the standard. Don't improvise.
- If a pattern is genuinely unprovable (ensures true, no useful spec),
  comment it out with `// SKIPPED: ensures true` and note in report.
- No subagents.

## STEP 20

## Report

Write `plans/agent2-r113-ptt-completion-report.md`. Include:
- Table of files with patterns before/after
- PTT pass count before/after
- Any patterns skipped with reason
