# R78 Agent 1 — UnionFindStEph `find` + cascade (Chap65, 5 holes)

## Objective

Prove `find` in UnionFindStEph.rs, which unblocks `union`, `equals`, `num_sets` (3 downstream)
and `kruskal_mst` in KruskalStEph.rs (1 downstream). 5 holes total.

## Baseline

- 4898 verified, 0 errors, 0 warnings
- UnionFindStEph.rs: 4 holes (find ROOT, union/equals/num_sets blocked by find)
- KruskalStEph.rs: 1 hole (kruskal_mst blocked by find/union/equals)

## Holes

| # | Chap | File | Line | Function | Type | Blocked by |
|---|------|------|------|----------|------|------------|
| 1 | 65 | UnionFindStEph.rs | 352 | find | external_body | ROOT |
| 2 | 65 | UnionFindStEph.rs | 377 | union | external_body | find |
| 3 | 65 | UnionFindStEph.rs | 398 | equals | external_body | find |
| 4 | 65 | UnionFindStEph.rs | 406 | num_sets | external_body | find |
| 5 | 65 | KruskalStEph.rs | 178 | kruskal_mst | external_body | find/union/equals |

## Context from R77

Agent 2 proved `insert` using:
- **Frame lemma pattern**: one `assert forall` covering all old keys
- **3-clone + original pattern**: avoid 4th clone `strictly_cloned` isolation issue
- **Proof function isolation**: standalone `proof fn` with `rlimit(50)`

These techniques are now on main. The `strictly_cloned` broadcasts in `vstdplus/feq.rs`
are also available.

## Strategy for `find`

`find` does path compression — walks parent pointers to root, then flattens the path.
Key challenges:
1. **Decreases measure**: each step increases rank. Use rank of current node as decreases.
2. **Loop invariant**: current node is in domain, on path to root.
3. **Path compression loop**: updating parent pointers preserves wf. Use the frame lemma
   pattern from `insert`.
4. **Clone equality**: each `parent.get().unwrap().clone()` needs equality chains.

Read `src/Chap65/UnionFindStEph.rs` fully. Read the wf predicate. Read `insert` (now proved)
for the frame lemma pattern.

## Cascade

Once `find` is proved:
- `union`: calls `find` twice, then updates rank/parent. Similar frame lemma.
- `equals`: calls `find` twice, compares roots. Should be straightforward.
- `num_sets`: calls `find` in a loop, counts distinct roots.
- `kruskal_mst`: loop calling insert/find/union/equals. Standard loop invariant.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round78-report.md` with holes before/after (table with Chap column).
