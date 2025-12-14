# Verusification Status and Trait Parity (moved from docs/)

## Trait parity checklist
- Keep APAS trait surfaces and comments verbatim when verusifying.
- Add `View` impls and spec accessors on concrete types; use those in trait `requires`/`ensures` without changing the trait signatures.
- Prefer view-based contracts in the trait (as in `SetStEph` and Chap06 graphs) so callers see the specs at the trait boundary.

## Chap05 sets/relations/mappings: APAS vs APAS-VERUS

| Module | Trait structure vs APAS | Comments parity | Trait requires/ensures | View bound in trait | Notes |
| --- | --- | --- | --- | --- | --- |
| SetStEph / SetMtEph | Same single trait | Mostly preserved | Yes (view-based) | Yes (`View<V = Set<…>>`) | Matches keep-trait + add-view pattern |
| RelationStEph | Same | Mostly preserved | Yes | Yes | Matches pattern |
| MappingStEph | Same | Mostly preserved | Yes | Yes | Matches pattern |

## Chap06 graph traits: APAS vs APAS-VERUS (including per-type weighted)

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

## Chap17 mathseq: APAS vs APAS-VERUS
- No traits in Chap17 (mathseq) in either codebase; parity not applicable.

## Chap18 sequences/lists: APAS vs APAS-VERUS

| Module | Trait structure vs APAS | Comments parity | Trait requires/ensures | View bound in trait | Notes |
| --- | --- | --- | --- | --- | --- |
| ArraySeq.rs | APAS single trait; VERUS single `ArraySeqTrait` | Partially preserved (some APAS comments kept; analysis notes reduced) | None | No | Specs live on impls; trait lacks view bound/specs |
| ArraySeqStPer / StEph | APAS single trait; VERUS split Base/Redefinable | Reduced | None | No | Diverges in structure/comments; specs on impls only |
| ArraySeqMtPer / MtEph | APAS single trait; VERUS split Base/Redefinable | Reduced/altered complexity | None | No | Diverges; specs on impls only |
| LinkedListStPer / StEph | APAS single trait; VERUS split Base/Redefinable | Reduced | None | No | Diverges; specs on impls only |

## Chap19 sequences: APAS vs APAS-VERUS

| Module | Trait structure vs APAS | Comments parity | Trait requires/ensures | View bound in trait | Notes |
| --- | --- | --- | --- | --- | --- |
| ArraySeqStPer.rs | Single trait (matches APAS) | Mostly preserved | Yes (view-based `spec_len`/`nth_spec`) | Yes | Matches keep-trait + add-view pattern |
| ArraySeqStEph.rs | Single trait | Mostly preserved | Yes (view-based) | Yes | Matches pattern |
| ArraySeqMtEph.rs | Single trait | Mostly preserved | Yes (view-based) | Yes | Matches pattern |

## Guidance moving forward
- If closer parity is needed for weighted graphs, consolidate the per-type traits back to the APAS int/float shapes while retaining view/spec contracts.
- For chapters beyond 06, apply the same pattern: keep APAS trait signatures/comments; add `View` + spec helpers on structs; express trait contracts via those specs.
- For Chap18, consider reverting to APAS single-trait structure with comments and adding view-based spec accessors to place `requires`/`ensures` on the trait itself.

