# Iterator migration study: pre-05.21 iterators to the 0.2026.09.13 prophetic model

Round r209, agent 2. Date: 2026-09-21. Plan: `plans/r209-agent2-iterator-study.md`.
Review only; no file under `src/`, `tests/`, `rust_verify_test/`, `scripts/` or
`plans/` was edited, and no Verus run was started.

State read: `main` at `4bd6b4f92`, working tree dirty (143 modified or untracked
paths under `src/`, `tests/`, `rust_verify_test/`). r209 agent 1 was editing
concurrently; the on-disk versions read for its write set were, by `git diff
--stat` at the time of reading: `src/Chap05/{KleeneStPer,MappingStEph,
RelationStEph,SetMtEph,SetStEph}.rs` (8, 322, 253, 541, 512 lines changed),
`src/Chap17/MathSeq.rs` (223), `src/Chap49/*` (8 files, 12 to 41 lines each),
`src/Chap50/*` (8 files, 33 to 68), `src/Chap51/*` (5 files), `src/Chap65/PrimStEph.rs`
(3), `src/Chap66/{BoruvkaMtEph,BoruvkaStEph}.rs` (349, 318),
`src/standards/using_hashmap_standard.rs` (334), `src/vstdplus/hash_map_with_view_plus.rs`
and `hash_set_with_view_plus.rs` (2 each), `src/vstdplus/seq_set.rs` renamed to
`seq_set_pre_0913.rs` with a new untracked `seq_set.rs`, and a new untracked
`src/vstdplus/hash_specs_plus.rs`. Chap61 to Chap64 had no uncommitted diff when
read. `src/Chap06/*` and the r207 standards edits are also uncommitted on disk;
they were read as-is.

Counts below are `grep -c` line counts over `src/`, `tests/`, and
`rust_verify_test/tests/`, excluding `src/experiments/`; the exact patterns are
in section 2. Classifications in section 3 come from reading each file's
iterator struct fields and the bodies of `iter()`, `into_iter()` and `next()`
(scratchpad extracts of all 59 files), not from names.

## 1. Summary

Iterator definitions, by class (section 3):

| # | Class | Structs | Files |
|---|-------|--------:|------:|
| 1 | delegated, `slice::Iter` wrap | 13 | 13 |
| 2 | delegated, flatten to `vec::IntoIter` | 24 | 22 |
| 3 | delegated, `Vec` snapshot + index | 13 | 13 |
| 4 | chained (wraps another APAS `*Iter`) | 5 | 5 |
| 5 | chained, no struct (Chap43 `Aug*`) | 0 | 3 |
| 6 | custom (lazy tree traversal) | 4 | 4 |
| 7 | live total, all pre-05.21 | 59 | 56 (+3) |
| 8 | dead (`vstdplus`, out of `lib.rs`) | 2 | 2 |
| 9 | already migrated (Chap05, 06, 17) | 13 | 13 |

Files and sites:

| # | Measurement | Value |
|---|-------------|------:|
| 1 | consumer-only files still old [b] | 16 |
| 2 | consumer files needing no edit (Chap61) | 3 |
| 3 | consumer files done by agent 1 [c] | 15 |
| 4 | old-model loops in `src/` [d] | 33 |
| 5 | PTT files on the old model (336 tests) | 84 |
| 6 | of which PTTs of Chap05, 06, 17 | 30 |
| 7 | PTT loops (373 `for`, 6 manual) | 379 |
| 8 | RTT files naming an `XxxIter` type | 0 |

[b] Chap57, 58, 59, 62, 63, 64, 65. [c] Chap66 (2 files) and the 13 Chap06
`WeightedDirGraphStEph*` files. [d] 17 manual loops, 16 `for` loops.

Old-model sites in `src/` by edit shape (section 4; 77 files):

| # | Shape | Sites |
|---|-------|------:|
| 1 | item deletions [e] | 667 |
| 2 | signature type substitutions | 179 |
| 3 | body unwraps `XxxIter { inner: e }` | 115 |
| 4 | constructor `ensures` to the triple | 123 |
| 5 | `iter_invariant(&it)` clause deletions | 112 |
| 6 | import lines | 20 |
| 7 | mechanical subtotal | 1,216 |
| 8 | `requires` on `into_iter` (decision) | 47 |
| 9 | consumer loops (33), iterator tokens | 123 |
| 10 | custom `IteratorSpecImpl` and `next` | 4 + 4 |
| 11 | trait `iter()` decls, behaviour 2 | 4 + 4 |
| 12 | proof or decision subtotal | 92 |

[e] Iterator structs, ghost structs, their `View` impls, the two
`ForLoopGhostIterator*` impls, `iter_invariant` definitions, delegated
`Iterator` impls, and the `Debug`/`Display` impls of the deleted types.

Old-model sites in PTTs (84 files): 290 `XxxIter` type annotations, 477
`iter.pos`/`iter.elements` invariant lines, 1,093 `it@.0`/`it@.1` lines, 156
`decreases` naming `it@`, 152 post-loop `assert(.. it@ ..)`; all mechanical
against the six templates of `src/standards/iterator_ptt_standard.rs`.

Chapters already migrated: 05, 06, 17 (definers; their 30 PTT files are not),
61 (no-op), 66 (agent 1). Migration order (section 7): 18, 19, 23 first; then
37, 38, 39, 40; then 41; then 42; then 43; consumers 57, 58, 62, 65 can start
now, 59 after 57 and 58, 63 and 64 after 62 and the hash migration. Tool
(section 8): a CST transformer for the 1,216 item-level `src/` edits and the
2,168 PTT lines pays for itself; the 92 proof and decision sites are hand work.

Two corrections to `plans/verus-0.2026.05.21-iterator-migration.md` §10:
`src/Chap37/AVLTreeSeqMtPer.rs` defines two iterators, and its borrowing one
(`AVLTreeSeqMtPerBorrowIter`, `next` = `self.tree.nth(self.pos)`, lines 858 to
889) is the same lazy `nth` traversal as `AVLTreeSeq.rs`, so the custom count is
4, not 3; and the three `src/Chap43/AugOrderedTable*.rs` files re-expose an
`OrderedTable*Iter` from constructors with the old `ensures` (6 sites) without
defining a struct, so they were not among the 71.

## 2. Inventory

Patterns counted, all `grep -cE` per file: `A` = `impl(<..>)? (std::iter::)?Iterator for`;
`B` = `ForLoopGhostIteratorNew`; `C` = `ForLoopGhostIterator[^N]`; `D` = `iter_invariant`;
`E` = `it@\.0`; `F` = `it@\.1`; `G` = `(iter|it|pit|eit|vit|rit|it_x|it_y|init|ghost_iter|...)\.pos`;
`H` = the same receivers `\.elements`; `I` = `for [a-z_]+ in [a-z_]+:`;
`J` = `match [a-z_.]+\.next\(\)`; `K` = `struct \w+GhostIterator`;
`L` = `IteratorSpecImpl`; `M` = `VerusForLoopWrapper`; `N` = `it\.(index|seq)\(\)` and
the same on `iter`, `pit`, `eit`, `vit`, `rit`. "Old" below is `B+C+D+E+F+G+H`;
"Loops" is `I+J`. The full per-pattern matrix is in the session scratchpad
(`inventory.csv`, 202 rows); the two tables below are its projection.

### 2.1 Source files (`src/`), one row per file with any counted construct

Struct names are the `pub struct *Iter*` definitions in the file (ghost structs
omitted; `[a]` = `AVLTreeSeqMtPerBorrowIter`, `AVLTreeSeqMtPerIter`). "Backing"
is the field of that struct, or what a consumer iterates, read from the source.
"Migrated" is yes when the file has zero old-model sites and either uses the
09.13 API or needs none.

