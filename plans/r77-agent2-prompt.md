# R77 Agent 2 — Chap65 UnionFindStEph + KruskalStEph + PrimStEph (8 holes)

## Objective

Prove holes in Chap65: UnionFindStEph (5 holes), KruskalStEph (1 hole), PrimStEph (2 holes).

## Baseline

- 4869 verified, 0 errors, 0 warnings
- Chap65: 8 holes across 3 files, all deps clean

## Holes

### UnionFindStEph.rs (5 holes)

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 219 | insert | external_body | ROOT |
| 2 | 231 | find | external_body | insert |
| 3 | 256 | union | external_body | find |
| 4 | 277 | equals | external_body | find |
| 5 | 285 | num_sets | external_body | find |

**Agent 4 R76 findings**: `insert` proof has a 14-conjunct wf invariant that exceeds Z3
rlimit even at `rlimit(60)`. Individual assertions verify with `spinoff_prover` but the
combined proof doesn't. Agent 4 added `strictly_cloned` broadcast axioms to feq.rs and
a `lemma_three_clones_eq` helper.

**New approach**: Agent 4's work is now merged on main. The `strictly_cloned` broadcasts
may help. Try:
1. Split the wf invariant into sub-predicates that verify individually.
2. Use `assert ... by { ... }` blocks to guide Z3 through the 14 conjuncts.
3. If `insert` can be proved, `find` should cascade (it's a path-compression walk).
4. `union`, `equals`, `num_sets` all depend on `find`.

### KruskalStEph.rs (1 hole)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 178 | kruskal_mst | external_body | Uses UnionFind internally |

Blocked by UnionFind. If UnionFind's `insert`/`find`/`union`/`equals` get proved,
kruskal_mst should be provable — it's a loop calling those operations.

### PrimStEph.rs (2 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 72 | total (TotalOrder proof) | assume | float TotalOrder totality |
| 2 | 74 | cmp | external_body | float comparison |

These are float TotalOrder holes. Check if `vstdplus/float.rs` has been updated with
axioms that can prove totality for `WrappedF64`. If not, these are structural and
should be left as-is.

## Key resources

- `src/Chap65/UnionFindStEph.rs` — Read fully
- `src/vstdplus/feq.rs` — Agent 4 added `strictly_cloned` broadcasts here
- `src/Chap65/KruskalStEph.rs` — Read fully
- `plans/agent4-round76-report.md` — Agent 4's findings on UnionFind

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round77-report.md` with holes before/after (table with Chap column).
