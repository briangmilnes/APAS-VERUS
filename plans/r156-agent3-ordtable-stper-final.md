# R156 Agent 3 — Final OrderedTableStPer Delegation. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — check ALL available methods.
Read `src/Chap43/OrderedTableStPer.rs` — your file.

Report file: `plans/r156-agent3-ordtable-stper-final-report.md`

## Problem

Same as Agent 2 but for StPer. R155 added methods that StPer can now
delegate to. Plus StPer-specific blockers may be unblocked:

- `union_with` / `intersect_with` — replaces combiner union/intersect
- `split` with disjointness — replaces split_key_iter
- `first_key` / `last_key` — replaces first_key_iter / last_key_iter
- `get_key_range` / `split_rank_key` — replaces range ops
- `new()` with wf — replaces empty/singleton manual construction

### StPer-specific: insert/delete

StPer insert/delete clone the OrdKeyMap for persistent semantics. If
OrdKeyMap now has Clone (agent 1 may add it), delegate. If not, keep
the current `self.tree.inner` approach.

## What to delegate

Everything Agent 2 does for StEph, adapted for StPer's persistent signatures.
Plus delete the remaining `#[cfg(never)]` dead code and any bridge lemmas
that become unreferenced.

## Expected reduction

From ~3,442 lines to ~1,500-2,000 lines.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or OrderedTableStEph.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.

## When done

RCP. Report line count before/after.
