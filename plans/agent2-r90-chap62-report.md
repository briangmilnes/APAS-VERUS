# R90 Agent 2 Report — StarContraction r_inv + StarPartition proof attempt

## Summary

Completed Task 2 (r_inv ghost predicate for star_contract_mt). Task 1
(prove parallel_star_partition) attempted but blocked by deep proof
structural issues; external_body retained.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 62   | StarContractionMtEph.rs | 0 | 0 | 0 |
| 2 | 62   | StarPartitionMtEph.rs | 1 | 1 | 0 |
| 3 | 63   | ConnectivityMtEph.rs | 0 | 0 | 0 |
| 4 | 64   | SpanTreeMtEph.rs | 1 | 0 | -1 |

**Total: 2 holes before, 1 hole after, -1 delta.**

## Task 2: Add r_inv to star_contract_mt (DONE)

Added `Ghost(r_inv): Ghost<spec_fn(R) -> bool>` parameter to
`star_contract_mt` and `star_contract_mt_fuel` in StarContractionMtEph.rs,
matching the pattern from the proved StEph version (StarContractionStEph.rs).

### Changes

**StarContractionMtEph.rs (Chap62):**
- Trait `StarContractionMtEphTrait::star_contract_mt`: added `Ghost(r_inv)` param,
  guarded expand requires (`wf && r_inv(r) ==> expand.requires(...)`), two new
  requires for base/expand ensures propagating r_inv, and `ensures r_inv(result)`.
- `star_contract_mt_fuel`: same signature changes, threaded `Ghost(r_inv)` through
  recursive call, added proof steps for r_inv in base case and expand call.
- `star_contract_mt` pub fn: same signature changes, passes r_inv to fuel fn.
- `contract_to_vertices_mt`: passes `Ghost(|r| true)` as trivial r_inv.

**ConnectivityMtEph.rs (Chap63):**
- `count_components_hof`: passes `Ghost(|_r: usize| true)`.
- `connected_components_hof`: passes `Ghost(|_r: (SetStEph<V>, HashMapWithViewPlus<V,V>)| true)`.

**SpanTreeMtEph.rs (Chap64):**
- `spanning_tree_star_contraction_mt`: removed `#[verifier::external_body]`,
  passes `Ghost(|r: SetStEph<Edge<V>>| r.spec_setsteph_wf())`. Base closure
  ensures `spec_setsteph_wf()` and expand closure ensures `spec_setsteph_wf()`,
  matching r_inv. The r_inv propagation proves the function's
  `ensures result.spec_setsteph_wf()` automatically.

### Verification

1246 verified, 0 errors (isolate Chap64). Full crate: 5312 verified,
1 pre-existing error in Chap42/TableMtEph.rs (same on base).

## Task 1: Prove parallel_star_partition (NOT DONE)

### What I tried

1. Removed external_body, identified 16 verification errors.
2. Root cause: `SetStEph::to_seq()` ensures `seq@.no_duplicates()` at
   VALUE level, but the proof reasons about VIEW-level equality
   (comparing `vertices_vec@[i]@`). Two values can differ but share a view.
3. Proved a bridge: `valid_key_type_Edge::<V>()` gives
   `obeys_feq_view_injective::<V>()` (`x@ == y@ ==> x == y`), so
   value no_duplicates implies view no_duplicates.
4. Added `#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]`
   to all 6 loops so the view-level fact is visible.
5. Fixed ghost/actual push mismatch: the original proof pre-computed
   `th_edges@.push(ghost_entry)` but the actual push creates a different
   exec value (via `clone_view()`). Restructured to prove invariant AFTER
   the push with explicit new-entry assertions.

### What blocks it

Even after fixes, 11-12 errors remain. The proof body (inside
external_body) was never verified and has pervasive structural issues:

- **loop_isolation(false) destabilizes loops 5 and 6**: Adding it to loops
  5 (apply th_edges to p_vec) and 6 (build centers/partition_map) causes
  invariant failures that don't occur without it. The enlarged SMT context
  from loop_isolation(false) overwhelms the solver.
- **Without loop_isolation(false), facts don't propagate**: Loops 3-6 need
  facts from earlier loops (vertex_to_index coverage, coin_flips domain,
  edge containment). Default loop isolation requires threading ALL these
  as explicit invariants through every loop.
- **Edge containment from to_seq**: `edge_vec@.map(f).contains(edge_vec@[k]@)`
  needs explicit `Seq::map` unfolding + contains witness. Each loop needs
  its own containment proof.
- **6 loops x 10+ invariants each**: The proof requires ~60 invariant clauses
  total, with complex inter-loop dependencies. Each loop mutation requires
  re-proving all invariants.

### Recommended approach for next round

1. **Thread ALL needed invariants explicitly** through each loop (no
   loop_isolation(false)). This is ~100 lines of invariant additions
   but predictable.
2. **Establish edge containment and coin_flips coverage as standalone
   assert-foralls** before each loop that needs them.
3. **Fix the push proofs** using the restructured approach (push first,
   prove after) with explicit new-entry case.
4. Budget 10+ STEP iterations for this single function.

## Pre-existing issues (not my changes)

- Chap42/TableMtEph.rs:2086 — invariant flakiness (full validate)
- RTT: 4 compile errors (float specs, BSTParaStEph, CycleDetect)
- PTT: 3 SetStEph test failures