| # | Chap | File | Iterator structs | Backing | Loops | Old | Migrated |
|---|------|------|------------------|---------|------:|----:|----------|
| 1 | 05 | MappingStEph.rs | - | - | 2 | 0 | yes |
| 2 | 05 | RelationStEph.rs | - | - | 2 | 0 | yes |
| 3 | 05 | SetMtEph.rs | - | - | 9 | 0 | yes |
| 4 | 05 | SetStEph.rs | - | - | 9 | 0 | yes |
| 5 | 06 | DirGraphStEph.rs | - | - | 5 | 0 | yes |
| 6 | 06 | LabDirGraphMtEph.rs | - | - | 3 | 0 | yes |
| 7 | 06 | LabDirGraphStEph.rs | - | - | 5 | 0 | yes |
| 8 | 06 | LabUnDirGraphMtEph.rs | - | - | 3 | 0 | yes |
| 9 | 06 | LabUnDirGraphStEph.rs | - | - | 4 | 0 | yes |
| 10 | 06 | UnDirGraphStEph.rs | - | - | 2 | 0 | yes |
| 11 | 06 | WeightedDirGraphStEphF64.rs | - | - | 4 | 0 | yes |
| 12 | 06 | WeightedDirGraphStEphI128.rs | - | - | 7 | 0 | yes |
| 13 | 06 | WeightedDirGraphStEphI16.rs | - | - | 7 | 0 | yes |
| 14 | 06 | WeightedDirGraphStEphI32.rs | - | - | 7 | 0 | yes |
| 15 | 06 | WeightedDirGraphStEphI64.rs | - | - | 7 | 0 | yes |
| 16 | 06 | WeightedDirGraphStEphI8.rs | - | - | 7 | 0 | yes |
| 17 | 06 | WeightedDirGraphStEphIsize.rs | - | - | 7 | 0 | yes |
| 18 | 06 | WeightedDirGraphStEphU128.rs | - | - | 7 | 0 | yes |
| 19 | 06 | WeightedDirGraphStEphU16.rs | - | - | 7 | 0 | yes |
| 20 | 06 | WeightedDirGraphStEphU32.rs | - | - | 7 | 0 | yes |
| 21 | 06 | WeightedDirGraphStEphU64.rs | - | - | 7 | 0 | yes |
| 22 | 06 | WeightedDirGraphStEphU8.rs | - | - | 7 | 0 | yes |
| 23 | 06 | WeightedDirGraphStEphUsize.rs | - | - | 7 | 0 | yes |
| 24 | 18 | ArraySeqMtEph.rs | ArraySeqMtEphIter | slice::Iter over seq | 0 | 16 | no |
| 25 | 18 | ArraySeqMtEphSlice.rs | ArraySeqMtEphSliceIter | slice::Iter over Arc slice | 0 | 14 | no |
| 26 | 18 | ArraySeqMtPer.rs | ArraySeqMtPerIter | slice::Iter over seq | 0 | 16 | no |
| 27 | 18 | ArraySeq.rs | ArraySeqIter | slice::Iter over seq | 0 | 16 | no |
| 28 | 18 | ArraySeqStEph.rs | ArraySeqStEphIter | slice::Iter over seq | 0 | 16 | no |
| 29 | 18 | ArraySeqStPer.rs | ArraySeqStPerIter | slice::Iter over seq | 0 | 16 | no |
| 30 | 18 | LinkedListStEph.rs | LinkedListStEphIter | slice::Iter over seq | 0 | 16 | no |
| 31 | 18 | LinkedListStPer.rs | LinkedListStPerIter | slice::Iter over seq | 0 | 16 | no |
| 32 | 19 | ArraySeqMtEph.rs | ArraySeqMtEphIter | slice::Iter over seq | 0 | 16 | no |
| 33 | 19 | ArraySeqMtEphSlice.rs | ArraySeqMtEphSliceIter | slice::Iter over Arc slice | 0 | 14 | no |
| 34 | 19 | ArraySeqStEph.rs | ArraySeqStEphIter | slice::Iter over seq | 0 | 16 | no |
| 35 | 19 | ArraySeqStPer.rs | ArraySeqStPerIter | slice::Iter over seq | 0 | 16 | no |
| 36 | 23 | BalBinTreeStEph.rs | InOrderIter, PostOrderIter, PreOrderIter | vec::IntoIter of in/pre/post order | 0 | 33 | no |
| 37 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStIter | slice::Iter over seq | 0 | 16 | no |
| 38 | 37 | AVLTreeSeqMtPer.rs | MtPerBorrowIter, MtPerIter [a] | &tree+pos (nth); Vec<T>+index | 0 | 13 | no |
| 39 | 37 | AVLTreeSeq.rs | AVLTreeSeqIter | &tree + pos + len; next = nth | 0 | 13 | no |
| 40 | 37 | AVLTreeSeqStEph.rs | AVLTreeSeqIterStEph | Vec<&Node> stack + ghost pos | 0 | 15 | no |
| 41 | 37 | AVLTreeSeqStPer.rs | AVLTreeSeqStPerIter | Vec<&Node> stack + current | 0 | 15 | no |
| 42 | 37 | BSTAVLMtEph.rs | BSTAVLMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 43 | 37 | BSTAVLStEph.rs | BSTAVLStEphIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 44 | 37 | BSTBBAlphaMtEph.rs | BSTBBAlphaMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 45 | 37 | BSTBBAlphaStEph.rs | BSTBBAlphaStEphIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 46 | 37 | BSTPlainMtEph.rs | BSTPlainMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 47 | 37 | BSTPlainStEph.rs | BSTPlainStEphIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 48 | 37 | BSTRBMtEph.rs | BSTRBMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 49 | 37 | BSTRBStEph.rs | BSTRBStEphIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 50 | 37 | BSTSetAVLMtEph.rs | BSTSetAVLMtEphIter | Vec<T> snapshot + pos | 0 | 13 | no |
| 51 | 37 | BSTSetBBAlphaMtEph.rs | BSTSetBBAlphaMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 52 | 37 | BSTSetPlainMtEph.rs | BSTSetPlainMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 53 | 37 | BSTSetRBMtEph.rs | BSTSetRBMtEphIter | Vec<T> snapshot + pos | 0 | 13 | no |
| 54 | 37 | BSTSetSplayMtEph.rs | BSTSetSplayMtEphIter | Vec<T> snapshot + pos | 0 | 13 | no |
| 55 | 37 | BSTSplayStEph.rs | BSTSplayStEphIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 56 | 38 | BSTParaStEph.rs | ParamBSTIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 57 | 39 | BSTParaTreapMtEph.rs | ParamTreapIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 58 | 39 | BSTSetTreapMtEph.rs | BSTSetTreapMtEphIter | vec::IntoIter of iter_in_order() | 0 | 10 | no |
| 59 | 39 | BSTTreapMtEph.rs | BSTTreapMtEphIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 60 | 39 | BSTTreapStEph.rs | BSTTreapStEphIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 61 | 40 | BSTKeyValueStEph.rs | BSTKeyValueStEphIter | vec::IntoIter of keys() | 0 | 10 | no |
| 62 | 40 | BSTReducedStEph.rs | BSTReducedStEphIter | vec::IntoIter of keys() | 0 | 10 | no |
| 63 | 40 | BSTSizeStEph.rs | BSTSizeStEphIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 64 | 41 | ArraySetStEph.rs | ArraySetStEphIter | ArraySeqStEphIter (chained) | 0 | 13 | no |
| 65 | 41 | AVLTreeSetMtEph.rs | AVLTreeSetMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 66 | 41 | AVLTreeSetMtPer.rs | AVLTreeSetMtPerIter | vec::IntoIter of in_order() | 0 | 10 | no |
| 67 | 41 | AVLTreeSetStEph.rs | AVLTreeSetStEphIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 68 | 41 | AVLTreeSetStPer.rs | AVLTreeSetStPerIter | vec::IntoIter of in_order() | 0 | 13 | no |
| 69 | 41 | OrdKeyMap.rs | OrdKeyMapIter | vec::IntoIter of collect() | 0 | 11 | no |
| 70 | 42 | TableMtEph.rs | TableMtEphIter | ArraySeqMtEphIter (chained) | 0 | 11 | no |
| 71 | 42 | TableStEph.rs | TableStEphIter | ArraySeqStEphIter (chained) | 0 | 13 | no |
| 72 | 42 | TableStPer.rs | TableStPerIter | ArraySeqStPerIter (chained) | 0 | 13 | no |
| 73 | 43 | AugOrderedTableMtEph.rs | - | re-exposes OrderedTableMtEphIter | 0 | 4 | no |
| 74 | 43 | AugOrderedTableStEph.rs | - | re-exposes OrderedTableStEphIter | 0 | 6 | no |
| 75 | 43 | AugOrderedTableStPer.rs | - | re-exposes OrderedTableStPerIter | 0 | 6 | no |
| 76 | 43 | OrderedSetMtEph.rs | OrderedSetMtEphIter | Vec<T> snapshot + pos | 0 | 11 | no |
| 77 | 43 | OrderedSetStEph.rs | OrderedSetStEphIter | vec::IntoIter of collect_in_order | 0 | 13 | no |
| 78 | 43 | OrderedSetStPer.rs | OrderedSetStPerIter | vec::IntoIter of collect_in_order | 0 | 10 | no |
| 79 | 43 | OrderedTableMtEph.rs | OrderedTableMtEphIter | Vec<Pair> snapshot + pos | 0 | 11 | no |
| 80 | 43 | OrderedTableMtPer.rs | OrderedTableMtPerIter | OrderedTableStPerIter (chained) | 0 | 11 | no |
| 81 | 43 | OrderedTableStEph.rs | OrderedTableStEphIter | vec::IntoIter of in_order() | 0 | 14 | no |
| 82 | 43 | OrderedTableStPer.rs | OrderedTableStPerIter | vec::IntoIter of in_order() | 0 | 14 | no |
| 83 | 57 | DijkstraStEphF64.rs | - | consumes SetStEph | 1 | 10 | no |
| 84 | 57 | DijkstraStEphU64.rs | - | consumes SetStEph | 1 | 12 | no |
| 85 | 58 | BellmanFordStEphF64.rs | - | consumes SetStEph | 2 | 8 | no |
| 86 | 58 | BellmanFordStEphI64.rs | - | consumes SetStEph | 2 | 8 | no |
| 87 | 59 | JohnsonMtEphF64.rs | - | consumes SetStEph | 2 | 6 | no |
| 88 | 59 | JohnsonMtEphI64.rs | - | consumes SetStEph | 2 | 6 | no |
| 89 | 59 | JohnsonStEphF64.rs | - | consumes SetStEph | 2 | 10 | no |
| 90 | 59 | JohnsonStEphI64.rs | - | consumes SetStEph | 2 | 11 | no |
| 91 | 61 | EdgeContractionStEph.rs | - | - | 3 | 0 | yes |
| 92 | 61 | VertexMatchingMtEph.rs | - | - | 1 | 0 | yes |
| 93 | 61 | VertexMatchingStEph.rs | - | - | 1 | 0 | yes |
| 94 | 62 | StarPartitionMtEph.rs | - | consumes SetStEph | 0 | 8 | no |
| 95 | 63 | ConnectivityMtEph.rs | - | consumes SetStEph + HashMapWVP | 3 | 2 | no |
| 96 | 63 | ConnectivityStEph.rs | - | consumes SetStEph + HashMapWVP | 3 | 4 | no |
| 97 | 64 | SpanTreeMtEph.rs | - | consumes SetStEph + HashMapWVP | 4 | 3 | no |
| 98 | 64 | SpanTreeStEph.rs | - | consumes SetStEph + HashMapWVP | 4 | 3 | no |
| 99 | 64 | TSPApproxStEph.rs | - | consumes SetStEph | 2 | 8 | no |
| 100 | 65 | KruskalStEph.rs | - | consumes SetStEph | 1 | 5 | no |
| 101 | 65 | PrimStEph.rs | - | consumes SetStEph | 2 | 20 | no |
| 102 | 66 | BoruvkaMtEph.rs | - | - | 10 | 0 | yes |
| 103 | 66 | BoruvkaStEph.rs | - | - | 6 | 0 | yes |
| 104 | standards | iterator_ptt_standard.rs | - | - | 5 | 0 | yes |
| 105 | standards | iterators_standard.rs | - | - | 3 | 0 | yes |
| 106 | standards | prophetic_iterators_standard.rs | - | - | 1 | 3 | yes |
| 107 | standards | table_of_contents_standard.rs | - | - | 0 | 0 | yes |
| 108 | standards | using_hashmap_standard.rs | - | - | 1 | 0 | yes |
| 109 | standards | wrapping_iterators_standard.rs | - | - | 0 | 2 | yes |
| 110 | vstdplus | hash_map_with_view_plus.rs | HashMapWithViewPlusIter | hash_map::Iter (dead; lib.rs) | 0 | 7 | no |
| 111 | vstdplus | hash_set_with_view_plus.rs | HashSetWithViewPlusIter | hash_set::Iter (dead; lib.rs) | 0 | 8 | no |

Notes on the source table: rows 1 to 24 (Chap05, Chap06) carry only 09.13
sites (`VerusForLoopWrapper`, `it.seq()`, `it.index()`; `M+N` = 525 lines) and
are the template for the rest. Chap61's three rows have `for x in iter: it`
loops whose invariants name only `spec_setsteph_wf` and `valid_key_type`, so
they compile under both models. The two `vstdplus` rows are dead code. The
Chap43 `AugOrderedTable*` rows have no struct; their `Old` count is the
constructor `ensures` they restate.

### 2.2 Proof-time test files (`rust_verify_test/tests/`), one row per file

Every non-standards PTT is on the old model. "Loops" is `for` plus manual
loops; every loop body names `it@.0`, `it@.1`, `iter.pos` or `iter.elements`.

