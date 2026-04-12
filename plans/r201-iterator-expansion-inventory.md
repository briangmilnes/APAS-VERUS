# R201 Iterator Expansion Inventory

## Phase 1 Audit Results

### Group 1 — St BST variants (IntoIter<T> snapshot, call root.in_order())

| # | Chap | File | Struct | Bounds | WF Predicate | Pattern | Traversal | PTT |
|---|------|------|--------|--------|--------------|---------|-----------|-----|
| 1 | 37 | BSTAVLStEph.rs | BSTAVLStEph<T> | T: TotalOrder | spec_bstavlsteph_wf() | IntoIter<T> | root.in_order() (inherent, no requires) | New |
| 2 | 37 | BSTBBAlphaStEph.rs | BSTBBAlphaStEph<T> | T: TotalOrder | spec_bstbbalphasteph_wf() | IntoIter<T> | root.in_order() | New |
| 3 | 37 | BSTPlainStEph.rs | BSTPlainStEph<T> | T: TotalOrder | spec_bstplainsteph_wf() | IntoIter<T> | root.in_order() | New |
| 4 | 37 | BSTRBStEph.rs | BSTRBStEph<T> | T: TotalOrder | spec_bstrbsteph_wf() | IntoIter<T> | root.in_order() | New |

Note: BalBinTree inherent `in_order()` has `where T: Clone + Eq` with no `requires` clause.
St iterators add `where T: Clone + Eq` to `iter()` and `IntoIterator for &Self`.

### Group 2 — Mt BST variants (snapshot+pos, call self.in_order())

| # | Chap | File | Struct | Bounds | WF Predicate | Extra Requires | PTT |
|---|------|------|--------|--------|--------------|----------------|-----|
| 5 | 37 | BSTAVLMtEph.rs | BSTAVLMtEph<T> | T: TotalOrder | spec_bstavlmteph_wf() | obeys_feq_clone::<T>() | New |
| 6 | 37 | BSTBBAlphaMtEph.rs | BSTBBAlphaMtEph<T> | T: TotalOrder | spec_bstbbalphasteph_wf() | obeys_feq_clone::<T>() | New |
| 7 | 37 | BSTPlainMtEph.rs | BSTPlainMtEph<T> | T: TotalOrder | spec_bstplainmteph_wf() | obeys_feq_clone::<T>() | New |
| 8 | 37 | BSTRBMtEph.rs | BSTRBMtEph<T> | T: StTInMtT+Ord+TotalOrder | spec_bstrbmteph_wf() | none | New |

Snapshot: call `self.in_order()` → `ArraySeqStPerS<T>` → take `.seq` field → `Vec<T>`.
All Mt iterators: `snapshot: Vec<T>, pos: usize`. Clone bridge via `assume` in next().

### Group 3 — OrdKeyMap (collect() snapshot)

| # | Chap | File | Struct | Element | Bounds | WF Predicate | PTT |
|---|------|------|--------|---------|--------|--------------|-----|
| 9 | 41 | OrdKeyMap.rs | OrdKeyMap<K,V> | Pair<K,V> | K: StT+Ord+TotalOrder, V: StT+Ord | spec_ordkeymap_wf() | New |

Snapshot: call `self.collect()` → `Vec<Pair<K,V>>`. Use IntoIter<Pair<K,V>>.
Note: `collect()` has complex axiom requires; iterator `iter()` will require them via `where`.

### Group 4 — Wrapping iterator (TableMtEph delegates to ArraySeqMtEphIter)

| # | Chap | File | Struct | Inner | Bounds | PTT |
|---|------|------|--------|-------|--------|-----|
| 10 | 42 | TableMtEph.rs | TableMtEph<K,V> | ArraySeqMtEphIter<'a, Pair<K,V>> | K: MtKey, V: MtVal | New |

TableMtEphIter wraps ArraySeqMtEphIter. View delegates to inner@. next() delegates to inner.next().

### Group 5 — Snapshot from locked/wrapped data

