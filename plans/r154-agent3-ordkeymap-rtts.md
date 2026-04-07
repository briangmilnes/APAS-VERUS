# R154 Agent 3 — OrdKeyMap RTTs + OrderedTable RTT Verification. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs`.
Read `tests/Chap38/TestOrdKeyMap.rs` — existing tests from R153.

Report file: `plans/r154-agent3-ordkeymap-rtts-report.md`

## Task A: Expand OrdKeyMap RTTs

Agent 2 added 27 tests in R153 covering basic operations and ordering.
Add comprehensive RTTs for the bulk operations:

### union tests
- union of disjoint maps
- union of overlapping maps (other wins on collision)
- union with empty
- union of identical maps
- union size correctness

### intersect tests
- intersect of disjoint maps (empty result)
- intersect of overlapping maps
- intersect with empty
- intersect preserves self's values

### difference tests
- difference of disjoint maps (no change)
- difference of overlapping maps
- difference from empty
- difference of identical maps (empty result)

### split tests
- split at existing key
- split at missing key
- split at min/max
- left/right partition correctness
- round-trip: union(left, right).insert(k, v) == original

### Integration tests
- Build large map (100+ entries), verify all operations
- Chain: insert N, union, split, verify invariants

## Task B: Verify OrderedTable RTTs Still Pass

After agents 1+2 delegate OrderedTable methods, the existing OrderedTable RTTs
are the safety net. Run them and confirm all pass:

```bash
scripts/rtt.sh
```

Report the count and any failures.

## Validation

`scripts/rtt.sh` — report total count. Full `scripts/validate.sh` to confirm
compilation.

## Rules

- Do NOT modify OrdKeyMap.rs or any Chap43 source files.
- Test files go in `tests/Chap38/TestOrdKeyMap.rs`.
- All existing RTTs must pass.

## When done

RCP.