| # | Chap | File | Loops | Old | Migrated |
|---|------|------|------:|----:|----------|
| 1 | 05 | MappingStEph.rs | 2 | 12 | no |
| 2 | 05 | ProveMappingStEph.rs | 6 | 22 | no |
| 3 | 05 | ProveRelationStEph.rs | 6 | 22 | no |
| 4 | 05 | ProveSetMtEph.rs | 6 | 22 | no |
| 5 | 05 | ProveSetStEph.rs | 6 | 22 | no |
| 6 | 05 | RelationStEph.rs | 2 | 12 | no |
| 7 | 05 | SetMtEph.rs | 2 | 13 | no |
| 8 | 05 | SetStEph.rs | 2 | 13 | no |
| 9 | 06 | ProveDirGraphMtEph.rs | 6 | 22 | no |
| 10 | 06 | ProveDirGraphStEph.rs | 6 | 22 | no |
| 11 | 06 | ProveLabDirGraphMtEph.rs | 4 | 22 | no |
| 12 | 06 | ProveLabDirGraphStEph.rs | 4 | 22 | no |
| 13 | 06 | ProveLabUnDirGraphMtEph.rs | 4 | 22 | no |
| 14 | 06 | ProveLabUnDirGraphStEph.rs | 4 | 22 | no |
| 15 | 06 | ProveUnDirGraphMtEph.rs | 6 | 22 | no |
| 16 | 06 | ProveUnDirGraphStEph.rs | 6 | 22 | no |
| 17 | 06 | ProveWeightedDirGraphStEphI128.rs | 4 | 22 | no |
| 18 | 06 | ProveWeightedDirGraphStEphI16.rs | 4 | 22 | no |
| 19 | 06 | ProveWeightedDirGraphStEphI32.rs | 4 | 22 | no |
| 20 | 06 | ProveWeightedDirGraphStEphI64.rs | 4 | 22 | no |
| 21 | 06 | ProveWeightedDirGraphStEphI8.rs | 4 | 22 | no |
| 22 | 06 | ProveWeightedDirGraphStEphIsize.rs | 4 | 22 | no |
| 23 | 06 | ProveWeightedDirGraphStEphU128.rs | 4 | 22 | no |
| 24 | 06 | ProveWeightedDirGraphStEphU16.rs | 4 | 22 | no |
| 25 | 06 | ProveWeightedDirGraphStEphU32.rs | 4 | 22 | no |
| 26 | 06 | ProveWeightedDirGraphStEphU64.rs | 4 | 22 | no |
| 27 | 06 | ProveWeightedDirGraphStEphU8.rs | 4 | 22 | no |
| 28 | 06 | ProveWeightedDirGraphStEphUsize.rs | 4 | 22 | no |
| 29 | 17 | prove_MathSeq_iters.rs | 1 | 0 | no |
| 30 | 17 | ProveMathSeq.rs | 9 | 32 | no |
| 31 | 18 | ProveArraySeqMtEph.rs | 9 | 32 | no |
| 32 | 18 | ProveArraySeqMtEphSlice.rs | 4 | 22 | no |
| 33 | 18 | ProveArraySeqMtPer.rs | 9 | 32 | no |
| 34 | 18 | ProveArraySeq.rs | 3 | 13 | no |
| 35 | 18 | ProveArraySeqStEph.rs | 8 | 32 | no |
| 36 | 18 | ProveArraySeqStPer.rs | 3 | 11 | no |
| 37 | 18 | ProveLinkedListStEph.rs | 3 | 13 | no |
| 38 | 18 | ProveLinkedListStPer.rs | 3 | 13 | no |
| 39 | 19 | ProveArraySeqMtEph.rs | 6 | 32 | no |
| 40 | 19 | ProveArraySeqMtEphSlice.rs | 3 | 19 | no |
| 41 | 19 | ProveArraySeqStEph.rs | 9 | 32 | no |
| 42 | 19 | ProveArraySeqStPer.rs | 6 | 32 | no |
| 43 | 23 | ProveBalBinTreeStEph.rs | 9 | 33 | no |
| 44 | 23 | ProvePrimTreeSeqStPer.rs | 8 | 32 | no |
| 45 | 37 | ProveAVLTreeSeqMtPer.rs | 4 | 22 | no |
| 46 | 37 | ProveAVLTreeSeq.rs | 6 | 22 | no |
| 47 | 37 | ProveAVLTreeSeqStEph.rs | 4 | 22 | no |
| 48 | 37 | ProveAVLTreeSeqStPer.rs | 4 | 22 | no |
| 49 | 37 | ProveBSTPlainMtEph.rs | 4 | 22 | no |
| 50 | 37 | ProveBSTRBMtEph.rs | 4 | 22 | no |
| 51 | 37 | ProveBSTSetAVLMtEph.rs | 6 | 33 | no |
| 52 | 37 | ProveBSTSetBBAlphaMtEph.rs | 5 | 23 | no |
| 53 | 37 | ProveBSTSetPlainMtEph.rs | 5 | 23 | no |
| 54 | 37 | ProveBSTSetRBMtEph.rs | 6 | 33 | no |
| 55 | 37 | ProveBSTSetSplayMtEph.rs | 6 | 33 | no |
| 56 | 37 | ProveBSTSplayStEph.rs | 3 | 11 | no |
| 57 | 38 | ProveParamBSTStEph.rs | 6 | 22 | no |
| 58 | 39 | ProveBSTParaTreapMtEph.rs | 3 | 11 | no |
| 59 | 39 | ProveBSTSetTreapMtEph.rs | 3 | 11 | no |
| 60 | 39 | ProveBSTTreapMtEph.rs | 3 | 11 | no |
| 61 | 39 | ProveBSTTreapStEph.rs | 3 | 11 | no |
| 62 | 40 | ProveBSTKeyValueStEph.rs | 3 | 11 | no |
| 63 | 40 | ProveBSTReducedStEph.rs | 3 | 11 | no |
| 64 | 40 | ProveBSTSizeStEph.rs | 3 | 11 | no |
| 65 | 41 | ProveArraySetStEph.rs | 6 | 22 | no |
| 66 | 41 | ProveAVLTreeSetMtEph.rs | 6 | 22 | no |
| 67 | 41 | ProveAVLTreeSetMtPer.rs | 3 | 11 | no |
| 68 | 41 | ProveAVLTreeSetStEph.rs | 6 | 22 | no |
| 69 | 41 | ProveAVLTreeSetStPer.rs | 3 | 11 | no |
| 70 | 41 | ProveOrdKeyMap.rs | 4 | 22 | no |
| 71 | 42 | ProveTableMtEph.rs | 4 | 22 | no |
| 72 | 42 | ProveTableStEph.rs | 6 | 22 | no |
| 73 | 42 | ProveTableStPer.rs | 6 | 22 | no |
| 74 | 43 | ProveOrderedSetMtEph.rs | 4 | 22 | no |
| 75 | 43 | ProveOrderedSetStEph.rs | 4 | 22 | no |
| 76 | 43 | ProveOrderedSetStPer.rs | 2 | 11 | no |
| 77 | 43 | ProveOrderedTableMtEph.rs | 4 | 22 | no |
| 78 | 43 | ProveOrderedTableMtPer.rs | 4 | 22 | no |
| 79 | 43 | ProveOrderedTableStEph.rs | 4 | 22 | no |
| 80 | 43 | ProveOrderedTableStPer.rs | 4 | 22 | no |
| 81 | 54 | ProveBFSMtEph.rs | 3 | 20 | no |
| 82 | 54 | ProveBFSStEph.rs | 3 | 20 | no |
| 83 | standards | Provedeep_view_standard.rs | 6 | 0 | yes |
| 84 | standards | Proveiterators_standard.rs | 9 | 0 | yes |
| 85 | standards | Provemod_standard.rs | 9 | 0 | yes |
| 86 | standards | Proveprophetic_iterators_standard.rs | 15 | 0 | yes |
| 87 | standards | Provetable_of_contents_standard.rs | 9 | 0 | yes |
| 88 | standards | Proveview_standard.rs | 6 | 0 | yes |
| 89 | standards | Provewrapping_iterators_standard.rs | 9 | 0 | yes |
| 90 | vstdplus | HashMapWithViewPlus.rs | 2 | 11 | no |
| 91 | vstdplus | HashSetWithViewPlus.rs | 4 | 12 | no |

The `Chap54` rows iterate `ArraySeqStEphS<usize>` from Chap19 (`td.iter()`,
`bu.iter()`), so they migrate with Chap19, not Chap54. The two `vstdplus` rows
test modules that are commented out of `src/lib.rs`; their `[[test]]` entries
are already commented out in `rust_verify_test/Cargo.toml`.

## 3. Classification, with cost

Read from the struct fields and the `iter()`/`into_iter()`/`next()` bodies.
"Target" is the 09.13 return type of `iter()`; a `T` stands for the file's
element type (`Pair<K, V>` for tables, `K` for the two Chap40 key iterators).
Cost is `iter()` work / `next()` work / iterator space, and it is the same
before and after because no exec body changes except the type substitution:
the migrated `iter()` returns the very `slice::Iter` or `vec::IntoIter` the
current struct wraps, or the `Vec` the current snapshot struct indexes. `†` is
amortized over a full traversal.

For the 13 snapshot iterators the old `next` cloned `snapshot[pos]`
(`AVLTreeSeqMtPer.rs:957`, `let val = self.values[self.index].clone()`);
`vec::IntoIter::next` moves the element out instead, so the migrated `next` does
one clone fewer per item and stays O(1). No `/// - Alg Analysis` line changes.

