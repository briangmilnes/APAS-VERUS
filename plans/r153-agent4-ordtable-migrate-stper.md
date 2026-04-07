# R153 Agent 4 — Migrate OrderedTableStPer to use OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/ordered-bst-refactor.md` — the design doc.
Read `src/Chap38/OrdKeyMap.rs` — the bridge layer.
Read `src/Chap43/OrderedTableStPer.rs` thoroughly — this is your file.
Read Agent 3's prompt (`plans/r153-agent3-ordtable-migrate-steph.md`) —
you're doing the same migration for the StPer variant.

Report file: `plans/r153-agent4-ordtable-migrate-stper-report.md`

## Problem

OrderedTableStPer has 4,339 lines with 18 duplicated bridge lemmas and
`bst_*_by_key` functions identical to OrderedTableStEph. Migrate to OrdKeyMap.

## What to do

Same steps as Agent 3's StEph migration:

1. Change `pub tree: ParamBST<Pair<K, V>>` to `pub tree: OrdKeyMap<K, V>`
2. Update View to `self.tree@` (OrdKeyMap already views as Map)
3. Simplify trait methods to delegate to `self.tree.method()`
4. Delete bridge lemmas (they live in OrdKeyMap)
5. For methods OrdKeyMap lacks, use `self.tree.inner` temporarily
6. Update wf to `self.tree.spec_ordkeymap_wf()`

## StPer differences from StEph

StPer uses persistent (path-copying) semantics — methods return new values
instead of mutating. Check that OrdKeyMap's insert/delete return new values
or adapt accordingly. OrdKeyMap may use `&mut self` (ephemeral) while StPer
needs consuming/returning patterns. If OrdKeyMap's interface doesn't match
StPer's needs, keep the existing implementation with `self.tree.inner` access
and document the mismatch.

## Expected reduction

From ~4,339 lines to ~1,500-2,000 lines.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs or any Chap38 file.
- Do NOT modify OrderedTableStEph (Agent 3 owns that).
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- All existing RTTs must pass.
- COMMENT OUT before deleting.

## When done

RCP.
