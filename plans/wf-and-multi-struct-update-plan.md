# Plan: spec_wf and multi_struct Standard Updates

Date: 2026-03-11
Standards: `src/standards/spec_wf_standard.rs`, `src/standards/multi_struct_standard.rs`,
`src/standards/spec_naming_convention.rs`

## Scope

Two related updates across the codebase:

1. **spec_wf**: Add `spec_<module_no_underscores>_wf` predicates (abstract in trait, open
   in impl) to all data-structure and algorithm modules that lack them.
2. **multi_struct**: Migrate tree/node modules from old inherent-delegation or free-spec-fn
   patterns to the per-type-trait pattern with `decreases *self` in trait impls.

## Current State

- 53 files already have new-style `spec_*_wf` predicates.
- 0 files use bare `spec_wf` (old naming eliminated).
- ~83 files need wf predicates added.
- ~12 files need multi_struct pattern fixes.
- No verification regressions from prior wf work.

## Chapters Needing spec_wf Updates

| # | Chap | Files | Count |
|---|------|-------|-------|
| 1 | 05 | SetMtEph | 1 |
| 2 | 18 | ArraySeqStEph, ArraySeqStPer, ArraySeqMtEph, ArraySeqMtPer, LinkedListStEph, LinkedListStPer | 6 |
| 3 | 19 | ArraySeqStEph, ArraySeqStPer, ArraySeqMtEph, ArraySeqMtEphSlice | 4 |
| 4 | 23 | BalBinTreeStEph, PrimTreeSeqStPer | 2 |
| 5 | 37 | BSTPlainStEph, BSTAVLStEph, BSTBBAlphaStEph, BSTRBStEph, BSTSplayStEph, BSTPlainMtEph, BSTAVLMtEph, BSTBBAlphaMtEph, BSTSetAVLMtEph, BSTSetBBAlphaMtEph, BSTSetPlainMtEph, BSTSetRBMtEph, BSTSetSplayMtEph | 13 |
| 6 | 38 | BSTParaStEph, BSTParaMtEph | 2 |
| 7 | 39 | BSTParaTreapMtEph, BSTSetTreapMtEph | 2 |
| 8 | 43 | AugOrderedTableMtEph, OrderedSetMtEph, OrderedTableMtEph, OrderedTableMtPer | 4 |
| 9 | 44 | DocumentIndex | 1 |
| 10 | 45 | BinaryHeapPQ, LeftistHeapPQ, SortedListPQ, UnsortedListPQ | 4 |
| 11 | 47 | DoubleHashFlatHashTableStEph, LinkedListChainedHashTableStEph, LinProbFlatHashTableStEph, ParaHashTableStEph, QuadProbFlatHashTableStEph, VecChainedHashTableStEph | 6 |
| 12 | 52 | AdjSeqGraphStEph, AdjSeqGraphStPer, AdjSeqGraphMtEph, AdjSeqGraphMtPer, AdjTableGraphStEph, AdjTableGraphStPer, AdjTableGraphMtPer, EdgeSetGraphStEph, EdgeSetGraphStPer, EdgeSetGraphMtPer | 10 |
| 13 | 53 | PQMinStEph, PQMinStPer | 2 |
| 14 | 56 | AllPairsResultStEphF64, AllPairsResultStPerF64, SSSPResultStEphF64, SSSPResultStPerF64 | 4 |
| 15 | 57 | StackStEph | 1 |
| 16 | 58 | BellmanFordStEphF64, BellmanFordStEphI64 | 2 |
| 17 | 59 | JohnsonMtEphF64, JohnsonMtEphI64, JohnsonStEphF64, JohnsonStEphI64 | 4 |
| 18 | 61 | EdgeContractionMtEph, EdgeContractionStEph, VertexMatchingMtEph, VertexMatchingStEph | 4 |
| 19 | 62 | StarContractionMtEph, StarContractionStEph, StarPartitionMtEph, StarPartitionStEph | 4 |
| 20 | 63 | ConnectivityMtEph, ConnectivityStEph | 2 |
| 21 | 64 | SpanTreeMtEph, SpanTreeStEph, TSPApproxStEph | 3 |
| 22 | 65 | KruskalStEph, PrimStEph | 2 |
| 23 | 66 | BoruvkaMtEph, BoruvkaStEph | 2 |

## Files Needing multi_struct Updates

Full migration (old inherent or free-spec-fn pattern):