| # | Chap | File | Struct | Class | Target | iter / next / space |
|---|------|------|--------|-------|--------|---------------------|
| 1 | 18 | ArraySeq.rs | ArraySeqIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 2 | 18 | ArraySeqStEph.rs | ArraySeqStEphIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 3 | 18 | ArraySeqStPer.rs | ArraySeqStPerIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 4 | 18 | ArraySeqMtEph.rs | ArraySeqMtEphIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 5 | 18 | ArraySeqMtEphSlice.rs | ArraySeqMtEphSliceIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 6 | 18 | ArraySeqMtPer.rs | ArraySeqMtPerIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 7 | 18 | LinkedListStEph.rs | LinkedListStEphIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 8 | 18 | LinkedListStPer.rs | LinkedListStPerIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 9 | 19 | ArraySeqMtEph.rs | ArraySeqMtEphIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 10 | 19 | ArraySeqMtEphSlice.rs | ArraySeqMtEphSliceIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 11 | 19 | ArraySeqStEph.rs | ArraySeqStEphIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 12 | 19 | ArraySeqStPer.rs | ArraySeqStPerIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 13 | 23 | BalBinTreeStEph.rs | InOrderIter, PostOrderIter, PreOrderIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 14 | 23 | PrimTreeSeqStPer.rs | PrimTreeSeqStIter | delegated | slice::Iter<'_, T> | O(1) / O(1) / O(1) |
| 15 | 37 | AVLTreeSeq.rs | AVLTreeSeqIter | custom | own type + IteratorSpecImpl | O(1) / O(lg n) / O(1) |
| 16 | 37 | AVLTreeSeqStEph.rs | AVLTreeSeqIterStEph | custom | own type + IteratorSpecImpl | O(lg n) / O(1)† / O(lg n) |
| 17 | 37 | AVLTreeSeqStPer.rs | AVLTreeSeqStPerIter | custom | own type + IteratorSpecImpl | O(lg n) / O(1)† / O(lg n) |
| 18 | 37 | BSTAVLStEph.rs | BSTAVLStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 19 | 37 | BSTBBAlphaStEph.rs | BSTBBAlphaStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 20 | 37 | BSTPlainStEph.rs | BSTPlainStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 21 | 37 | BSTRBStEph.rs | BSTRBStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 22 | 37 | BSTSplayStEph.rs | BSTSplayStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 23 | 37 | BSTAVLMtEph.rs | BSTAVLMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 24 | 37 | BSTBBAlphaMtEph.rs | BSTBBAlphaMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 25 | 37 | BSTPlainMtEph.rs | BSTPlainMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 26 | 37 | BSTRBMtEph.rs | BSTRBMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 27 | 37 | BSTSetAVLMtEph.rs | BSTSetAVLMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 28 | 37 | BSTSetBBAlphaMtEph.rs | BSTSetBBAlphaMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 29 | 37 | BSTSetPlainMtEph.rs | BSTSetPlainMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 30 | 37 | BSTSetRBMtEph.rs | BSTSetRBMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 31 | 37 | BSTSetSplayMtEph.rs | BSTSetSplayMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 32 | 38 | BSTParaStEph.rs | ParamBSTIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 33 | 39 | BSTParaTreapMtEph.rs | ParamTreapIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 34 | 39 | BSTSetTreapMtEph.rs | BSTSetTreapMtEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 35 | 39 | BSTTreapMtEph.rs | BSTTreapMtEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 36 | 39 | BSTTreapStEph.rs | BSTTreapStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 37 | 40 | BSTKeyValueStEph.rs | BSTKeyValueStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 38 | 40 | BSTReducedStEph.rs | BSTReducedStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 39 | 40 | BSTSizeStEph.rs | BSTSizeStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 40 | 41 | ArraySetStEph.rs | ArraySetStEphIter | chained | slice::Iter (inner) | O(1) / O(1) / O(1) |
| 41 | 41 | AVLTreeSetMtEph.rs | AVLTreeSetMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 42 | 41 | AVLTreeSetMtPer.rs | AVLTreeSetMtPerIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 43 | 41 | AVLTreeSetStEph.rs | AVLTreeSetStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 44 | 41 | AVLTreeSetStPer.rs | AVLTreeSetStPerIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 45 | 41 | OrdKeyMap.rs | OrdKeyMapIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 46 | 42 | TableStEph.rs | TableStEphIter | chained | slice::Iter (inner) | O(1) / O(1) / O(1) |
| 47 | 42 | TableMtEph.rs | TableMtEphIter | chained | slice::Iter (inner) | O(1) / O(1) / O(1) |
| 48 | 42 | TableStPer.rs | TableStPerIter | chained | slice::Iter (inner) | O(1) / O(1) / O(1) |
| 49 | 43 | OrderedSetMtEph.rs | OrderedSetMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 50 | 43 | OrderedSetStEph.rs | OrderedSetStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 51 | 43 | OrderedSetStPer.rs | OrderedSetStPerIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 52 | 43 | OrderedTableMtEph.rs | OrderedTableMtEphIter | delegated | vec::IntoIter<T> of snapshot | O(n) / O(1) / O(n) |
| 53 | 43 | OrderedTableMtPer.rs | OrderedTableMtPerIter | chained | vec::IntoIter (inner) | O(n) / O(1) / O(n) |
| 54 | 43 | OrderedTableStEph.rs | OrderedTableStEphIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 55 | 43 | OrderedTableStPer.rs | OrderedTableStPerIter | delegated | vec::IntoIter<T> | O(n) / O(1) / O(n) |
| 56 | 43 | AugOrderedTableStEph.rs | (none) | chained, no struct | inner OrderedTable target | O(n) / O(1) / O(n) |
| 57 | 43 | AugOrderedTableStPer.rs | (none) | chained, no struct | inner OrderedTable target | O(n) / O(1) / O(n) |
| 58 | 43 | AugOrderedTableMtEph.rs | (none) | chained, no struct | inner OrderedTable target | O(n) / O(1) / O(n) |
| 59 | 37 | AVLTreeSeqMtPer.rs | AVLTreeSeqMtPerBorrowIter | custom | own type + IteratorSpecImpl | O(1) / O(lg n) / O(1) |
| 60 | 37 | AVLTreeSeqMtPer.rs | AVLTreeSeqMtPerIter | delegated | vec::IntoIter<T> of values | O(n) / O(1) / O(n) |

Evidence for the four custom rows and the one that looks custom but is not:

- `src/Chap37/AVLTreeSeq.rs:1220-1250`: `next` is `self.tree.nth(self.pos); self.pos += 1`
  on `{ tree: &AVLTreeS<T>, pos, len }`; `iter()` (line 1027) is O(1). `nth` is
  annotated O(lg n) (line 365). Flattening would make `iter()` O(n).
- `src/Chap37/AVLTreeSeqMtPer.rs:858-889`: the borrowing iterator is the same
  `nth` shape, `#[verifier::external_body]` `next`, `iter()` at line 812 is O(1).
  The consuming `AVLTreeSeqMtPerIter` (line 951) indexes a `values_in_order()`
  `Vec` and is delegated. The May plan's row 31 saw only the second.
- `src/Chap37/AVLTreeSeqStEph.rs:294-305, 441-451, 1291-1319`: `iter()` builds
  an empty `Vec<&AVLTreeNode<T>>` stack and calls `push_left_iter` down the left
  spine (O(lg n)); `next` pops, then pushes the right child's left spine.
- `src/Chap37/AVLTreeSeqStPer.rs:909-921, 927-940, 975-1010`: same shape over
  `Arc` links with a `current` field that defers the first spine push to the
  first `next`.
- `src/Chap43/OrderedTableMtEph.rs:820-837, 970-996`: `iter()` copies the tree
  into `snapshot: Vec<Pair<K, V>>` under the read lock (O(n)), and `next` is an
  `external_body` index step. Returning `snapshot.into_iter()` deletes that
  `external_body` (a hole closed by the type substitution) and keeps O(n)/O(1).

Summary by class, live structs only:

| # | Class | Structs | Files | Target type |
|---|-------|--------:|------:|-------------|
| 1 | delegated, `slice::Iter` wrap | 13 | 13 | `std::slice::Iter<'_, T>` |
| 2 | delegated, flatten to `vec::IntoIter` | 24 | 22 | `std::vec::IntoIter<T>` |
| 3 | delegated, `Vec` snapshot + index | 13 | 13 | `std::vec::IntoIter<T>` |
| 4 | chained (struct wraps an APAS `*Iter`) | 5 | 5 | the inner collection's target |
| 5 | chained, no struct (Chap43 `Aug*`) | 0 | 3 | the inner collection's target |
| 6 | custom | 4 | 4 | own type, five `IteratorSpecImpl` fns |
| 7 | total | 59 | 56 (+3) | |

## 4. Construct mapping, edit shapes, and counts

Edit shapes: `DEL` = delete a whole item; `TOK` = replace one token; `TYP` =
replace a type in a signature; `CLS` = replace a `requires`/`ensures`/
`invariant`/`decreases` clause; `WRP` = wrap a manual loop's iterator and add
the three wrapper clauses; `SPC` = a specification decision a human makes;
`PRF` = a rewrite that needs new proof text. The "May §4" column is the row of
`plans/verus-0.2026.05.21-iterator-migration.md` §4 or the r204 class
(`plans/veracity-iterator-upgrade-detect.md` §166-234) the edit corresponds to,
checked against the r207 standards; two of those templates are now wrong and
are marked.

| # | Old construct | New construct (09.13) | Shape | May §4 / r204 | Sites |
|---|---------------|-----------------------|-------|---------------|------:|
| 1 | `pub struct XxxGhostIterator` | deleted | DEL | 9 / D1 | 60 |
| 2 | `impl View for XxxGhostIterator` | deleted | DEL | 9 / D2 | 60 |
| 3 | `impl ForLoopGhostIteratorNew` block | deleted | DEL | 9 / D3 | 60 |
| 4 | `impl ForLoopGhostIterator` block | deleted | DEL | 9 / D4 | 60 |
| 5 | `spec fn iter_invariant(it)` | deleted | DEL | 7 / D10 | 60 |
| 6 | delegated `pub struct XxxIter` | deleted | DEL | 6 / D6 | 55 |
| 7 | `impl View for XxxIter` | deleted | DEL | 8 / D7 | 55 |
| 8 | delegated `impl Iterator for XxxIter` | deleted | DEL | 6 / D8 | 55 |
| 9 | `Debug`/`Display` of deleted types | deleted | DEL | D5, D9 | 186 |
| 10 | `use vstd::pervasive::ForLoop...` | `use vstd::std_specs::iter::*;` | TOK | – | 20 |
| 11 | `XxxIter` in `iter`/`IntoIter` types | std iterator type [n1] | TYP | – | 192 |
| 12 | body `XxxIter { inner: e }` | `e` | TOK | 6 | 115 |
| 13 | ctor `ensures it@.0 == 0, it@.1 ..` | the constructor triple [n2] | CLS | 10 / T1-T3, T8 [n3] | 123 |
| 14 | `iter_invariant(&it)` clause | deleted | CLS | 7 / T4 | 112 |
| 15 | `requires` on `into_iter` impl | conditional `ensures` [n4] | SPC | – | 47 |
| 16 | loop invariant `it@.0`, `it@.1`, ... | `it.index()`, `it.seq()` [n5] | TOK / CLS | 1-4, 10 | 123 |
| 17 | manual `loop { match it.next() }` | `VerusForLoopWrapper` [n6] | WRP | – | 17 |
| 18 | `decreases s.len() - it@.0` | `decrease(&it.iter)->0` [n7] | CLS | T6 [n3] | 17 |
| 19 | `ghost pos = it@.0 - 1` after `next` | `old_pos` before `match` [n8] | PRF | U-OTHER | 3 |
| 20 | `it@` facts drawn after `break` | drawn before `break` [n9] | CLS / PRF | U-POST | 4 |
| 21 | custom `XxxIter` + ghost pair | `IteratorSpecImpl` port [n10] | PRF | U-CUSTOM | 4 |
| 22 | custom `external_body next` | `next` checked or trusted [n11] | PRF / SPC | – | 4 (+1) |
| 23 | trait `iter()` naming `IteratorSpec` | `elts()`/`spec_pos()` + lemma [n12] | SPC / PRF | – | 4 + 4 |
| 24 | PTT `let mut it: XxxIter<T> = ..` | wrapper type or none [n13] | TYP | – | 290 |
| 25 | PTT `iter.pos`, `iter.elements`, `it@` | `iter.index()`, `iter.seq()` | TOK / CLS | – | 477 + 1,093 |
| 26 | PTT `decreases ... it@` | `decrease(&it.iter)->0` | CLS | – | 156 |
| 27 | PTT post-loop `assert(.. it@ ..)` | `assert(collected@ =~= orig)` [n14] | CLS | – | 152 |

