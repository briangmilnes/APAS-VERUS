# R154 Agent 2 — Delegate OrderedTableStPer Methods to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — all 14 methods available.
Read `src/Chap43/OrderedTableStPer.rs` — your file.

Report file: `plans/r154-agent2-ordtable-stper-delegate-report.md`

## Problem

Same as Agent 1 but for the StPer (persistent) variant. OrderedTableStPer
wraps OrdKeyMap but most methods still access `self.tree.inner` with full
bridge proofs. Delegate to OrdKeyMap and delete ~2,000 lines of bridge code.

## What to do

Same pattern as Agent 1:
1. For each method, delegate to `self.tree.method()`
2. Delete bridge proof bodies
3. After all delegation, comment out then delete dead `bst_*_by_key` functions
   and bridge lemmas

### StPer-specific: persistent semantics

StPer methods return new values instead of mutating. OrdKeyMap's insert/delete
use `&mut self`. For StPer delegation:
- `insert`: create a new OrdKeyMap, insert into it, wrap in new OrderedTableStPer
- OR: access `self.tree.inner` for methods that need consuming semantics and
  wrap the result in `OrdKeyMap { inner: result }`

Check each method's signature carefully.

## Expected reduction

From ~4,326 lines to ~1,200-1,500 lines.

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
