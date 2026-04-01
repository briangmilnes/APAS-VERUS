# R130 Agent 2 — Replace HashMap with OrderedTable in Chap62 StarPartition. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- Standard 8 (`using_closures_standard.rs`) — named closures with ensures
- Standard 15 (`hfscheduler_standard.rs`) — join() patterns
- Standard 23 (`mt_type_bounds_standard.rs`) — use trait aliases, not raw bounds

Report file: `plans/r130-agent2-chap62-report.md`

## Problem

`src/Chap62/StarPartitionMtEph.rs` uses `HashMapWithViewPlus` for vertex-to-index and
coin-flip maps. HashMapWithViewPlus is an unverified type. Chap62 is after Chap43
(OrderedTable) — APAS builds bottom-up and later chapters should use earlier data
structures.

The HashMap build in Loop 1 is sequential at O(n), which is the span bottleneck.
Replacing with a parallel-buildable ordered table could reduce this.

## What to do

1. Read `src/Chap43/OrderedTableMtEph.rs` and `src/Chap43/OrderedTableStEph.rs` to
   understand the API (tabulate, find, insert, domain).

2. In `src/Chap62/StarPartitionMtEph.rs`, replace `HashMapWithViewPlus` usage:
   - `vertex_to_index: HashMapWithViewPlus<V, usize>` → `OrderedTableMtEph` or equivalent
   - `coin_flips: HashMapWithViewPlus<V, bool>` → `OrderedTableMtEph` or equivalent
   - Update all `.get()`, `.insert()`, `@.contains_key()` calls to match the new API

3. If OrderedTable's API doesn't map cleanly (e.g., it uses `(K, V)` pairs differently),
   document the gap and use the closest fit.

4. Parallelize Loop 1 (vertex-to-index build) using tabulate or parallel build if the
   ordered table supports it.

5. Attempt to parallelize Loops 4, 5, 6 with D&C if feasible. Update DIFFERS annotations.

6. The coin_flips map was already parallelized in R128b via `hash_coin_flips_mt` which
   returns a `HashMapWithViewPlus`. If switching to OrderedTable, update that helper too.

## Key constraint

`HashMapWithViewPlus` has O(1) expected lookup. `OrderedTableMtEph` has O(lg n) lookup.
This changes Work from O(n + m) to O((n + m) lg n). APAS specifies O(n + m) work with
O(lg n) span. Document the work/span trade-off if the ordered table lookup cost increases
work.

If the work regression is unacceptable, an alternative is to keep HashMap for lookups
but build a parallel coin_flips table. Document the decision either way.

## Validation

Run `scripts/validate.sh isolate Chap62`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Use trait aliases from Concurrency.rs (standard 23), not raw bounds.
- Named closures with ensures for all join() calls (standard 8).