Notes to the table:

- [n1] `-> (it: XxxIter<'_, T>)` and `type IntoIter = XxxIter<..>` become
  `std::slice::Iter<'_, T>` (borrowing over a slice) or
  `std::vec::IntoIter<T>` (a flattened or snapshot `Vec`).
- [n2] `IteratorSpec::remaining(&it) == X.as_ref()` for a borrowing iterator
  or `== X` for a consuming one; `vstd::std_specs::slice::into_iter_elts(it)
  == X` or the `vec` form; `IteratorSpec::decrease(&it) is Some`.
- [n3] The r204 T1, T5, T6 and T8 templates emit the 05.21 forms
  (`initial_value_relation`, `.unwrap()`); see the paragraph below.
- [n4] The `requires` is removed (Verus 09.13 rejects it on a trait impl) and
  the `ensures` becomes `self.spec_x_wf() ==> { triple }`, as
  `docs/Chap02to06Validation.md` §5.6 row 1 did; a spec decision, since
  callers of a non-wf collection then get no facts instead of an error.
- [n5] Borrowed items need `it.seq().unref()` against a `Seq<T>` and
  `*it.seq()[i]` for an element; owned items are one-token.
- [n6] `let mut it = VerusForLoopWrapper::new(c.iter());` plus the three
  invariant clauses `it.wf()`,
  `IteratorSpec::obeys_prophetic_iter_laws(&it.iter)`,
  `IteratorSpec::decrease(&it.iter) is Some`.
- [n7] `decreases IteratorSpec::decrease(&it.iter)->0`; `it.seq()` is
  prophetic and may not appear in `decreases`.
- [n8] `let ghost old_pos = it.index();` before the `match`, and
  `assert(s[old_pos] == *x)` in the `Some` arm.
- [n9] The prophetic equality does not survive `break`; the conclusion is
  asserted in the `None` arm.
- [n10] Private fields, `#[verifier::type_invariant]`, closed `elts()`, the
  five `IteratorSpecImpl` spec fns, a `when_used_as_spec` constructor
  (section 5).
- [n11] `next` loses its `ensures`; its body is checked against the spec impl,
  or keeps `#[verifier::external_body]` with the trust it has today. The
  `+1` is `OrderedTableMtEph.rs`, whose `external_body` `next` is deleted by
  the type substitution.
- [n12] Behaviour 2 (`docs/StandardsUpgrade.md` §2.3): the trait `ensures`
  name `it.elts()` and `it.spec_pos()`, and an inherent `proof fn` gives
  `IteratorSpec::remaining` (section 5.3). Four trait declarations and four
  `IntoIterator` impls.
- [n13] A manual-loop test annotates
  `VerusForLoopWrapper<std::slice::Iter<'_, T>>` (or the `vec` type); a `for`
  test has no annotation.
- [n14] `assert(collected@ =~= orig)`, from `it.index() == it.seq().len()`
  after the loop.

Row 13 and row 18 note that the r204 rewriter's templates emit the 05.21 forms
(`IteratorSpec::initial_value_relation(&it, &it)`, which PR #2739 removed, and
`decrease(&it).unwrap()` rather than `->0`; `docs/StandardsUpgrade.md` §3.2 also
records the two-argument `VerusForLoopWrapper::new` of 05.21). Any reuse of
that tool needs its T1, T5, T6 and T8 templates re-derived from
`src/standards/iterators_standard.rs`.

Per-file counts for the 77 non-migrated `src/` files, one row per file. `DEL`
is the number of items deleted (rows 1 to 9); `TYP` signature sites (row 11);
`CLS` constructor triples (row 13); `T4` `iter_invariant(&it)` clauses (row
14); `SPC` `into_iter` `requires` (row 15); `Loops` manual + `for` loops (rows
16 to 20); `TOK` iterator tokens in those loops. A definer file has all its
tokens inside items that rows 1 to 14 delete or replace, so its `TOK` is the
token count inside those items, not extra work; for a consumer file `TOK` is the
loop-invariant work.

| # | Chap | File | DEL | TYP | CLS | T4 | SPC | Loops | TOK |
|---|------|------|----:|----:|----:|---:|----:|------:|----:|
| 1 | 18 | ArraySeqMtEph.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 2 | 18 | ArraySeqMtEphSlice.rs | 12 | 3 | 2 | 2 | 1 | 0 | 7 |
| 3 | 18 | ArraySeqMtPer.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 4 | 18 | ArraySeq.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 5 | 18 | ArraySeqStEph.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 6 | 18 | ArraySeqStPer.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 7 | 18 | LinkedListStEph.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 8 | 18 | LinkedListStPer.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 9 | 19 | ArraySeqMtEph.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 10 | 19 | ArraySeqMtEphSlice.rs | 12 | 3 | 2 | 2 | 1 | 0 | 7 |
| 11 | 19 | ArraySeqStEph.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 12 | 19 | ArraySeqStPer.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 13 | 23 | BalBinTreeStEph.rs | 36 | 3 | 3 | 3 | 0 | 0 | 15 |
| 14 | 23 | PrimTreeSeqStPer.rs | 12 | 3 | 3 | 2 | 0 | 0 | 9 |
| 15 | 37 | AVLTreeSeqMtPer.rs | 15 | 4 | 2 | 2 | 1 | 0 | 7 |
| 16 | 37 | AVLTreeSeq.rs | 12 | 3 | 2 | 2 | 1 | 0 | 7 |
| 17 | 37 | AVLTreeSeqStEph.rs | 12 | 4 | 2 | 2 | 1 | 0 | 9 |
| 18 | 37 | AVLTreeSeqStPer.rs | 12 | 4 | 2 | 2 | 1 | 0 | 9 |
| 19 | 37 | BSTAVLMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 20 | 37 | BSTAVLStEph.rs | 8 | 2 | 2 | 2 | 1 | 0 | 7 |
| 21 | 37 | BSTBBAlphaMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 22 | 37 | BSTBBAlphaStEph.rs | 8 | 2 | 2 | 2 | 1 | 0 | 7 |
| 23 | 37 | BSTPlainMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 24 | 37 | BSTPlainStEph.rs | 8 | 2 | 2 | 2 | 1 | 0 | 7 |
| 25 | 37 | BSTRBMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 26 | 37 | BSTRBStEph.rs | 8 | 2 | 2 | 2 | 1 | 0 | 7 |
| 27 | 37 | BSTSetAVLMtEph.rs | 12 | 8 | 3 | 3 | 2 | 0 | 6 |
| 28 | 37 | BSTSetBBAlphaMtEph.rs | 12 | 6 | 2 | 2 | 2 | 0 | 5 |
| 29 | 37 | BSTSetPlainMtEph.rs | 12 | 6 | 2 | 2 | 2 | 0 | 5 |
| 30 | 37 | BSTSetRBMtEph.rs | 12 | 8 | 3 | 3 | 2 | 0 | 6 |
| 31 | 37 | BSTSetSplayMtEph.rs | 12 | 8 | 3 | 3 | 2 | 0 | 6 |
| 32 | 37 | BSTSplayStEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 33 | 38 | BSTParaStEph.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 34 | 39 | BSTParaTreapMtEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 35 | 39 | BSTSetTreapMtEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 36 | 39 | BSTTreapMtEph.rs | 12 | 1 | 1 | 1 | 0 | 0 | 5 |
| 37 | 39 | BSTTreapStEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 38 | 40 | BSTKeyValueStEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 39 | 40 | BSTReducedStEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 40 | 40 | BSTSizeStEph.rs | 12 | 1 | 1 | 1 | 1 | 0 | 5 |
| 41 | 41 | ArraySetStEph.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 42 | 41 | AVLTreeSetMtEph.rs | 12 | 6 | 2 | 2 | 1 | 0 | 5 |
| 43 | 41 | AVLTreeSetMtPer.rs | 12 | 1 | 1 | 1 | 0 | 0 | 5 |
| 44 | 41 | AVLTreeSetStEph.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 45 | 41 | AVLTreeSetStPer.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 46 | 41 | OrdKeyMap.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 47 | 42 | TableMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 48 | 42 | TableStEph.rs | 12 | 2 | 2 | 2 | 0 | 0 | 7 |
| 49 | 42 | TableStPer.rs | 12 | 2 | 2 | 2 | 0 | 0 | 7 |
| 50 | 43 | AugOrderedTableMtEph.rs | 0 | 4 | 2 | 2 | 1 | 0 | 2 |
| 51 | 43 | AugOrderedTableStEph.rs | 0 | 2 | 2 | 2 | 1 | 0 | 4 |
| 52 | 43 | AugOrderedTableStPer.rs | 0 | 2 | 2 | 2 | 1 | 0 | 4 |
| 53 | 43 | OrderedSetMtEph.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 54 | 43 | OrderedSetStEph.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 55 | 43 | OrderedSetStPer.rs | 12 | 1 | 1 | 1 | 0 | 0 | 5 |
| 56 | 43 | OrderedTableMtEph.rs | 12 | 4 | 2 | 2 | 1 | 0 | 5 |
| 57 | 43 | OrderedTableMtPer.rs | 8 | 4 | 2 | 2 | 1 | 0 | 5 |
| 58 | 43 | OrderedTableStEph.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 59 | 43 | OrderedTableStPer.rs | 12 | 2 | 2 | 2 | 1 | 0 | 7 |
| 60 | 57 | DijkstraStEphF64.rs | 0 | 0 | 0 | 0 | 0 | 1 | 8 |
| 61 | 57 | DijkstraStEphU64.rs | 0 | 0 | 0 | 0 | 0 | 1 | 10 |
| 62 | 58 | BellmanFordStEphF64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 4 |
| 63 | 58 | BellmanFordStEphI64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 4 |
| 64 | 59 | JohnsonMtEphF64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 6 |
| 65 | 59 | JohnsonMtEphI64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 6 |
| 66 | 59 | JohnsonStEphF64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 7 |
| 67 | 59 | JohnsonStEphI64.rs | 0 | 0 | 0 | 0 | 0 | 2 | 8 |
| 68 | 62 | StarPartitionMtEph.rs | 0 | 0 | 0 | 0 | 0 | 0 | 7 |
| 69 | 63 | ConnectivityMtEph.rs | 0 | 0 | 0 | 0 | 0 | 3 | 2 |
| 70 | 63 | ConnectivityStEph.rs | 0 | 0 | 0 | 0 | 0 | 3 | 4 |
| 71 | 64 | SpanTreeMtEph.rs | 0 | 0 | 0 | 0 | 0 | 4 | 3 |
| 72 | 64 | SpanTreeStEph.rs | 0 | 0 | 0 | 0 | 0 | 4 | 3 |
| 73 | 64 | TSPApproxStEph.rs | 0 | 0 | 0 | 0 | 0 | 2 | 4 |
| 74 | 65 | KruskalStEph.rs | 0 | 0 | 0 | 0 | 0 | 1 | 4 |
| 75 | 65 | PrimStEph.rs | 0 | 0 | 0 | 0 | 0 | 2 | 19 |
| 76 | vstdplus | hash_map_with_view_plus.rs | 8 | 0 | 0 | 0 | 0 | 0 | 2 |
| 77 | vstdplus | hash_set_with_view_plus.rs | 8 | 0 | 0 | 0 | 0 | 0 | 3 |

