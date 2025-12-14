# Verusification Status and Trait Parity (moved from docs/)

## Trait parity checklist
- Keep APAS trait surfaces and comments verbatim when verusifying.
- Add `View` impls and spec accessors on concrete types; use those in trait `requires`/`ensures` without changing the trait signatures.
- Prefer view-based contracts in the trait (as in `SetStEph` and Chap06 graphs) so callers see the specs at the trait boundary.

## Chap06 graph traits: APAS vs APAS-VERUS

| Module | Trait structure vs APAS | Comments parity | Trait requires/ensures | View bound in trait | Notes |
| --- | --- | --- | --- | --- | --- |
| DirGraphStEph | Same single trait | Mostly preserved | Yes (view-based) | Yes (`View<V = GraphView<…>>`) | Matches keep-trait + add-view pattern |
| LabDirGraphStEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| LabUnDirGraphStEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| UnDirGraphStEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| WeightedDirGraphStEph* (per-type in VERUS) | APAS has int/float variants; VERUS split into many numeric-specific traits | Partially preserved | Yes | Yes | Diverges in type granularity; otherwise follows view/spec pattern |
| WeightedUnDirGraphStEph* | APAS int/float; VERUS per-type | Partially preserved | Yes | Yes | Same divergence as above |
| DirGraphMtEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| UnDirGraphMtEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| LabDirGraphMtEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| LabUnDirGraphMtEph | Same single trait | Mostly preserved | Yes | Yes | Matches pattern |
| WeightedDirGraphMtEphInt | APAS int/float variants; VERUS int only | Partially preserved | Yes | Yes | Diverges in numeric coverage |
| WeightedUnDirGraphMtEphInt | APAS int/float; VERUS int only | Partially preserved | Yes | Yes | Diverges in numeric coverage |

## Guidance moving forward
- If closer parity is needed for weighted graphs, consolidate the per-type traits back to the APAS int/float shapes while retaining view/spec contracts.
- For chapters beyond 06, apply the same pattern: keep APAS trait signatures/comments; add `View` + spec helpers on structs; express trait contracts via those specs.