| # | Chap | File | Issue |
|---|------|------|-------|
| 1 | 23 | BalBinTreeStEph.rs | Inherent `impl<T> BalBinTree<T>` with all spec fns; trait delegates |
| 2 | 37 | AVLTreeSeq.rs | Free spec fns for node traversals, no NodeTrait |
| 3 | 37 | AVLTreeSeqStEph.rs | Same |
| 4 | 37 | AVLTreeSeqStPer.rs | Same |
| 5 | 37 | AVLTreeSeqMtPer.rs | Same |
| 6 | 37 | BSTSplayStEph.rs | Free spec fns delegated from trait |
| 7 | 37 | BSTSplayMtEph.rs | Free `link_spec_size` |
| 8 | 37 | BSTRBMtEph.rs | Free `link_spec_size` |
| 9 | 47 | StructChainedHashTable.rs | Free `spec_chain_to_map`, no NodeTrait |

Partial cleanup (NodeTrait correct, free module-scope spec fns remain):

| # | Chap | File | Issue |
|---|------|------|-------|
| 10 | 40 | BSTSizeStEph.rs | 4 free spec fns at module scope |
| 11 | 40 | BSTReducedStEph.rs | 4 free spec fns at module scope |
| 12 | 40 | BSTKeyValueStEph.rs | 5 free spec fns at module scope |

## Agent Work Split

### agent1 — Chap05, 18, 19, 23, 37

Chapters: 05, 18, 19, 23, 37
spec_wf files: ~25
multi_struct files: 8 (BalBinTreeStEph, AVLTreeSeq x4, BSTSplay x2, BSTRBMtEph)
Total: ~33 files

Rationale: Hardest batch. Chap23 BalBinTreeStEph is the foundation struct that many Chap37
BST wrappers depend on. The multi_struct migration of BalBinTree + AVLTreeSeq is the most
architecturally sensitive work. Chap18/19 are straightforward collection wf additions that
balance the load.

Key dependencies:
- BalBinTreeStEph changes may affect BSTPlain/AVL/BBAlpha/RB StEph (which wrap it).
- AVLTreeSeq changes affect AVLTreeSeqStEph/StPer/MtPer and downstream Chap41 sets.
- Validate after BalBinTree changes before proceeding to wrappers.

### agent2 — Chap38, 39, 40, 43, 44, 45, 47

Chapters: 38, 39, 40, 43, 44, 45, 47
spec_wf files: ~21
multi_struct files: 4 (Chap40 partial cleanup, StructChainedHashTable)
Total: ~25 files

Rationale: Mid-complexity. Chap40 has the partial multi_struct cleanup (free spec fns).
Chap43 ordered table/set Mt files follow established St patterns. Chap45 PQ files and
Chap47 hash tables are mechanical wf additions.

Key dependencies:
- Chap43 Mt files should mirror their St counterparts' wf predicates.
- Chap45 PQ files are independent of each other.

### agent3 — Chap52, 53, 56, 57, 58, 59

Chapters: 52, 53, 56, 57, 58, 59
spec_wf files: ~23
multi_struct files: 0
Total: ~23 files

Rationale: Graph representation (Chap52) + shortest-path algorithm chapters. Mostly
mechanical wf additions. Chap56 F64 variants should mirror existing I64 patterns exactly.
Chap52 AdjSeq/AdjTable/EdgeSet files should mirror AdjMatrix pattern.

Key dependencies:
- Chap52 graph representations are used by Chap53-59 algorithms.
- Add wf to representations first, then algorithm files.

### agent4 — Chap61, 62, 63, 64, 65, 66

Chapters: 61, 62, 63, 64, 65, 66
spec_wf files: ~15
multi_struct files: 0
Total: ~15 files

Rationale: Late graph algorithm chapters. Smallest batch because these files are less
mature (more external_body, fewer proven specs). Mechanical wf additions. These files
are independent of each other.

Key dependencies:
- None between files. Each St/Mt pair is self-contained.

## Workflow Per Agent

1. Read the standards files before starting:
   - `src/standards/spec_wf_standard.rs`
   - `src/standards/multi_struct_standard.rs`
   - `src/standards/spec_naming_convention.rs`
2. For each file:
   a. Read the file (RMF).
   b. Add `spec_<module_no_underscores>_wf` — abstract in trait, open in impl.
   c. Thread wf into requires/ensures on all trait methods per the spec_wf_standard.
   d. If multi_struct update needed: migrate free spec fns into per-type traits.
   e. Run `scripts/validate.sh` after each file or small batch.
   f. Fix verification errors before moving to next file.
3. After all files: run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.
4. Commit to agent branch, push.

## Validation

After all agents merge:
- `scripts/validate.sh` — zero errors
- `scripts/rtt.sh` — all runtime tests pass
- `scripts/ptt.sh` — all proof time tests pass
- `scripts/all-holes-by-chap.sh` — regenerate hole analysis
- `scripts/all-style-by-chap.sh` — regenerate style analysis
- No new `assume`, `admit`, or `external_body` introduced