Totals over the 77 rows: DEL 667, TYP 179, CLS 123, T4 112, SPC 47, Loops 33,
TOK 493 (of which 123 are in consumer loops; the rest sit inside deleted or
replaced items).

## 5. The custom iterators

Four structs, two shapes. Both shapes follow `struct CountIter` in
`src/standards/prophetic_iterators_standard.rs` and `VecIterator` in
`~/projects/verus/examples/guide/iterators.rs`: private fields, a
`#[verifier::type_invariant]`, a closed `elts()` for the creation-time
contents, a plain `next` whose contract is vstd's `Iterator::next`
specification checked against the five `IteratorSpecImpl` spec fns, and a
constructor with the triple. Today every one of the four `next` bodies is
`#[verifier::external_body]` with the old two-arm `ensures`
(`AVLTreeSeq.rs:1224`, `AVLTreeSeqMtPer.rs:865`, `AVLTreeSeqStEph.rs:1295`,
`AVLTreeSeqStPer.rs:979`), so the traversal is trusted now; the port can
keep the attribute (trust unchanged, and vstd's `next` contract is then
assumed against the spec impl) or discharge it, which is the proof work
described under each shape.

### 5.1 Shape 1: index over `nth` (`AVLTreeSeq.rs`, `AVLTreeSeqMtPer.rs` borrow)

Sketch for `src/Chap37/AVLTreeSeq.rs`; `AVLTreeSeqMtPerBorrowIter` is the same
text with `AVLTreeSeqMtPerS`, `spec_inorder` and `spec_avltreeseqmtper_wf`.

```rust
// 4b. type definitions — struct AVLTreeSeqIter. Private fields: a type_invariant
// struct may not expose crate-public fields.
#[verifier::reject_recursive_types(T)]
pub struct AVLTreeSeqIter<'a, T: StT> {
    tree: &'a AVLTreeS<T>,
    pos: usize,
    len: usize,
}

// 6b. spec fns — struct AVLTreeSeqIter
impl<'a, T: StT> AVLTreeSeqIter<'a, T> {
    #[verifier::type_invariant]
    pub closed spec fn spec_avltreeseqiter_wf(self) -> bool {
        self.tree.spec_avltreeseq_wf()
        && self.len == spec_avltreeseq_inorder_values(self.tree.root).len()
        && self.pos <= self.len
    }

    pub closed spec fn spec_new(tree: &'a AVLTreeS<T>) -> Self {
        AVLTreeSeqIter { tree, pos: 0, len: spec_avltreeseq_inorder_values(tree.root).len() as usize }
    }

    /// The creation-time contents peek reads: the in-order values (today's it@.1).
    pub closed spec fn elts(self) -> Seq<T> { spec_avltreeseq_inorder_values(self.tree.root) }

    pub closed spec fn spec_pos(self) -> int { self.pos as int }

    /// Behaviour 2 bridge: the trait's ensures name elts() and spec_pos();
    /// a caller that wants the prophetic sequence calls this.
    pub proof fn lemma_remaining(&self)
        ensures
            IteratorSpec::remaining(self) == self.elts().subrange(self.spec_pos(), self.elts().len() as int).as_ref(),
            IteratorSpec::decrease(self) is Some,
    {
    }
}

pub open spec fn avltreeseq_iter_spec<'a, T: StT>(tree: &'a AVLTreeS<T>) -> AVLTreeSeqIter<'a, T> {
    AVLTreeSeqIter::spec_new(tree)
}

// 8b. traits — the constructor stays a trait method (trait-impl pattern), so
// its ensures name the non-prophetic fields, not IteratorSpec::remaining(&it)
// (docs/StandardsUpgrade.md §2.3, behaviour 2, rows 10 to 12).
fn iter<'a>(&'a self) -> (it: AVLTreeSeqIter<'a, T>)
    requires self.spec_avltreeseq_wf(),
    ensures
        it == avltreeseq_iter_spec(self),
        it.elts().map_values(|t: T| t@) =~= self.spec_avltreeseq_seq(),   // today's it@.1 clause
        it.spec_pos() == 0;

// 9b. impls
#[verifier::when_used_as_spec(avltreeseq_iter_spec)]
fn iter<'a>(&'a self) -> (it: AVLTreeSeqIter<'a, T>) {
    proof { lemma_inorder_values_maps_to_inorder::<T>(self.root); }
    AVLTreeSeqIter { tree: self, pos: 0, len: self.length() }
}

// 10b. iterators — struct AVLTreeSeqIter
impl<'a, T: StT> Iterator for AVLTreeSeqIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> (ret: Option<&'a T>) {
        proof { use_type_invariant(&*self); }
        if self.pos < self.len {
            let elem = self.tree.nth(self.pos);
            self.pos = self.pos + 1;
            Some(elem)
        } else {
            None
        }
    }
}

impl<'a, T: StT> IteratorSpecImpl for AVLTreeSeqIter<'a, T> {
    open spec fn obeys_prophetic_iter_laws(&self) -> bool { true }

    closed spec fn remaining(&self) -> Seq<&'a T> {
        self.elts().subrange(self.pos as int, self.len as int).as_ref()
    }

    closed spec fn will_return_none(&self) -> bool { true }

    closed spec fn decrease(&self) -> Option<nat> { Some((self.len - self.pos) as nat) }

    open spec fn peek(&self, index: int) -> Option<&'a T> {
        if 0 <= index < self.elts().len() { Some(&self.elts()[index]) } else { None }
    }
}

impl<'a, T: StT> std::iter::IntoIterator for &'a AVLTreeS<T> {
    type Item = &'a T;
    type IntoIter = AVLTreeSeqIter<'a, T>;
    fn into_iter(self) -> (it: Self::IntoIter)
        ensures self.spec_avltreeseq_wf() ==> {
            &&& it == avltreeseq_iter_spec(self)
            &&& it.elts().map_values(|t: T| t@) =~= self.spec_avltreeseq_seq()
            &&& it.spec_pos() == 0
        },
    { self.iter() }
}
```

What the `next` check needs that the file lacks: vstd's contract requires
`ret == Some(IteratorSpec::remaining(old(self))[0])` at value level, i.e.
`*elem == spec_avltreeseq_inorder_values(root)[pos]`, but `nth`'s ensures
(`AVLTreeSeq.rs:368`) is view-level, `elem@ == self.spec_avltreeseq_seq()[index]`.
Either `nth` gains the value-level clause (provable: `nth` descends to the
node whose value is at in-order position `index`, and
`lemma_inorder_values_maps_to_inorder` already relates the two sequences), or
`next` keeps `#[verifier::external_body]`. The `IntoIterator` `requires` goes
(row 15 of section 4); the wf premise moves into a conditional `ensures` as
Chap05 did. The `requires self.spec_avltreeseq_wf()` on `iter()` itself stays
because it is a same-crate trait method.

### 5.2 Shape 2: explicit stack (`AVLTreeSeqStEph.rs`, `AVLTreeSeqStPer.rs`)

Sketch for `src/Chap37/AVLTreeSeqStEph.rs`. `AVLTreeSeqStPer.rs` differs in
the `current: Option<&'a Node<T>>` field, whose first-`next` spine push is
folded into the invariant below as `spec_inorder_refs(current) +
spec_stack_remaining(stack)`, and in `Arc` links (`&**arc`).

```rust
// 6c. spec fns
/// In-order value references of a subtree (the Item type is &'a T, so the
/// prophetic sequence is a sequence of references, not of views).
pub open spec fn spec_inorder_refs<'a, T: StT>(link: &'a Link<T>) -> Seq<&'a T>
    decreases *link,
{
    match link {
        None => Seq::empty(),
        Some(node) => spec_inorder_refs(&node.left) + seq![&node.value] + spec_inorder_refs(&node.right),
    }
}

/// What the stack still yields, top of stack first: each stacked node's value,
/// then its right subtree, then the node below it.
pub open spec fn spec_stack_remaining<'a, T: StT>(stack: Seq<&'a AVLTreeNode<T>>) -> Seq<&'a T>
    decreases stack.len(),
{
    if stack.len() == 0 {
        Seq::empty()
    } else {
        let n = stack.last();
        seq![&n.value] + spec_inorder_refs(&n.right) + spec_stack_remaining(stack.drop_last())
    }
}

// 4c. type definitions — struct AVLTreeSeqIterStEph
#[verifier::reject_recursive_types(T)]
pub struct AVLTreeSeqIterStEph<'a, T: StT> {
    stack: Vec<&'a AVLTreeNode<T>>,
    elts: Ghost<Seq<&'a T>>,   // today's `elements`, as references
    pos: Ghost<int>,           // today's `pos`
}

impl<'a, T: StT> AVLTreeSeqIterStEph<'a, T> {
    #[verifier::type_invariant]
    pub closed spec fn spec_avltreeseqitersteph_wf(self) -> bool {
        0 <= self.pos@ <= self.elts@.len()
        && spec_stack_remaining(self.stack@) == self.elts@.skip(self.pos@)
    }
    pub closed spec fn elts(self) -> Seq<&'a T> { self.elts@ }
    pub closed spec fn spec_pos(self) -> int { self.pos@ }
    pub proof fn lemma_remaining(&self)
        ensures
            IteratorSpec::remaining(self) == self.elts().skip(self.spec_pos()),
            IteratorSpec::decrease(self) is Some,
    {
    }
}

/// push_left_iter keeps the ensures it has (pos and elts unchanged) and gains
/// the clause that makes `next` provable.
fn push_left_iter<'a, T: StT>(it: &mut AVLTreeSeqIterStEph<'a, T>, link: &'a Link<T>)
    ensures
        it.spec_pos() == old(it).spec_pos(),
        it.elts() == old(it).elts(),
        spec_stack_remaining(it.stack@) == spec_inorder_refs(link) + spec_stack_remaining(old(it).stack@),
    decreases *link,
{ /* body unchanged */ }

// 8c. trait (behaviour 2: no IteratorSpec::remaining(&it) here)
fn iter<'a>(&'a self) -> (it: AVLTreeSeqIterStEph<'a, T>)
    requires self.spec_avltreeseqsteph_wf(),
    ensures
        it.elts() == spec_inorder_refs(&self.root),
        it.elts().map_values(|r: &T| r@) == self.spec_seq(),    // today's it@.1 == self.spec_seq()
        it.spec_pos() == 0;

// 10c. iterators
impl<'a, T: StT> Iterator for AVLTreeSeqIterStEph<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> (ret: Option<&'a T>) {
        proof { use_type_invariant(&*self); }
        let node = match self.stack.pop() { None => return None, Some(n) => n };
        push_left_iter(self, &node.right);
        proof { self.pos = Ghost(self.pos@ + 1); }
        Some(&node.value)
    }
}

impl<'a, T: StT> IteratorSpecImpl for AVLTreeSeqIterStEph<'a, T> {
    open spec fn obeys_prophetic_iter_laws(&self) -> bool { true }
    closed spec fn remaining(&self) -> Seq<&'a T> { spec_stack_remaining(self.stack@) }
    closed spec fn will_return_none(&self) -> bool { true }
    closed spec fn decrease(&self) -> Option<nat> { Some((self.elts@.len() - self.pos@) as nat) }
    open spec fn peek(&self, index: int) -> Option<&'a T> {
        if 0 <= index < self.elts().len() { Some(self.elts()[index]) } else { None }
    }
}
```

