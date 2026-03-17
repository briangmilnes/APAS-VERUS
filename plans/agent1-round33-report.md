# Agent1 Round 33 Report

## Summary

Replaced `Result<..., String>` error type with `BellmanFordError` enum in BellmanFord
and Johnson graph algorithms, eliminating 3 `external_body` holes. Fixed 5
`requires_true` warnings across Chap57/58/59.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 58 | BellmanFordStEphI64.rs | 2 | 0 | −2 |
| 2 | 59 | JohnsonStEphI64.rs | 1 | 0 | −1 |
| 3 | 57 | DijkstraStEphI64.rs | 4 | 4 | 0 |

**Net: −3 holes** (3 external_body eliminated).

## Chapters Closed

- **Chap58** (BellmanFord): 0 holes, clean.
- **Chap59** (Johnson): 0 holes, clean.

## Changes

### TASK 1 — BellmanFordError enum (−3 external_body)

- `src/Chap58/BellmanFordStEphI64.rs`: Added `pub enum BellmanFordError { NegativeCycleDetected, AlgorithmError }` inside `verus!`. Deleted `neg_cycle_error_string` and `algorithm_error_string` (both `external_body`). Changed `bellman_ford()` return type. Added `Debug`/`Display` impls outside `verus!`.
- `src/Chap59/JohnsonStEphI64.rs`: Imported `BellmanFordError`, deleted `neg_cycle_error_string` (`external_body`). `Err(_)` match is transparent.
- `tests/Chap58/TestBellmanFordStEphI64.rs`: Replaced `err_msg.contains("Negative-weight cycle")` with `matches!(result, Err(BellmanFordError::NegativeCycleDetected))`.

### TASK 2 — Fix requires_true warnings (5 warnings)

| # | Chap | File | Function | Action |
|---|------|------|----------|--------|
| 1 | 58 | BellmanFordStEphI64.rs | `clamp_weight` | `// veracity: no_requires` |
| 2 | 57 | DijkstraStEphI64.rs | `pq_entry_new` | `// veracity: no_requires` |
| 3 | 59 | JohnsonStEphI64.rs | `adjust_distance` | `// veracity: no_requires` |
| 4 | 59 | JohnsonStEphI64.rs | `reweight_edge` | `// veracity: no_requires` |
| 5 | 59 | JohnsonStEphI64.rs | `create_negative_cycle_result` | `// veracity: no_requires` |

All five functions genuinely have no preconditions (pure arithmetic or constructors).

## Verification

- **Validate:** 4117 verified, 0 errors.
- **RTT:** 2613 passed, 0 failed.

## Remaining Holes (Chap57 DijkstraStEphI64.rs)

| # | Chap | File | Hole | Reason |
|---|------|------|------|--------|
| 1 | 57 | DijkstraStEphI64.rs | `external_body` on `Ord::cmp` | Verus can't verify `Ordering` return in trait impl |
| 2 | 57 | DijkstraStEphI64.rs | `external_body` on `PartialOrd::partial_cmp` | Same limitation |
| 3 | 57 | DijkstraStEphI64.rs | `assume(pq@.len() * 2 <= usize::MAX)` | PQ size bound |
| 4 | 57 | DijkstraStEphI64.rs | `assume(pq@.len() + 1 <= usize::MAX)` | PQ size bound |

## Techniques Used

- Enum refactoring to eliminate external_body string construction.
- `// veracity: no_requires` for functions with genuinely no preconditions.
