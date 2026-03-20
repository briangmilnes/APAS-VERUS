# R49 Agent 4 Prompt — Chap62 Star Contraction (2 holes)

## REQUIRED READING (before writing ANY code)

1. `src/standards/using_closures_standard.rs`
2. `src/standards/arc_usage_standard.rs`
3. `src/Chap62/StarContractionStEph.rs` (full file)
4. `src/Chap62/StarContractionMtEph.rs` (full file)
5. `src/Chap62/StarPartitionStEph.rs` (full file — understand partition ensures)
6. `src/Chap62/StarPartitionMtEph.rs` (full file — understand partition ensures)
7. `src/Chap06/UnDirGraphStEph.rs` (graph type, spec_graphview_wf)
8. `src/Chap05/SetStEph.rs` (set operations used in graph algorithms)

## Target Files

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 62 | StarContractionStEph.rs | 1 | external_body on star_contract |
| 2 | 62 | StarContractionMtEph.rs | 1 | external_body on star_contract_mt |

## Hole Inventory

| # | Chap | File | Line | Function | Type |
|---|------|------|------|----------|------|
| 1 | 62 | StarContractionStEph.rs | 72 | `star_contract` | external_body |
| 2 | 62 | StarContractionMtEph.rs | 72 | `star_contract_mt` | external_body |

## Architecture

Star contraction is a recursive algorithm:
1. Base case: graph has 0 edges → call `base(&vertices)`
2. Recursive case:
   a. Partition graph into stars (`sequential_star_partition` / `parallel_star_partition`)
   b. Build quotient graph from partition
   c. Recurse on quotient graph
   d. Call `expand(vertices, edges, centers, partition_map, recursive_result)`

Both `base` and `expand` are user-supplied Fn closures. The algorithm is higher-order.

## Strategy

### Step 1: Strengthen helper ensures (currently `ensures true`)

The helpers have minimal specs. You need to strengthen them before star_contract can be proved.

**`sequential_star_partition`** (StarPartitionStEph.rs): Check its current ensures. It should
guarantee that:
- `centers` is a subset of graph vertices
- `partition_map` maps every non-center vertex to a center
- Every mapped center is in the centers set
- Centers and non-centers partition the vertex set

**`build_quotient_graph`** (StarContractionStEph.rs line 98): Currently `ensures true`. Needs:
- `quotient.V == centers` (quotient vertices are the centers)
- `quotient.E` contains only edges between different centers
- Edge count: `quotient.sizeE() <= graph.sizeE()` (no new edges created)
- Well-formedness: `spec_graphview_wf(quotient@)` (if needed by star_contract requires)

### Step 2: Add termination measure

`star_contract` recurses on the quotient graph. Termination requires that the quotient
has strictly fewer edges than the original. The measure is `decreases graph.sizeE()`.

For this you need: `build_quotient_graph` ensures `quotient.sizeE() < graph.sizeE()`
(strict decrease). This holds because: the graph has at least 1 edge (we checked `sizeE() != 0`),
the partition maps at least one edge to a self-loop (which is removed), so at least one edge
is lost.

### Step 3: Remove external_body from star_contract (StEph)

Once helpers have strong enough ensures and the decreases clause is established:
1. Remove `#[verifier::external_body]`
2. The function body is already correct — it's the proof that's missing
3. Key proof obligations:
   - Base case: `base` returns R — need closure ensures
   - Recursive case: partition + build_quotient + recurse + expand
   - The `base` and `expand` closures are opaque `Fn` — you may need Ghost spec
     parameters to capture their logical behavior, OR keep external_body on star_contract
     but add real ensures about the recursion structure

**Important**: The closures `base: &F` and `expand: &G` are opaque Fn trait objects. Verus
cannot reason about their behavior without requires/ensures annotations. This is a key
challenge. Options:
- If the function's ensures is just about the recursion terminating and calling base/expand
  in the right order, you may not need closure specs.
- If the ensures needs to say something about the result R, you'll need Ghost spec parameters
  for the closure behavior (same pattern as Chap38's filter_inner).
- Check what the trait `StarContractionStEphTrait::star_contract` requires/ensures. Currently
  just `requires spec_starcontractionsteph_wf(graph)` with no ensures on the result — so
  you may only need to prove termination and well-formedness of recursive calls.

### Step 4: star_contract_mt (MtEph)

Mirror the StEph proof for the parallel version. Key difference: uses `parallel_star_partition`
instead of `sequential_star_partition`, and `build_quotient_graph_parallel` instead of
`build_quotient_graph`. Same termination argument applies.

**Do NOT sequentialize the Mt version.** It must remain parallel.

## Validation

After changes:
```bash
scripts/validate.sh
scripts/rtt.sh
scripts/ptt.sh
```
Run sequentially, not in parallel.

## Report

Write `plans/agent4-round49-report.md` with holes before/after per file, techniques used,
remaining holes with blockers. Include Chap column in all tables.
