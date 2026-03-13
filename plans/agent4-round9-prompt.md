# Agent 4 — Round 9: Chap41 MtEph/MtPer + Chap53

## Mission

Prove the multi-threaded AVL tree set variants and graph search algorithms.

**CRITICAL**: You violated project rules last round by converting assume() to accept()
across 6 files. That work has been reverted. Do NOT use accept() anywhere. Do NOT convert
assumes to accepts. If you can't prove something, leave the assume and explain what blocks it.
Every accept you add is 30 minutes of human cleanup time. Don't be that agent.

## Your Files (ONLY touch these)

Chap41 MtEph/MtPer:
1. `src/Chap41/AVLTreeSetMtEph.rs` — 16 assume, 7 ext_body, 1 wf{true} (24 holes)
2. `src/Chap41/AVLTreeSetMtPer.rs` — 6 assume, 7 ext_body (13 holes)

Chap53 (all 5 files, 7 ext_body total):
3. `src/Chap53/GraphSearchStEph.rs` — 1 ext_body
4. `src/Chap53/GraphSearchStPer.rs` — 2 ext_body
5. `src/Chap53/GraphSearchMtPer.rs` — 1 ext_body
6. `src/Chap53/PQMinStPer.rs` — 3 ext_body
7. `src/Chap53/PQMinStEph.rs` — 0 ext_body (fn_missing_ensures only)

**DO NOT touch Chap41 StEph/StPer files (AVLTreeSetStEph, AVLTreeSetStPer, ArraySet*,
Example41_3). Those are Agent 3's.**

**DO NOT touch any other chapter.**

## Chap41 Strategy

### AVLTreeSetMtEph (24 holes)

This is an Arc<RwLock>-wrapped version of AVLTreeSetStEph. The StEph file proves the
algorithms; the MtEph file wraps with threading.

**wf{true}**: The RwLockPredicate inv is `true`. Give it a real invariant. See
`src/standards/rwlock_predicate_naming_standard.rs` — the struct should be named
`AVLTreeSetMtEphInv` and the inv should relate the locked value to construction-time
ghost state.

**ext_body stubs (7)**: These are thread-boundary wrappers. Pattern:
1. Read the StEph function for the algorithm and specs.
2. The MtEph version acquires the RwLock, calls the algorithm on the inner data, releases.
3. Use `vstdplus/arc_rwlock.rs` bridge functions (`new_arc_rwlock`, `clone_arc_rwlock`).
4. Keep `external_body` ONLY on the thread-spawn boundary, not around algorithmic logic.

**assumes (16)**: Many are closure requires assumptions. Read
`src/standards/using_closures_standard.rs` — lift `f.requires((x,))` into the function's
own `requires` clause, then remove the assume.

**WARNING**: When you add requires to trait functions, you MUST update ALL callers.
Run `scripts/validate.sh` after every change.

### AVLTreeSetMtPer (13 holes)

Same pattern as MtEph but persistent. 6 assumes + 7 ext_body. The StPer file
(Agent 3's) proves the algorithms; you wrap with threading.

## Chap53 Strategy

All 5 files have clean dependencies — nothing blocks you.

**GraphSearchStEph.rs (1 ext_body)**: `graph_search_explore` is the core loop. Read
the function — it iterates: select from frontier, explore neighbors, add to visited.
The loop body is straightforward set operations. Prove the loop invariant:
`visited ∪ frontier` grows monotonically, `visited ∩ frontier` stays empty.

**GraphSearchStPer.rs (2 ext_body)**: `select` in SelectAll trait impl + `graph_search_explore`.
The SelectAll::select just returns (frontier, true). Trivial to prove. For graph_search_explore,
same pattern as StEph but with persistent sets.

**GraphSearchMtPer.rs (1 ext_body)**: Thread-boundary wrapper around graph_search_explore.
Use Arc<RwLock> standard pattern.

**PQMinStPer.rs (3 ext_body)**: `pq_find_min_priority`, `pq_explore`, `pq_min_multi`.
These are priority-first search functions. Read `PQMinStEph.rs` for the sequential versions.
The StPer versions use persistent sets — same algorithms, different set type.

**PQMinStEph.rs**: No ext_body, just fn_missing_ensures on `pq_find_min_priority`.
Add the ensures clause — it returns the vertex with minimum priority from the frontier.

## Standards to Read First

1. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — MtEph/MtPer pattern
2. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridges
3. `src/standards/using_closures_standard.rs` — closure requires/ensures pattern
4. `src/standards/rwlock_predicate_naming_standard.rs` — RwLockPredicate naming + real inv

## CRITICAL RULES

- Do NOT use accept() anywhere. Do NOT convert assumes to accepts.
- If you can't prove something, leave the assume and explain what blocks it.
- Run `scripts/validate.sh` after EVERY change.
- Do NOT touch files outside your assignment.

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap41/ # track Chap41
scripts/holes.sh src/Chap53/ # track Chap53
```

## Target

**AVLTreeSetMtEph**: 24 → ≤ 12. Fix the wf{true}. Prove ext_body stubs. Remove assumes.
**AVLTreeSetMtPer**: 13 → ≤ 6. Same pattern as MtEph.
**Chap53**: 7 → ≤ 3. Close GraphSearchStEph. Prove PQMinStPer stubs.

## When Done

Push to `agent4/ready`. Write `plans/agent4-round9-report.md`.
Include: holes before/after per file (table), techniques used, remaining holes with what blocks them.
