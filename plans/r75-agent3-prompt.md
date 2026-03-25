# R75 Agent 3 — Prove Chap66 BoruvkaMtEph parallel Boruvka (12 holes)

## Objective

Prove or eliminate 12 holes in `src/Chap66/BoruvkaMtEph.rs` — the parallel (multi-threaded)
Boruvka MST algorithm.

## Files and holes

| # | Chap | File | Holes | Root causes |
|---|------|------|-------|-------------|
| 1 | 66 | BoruvkaMtEph.rs | 12 | 8 root, 3 downstream + 1 external |

### BoruvkaMtEph.rs (12 holes — 11 external_body + 1 external)

Root causes (8):
- `hash_coin_flips_mt()` — line ~166 — external_body root cause
- `compute_remaining_mt()` — line ~235 — external_body root cause
- `collect_mst_labels_mt()` — line ~295 — external_body root cause
- `build_partition_map_mt()` — line ~355 — external_body root cause
- `vertex_bridges_mt()` — line ~434 — external_body root cause
- `filter_tail_to_head_mt()` — line ~586 — external_body root cause
- `reroute_edges_mt()` — line ~801 — external_body root cause
- `mst_weight()` — line ~937 — external_body root cause

Downstream (3):
- `bridge_star_partition_mt()` — line ~520 — external_body downstream (blocked by compute_remaining_mt)
- `boruvka_mst_mt()` — line ~681 — external_body downstream (blocked by build_partition_map_mt)
- `boruvka_mst_mt_with_seed()` — line ~883 — external_body downstream (blocked by boruvka_mst_mt)

External (1):
- `PartialEq for LabeledEdge` — line ~49 — external root cause

## Strategy

### PartialEq — quick win
Apply the standard eq/clone workaround pattern from
`src/standards/partial_eq_eq_clone_standard.rs`. This should be a straightforward fix.

### Parallel functions — core work
Each `_mt()` function is the parallel version of a sequential counterpart in BoruvkaStEph.
The Mt versions use `HFScheduler` or `join()` for parallelism. The key challenge is:

1. **Thread-boundary wrapping**: Only the thread spawn boundary needs `external_body`, not
   the whole function. Factor out the algorithmic logic into a verifiable helper, then have
   the Mt function call the helper through the thread boundary.
2. **RwLock patterns**: If functions use shared state through RwLock, follow the
   `toplevel_coarse_rwlocks_for_mt_modules.rs` standard.
3. **ParaPair / join()**: If functions use fork-join parallelism, follow the
   `using_closures_standard.rs` for named closures with explicit ensures.

### Approach

1. Read BoruvkaMtEph.rs thoroughly — understand the parallel Boruvka algorithm structure.
2. Read BoruvkaStEph.rs — understand the sequential versions that the Mt functions parallel-ize.
3. For each function:
   a. Read the body inside `external_body`.
   b. Identify whether the issue is thread-spawn boundary, algorithmic logic, or both.
   c. Factor out verifiable logic if possible.
   d. Remove `external_body`, fix verification errors iteratively.
4. `mst_weight` has float arithmetic — check `src/vstdplus/float.rs` for axioms.

## Key resources

- `src/Chap66/BoruvkaMtEph.rs` — target file
- `src/Chap66/BoruvkaStEph.rs` — sequential counterpart
- `src/Chap02/HFSchedulerMtEph.rs` — HFScheduler for parallelism
- `src/standards/using_closures_standard.rs` — closure patterns for join
- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — RwLock pattern
- `src/standards/partial_eq_eq_clone_standard.rs` — PartialEq workaround
- `src/vstdplus/float.rs` — float axioms

## Important rules

- **Never sequentialize parallel code.** The Mt functions must remain parallel.
- **Never propose serializing.** If parallelism is hard to verify, factor the proof away
  from the thread boundary — don't replace threads with loops.
- Follow the Arc deref pattern: write a helper taking `f: &F` with full proof, then have
  the trait impl delegate through `&*f`.

## Validation

Run `scripts/validate.sh` after each file change. Run `scripts/rtt.sh` and `scripts/ptt.sh`
before committing. Push to `agent3/ready`.

## Report

Write `plans/agent3-round75-report.md` with holes before/after per file (table with Chap column).
