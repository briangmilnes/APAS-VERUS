# R144 Agent 1 — Parallelize OrderStatSelect filter (Chap35). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap35/OrderStatSelectMtEph.rs` — current sequential select.
Read `src/Chap35/OrderStatSelectMtPer.rs` — same, persistent version.
Read `src/Chap36/QuickSortMtEphSlice.rs` — `partition_three_dc` (parallel
three-way partition just merged in R143). Use this as your pattern.

Report file: `plans/r144-agent1-parallel-select-report.md`

## Problem

2 DIFFERS:
```
OrderStatSelectMtEph.rs: select — parallel recursion via join(), but sequential O(n) filter loops dominate span
OrderStatSelectMtPer.rs: select — same
```

The select algorithm partitions around a pivot, then recurses on the appropriate
half. The partition is sequential O(n) filter loops — that's the bottleneck.

## Fix

Replace the sequential partition with parallel filter. The select algorithm does:
1. Pick pivot
2. Partition into elements < pivot and elements >= pivot (or three-way: <, ==, >)
3. Recurse on the half containing the k-th element

The partition step can use parallel filter on the slice-backed sequence, or
use the `partition_three_dc` pattern from QuickSortMtEphSlice.rs.

Read the current implementation to understand which sequence type it uses. If
it uses ArraySeqMtEphSliceS, parallel filter is directly available. If it uses
Vec-backed ArraySeqMtEphS, you may need to:
- Convert to use the slice type, or
- Implement a local parallel partition similar to quicksort

## Both files

Make the same fix in both MtEph and MtPer per the standalone rule.

## Validation

Run `scripts/validate.sh isolate Chap35`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Named closures with explicit ensures for join() calls.
- Update annotations from DIFFERS to matches APAS.

## When done

RCP.
