<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Derive vs Manual Impl Audit

## Who Fixed the Derives? (git log)

| Commit | Author | Files | Summary |
|--------|--------|-------|---------|
| 95a2f36 | Brian G. Milnes | MatrixChain, Dijkstra, SpanTree, Prim, UnionFind | Agent3 worktree, MatrixChain structs in verus!, PathWeightUtils, Dijkstra, Prim, UnionFind, Boruvka |
| db7f545 | Brian G. Milnes | Chap45 PQ, Chap64-66 | Chap45 PQ Clone proofs; F64Dist Ord/Hash/Add; OrderedFloat→F64Dist |
| ca8da00 | Brian G. Milnes | Chap26, Chap47-57 | Verusify Chap47-57; fix Chap26 cost annotations and parallel merge/scan/eTSP |

The **safe derive replacements** (MatrixDim, DijkstraStEphF64 PQEntry, PrimStEph PQEntry) were applied in this session (AI-assisted). The original verusification and trait wiring were done by **Brian G. Milnes** in the commits above.

## Who Checked the Derives?

The derive experiments (simple struct/enum, and struct-with-Vec) were **not** formally audited by a human. They were:

- **Created** by AI (Cursor/Composer) in this session
- **Validated** by running `verus --cfg 'feature="experiments_only"'` — all verify
- **Documented** in `derive-experiments-results.md` and in each experiment file header

No human reviewer, no PR, no formal sign-off. The experiments are exploratory: they show that Verus accepts `#[derive(Clone, PartialEq, Eq, ...)]` on structs with `Vec` inside `verus!`. Whether to apply derive in production APAS modules is a separate decision (see table below).

---

## Chap/Module: Derive to Put In vs Manual Impl to Take Out

