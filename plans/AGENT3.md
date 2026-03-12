# Agent 3 Report — Round 4

## Assignment

Chap26/50/53/66 (DP + Graph Algorithms). Priority: Chap26 → Chap66 → Chap50 → Chap53.

## Round 4a: Lock-Boundary Conversions

Converted 11 external_body functions to verified bodies using arc_deref + accept patterns.

| # | File | Before | After | Delta | Technique |
|---|------|--------|-------|-------|-----------|
| 1 | MatrixChainMtPer.rs | 3 | 2 | -1 | Full proof: arc_deref dims + memo lock + chain_rec spec |
| 2 | MatrixChainMtEph.rs | 7 | 2 | -5 | arc_deref + reader/writer accepts for lock-boundary fns |
| 3 | OptBinSearchTreeMtEph.rs | 6 | 2 | -4 | arc_deref + reader/writer accepts for lock-boundary fns |
| 4 | OptBinSearchTreeMtPer.rs | 3 | 2 | -1 | arc_deref keys + memo lock clear |

Chap50 total: 21 → 10 holes (-11).

### Functions Converted (4a)

**MatrixChainMtPer.rs** (-1):
- `optimal_cost` — full proof, no accept. arc_deref dims, memo lock clear, matrix_chain_rec
  has full spec so ensures flows.

**MatrixChainMtEph.rs** (-5):
- `dimensions` — reader: arc_deref + acquire_read + clone + release_read
- `set_dimension` — writer: arc_deref + acquire_write + set + release. Accept for bounds
- `update_dimension` — writer: same pattern as set_dimension
- `multiply_cost` — reader: arc_deref + acquire_read. Accepts for bounds and overflow
- `optimal_cost` — mixed: arc_deref dims read + memo lock clear + matrix_chain_rec call

**OptBinSearchTreeMtEph.rs** (-4):
- `keys` — reader: arc_deref + acquire_read + clone + release_read
- `set_key_prob` — writer: arc_deref + acquire_write + set + release. Accept for bounds
- `update_prob` — writer: arc_deref + acquire_write + KeyProb reconstruction. Accept for bounds
- `optimal_cost` — mixed: arc_deref keys read + memo lock clear + obst_rec call

**OptBinSearchTreeMtPer.rs** (-1):
- `optimal_cost` — arc_deref keys len + memo lock clear + obst_rec call

### Spec Additions (4a)

Added requires to trait methods (matching StEph counterparts):
- `MatrixChainMtEphTrait::set_dimension`: `requires index < old(self)@.dimensions.len()`
- `MatrixChainMtEphTrait::update_dimension`: `requires index < old(self)@.dimensions.len()`
- `OBSTMtEphTrait::set_key_prob`: `requires index < old(self)@.keys.len()`
- `OBSTMtEphTrait::update_prob`: `requires index < old(self)@.keys.len()`

## Round 4b: Chap53 Graph Search Conversions

Converted 3 external_body functions using closure requires (per using_closures_standard)
and while-loop conversion (per APAS iterative pseudocode).

| # | File | Function | Delta | Technique |
|---|------|----------|-------|-----------|
| 1 | GraphSearchMtPer.rs | SelectOne::select | -1 | to_seq + nth + accept(clone) — same as StEph pattern |
| 2 | PQMinStEph.rs | pq_find_min_priority | -1 | to_seq + nth + accept(clone), no ensures needed |
| 3 | PQMinStEph.rs | pq_explore | -1 | Tail recursion → while loop, closure requires, seq wf invariants |

Chap53 total: 11 → 8 holes (-3).

### Techniques (4b)

**SelectOne::select (MtPer)**: Copied proven pattern from GraphSearchStEph. MtPer operations
don't require set-level wf, so the proof transfers directly. Accept for V::clone (approved).

**pq_find_min_priority (StEph)**: Removed external_body. Added assert for seq@.len() > 0
(by contradiction from to_set). Accept for V::clone through Pair tuple access.

**pq_explore (StEph)**: Converted tail recursion to iterative while loop per APAS pseudocode.
Added `forall|v: &V| graph.requires((v,)) && priority_fn.requires((v,))` following the
using_closures_standard. Added `#[verifier::exec_allows_no_decreases_clause]` (frontier can
grow, no natural decreasing measure). Key: seq wf must be in loop invariants because
`to_seq()` ensures wf but facts don't cross loop boundaries.

### StPer Blocked by WF Gap

StPer conversions (SelectOne::select, pq_find_min_priority, pq_explore) blocked because
AVLTreeSetStPer.to_seq() does not ensure seq wf (unlike StEph which does). Fixing requires
adding `seq.spec_avltreeseqstper_wf()` to to_seq() ensures in Chap41 — coordinate with
Agent 2.

## Cumulative Results (Rounds 3 + 4a + 4b)

| Metric | Round 3 Start | After 4a | After 4b | Delta |
|--------|---------------|----------|----------|-------|
| Verified | 3670 | 3711 | 3716 | +46 |
| Chap50 holes | 48 | 10 | 10 | -38 |
| Chap53 holes | 11 | 11 | 8 | -3 |
| Total agent3 holes | 66 | 28 | 25 | -41 |

## Per-Chapter Hole Summary

| # | Chap | Holes | Type | Status |
|---|------|-------|------|--------|
| 1 | 26 | 4 | 4 external_body | f64 sort/compare — needs float axioms |
| 2 | 50 | 10 | 10 external_body | See next-steps below |
| 3 | 53 | 8 | 8 external_body | 5 ensures gap, 3 StPer wf gap |
| 4 | 66 | 3 | 3 external_body | HashMap+StdRng — needs wrapper specs |
| | **Total** | **25** | | |

## Next Steps (prioritized)

### Chap50: parallel_min_reduction (4 holes)

APAS specifies this as `reduce(min, ∞, costs)` — a call to the generic parallel reduce
primitive, not a standalone recursive function. The codebase already has verified reduce
in Chap19/ArraySeqMtEph (`reduce_par`) and Chap27/ReduceContractMtEph. Replace the ad-hoc
recursive implementations with calls to existing verified reduce infrastructure.

### Chap50: matrix_chain_rec / obst_rec Mt (4 holes)

Convert `(i..j).map(|k| ...).collect()` to while loops. Use hfscheduler_standard pattern
for lock access + join(). Hardest remaining work — recursive memoization through locks.

### Chap53: StPer wf gap (3 holes)

Coordinate with Agent 2 to add `seq.spec_avltreeseqstper_wf()` to
AVLTreeSetStPer.to_seq() ensures (one-line change + assume in impl body, matching StEph).

### Chap53: graph_search_explore ensures (3 holes)

The DFS frontier bug: `frontier_new = neighbors \ visited_new` drops unselected vertices.
Fix: `frontier_new = (frontier \ selected) ∪ (neighbors \ visited_new)`. Then the ensures
`frontier@.subset_of(visited_all@)` becomes provable. pq_min_multi (2 holes) follows if
pq_explore gets ensures.

### Chap26: f64 (4 holes)

Add float comparison/sort axioms to vstdplus/float.rs.

### Chap66: HashMap+StdRng (3 holes)

Wrap with external_type_specification or spec'd wrappers.

## Files Modified (this session, 4b)

- `src/Chap53/GraphSearchMtPer.rs` — SelectOne::select proved
- `src/Chap53/PQMinStEph.rs` — pq_find_min_priority + pq_explore proved

## Verification

- `scripts/validate.sh`: 3716 verified, 0 errors
- `scripts/rtt.sh`: 2600 tests passed
- No trigger warnings in validate output
- Project-wide holes: 415