The `next` obligation is one sequence identity: before the pop,
`stack_remaining(stack) == [top.value] + inorder_refs(top.right) + stack_remaining(rest)`
by unfolding one step; after the pop and `push_left_iter(top.right)` the new
clause gives `stack_remaining(new) == inorder_refs(top.right) + stack_remaining(rest)`,
which is `remaining(old).drop_first()`. The `?` operator in today's body
(`self.stack.pop()?`, line 1315) becomes a `match`, since the ghost `pos`
update must sit after the `push_left_iter` call. `push_left_iter` is
`// veracity: no_requires` today (line 440); the new clause is its
contract and needs no `requires` beyond the link's `decreases`. Every
in-order reference the traversal yields is one that `spec_inorder_refs` of the
root contains, so `spec_seq` (views) follows from `elts().map_values`.

### 5.3 Where `iter()` may live

All four constructors are trait methods today (`AVLTreeSeq.rs:432`,
`AVLTreeSeqStEph.rs:876`, `AVLTreeSeqStPer.rs:381`, `AVLTreeSeqMtPer.rs:340`),
and `docs/StandardsUpgrade.md` §2.3 measured that a same-crate trait method
whose `ensures` names `IteratorSpec::remaining(&it)` or `decrease(&it)` on a
same-crate iterator type makes that type's `next` fail (probes
`prophetic_adaptor_pe_noinner.rs`, FAILS, and `pg_innerensures.rs`,
SUCCEEDS). The sketches therefore state the trait `ensures` over the
non-prophetic accessors `elts()` and `spec_pos()`, keep `remaining` closed,
and supply one inherent `proof fn lemma_remaining` that a caller invokes
after `iter()` to obtain the prophetic equality and `decrease is Some`. Two
consequences the migrator must accept or probe: a `for x in it: s.iter()`
loop needs `proof { it.iter.lemma_remaining() }` (or a free-function
constructor with the full triple, which row 4 of §2.3 shows is allowed) before
`it.seq()` is known; and whether behaviour 2 also fires for an
`IntoIterator::into_iter` impl (an external trait's impl returning the custom
type) was not measured by the eleven probes, all of whose `IntoIterator` impls
return `slice::Iter`. That is a twelfth probe, `prophetic_adaptor_ph_intoiter.rs`.

## 6. Consumer-only files: loop rewrites

Sixteen files in seven chapters loop over `SetStEph` (via
`out_neighbors_weighed`, `in_neighbors_weighed`, `ng`, `labeled_arcs`,
`edges`, `vertices`, `mst_edges`, `tree_edges`, `right`) and, in Chap63 and
Chap64, over `HashMapWithViewPlus` (`partition_map`). Chap05 and Chap06 are
migrated, so these files fail rustc name resolution today (`it@` on
`hash_set::Iter`); Chap63 and Chap64 also name the retired wrapper, which is
agent 1's hash migration. Every loop belongs to one of four shapes.

| # | Chap | File | Loops | Shape | Lines |
|---|------|------|------:|-------|-------|
| 1 | 57 | DijkstraStEphF64.rs | 1 | M | 247-300 |
| 2 | 57 | DijkstraStEphU64.rs | 1 | M | 233-290 |
| 3 | 58 | BellmanFordStEphF64.rs | 2 | M | 123-136, 222-230 |
| 4 | 58 | BellmanFordStEphI64.rs | 2 | M | 148-160, 247-255 |
| 5 | 59 | JohnsonMtEphF64.rs | 2 | F | 318-324, 418-428 |
| 6 | 59 | JohnsonMtEphI64.rs | 2 | F | 307-313, 406-416 |
| 7 | 59 | JohnsonStEphF64.rs | 2 | M | 186-202, 272-298 |
| 8 | 59 | JohnsonStEphI64.rs | 2 | M | 190-206, 274-300 |
| 9 | 62 | StarPartitionMtEph.rs | 1 | S | 203-243 |
| 10 | 63 | ConnectivityMtEph.rs | 3 | F, F, H | 165, 232-236, 257 |
| 11 | 63 | ConnectivityStEph.rs | 3 | F, F, H | 157-163, 234-238, 259 |
| 12 | 64 | SpanTreeMtEph.rs | 4 | H, F, M, F | 117-119, 137-142, 149-159, 203-209 |
| 13 | 64 | SpanTreeStEph.rs | 4 | H, F, M, F | 110-112, 130-135, 142-152, 197-203 |
| 14 | 64 | TSPApproxStEph.rs | 2 | M | 192-198, 234-239 |
| 15 | 65 | KruskalStEph.rs | 1 | M | 428-437 |
| 16 | 65 | PrimStEph.rs | 2 | M | 371-440, 477-486 |

Shape M, a manual loop over `hash_set::Iter` with per-element facts. The
template is `docs/Chap02to06Validation.md` §6.1 rows 3 to 9, as Chap06 did it.
`src/Chap57/DijkstraStEphU64.rs:233-284`, abridged:

```rust
// old
let mut it = neighbors.iter();
proof { assert forall |j: int| 0 <= j < it@.1.len() implies graph@.A.contains((v, it@.1[j]@.0, it@.1[j]@.1)) by { ... } }
loop
    invariant
        it@.0 <= it@.1.len(),
        it@.1.no_duplicates(),
        forall |j: int| 0 <= j < it@.1.len() ==> graph@.A.contains((v, (#[trigger] it@.1[j])@.0, it@.1[j]@.1)),
        forall |e| ... (e.0 != v || (exists |j: int| 0 <= j < it@.0 && #[trigger] it@.1[j]@ == (e.1, e.2))),
    decreases it@.1.len() - it@.0,
{
    match it.next() {
        Some(nb) => { let ghost pos = (it@.0 - 1) as int; ... }
        None => break,
    }
}

// new
let n_iter = neighbors.iter();
let ghost s = into_iter_hash_keys(n_iter);            // the non-prophetic key sequence
proof { assert forall |j: int| 0 <= j < s.len() implies graph@.A.contains((v, s[j]@.0, s[j]@.1)) by { ... } }
let mut it = VerusForLoopWrapper::new(n_iter);
loop
    invariant
        it.wf(),
        IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
        IteratorSpec::decrease(&it.iter) is Some,
        it.seq().unref() == s,
        s.no_duplicates(),
        forall |j: int| 0 <= j < s.len() ==> graph@.A.contains((v, (#[trigger] s[j])@.0, s[j]@.1)),
        forall |e| ... (e.0 != v || (exists |j: int| 0 <= j < it.index() && #[trigger] s[j]@ == (e.1, e.2))),
    decreases IteratorSpec::decrease(&it.iter)->0,
{
    let ghost old_pos = it.index();
    match it.next() {
        Some(nb) => { proof { assert(s[old_pos] == *nb); } ... }
        None => break,
    }
}
```

The facts `s.no_duplicates()` and `s.len() == neighbors@.len()` come from
`SetStEph::iter`'s new `ensures` (`docs/Chap02to06Validation.md` §5.1 row 8),
and `into_iter_hash_keys(it) == remaining(&it).unref()` ties `s` to the
prophetic sequence. The `it@.0 - 1` after `next()` becomes the ghost `old_pos`
read before it; the `assert(s[old_pos] == *nb)` is what links the returned
reference to the sequence. Any conclusion the loop draws from `it.seq()` is
asserted before `break`.

Shape F, a `for` loop whose invariant names `iter.elements` and `iter.pos`.
`src/Chap59/JohnsonMtEphI64.rs:406-416`:

```rust
// old
let it = arcs.iter();
let ghost arcs_seq = it@.1;
for labeled_edge in iter: it
    invariant
        iter.elements == arcs_seq,
        iter.pos <= arcs_seq.len(),
        reweighted_edges@.len() <= iter.pos as nat,
{ ... }

// new
let it0 = arcs.iter();
let ghost arcs_seq = into_iter_hash_keys(it0);
for labeled_edge in iter: it0
    invariant
        iter.seq().unref() == arcs_seq,
        reweighted_edges@.len() <= iter.index(),
{ ... }
```

`iter.pos <= arcs_seq.len()` is implied by the wrapper and is dropped. The
`ConnectivityStEph.rs:157-163` and `SpanTreeStEph.rs:197-203` loops are the
same text over `edges()`.

Shape S, a manual loop whose `decreases` mixes a flag with the position.
`src/Chap62/StarPartitionMtEph.rs:203-243`: `decreases (!merge_done) as int, it_seq.len() - it@.0`
becomes `decreases (!merge_done) as int, IteratorSpec::decrease(&it.iter)->0`;
the invariant `it_seq == it@.1` becomes `it.seq().unref() == it_seq` with
`it_seq` bound to `into_iter_hash_keys(...)` before the wrapper; the
post-loop comment at line 243 ("invariant gives `it@.0 >= it_seq.len()`") is a
fact the loop must assert before `break` on the `merge_done` path, because
`it` is not usable after the loop for the prophetic equality (r204 class
U-POST).

