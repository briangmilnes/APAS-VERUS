<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 4 — Round 50 Report

## Objective

Remove the 2 `assume(spec_graphview_wf(quotient@))` holes in Chap62 StarContraction by
creating a `ClonePreservesView` trait that allows generic code to reason about `clone()`
preserving the view.

## Results

| # | Metric | Before R50 | After R50 |
|---|---|:---:|:---:|
| 1 | Verified | 4450 | 4462 |
| 2 | RTT | 2613 | 2613 |
| 3 | PTT | 147 | 147 |
| 4 | Errors | 0 | 0 |

## Holes Changed

| # | Chap | File | Hole Before R50 | Hole After R50 |
|---|:---:|---|---|---|
| 1 | 62 | `StarContractionStEph.rs` | `assume(spec_graphview_wf(quotient@))` | **PROVED** |
| 2 | 62 | `StarContractionMtEph.rs` | `assume(forall u w, quotient@.A.contains((u,w))...)` | **PROVED** |

The 2 pre-existing `assume(spec_valid_partition_map(...))` holes (about partition
correctness from the partition loop) remain — they were not R50 targets.

## New File: src/vstdplus/clone_view.rs

Defines the `ClonePreservesView` trait:

```rust
pub trait ClonePreservesView: Clone + View + Sized {
    fn clone_view(&self) -> (result: Self)
        ensures result@ == self@;
}
```

Implementations for `usize`, `u64`, `i64`, `u32`, `i32`, `u16`, `i128`, `bool` use
`*self` (Copy), so no assume is needed. Compound types (Edge, LabEdge, WeightedEdge, Pair)
in `Types.rs` delegate to component `clone_view()` calls.

## Key Technique: if-let with Branch-Local Proof Steps

The core challenge: after `match partition_map.get(u) { Some(val) => val.clone_view(), ... }`,
Verus loses track of the specific postcondition `*val == partition_map@[u@]` from the Some
branch. The fix: restructure as `if let Some(val) = ...` so `val` stays in scope, then
add explicit proof steps inside the arm:

```rust
let u_center = if let Some(val) = partition_map.get(u) {
    let c = val.clone_view();
    proof {
        assert(*val == partition_map@[(*u)@]);       // from get ensures (Some branch)
        assert(c@ == (*val)@);                        // from clone_view ensures
        assert(c@ == partition_map@[(*u)@]@);         // combined
        assert(centers@.contains(c@));                // from spec_valid_partition_map
    }
    c
} else {
    proof { assert(false); }   // contradiction: partition_map@.contains_key(u@) proved
    u.clone_view()
};
proof { assert(centers@.contains(u_center@)); }
```

## StEph Proof (StarContractionStEph.rs)

`build_quotient_graph` had `assert(centers@.contains(u_center@))` failing because the
solver couldn't bridge from the loop invariant to the specific edge's endpoints. Fixed by:

1. Adding proof block before the if-let establishing the chain:
   - `edge_vec@.map(f)[i] == edge_vec@[i]@` → `graph.E@.contains(edge_view)` (from to_seq)
   - `graph@.A.contains((u@, v@))` → `graph@.V.contains(u@)` (from spec_graphview_wf)
   - `partition_map@.contains_key(u@)` (from spec_valid_partition_map part 1)
2. Using if-let with branch-local proof steps as described above.

## MtEph Proof (StarContractionMtEph.rs)

`build_quotient_graph_parallel` delegates to `route_edges_parallel` (parallel divide-and-conquer).
The original function had only `result.spec_setsteph_wf()` as a postcondition.

**Fix**: Added two ghost parameters to `route_edges_parallel`:
```rust
Ghost(graph_v_view): Ghost<Set<V::V>>,
Ghost(centers_view): Ghost<Set<V::V>>,
```

With new requires:
```rust
forall |j: int| start <= j < end ==>
    graph_v_view.contains((*edges).spec_index(j)@.0) && ...,
spec_valid_partition_map::<V>(graph_v_view, centers_view, (*partition_map)@),
```

And new ensures:
```rust
forall |u_v: V::V, w_v: V::V|
    #[trigger] result@.contains((u_v, w_v)) ==>
        centers_view.contains(u_v) && centers_view.contains(w_v),
```

Key insight for `spec_index(j)@.0`: `ArraySeqStEphS<Edge<V>>@` = `Seq<(V::V,V::V)>` (view-of-view),
so using `spec_index(j): Edge<V>` (exec) avoids the `@[j]@.0` vs `@[j].0` confusion.

The recursive fork-join closures capture the ghost parameters (ghost values are Copy) and
pass them to recursive calls, propagating the centers-endpoint property through both halves.
The union postcondition follows from `Set::union` semantics plus both halves' ensures.

## Files Changed

| # | Chap | File | Change |
|---|:---:|---|---|
| 1 | - | `src/vstdplus/clone_view.rs` (NEW) | ClonePreservesView trait + impls |
| 2 | - | `src/Types.rs` | Added CPV to HashOrd; impls for Edge, etc. |
| 3 | - | `src/lib.rs` | `pub mod clone_view` in vstdplus |
| 4 | 62 | `StarContractionStEph.rs` | if-let proof, removed spec_graphview_wf assume |
| 5 | 62 | `StarContractionMtEph.rs` | Ghost params on route_edges_parallel, removed assume |

## Remaining Holes in Chap62

| # | Chap | File | Hole | Notes |
|---|:---:|---|---|---|
| 1 | 62 | `StarContractionStEph.rs` | `assume(spec_valid_partition_map)` in `star_contract_fuel` | Pre-R49 |
| 2 | 62 | `StarContractionMtEph.rs` | `assume(spec_valid_partition_map)` in `star_contract_mt_fuel` | Pre-R49 |

These concern proving that the `sequential_star_partition` function produces a valid
partition map — a separate proof obligation about the partition loop structure.
