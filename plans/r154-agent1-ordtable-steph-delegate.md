# R154 Agent 1 — Delegate OrderedTableStEph Methods to OrdKeyMap. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — all 14 methods are now available.
Read `src/Chap43/OrderedTableStEph.rs` — your file.

Report file: `plans/r154-agent1-ordtable-steph-delegate-report.md`

## Problem

OrderedTableStEph wraps OrdKeyMap but most methods still access `self.tree.inner`
(the raw ParamBST) with full bridge proofs. OrdKeyMap now has: new, size, is_empty,
find, insert, delete, split, union, intersect, difference, next_key, prev_key,
rank_key, select_key. These should be used.

## What to do

For each method in OrderedTableStEphTrait:

1. Check if OrdKeyMap has a corresponding method
2. If yes: rewrite the body to delegate to `self.tree.method()`
3. Delete the bridge proof in the body — OrdKeyMap's ensures provide it
4. Validate after each method: `scripts/validate.sh isolate Chap43`

### Methods to delegate

| OrderedTable method | OrdKeyMap method | Current lines | Expected after |
|---|---|---|---|
| find | find | ~145 (bst_find_by_key) | ~3 |
| insert | insert | ~65 | ~5 |
| delete | delete | ~30 | ~3 |
| split_key | split | ~410 (bst_split_by_key) | ~10 |
| union | union | ~255 | ~5 |
| intersection | intersect | ~130 | ~5 |
| difference | difference | ~100 | ~5 |
| next_key | next_key | ~305 (bst_next_by_key) | ~5 |
| previous_key | prev_key | ~286 (bst_prev_by_key) | ~5 |
| rank_key | rank_key | ~279 (bst_rank_by_key) | ~5 |
| select_key | select_key | ~325 (bst_select_by_rank) | ~5 |
| size | size | already delegated | — |
| is_empty | is_empty | already delegated | — |
| empty | new | already delegated | — |

### After delegation: delete dead code

Once all methods delegate, these become dead code:
- `bst_find_by_key` (~145 lines)
- `bst_next_by_key` (~305 lines)
- `bst_prev_by_key` (~286 lines)
- `bst_split_by_key` (~410 lines)
- `bst_select_by_rank` (~325 lines)
- `bst_rank_by_key` (~279 lines)
- 17 bridge lemmas (~500 lines)
- Bridge spec fns (~100 lines)

COMMENT OUT first with `// BYPASSED:`, validate, then delete if clean.

### Ensures compatibility

OrdKeyMap's ensures may use slightly different terms than OrderedTable's.
If the delegation doesn't satisfy OrderedTable's ensures directly, add a
small proof block (1-5 lines) to bridge. Do NOT keep the full 300-line
proof — the bridge should be trivial since OrdKeyMap proves the same thing.

### wf predicate

If not already done, simplify:
```rust
open spec fn spec_orderedtablesteph_wf(&self) -> bool {
    self.tree.spec_ordkeymap_wf()
}
```

## Expected reduction

From ~5,466 lines to ~1,500-2,000 lines. ~3,000-3,500 lines eliminated.

## Validation

`scripts/validate.sh isolate Chap43` after each method.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify OrdKeyMap.rs.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures on OrderedTableStEphTrait.
- All existing RTTs must pass.

## When done

RCP. Report: line count before/after, which methods delegated, which needed
bridge proof adjustments.