| # | Chap | Module | Type | Current | Derive to add | Manual impl to remove | Notes |
|---|------|--------|------|---------|---------------|----------------------|-------|
| 1 | 26 | ETSPStEph | Point | manual Clone | `#[derive(Clone, Copy)]` | impl Clone for Point | Point is (f64,f64); Copy+Clone trivial |
| 2 | 26 | ETSPStEph | Edge | manual Clone | `#[derive(Clone, Copy)]` | impl Clone for Edge | Edge is (usize,usize); Copy+Clone trivial |
| 3 | 26 | ETSPMtEph | Point | manual Clone | `#[derive(Clone, Copy)]` | impl Clone for Point | same as ETSPStEph |
| 4 | 26 | ETSPMtEph | Edge | manual Clone | `#[derive(Clone, Copy)]` | impl Clone for Edge | same as ETSPStEph |
| 5 | 50 | MatrixChainStEph | MatrixDim | manual Clone,PartialEq,Eq,Debug | `#[derive(Clone,PartialEq,Eq,Debug)]` | all 4 impls | Struct is (rows,cols); all Copy |
| 6 | 50 | MatrixChainStPer | MatrixDim | manual Clone,PartialEq,Eq,Debug | `#[derive(Clone,PartialEq,Eq,Debug)]` | all 4 impls | same as MatrixChainStEph |
| 7 | 50 | MatrixChainStEph | MatrixChainStEphS | external_body Clone,PartialEq | — | — | Has HashMap; Verus can't verify. Keep external_body |
| 8 | 50 | MatrixChainStPer | MatrixChainStPerS | manual Clone,PartialEq,Eq | — | — | Has HashMap; same as above |
| 9 | 57 | DijkstraStEphI64 | PQEntry | derive Clone,Eq,PartialEq; manual Ord,PartialOrd,Debug | — | — | Ord/PartialOrd need custom cmp; Debug outside |
| 10 | 57 | DijkstraStEphF64 | PQEntry | manual all | `#[derive(Clone,PartialEq,Eq)]` | Clone,PartialEq,Eq | Ord/PartialOrd custom; Debug outside |
| 11 | 65 | PrimStEph | PQEntry | manual Clone,PartialEq,Eq,Ord,PartialOrd,Debug | `#[derive(Clone,PartialEq,Eq)]` | Clone,PartialEq,Eq | Ord/PartialOrd custom (by weight); Debug outside |
| 12 | 66 | BoruvkaStEph | LabeledEdge | manual inside verus!; Ord,PartialOrd,Hash,Debug outside | — | — | Has custom Ord (lexicographic); outside impls cfg-gated |
| 13 | 66 | BoruvkaMtEph | LabeledEdge | same as BoruvkaStEph | — | — | same |
| 14 | 37 | AVLTreeSeq | AVLTreeS | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based; needs PartialEqSpecImpl. Keep manual |
| 15 | 37 | AVLTreeSeqStEph | AVLTreeSeqStEphS | manual Clone,PartialEq,Eq,Default | — | — | View-based; needs PartialEqSpecImpl |
| 16 | 41 | ArraySetStEph | ArraySetStEph | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based; needs PartialEqSpecImpl |
| 17 | 42 | TableStPer | TableStPer | manual Clone,PartialEq,Debug | — | — | View-based; needs PartialEqSpecImpl |
| 18 | 18 | ArraySeqMtEph | ArraySeqMtEphS | manual Clone,PartialEq,Eq,Debug | — | — | View-based; needs PartialEqSpecImpl |
| 19 | 23 | PrimTreeSeqStPer | PrimTreeSeqStS, PrimTreeSeqStTree | manual Clone,PartialEq,Eq,Debug | — | — | View-based; needs PartialEqSpecImpl |
| 20 | 45 | BinaryHeapPQ | BinaryHeapPQ | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based or custom logic |
| 21 | 45 | SortedListPQ | SortedListPQ | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based or custom logic |
| 22 | 45 | UnsortedListPQ | UnsortedListPQ | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based or custom logic |
| 23 | 45 | LeftistHeapPQ | LeftistHeapNode, LeftistHeapPQ | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based or custom logic |
| 24 | 45 | BalancedTreePQ | BalancedTreePQ | manual Clone,PartialEq,Eq,Default,Debug | — | — | View-based or custom logic |
| 25 | 45 | HeapsortExample | HeapsortComparison | manual Clone,PartialEq,Eq,Debug | — | — | Custom logic |
| 26 | 47 | StructChainedHashTable | Node, ChainList | manual Clone,PartialEq,Eq,Default,Debug | `#[derive(Clone,PartialEq,Eq,Default)]` on Node? | — | Node is (Key,Value,Option); ChainList is Vec. Complex |
| 27 | 05 | MappingStEph | MappingStEph | manual Clone,PartialEq,Eq,Hash,Debug | — | — | HashMap-based; external_body or manual |
| 28 | 12 | Exercise12_1 | SpinLock | manual Default,Debug | — | — | Outside verus!; exercise file |
| 29 | 12 | Exercise12_5 | ConcurrentStackMt | manual Default | — | — | Outside verus!; exercise file |
| 30 | 37 | BSTRBMtEph | Color, Node | derive Clone, etc. | — | — | Already uses derive |
| 31 | 37 | BSTSplayMtEph | Node, BSTSplayMtEph | derive Clone; manual Default,Debug | — | — | Default,Debug outside verus! (correct) |
| 32 | vstdplus | float | WrappedF64 | manual PartialEq,Eq,Hash,PartialOrd,Ord,Debug | — | — | f64 wrapper; custom spec bridge |
| 33 | 51 | BottomUpDP* | BottomUpDP*S | manual Clone,PartialEq,Eq,Default,Debug | — | — | Some use HashMap; check per file |
| 34 | 51 | TopDownDP* | TopDownDP*S | manual Default,Debug or PartialEq | — | — | Check per file |
| 35 | 49 | MinEditDist*, SubsetSum* | *S | derive Clone,PartialEq,Eq in some; manual Debug | — | — | Mixed; Debug outside |
| 36 | 38 | BSTParaStEph | Exposed, NodeInner, ParamBST | manual Clone,Debug | — | — | Recursive; Clone may need manual |
| 37 | 38 | BSTParaMtEph | Exposed, NodeInner, ParamBST | manual Clone,Debug | — | — | Same |
| 38 | 41 | AVLTreeSetMtPer | AVLTreeSetMtPer | manual Clone,PartialEq,Eq,PartialOrd,Ord,Default,Debug | — | — | View-based; keep manual |
| 39 | 41 | ArraySetEnumMtEph | ArraySetEnumMtEph | manual Clone,PartialEq | — | — | bitvec; Verus can't link |
| 40 | 44 | Example44_1 | TweetQueryExamples | manual Default | `#[derive(Default)]`? | impl Default | If struct is simple |
| 41 | 44 | DocumentIndex | DocumentIndex | manual Clone,PartialEq | — | — | Check structure |
| 42 | 65 | UnionFindStEph | UnionFindStEph | manual Default | — | — | HashMap-based |
| 43 | 52 | EdgeSetGraphMtPer, AdjTableGraphMtPer | * | manual Default | — | — | Complex |
| 44 | 53 | PQMinStEph, PQMinStPer | PQMinResult | manual Debug | — | — | Debug outside (correct) |

