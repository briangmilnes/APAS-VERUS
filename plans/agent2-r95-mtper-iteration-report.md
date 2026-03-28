# Agent 2 R95 Report: MtPer iteration external_body removal

## Objective

Remove `external_body` from `num_edges` in `AdjTableGraphMtPer.rs` and
assess `delete_vertex` for further proof work.

## Results

| # | Chap | File | Function | Change | Holes Before | Holes After |
|---|------|------|----------|--------|-------------|-------------|
| 1 | 52 | AdjTableGraphMtPer.rs | num_edges | Removed external_body, added loop with invariants + 3 assumes | 1 ext_body | 3 assumes |
| 2 | 52 | AdjTableGraphMtPer.rs | delete_vertex | No change needed — already implemented without external_body | 2 assumes | 2 assumes |

### File-level hole summary

| Metric | Before | After |
|--------|--------|-------|
| external_body | 1 | 0 |
| assume (algorithmic) | 12 | 14 |
| assume (rwlock:predicate) | 8 | 9 |
| Total holes | 21 | 23 |
| Real proof targets | 13 | 14 |

Net: -1 external_body, +3 assumes. The external_body hid all algorithmic
logic from the verifier. The replacement exposes the iteration loop (domain
traversal, find, size accumulation) to Verus with explicit loop invariants.

## What was done

### num_edges (lines 191-230)

Removed `#[verifier::external_body]`. The loop iterates over `domain().to_seq()`,
calls `find()` per vertex, accumulates `neighbors.size()`. Added:

- Loop invariant: `count as nat <= self.spec_num_edges()` (overflow bound)
- 3 assumes, all blocked by weak OrderedTableMtPer API:
  1. `neighbors.spec_avltreesetmtper_wf()` — find() has no ensures
  2. `count + neighbors@.len() <= spec_num_edges()` — overflow guard, find/domain ensures too weak
  3. `count == spec_num_edges()` — bridge from loop sum to recursive map sum

### delete_vertex (lines 279-297)

Already implemented without external_body. Uses `delete()` then `map()` to
remove v from all neighbor sets. The 2 existing assumes (wf + !dom.contains)
are blocked by:
- `OrderedTableMtPer::delete` ensures only `dom().finite()`
- `OrderedTableMtPer::map` ensures only `dom().finite()`

No further progress possible without stronger MtPer API.

## Blocking issues

All remaining assumes in this file share the same root cause:
**OrderedTableMtPer has minimal ensures on its API functions.**

| Function | Missing ensures |
|----------|----------------|
| `find()` | No ensures at all (no dom.contains, no value spec) |
| `insert()` | Only `dom().finite()` (no dom update, no value spec) |
| `delete()` | Only `dom().finite()` (no dom removal spec) |
| `map()` | Only `dom().finite()` (no value transformation spec) |
| `domain()` | Only `self@.dom().finite()` (no domain coverage spec) |

Agent1 (R95) is strengthening these ensures. Once that work lands and is
merged, the assumes in this file can be systematically replaced with real
proofs connecting to the stronger specs.

## Verification

- `scripts/validate.sh isolate Chap52`: 2811 verified, 0 errors
- Full validation OOM on this machine (32GB, Verus+Z3 peaked at 19GB)
- No warnings in Chap52 except pre-existing Clone derive notes

## Steps used: 3 (of 20)

1. Read MtPer, StEph, StPer, OrderedTableMtPer, AVLTreeSetMtPer APIs
2. Removed external_body from num_edges, added loop invariants + assumes
3. Simplified redundant assume, confirmed clean validation
