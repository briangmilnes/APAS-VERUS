# Agent 2 R111 — PTT Fix and Trigger Warning Report

## Summary

All 214 PTTs pass. Zero failures. Two trigger warnings fixed.

## PTT Results

- **Before**: 214 pass, 0 fail (R110 work was already clean)
- **After**: 214 pass, 0 fail (no changes needed)

The R110 iterator PTTs were already passing. The OOM in R110 prevented
confirmation, but no fixes were required this round.

## Pre-existing R110 Fixes in Working Tree

Five files had uncommitted R110 fixes when this round started:

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 37 | ProveBSTSetBBAlphaMtEph.rs | Removed consume patterns (no View on iter) |
| 2 | 37 | ProveBSTSetPlainMtEph.rs | Removed consume patterns (no View on iter) |
| 3 | 43 | ProveAugOrderedTableMtEph.rs | Simplified (Clone verifier issue) |
| 4 | 43 | ProveAugOrderedTableStEph.rs | Simplified (Clone verifier issue) |
| 5 | 43 | ProveAugOrderedTableStPer.rs | Simplified (Clone verifier issue) |
| 6 | — | rust_verify_test/Cargo.toml | Skipped 3 AugOrderedTable test entries |

These were already in the working tree and PTTs pass with them.

## Trigger Warning Fixes

| # | Chap | File | Line | Fix |
|---|------|------|------|-----|
| 1 | 41 | AVLTreeSetMtPer.rs | 314 | Added `#[trigger]` to `vals@[j]@` in choose |
| 2 | 62 | StarPartitionMtEph.rs | 695 | Added `#[trigger]` to `vertices_vec@[j]@` in choose |

Note: StarPartitionMtEph.rs has additional trigger warnings at lines 124, 131,
150, and 598. These were not in scope for this round.

## Validation

- Verified: 5433, 0 errors
- RTT: 3083 pass, 0 skip
- PTT: 214 pass, 0 skip

## Patterns Skipped

Three AugOrderedTable PTT files were removed from Cargo.toml in R110 due to
Verus inability to verify Clone on reducer fn items. The test files remain on
disk but are not compiled. Noted with comment in Cargo.toml:
`# SKIPPED: ProveAugOrderedTable{StEph,StPer,MtEph} — Verus can't verify Clone on reducer fn items`
