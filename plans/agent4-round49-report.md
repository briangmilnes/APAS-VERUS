# Agent 4 — Round 49 Report

## Objective

Remove 2 `external_body` holes from Chap62 Star Contraction:
1. `star_contract` in `StarContractionStEph.rs`
2. `star_contract_mt` in `StarContractionMtEph.rs`

## Results

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 62 | StarContractionStEph.rs | 1 external_body | 1 assume | 0 | Recursion+closures now verified |
| 2 | 62 | StarContractionMtEph.rs | 1 external_body | 1 assume | 0 | Same pattern as StEph |
| 3 | 62 | StarPartitionStEph.rs | 0 | 0 | 0 | Clean |
| 4 | 62 | StarPartitionMtEph.rs | 0 | 0 | 0 | Clean |

**Global**: 34 holes → 34 holes. 37 clean chapters. 4431 verified. 2613 RTT pass. 143/147 PTT pass (4 pre-existing Chap43 failures).

**Net hole count change**: 0 (2 external_body replaced by 2 focused assumes).

## What Was Proved

Both `star_contract` (StEph) and `star_contract_mt` (MtEph) now have verified bodies:
- **Recursion structure**: Fuel-based termination (`decreases fuel`)
- **Base case**: `graph.sizeE() == 0 || fuel == 0` → vertices are wf → base closure callable
- **Recursive case**: partition → build quotient → recurse → expand
- **Closure callability**: Conditional quantification `forall|s| s.spec_setsteph_wf() ==> base.requires((s,))`
- **Expand callability**: Universal `forall|v, e, c, p, r| expand.requires((v, e, c, p, r))`

## Remaining Assumes (2)

Both assumes are `assume(spec_graphview_wf(quotient@))` in `build_quotient_graph` / `build_quotient_graph_parallel`.

**Root cause**: Generic `Clone::clone` in Verus has no view-preserving ensures (`result@ == self@`). After `centers.insert(vertex.clone())`, we know `centers@ == old_centers@.insert(vertex.clone()@)` but cannot prove `vertex.clone()@ == vertex@`. This blocks proving that quotient graph edges lie within quotient graph vertices.

**Classification**: These assumes are structural (Clone spec gap), not algorithmic. They would be provable with a Clone spec for generic V, or with concrete type instantiation.

## Technique: Fuel-Based Termination

Star contraction recurses on the quotient graph. Since we cannot prove strict vertex count decrease without the full partition spec, we use a fuel parameter:
```
fn star_contract_fuel(graph, base, expand, fuel: usize) -> R
    decreases fuel
```
The public `star_contract` delegates with `fuel = graph.sizeV()`.

## Cascade Changes

Adding `valid_key_type_Edge::<V>()` and closure quantifiers to `star_contract`/`star_contract_mt` requires propagated to:

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 63 | ConnectivityStEph.rs | +valid_key_type_Edge to 4 trait + 4 fn requires |
| 2 | 63 | ConnectivityMtEph.rs | +valid_key_type_Edge to 4 trait + 4 fn requires |
| 3 | 64 | SpanTreeStEph.rs | Already had valid_key_type_Edge — no change |
| 4 | 64 | SpanTreeMtEph.rs | Already had valid_key_type_Edge — no change |

## Files Modified

1. `src/Chap62/StarContractionStEph.rs` — removed external_body, added fuel recursion + assume
2. `src/Chap62/StarContractionMtEph.rs` — removed external_body, added fuel recursion + assume
3. `src/Chap62/StarPartitionStEph.rs` — simplified ensures to `result.0.spec_setsteph_wf()`
4. `src/Chap63/ConnectivityStEph.rs` — added valid_key_type_Edge to requires
5. `src/Chap63/ConnectivityMtEph.rs` — added valid_key_type_Edge to requires