Shape H, a `for` loop over `HashMapWithViewPlus::iter` (`partition_map.iter()`
in Chap63 and Chap64, `pair in iter:`). The wrapper is gone; the field becomes
`HashMap<V, V>` and `iter()` returns `hash_map::Iter<'_, V, V>` with the
`ensures` of `src/standards/using_hashmap_standard.rs` (`remaining(&it).unref().to_set() == self@.kv_pairs()`,
`no_duplicates`, `into_iter(it) == remaining(&it).unref()`); the loop body's
`pair.0`, `pair.1` become `*pair.0`, `*pair.1`. These four loops are agent 1's
hash migration, not this study's.

The PTT rewrites are the seventh shape and are entirely templated by
`src/standards/iterator_ptt_standard.rs`: the type annotation goes (row 24 of
section 4), `iter.pos`/`iter.elements` become `it.index()`/`it.seq()`, a
manual loop gains the three wrapper clauses and the `->0` measure, and the
post-loop `assert(it@.0 == ...)` becomes `assert(collected@ =~= orig)` drawn
from `it.index() == it.seq().len()`.

## 7. Migration order

From `Cargo.toml` `[features]`, restricted to chapters that define or consume
iterators; the foundation modules and Chap02 are always present. A chapter's
own definers are independent of one another except where a struct wraps
another's iterator (chained), which is noted.

| # | Chap | Depends on (iterator-relevant) | Defines | Consumes | Blocks |
|---|------|--------------------------------|--------:|---------:|--------|
| 1 | 05 | 02 | done | done | 06, 57-66 |
| 2 | 06 | 05 | done | done | 57-65 |
| 3 | 17 | none | done | – | none |
| 4 | 18 | 02 | 8 | – | 37, 38, 39, 40, 41, 47, 49, 51, 54 |
| 5 | 19 | 02 | 4 | – | 37, 41, 42, 54 PTT, 62 |
| 6 | 23 | none | 4 (2 files) | – | 37 |
| 7 | 37 | 18, 19, 23 | 19 (18 files) | – | 41, 43, 44, 45, 52, 53, 55 |
| 8 | 38 | 18 | 1 | – | 41, 43, 52, 53 |
| 9 | 39 | 18 | 4 | – | none |
| 10 | 40 | 18 | 3 | – | none |
| 11 | 41 | 18, 19, 37, 38 | 6 (`ArraySet` chained on 19) | – | 42, 43, 44, 52, 53, 55 |
| 12 | 42 | 19, 41 | 3 (all chained on 19) | – | 43, 44 |
| 13 | 43 | 18, 19, 37, 38, 41, 42 | 7 + 3 re-exposers | – | 52 |
| 14 | 54 PTT | 19 | – | 2 PTT files | none |
| 15 | 57 | 05, 06, 45, 56 | – | 2 | 59 |
| 16 | 58 | 05, 06, 56 | – | 2 | 59 |
| 17 | 59 | 05, 06, 19, 56, 57, 58 | – | 4 | none |
| 18 | 61 | 05, 06, 19 | – | 0 (no-op) | none |
| 19 | 62 | 05, 06, 19 | – | 1 | 63, 64 |
| 20 | 63 | 05, 06, 62 | – | 2 (+hash) | none |
| 21 | 64 | 05, 06, 62 | – | 3 (+hash) | none |
| 22 | 65 | 05, 06, 45 | – | 2 | none |
| 23 | 66 | 05 | – | done (agent 1) | none |

Chap44, 45, 47, 49, 51, 52, 53, 55, 56 have no iterator site of either model
(absent from the inventory) but depend on the definers, so they re-enter
validation when their dependency migrates. Chap45 and Chap56 define no
`iter()`, so Chap57 and Chap58 wait on nothing that is not already migrated.

The order, each wave's members independent of one another:

| # | Wave | Chapters | Waits on |
|---|------|----------|----------|
| 1 | now | 18, 19, 23; 57, 58, 62, 65 [w1] | nothing |
| 2 | after 18, 19, 23 | 37, 38, 39, 40; Chap54 PTT [w2] | wave 1 |
| 3 | after 57, 58; after 62 | 59; 63, 64 [w3] | wave 1 |
| 4 | after 37, 38 | 41 [w4] | wave 2 |
| 5 | after 41 | 42 [w5] | wave 4 |
| 6 | after 41, 42 | 43 [w6] | wave 5 |
| 7 | after 43 | full validate, rtt, ptt [w7] | wave 6 |

- [w1] Also the 30 PTT files of Chap05, 06 and 17, whose modules are already
  migrated.
- [w2] The two Chap54 PTTs iterate Chap19's `ArraySeqStEphS` and follow Chap19.
- [w3] Chap63 and Chap64 also wait on agent 1's `HashMap` migration (shape H
  in section 6).
- [w4] `ArraySetStEph.rs` last, after Chap19's `ArraySeqStEph.rs`, whose
  iterator it wraps (`src/Chap41/ArraySetStEph.rs:41`).
- [w5] The three `Table*` iterators wrap Chap19's `ArraySeqStEph`,
  `ArraySeqMtEph` and `ArraySeqStPer` iterators (`src/Chap42/*.rs:29-36`).
- [w6] Order inside Chap43: `OrderedSet*`, `OrderedTableStEph`,
  `OrderedTableStPer`, `OrderedTableMtEph`; then `OrderedTableMtPer`, which
  wraps `OrderedTableStPer`'s; then the three `AugOrderedTable*`, which
  re-expose `OrderedTable*`'s.
- [w7] `scripts/validate.sh`, then `rtt.sh`, then `ptt.sh`; Chap44, 45, 52,
  53 and 55 have no iterator site but re-enter validation here.

The four custom iterators sit in wave 2 (Chap37) and do not block anything
inside Chap37: `AVLTreeSeq*` are not wrapped by any other module's iterator.
They can be scheduled as the last item of wave 2 or deferred with their
`external_body` `next` kept, without holding Chap41 to Chap43.

## 8. Tool decision

Counts: 1,216 item-level mechanical edits in `src/` (section 4 rows 1 to 14
and 10) plus about 2,170 mechanical PTT lines (rows 24 to 27, counting the 379
loop headers), against 92 sites that need a proof or a decision (rows 15 to
23). The ratio is about 37 to 1. The 55 delegated and chained definers are
byte-similar in their section 10: the extracted `next` `ensures` blocks are
identical across all 55, the constructor `ensures` differ only in the
right-hand side of `it@.1 == ...`, and the r207 standards give the exact
replacement text for each item.

Three ways to apply the mechanical part:

1. Hand editing by agents. At the r208 rate (Chap05 and Chap06, 12 definers
   and 13 consumers in one session) this is roughly one definer file per 15
   to 20 minutes and one PTT file per 15 minutes, about 38 agent-hours for 56
   definers and 84 PTTs, with the r207 measurement that hand-written 05.21
   templates were wrong in three places (`initial_value_relation`, the
   two-argument wrapper, `VerusForLoopWrapper<'a, I>`).
2. The r204 veracity rewriter (`veracity-iterator-upgrade --apply`). It
   round-tripped 1,256 findings on a fixture, but no `iterator_upgrade`
   source or binary exists under `~/projects/veracity` on this machine
   (`find -maxdepth 3`; there is no `src/bin` directory); its T1, T5, T6 and T8
   templates emit 05.21 forms (section 4 rows 13 and 18); and `CLAUDE.md`
   forbids building or modifying veracity from this project. Reuse means a
   request to the veracity side with the 09.13 templates attached.
3. A CST transformer on `~/projects/CSTs`. `datastructs/vcst-lib` is a
   lossless rowan CST for Verus, and `processes/comment-out-verus` already
   selects whole items in APAS-VERUS by kind and attribute and rewrites them
   (its README measures 993 files). `vcst-lib/src/edit.rs` is a `TODO(r004+)`
   stub and `subst.rs` is "intentionally empty", so no splice or template
   layer exists yet.

Recommendation: build the transformer, scoped to item-level operations,
because every `src/` edit in rows 1 to 14 is one of two item operations that
`comment-out-verus`'s selection already supports: delete an item (rows 1 to 9,
667 items, selected by kind and by the `*Iter`/`*GhostIterator` name suffix or
the `ForLoopGhostIterator*` trait path) or replace an item's text with a
generated one (rows 11 to 14: the `iter()`/`into_iter()` fns and the
`IntoIterator` impls, 123 items, generated from the collection's view
expression, the borrowing/consuming choice and the `slice`/`vec` target read
from the deleted struct's field type). Row 10 is a `use` item replacement.
That leaves the splice layer as the one new piece, and the PTT rewrites (rows
24 to 27) as sub-item token edits inside `invariant` and `decreases` clauses,
which a clause-level replacement (the whole `invariant` block of a test
regenerated from the six templates, keyed by the test name's pattern suffix)
also reduces to an item operation. The 92 proof and decision sites are hand
work in any option, and the four custom iterators are section 5. The
transformer's rewrite classes: D1 to D10 as in r204, T-import, T-signature
(rows 11 and 12 as one item replacement), T-triple (row 13 and 14, with the
09.13 triple and the `into_iter_elts` middle clause), T-intoiter (row 15's
conditional `ensures`, emitted for a human to accept), and P-ptt (rows 24 to
27). The decision criterion is the splice layer's cost against about 38
agent-hours of hand edits that will recur at the next vstd iterator change
(three API changes in five months: 04.20, 05.21, 09.13).

## 9. What could not be determined

1. Whether behaviour 2 fires for an `IntoIterator::into_iter` impl returning
   a same-crate custom iterator. The eleven probes return `slice::Iter` from
   `IntoIterator`; a twelfth probe is needed before the four `IntoIterator`
   impls of section 5 are written.
2. Whether `#[verifier::when_used_as_spec]` is accepted on a trait impl
   method. `prophetic_iterators_standard.rs` puts it on a free function; the
   four constructors are trait methods. If it is rejected, the spec form must
   be a free function that the trait method calls, or `for` loops over the
   custom iterators use the lemma of section 5.3.
3. Whether `nth`'s ensures in `AVLTreeSeq.rs` and `AVLTreeSeqMtPer.rs` can be
   strengthened to the value level without new lemma work; that is a
   `validate.sh isolate Chap37` question, which this review could not run.
4. The exact edit counts inside agent 1's write set (Chap61 to Chap66,
   `vstdplus`, Chap17) at the end of its round; the figures here are the
   on-disk state at read time, recorded at the top.
5. Whether Chap62's `right.iter()` (line 203) is a `SetStEph` in every
   instantiation; its declaration was not found by the type grep and was
   inferred from the surrounding `SetStEph` code. If it is an `ArraySeq`, Chap62
   moves to wave 2.
6. Whether the two dead `vstdplus` iterator files are to be deleted or kept
   as records like `seq_set_pre_0913.rs`; both are commented out of `lib.rs`
   with an r207 reason and were left out of the migration counts.
7. The cost of `vcst-lib`'s splice layer, which decides section 8; the CSTs
   repository's plans were not read beyond `README.md`, `docs/CommentOutVerus.md`
   and the two stub modules.
