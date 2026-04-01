# R131 Agent 4 — Chap37/39: assess lock-boundary accepts. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `toplevel_coarse_rwlocks_for_mt_modules.rs` — the accept categories
- `tsm_standard.rs` — tokenized state machine pattern

Report file: `plans/r131-agent4-chap37-lock-report.md`

## Background

Chap37 has 5 holes (one per BST Mt variant) and Chap39 has 1 hole. All are the
standard Mt lock-boundary accept pattern: `assume(self.ghost_root@ == tree)` or
similar — the ghost shadow ↔ locked inner value gap.

These files:
```
src/Chap37/BSTAVLMtEph.rs     — 1 hole
src/Chap37/BSTBBAlphaMtEph.rs — 1 hole
src/Chap37/BSTPlainMtEph.rs   — 1 hole
src/Chap37/BSTRBMtEph.rs      — 1 hole
src/Chap37/BSTSplayMtEph.rs   — 1 hole
src/Chap39/BSTTreapMtEph.rs   — 1 hole
```

## Task

This is an ASSESSMENT round, not a fix round. We have two experiments that
eliminate lock-boundary assumes:

1. `src/experiments/bst_plain_mt_tsm.rs` — TSM with token inside RwLock. Zero assumes
   but no View on the outer struct.
2. `src/experiments/bst_plain_mt_pcell.rs` — PCell + PointsTo. Zero assumes but no
   View (Approach A) or View with vacuous specs (Approach B).

## What to do

1. Read ALL SIX holed files. For each, document:
   - The exact assume line and what it assumes
   - What operations use the assume (write ops? read ops? both?)
   - Whether the file uses `accept()` or bare `assume()`
   - Whether the file has a `ghost_root` / `ghost_view` / `ghost_locked_*` field

2. Read both experiments (`bst_plain_mt_tsm.rs` and `bst_plain_mt_pcell.rs`).

3. For ONE file (pick `BSTPlainMtEph.rs` since it's the simplest), write a SKETCH
   (not code, just a plan in the report) of what the TSM migration would look like:
   - What fields change on the struct
   - What the state machine looks like
   - Which operations need transitions vs just reads
   - Whether View is needed for callers (check who calls BSTPlainMtEph methods)
   - Estimated lines of change

4. Assess: is TSM migration worth doing for all 6 files, or should we wait for
   Verus's `make-ghost-send-sync` branch (which fixes Ghost Send/Sync and may
   enable better patterns)?

## Do NOT modify source files. Report only.

## Report format

For each file, one entry:
```
/full/path/file.rs:LINE: assume(exact text) — used by: [list of functions] — pattern: ghost_root/ghost_view
```

Then the BSTPlainMtEph migration sketch. Then the assessment.
