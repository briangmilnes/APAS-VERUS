# Agent4 Plan: spec_wf + Namespace Struct Updates for Chap61–66

Date: 2026-03-11

## Scope

17 files across 6 chapters need:
1. **Namespace struct**: Create `pub struct <ModuleName>;` where missing.
2. **Trait impl**: Implement the trait on the namespace struct (exec methods as `external_body`).
3. **spec_wf**: Add `spec_<module_no_underscores>_wf` — abstract in trait, open in impl.
4. **Thread wf**: Add requires/ensures on trait methods referencing the wf.

Additional fixes:
- **SpanTreeMtEph**: Fix trivial `RwLockPredicate` inv.
- **TSPApproxStEph**: Move trait inside `verus!`.

## Verus Constraint

Generic static spec fns in traits are a parse error. If `spec fn wf<V>(&self, graph)` fails,
fall back to free spec fn and reference in requires.

## Files

| # | Chap | File | Graph Type | Has Struct | Has Impl | Extra Work |
|---|------|------|-----------|-----------|---------|------------|
| 1 | 61 | VertexMatchingStEph | UnDirGraphStEph | No | No | |
| 2 | 61 | VertexMatchingMtEph | UnDirGraphMtEph | No | No | |
| 3 | 61 | EdgeContractionStEph | UnDirGraphStEph | No | No | |
| 4 | 61 | EdgeContractionMtEph | UnDirGraphMtEph | No | No | |
| 5 | 62 | StarPartitionStEph | UnDirGraphStEph | No | No | |
| 6 | 62 | StarPartitionMtEph | UnDirGraphMtEph | No | No | |
| 7 | 62 | StarContractionStEph | UnDirGraphStEph | No | No | |
| 8 | 62 | StarContractionMtEph | UnDirGraphMtEph | No | No | |
| 9 | 63 | ConnectivityStEph | UnDirGraphStEph | No | No | |
| 10 | 63 | ConnectivityMtEph | UnDirGraphMtEph | No | No | |
| 11 | 64 | SpanTreeStEph | UnDirGraphStEph | No | No | |
| 12 | 64 | SpanTreeMtEph | UnDirGraphMtEph | No | No | Fix trivial inv |
| 13 | 64 | TSPApproxStEph | LabUnDirGraph | No | No | Move trait into verus! |
| 14 | 65 | KruskalStEph | LabUnDirGraph | No | No | |
| 15 | 65 | PrimStEph | LabUnDirGraph | PQEntry | No | |
| 16 | 66 | BoruvkaStEph | SetStEph | Namespace | Yes | Add wf only |
| 17 | 66 | BoruvkaMtEph | SetStEph | LabeledEdge | No | |

## Execution Order

1. Chap61 (4 files) → validate
2. Chap62 (4 files) → validate
3. Chap63 (2 files) → validate
4. Chap64 (3 files) → validate
5. Chap65 (2 files: Kruskal, Prim) → validate
6. Chap66 (2 files: BoruvkaStEph, BoruvkaMtEph) → validate
7. Final: validate + rtt + ptt
