# Agent 4 — Round 9b: Continue Chap41 MtPer + Chap53

## Mission

You made progress on MtEph and PQMinStPer but left 4 files untouched. Keep going.

## Remaining Work (ONLY touch these files)

Chap41:
1. `src/Chap41/AVLTreeSetMtPer.rs` — 13 holes (6 assume, 7 ext_body). You got ZERO
   progress last round. This is the same pattern as MtEph. Apply whatever worked there.

Chap53:
2. `src/Chap53/GraphSearchStEph.rs` — 1 ext_body (graph_search_explore)
3. `src/Chap53/GraphSearchStPer.rs` — 1 ext_body remaining (you proved select, keep going)
4. `src/Chap53/GraphSearchMtPer.rs` — 1 ext_body (thread-boundary wrapper)
5. `src/Chap53/PQMinStEph.rs` — fn_missing_ensures on pq_find_min_priority. Add the ensures.

## Strategy

**AVLTreeSetMtPer**: You proved 5 holes in MtEph. Apply the same techniques. The MtPer
pattern is identical — Arc<RwLock> wrapper around StPer algorithms. If the MtEph ext_body
stubs followed a pattern, MtPer stubs follow the same pattern.

**GraphSearch ext_body (3 files)**: These are the core graph search loop. The algorithm
is: while frontier is non-empty, select from frontier, explore neighbors, add unseen
to frontier, add explored to visited. Prove the loop or at minimum prove the SelectAll
implementations and the thread-boundary wrapper.

**PQMinStEph**: Just needs an ensures clause on pq_find_min_priority. It returns the
vertex with minimum priority from the frontier. Read the function, add the ensures.

## CRITICAL RULES

- Do NOT use accept() anywhere.
- Do NOT touch files outside your assignment.
- Run `scripts/validate.sh` after EVERY change.

## Validation

```bash
scripts/validate.sh
scripts/holes.sh src/Chap41/
scripts/holes.sh src/Chap53/
```

## Target

**AVLTreeSetMtPer**: 13 → ≤ 8. At least match MtEph's ratio.
**Chap53**: Close GraphSearchStEph, GraphSearchMtPer. Fix PQMinStEph ensures. 4 → ≤ 1.

## When Done

Push to `agent4/ready`. Update `plans/agent4-round9-report.md` with round 9b results.
