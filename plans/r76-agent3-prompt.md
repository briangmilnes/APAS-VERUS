# R76 Agent 3 — BoruvkaMtEph + BoruvkaStEph (14 holes)

## Objective

Eliminate holes in Chap66 Boruvka files: BoruvkaMtEph.rs (12 holes) and BoruvkaStEph.rs
(2 holes). This is the largest single-file hole count remaining.

## Baseline

- 4794 verified, 0 errors, 0 warnings
- BoruvkaMtEph.rs: 12 holes (11 external_body + 1 external)
- BoruvkaStEph.rs: 2 holes (2 external_body)
- All dependencies clean

## BoruvkaMtEph.rs holes (12)

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 49 | PartialEq for LabeledEdge | external | — |
| 2 | 166 | hash_coin_flips_mt | external_body | ROOT |
| 3 | 235 | compute_remaining_mt | external_body | ROOT |
| 4 | 295 | collect_mst_labels_mt | external_body | ROOT |
| 5 | 355 | build_partition_map_mt | external_body | ROOT |
| 6 | 434 | vertex_bridges_mt | external_body | ROOT |
| 7 | 520 | bridge_star_partition_mt | external_body | compute_remaining_mt |
| 8 | 586 | filter_tail_to_head_mt | external_body | ROOT |
| 9 | 681 | boruvka_mst_mt | external_body | collect_mst_labels_mt |
| 10 | 801 | reroute_edges_mt | external_body | ROOT |
| 11 | 883 | boruvka_mst_mt_with_seed | external_body | boruvka_mst_mt |
| 12 | 937 | mst_weight | external_body | ROOT |

## BoruvkaStEph.rs holes (2)

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 210 | vertex_bridges | external_body | ROOT (iterator finiteness) |
| 2 | 378 | boruvka_mst | external_body | vertex_bridges (termination) |

## Strategy

### BoruvkaMtEph.rs

This is an Mt file — parallelism via threads. Each `external_body` function wraps threaded
computation. The strategy from CLAUDE.md: "Parallel algorithms have two parts: structural
logic (verifiable) and thread spawning (not). Wrap only the spawn boundary in external_body,
not the whole algorithm."

**Approach for each function:**
1. Read the function body — understand what is algorithmic logic vs thread spawning.
2. Factor out the algorithmic core into a verified helper if possible.
3. Narrow the external_body to just the thread spawn/join boundary.
4. For `mst_weight`: Agent 2 proved the St version in R75. Check `BoruvkaStEph::mst_weight`
   for the pattern — the Mt version likely just wraps it through RwLock.

**PartialEq for LabeledEdge** (hole #1): This is `#[verifier::external]`. Follow the
eq/clone standard (`src/standards/partial_eq_eq_clone_standard.rs`). Bring inside verus!
with the standard assume pattern.

**mst_weight** (#12): Check if this can delegate to the St version or use a simple
sequential loop (mst_weight just sums edge weights — no parallelism needed).

### BoruvkaStEph.rs

**vertex_bridges** (#1): Uses iterator with `min_by_key` or similar. The R75 report says
"iterator finiteness" is the blocker. Check if replacing with an explicit `while` loop
over `adj_list()` with finiteness invariant solves it.

**boruvka_mst** (#2): Blocked by vertex_bridges. Also needs a termination proof (graph
contraction reduces vertex count). This is a genuine algorithmic proof — may need a
`decreases` argument or a ghost counter showing vertex count shrinks each round.

## Key resources

- `src/Chap66/BoruvkaStEph.rs` — Sequential version (reference for Mt)
- `src/standards/partial_eq_eq_clone_standard.rs` — For PartialEq rewrite
- `src/standards/using_closures_standard.rs` — For any join/closure patterns
- `src/Chap37/BSTRBMtEph.rs` — Example of Mt file with narrowed external_body
- Agent 2 R75 report: `plans/agent2-round75-report.md` — Documents boruvka work done

## Approach

1. Read BoruvkaStEph.rs fully — understand the sequential algorithm.
2. Read BoruvkaMtEph.rs fully — understand the Mt parallelization.
3. Start with the easiest wins: PartialEq (#1), mst_weight (#12).
4. Then vertex_bridges_mt (#6) and other leaf functions.
5. Work bottom-up through the dependency chain.
6. For BoruvkaStEph: try vertex_bridges with explicit loop, then boruvka_mst.

## Important

- Do NOT sequentialize parallel code. Mt must stay parallel.
- Do NOT add `assume` or `accept` without user approval.
- The `hash_coin` structural false positive is info-only — ignore it.
- Agent 3 stalled in R75 on /tmp write permissions. This round there should be no /tmp
  issues — all output goes to the worktree.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent3/ready`.

## Report

Write `plans/agent3-round76-report.md` with holes before/after (table with Chap column).
