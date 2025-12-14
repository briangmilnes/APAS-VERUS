# Verusification Status and Trait Parity (moved from docs/)

## Trait parity checklist
- Keep APAS trait surfaces and comments verbatim when verusifying.
- Add `View` impls and spec accessors on concrete types; use those in trait `requires`/`ensures` without changing the trait signatures.
- Prefer view-based contracts in the trait (as in `SetStEph` and Chap06 graphs) so callers see the specs at the trait boundary.

## Chap05 sets/relations/mappings: APAS vs APAS-VERUS

| Module | Trait structure vs APAS (Yes/No) | Comments parity | Trait With View (Yes/No) | Requires and Ensures (Yes/No) | Notes |
| --- | --- | --- | --- | --- | --- |
| SetStEph / SetMtEph | YES (same single trait) | Mostly preserved | Yes | Yes (view-based) | Matches keep-trait + add-view pattern |
| RelationStEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| MappingStEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |

## Chap06 graph traits: APAS vs APAS-VERUS (including per-type weighted)

| Module | Trait structure vs APAS (Yes/No) | Comments parity | Trait With View (Yes/No) | Requires and Ensures (Yes/No) | Notes |
| --- | --- | --- | --- | --- | --- |
| DirGraphStEph | YES (same single trait) | Mostly preserved | Yes | Yes (view-based) | Matches keep-trait + add-view pattern |
| LabDirGraphStEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| LabUnDirGraphStEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| UnDirGraphStEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| WeightedDirGraphStEph* (per-type in VERUS) | NO (split per numeric type; APAS int/float) | Partially preserved | Yes | Yes | Diverges type granularity; otherwise keep-trait + add-view pattern |
| WeightedUnDirGraphStEph* | NO | Partially preserved | Yes | Yes | Diverges type granularity; otherwise keep-trait + add-view pattern |
| DirGraphMtEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| UnDirGraphMtEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| LabDirGraphMtEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| LabUnDirGraphMtEph | YES | Mostly preserved | Yes | Yes | Matches keep-trait + add-view pattern |
| WeightedDirGraphMtEphInt | NO (only int; APAS also float) | Partially preserved | Yes | Yes | Diverges numeric coverage; otherwise keep-trait + add-view pattern |
| WeightedUnDirGraphMtEphInt | NO (only int; APAS also float) | Partially preserved | Yes | Yes | Diverges numeric coverage; otherwise keep-trait + add-view pattern |

## Chap17 mathseq: APAS vs APAS-VERUS

| Module | Trait structure vs APAS (Yes/No) | Comments parity | Trait With View (Yes/No) | Requires and Ensures (Yes/No) | Notes |
| --- | --- | --- | --- | --- | --- |
| MathSeq (no trait) | N/A | N/A | N/A | N/A | No traits in APAS or VERUS |

## Chap18 sequences/lists: APAS vs APAS-VERUS

| Module | Trait structure vs APAS (Yes/No) | Comments parity | Trait With View (Yes/No) | Requires and Ensures (Yes/No) | Notes |
| --- | --- | --- | --- | --- | --- |
| ArraySeq.rs | YES (single trait) | Partially preserved (some APAS comments kept; analysis notes reduced) | No | No | Specs live on impls; trait lacks view bound/specs |
| ArraySeqStPer / StEph | NO (split Base/Redefinable) | Reduced | No | No | Diverges; specs on impls only |
| ArraySeqMtPer / MtEph | NO | Reduced/altered complexity | No | No | Diverges; specs on impls only |
| LinkedListStPer / StEph | NO | Reduced | No | No | Diverges; specs on impls only |

## Chap19 sequences: APAS vs APAS-VERUS

| Module | Trait structure vs APAS (Yes/No) | Comments parity | Trait With View (Yes/No) | Requires and Ensures (Yes/No) | Notes |
| --- | --- | --- | --- | --- | --- |
| ArraySeqStPer.rs | YES (single trait) | Mostly preserved | Yes | Yes (view-based `spec_len`/`nth_spec`) | Matches keep-trait + add-view pattern |
| ArraySeqStEph.rs | YES | Mostly preserved | Yes | Yes (view-based) | Matches keep-trait + add-view pattern |
| ArraySeqMtEph.rs | YES | Mostly preserved | Yes | Yes (view-based) | Matches keep-trait + add-view pattern |

## Chap18 proof-hole status (veracity-review-proof-holes)
- Command: `~/projects/veracity/target/release/veracity-review-proof-holes -d src/Chap18`
- Results: 7 holed modules, each with 1 × `assume()` iterator bound check (`self.pos <= self.elements.len()`)
  - ArraySeq.rs
  - ArraySeqMtEph.rs
  - ArraySeqMtPer.rs
  - ArraySeqStEph.rs
  - ArraySeqStPer.rs
  - LinkedListStEph.rs
  - LinkedListStPer.rs


## Guidance moving forward
- If closer parity is needed for weighted graphs, consolidate the per-type traits back to the APAS int/float shapes while retaining view/spec contracts.
- For chapters beyond 06, apply the same pattern: keep APAS trait signatures/comments; add `View` + spec helpers on structs; express trait contracts via those specs.
- For Chap18, consider reverting to APAS single-trait structure with comments and adding view-based spec accessors to place `requires`/`ensures` on the trait itself.