---

## Summary

| Action | Count | Examples |
|--------|-------|----------|
| Derive can replace manual | 6 | ETSP Point/Edge, MatrixDim, DijkstraStEphF64 PQEntry, PrimStEph PQEntry |
| Keep manual (View/spec) | 20+ | AVLTree*, ArraySeq*, Table*, Mapping*, PQ types with custom Ord |
| Keep external_body | 2 | MatrixChainStEphS, MatrixChainStPerS (HashMap) |
| Already use derive | 15+ | BSTRB, BSTSplay, MinEditDist, SubsetSum, Chap50/52 graph types |
| Outside verus! (correct) | 5+ | Exercise12_*, Boruvka LabeledEdge Ord/Hash/Debug |

---

## Safe Derive Replacements (Low Risk)

| Chap | Module | Type | Replace | Result |
|------|--------|------|---------|--------|
| 26 | ETSPStEph, ETSPMtEph | Point, Edge | manual Clone → `#[derive(Clone, Copy)]` | **Reverted** — f64 not Copy in Verus; code relies on Copy |
| 50 | MatrixChainStEph, MatrixChainStPer | MatrixDim | manual Clone,PartialEq,Eq,Debug → `#[derive(Clone,Copy,PartialEq,Eq,Debug)]` | **Done** |
| 50 | MatrixChainMtEph, MatrixChainMtPer | MatrixDim | same as above | **Done** |
| 57 | DijkstraStEphF64 | PQEntry | manual Clone,PartialEq,Eq → `#[derive(Clone,PartialEq,Eq)]` | **Done** |
| 65 | PrimStEph | PQEntry | manual Clone,PartialEq,Eq → `#[derive(Clone,PartialEq,Eq)]` | **Done** (added Clone bound on V) |

These types have no View, no custom equality, and all fields implement the traits. The APAS pattern requires `PartialEqSpecImpl` + `ensures` for types with View; for plain structs like MatrixDim and Point/Edge, derive is equivalent and simpler.

---

## Empty Impls: Do We Still Need Them?

**With derive:** No. `#[derive(PartialEq, Eq)]` generates both the PartialEq impl and the Eq marker. No separate `impl Eq for X {}` needed.

**Without derive (ETSP Point/Edge):** Yes. For types containing f64, Verus's `#[derive(Copy)]` fails (f64 not Copy in Verus's view). The workaround: keep **manual** `impl Clone for Point` (body `{ *self }`) and **empty** `impl Copy for Point {}`. Copy requires Clone; the empty impl says "Point is Copy" and the manual Clone satisfies the Clone requirement. Same for Edge.

---

## Clone Solution for ETSP (f64 Types)

**Problem:** Point and Edge contain f64. Verus does not treat f64 as Copy for derive purposes, so `#[derive(Clone, Copy)]` fails. The code relies on Copy (e.g. `points[0]` when building Edge).

**Solution (current):** Keep manual impls:

```rust
impl Clone for Point {
    fn clone(&self) -> (cloned: Point) ensures cloned == *self { *self }
}
impl Copy for Point {}
```

The manual Clone returns `*self` (copy semantics). The empty `impl Copy for Point {}` marks Point as Copy. This works because the impl is accepted by Verus even though derive Copy fails. Alternative: add `.clone()` at every call site (many changes, no benefit).

---

## Lines of Code Reduced (derive replacements)

| Change | Files | Deletions | Insertions | Net LOC reduced |
|--------|-------|-----------|------------|-----------------|
| MatrixDim derive (StEph, StPer, MtEph, MtPer) | 4 | 128 | 4 | 124 |
| DijkstraStEphF64 PQEntry derive | 1 | 21 | 1 | 20 |
| PrimStEph PQEntry derive | 1 | 28 | 1 | 27 |
| **Total** | **6** | **170** | **7** | **163** |

Verification: 2584 verified, 0 errors. RTT: 74 MatrixChain tests passed.
