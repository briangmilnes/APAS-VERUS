# R79 Agent 4 — UnionFind find + cascade (Chap65, 5 holes)

## Objective

Continue agent1 R78's work on UnionFindStEph. Agent1 may still be running or may have
results. Read `plans/agent1-round78-report.md` if it exists. If agent1 proved `find`,
work on the cascade (union/equals/num_sets/kruskal_mst). If not, continue where agent1
left off.

## Baseline

- 4905 verified, 0 errors, 0 warnings
- UnionFindStEph.rs: 4 holes (find ROOT, union/equals/num_sets blocked)
- KruskalStEph.rs: 1 hole (kruskal_mst blocked by find/union)

## Holes

| # | Chap | File | Line | Function | Type | Blocked by |
|---|------|------|------|----------|------|------------|
| 1 | 65 | UnionFindStEph.rs | 352 | find | external_body | ROOT |
| 2 | 65 | UnionFindStEph.rs | 377 | union | external_body | find |
| 3 | 65 | UnionFindStEph.rs | 398 | equals | external_body | find |
| 4 | 65 | UnionFindStEph.rs | 406 | num_sets | external_body | find |
| 5 | 65 | KruskalStEph.rs | 178 | kruskal_mst | external_body | find/union |

## Strategy

`find` does path compression — walks parent pointers to root, then flattens the path.
Agent 2 R77 proved `insert` using:
- **Frame lemma pattern**: one `assert forall` for all old keys
- **3-clone + original pattern**: avoid 4th clone isolation issue
- **Proof function isolation**: standalone `proof fn` with `rlimit(50)`

Apply these same techniques to `find`. Key challenges:
1. **Decreases**: rank of current node decreases each step
2. **Loop invariant**: current node in domain, on path to root
3. **Path compression**: updating parent pointers preserves wf (frame lemma)

## Key resources

- `src/Chap65/UnionFindStEph.rs` — Read fully
- `plans/agent1-round78-report.md` — Agent1's progress (if exists)

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round79-report.md` with holes before/after (table with Chap column).