| # | Chap | File | Struct | Element | Traversal | Bounds | PTT |
|---|------|------|--------|---------|-----------|--------|-----|
| 11 | 43 | OrderedSetMtEph.rs | OrderedSetMtEph<T> | T | to_seq() → ArraySeqStPerS<T> → .seq | T: MtKey+TotalOrder+'static | New |
| 12 | 43 | OrderedTableMtPer.rs | OrderedTableMtPer<K,V> | Pair<K,V> | acquire read, collect() → Vec | K: MtKey+TotalOrder+'static, V: StTInMtT+Ord+'static | New |

### PTTs only (iterators already exist)

| # | Chap | File | Existing Iterator | PTT Status |
|---|------|------|-------------------|------------|
| 13 | 43 | AugOrderedTableStEph.rs | OrderedTableStEphIter<K,V> via iter() | Missing |
| 14 | 43 | AugOrderedTableStPer.rs | OrderedTableStPerIter<K,V> via iter() | Missing |
| 15 | 43 | AugOrderedTableMtEph.rs | OrderedTableMtEphIter<'a,K,V> via iter() | Missing |

### Skipped (graph files)

Graph files (AdjTableGraphStEph, AdjTableGraphStPer, EdgeSetGraphStEph, EdgeSetGraphStPer)
expose vertices() and edges() as references to typed sets (AVLTreeSetStEph/StPer) which
already have full iterator infrastructure. No additional iterator layer is needed.

## Implementation Order

1. Group 1: BSTAVLStEph, BSTBBAlphaStEph, BSTPlainStEph, BSTRBStEph (uniform pattern)
2. Group 2: BSTAVLMtEph, BSTBBAlphaMtEph, BSTPlainMtEph (uniform pattern)
3. Group 2: BSTRBMtEph (slightly different bounds)
4. Group 4: TableMtEph (wrapping, simplest Mt)
5. Group 3: OrdKeyMap (complex bounds)
6. Group 5: OrderedSetMtEph, OrderedTableMtPer
7. Phase 3: validate + RTTs
8. Phase 4: PTTs for all 12 new + 3 existing = 15 PTT files

## PTT Files to Create (Phase 4)

| # | Chap | PTT File | Iterator Tested |
|---|------|----------|-----------------|
| 1 | 37 | BSTAVLStEph_iter_ptt.rs | BSTAVLStEphIter |
| 2 | 37 | BSTBBAlphaStEph_iter_ptt.rs | BSTBBAlphaStEphIter |
| 3 | 37 | BSTPlainStEph_iter_ptt.rs | BSTPlainStEphIter |
| 4 | 37 | BSTRBStEph_iter_ptt.rs | BSTRBStEphIter |
| 5 | 37 | BSTAVLMtEph_iter_ptt.rs | BSTAVLMtEphIter |
| 6 | 37 | BSTBBAlphaMtEph_iter_ptt.rs | BSTBBAlphaMtEphIter |
| 7 | 37 | BSTPlainMtEph_iter_ptt.rs | BSTPlainMtEphIter |
| 8 | 37 | BSTRBMtEph_iter_ptt.rs | BSTRBMtEphIter |
| 9 | 41 | OrdKeyMap_iter_ptt.rs | OrdKeyMapIter |
| 10 | 42 | TableMtEph_iter_ptt.rs | TableMtEphIter |
| 11 | 43 | OrderedSetMtEph_iter_ptt.rs | OrderedSetMtEphIter |
| 12 | 43 | OrderedTableMtPer_iter_ptt.rs | OrderedTableMtPerIter |
| 13 | 43 | AugOrderedTableStEph_iter_ptt.rs | OrderedTableStEphIter (via AugOrderedTableStEph) |
| 14 | 43 | AugOrderedTableStPer_iter_ptt.rs | OrderedTableStPerIter (via AugOrderedTableStPer) |
| 15 | 43 | AugOrderedTableMtEph_iter_ptt.rs | OrderedTableMtEphIter (via AugOrderedTableMtEph) |
