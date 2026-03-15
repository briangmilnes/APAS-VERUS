# Agent 3 — Round 21 Report: Review Against Prose

Date: 2026-03-15
Commit: ae54b3b4

## Mission

Full 8-phase review-against-prose for 10 chapters (37–45, 47) covering BSTs, ordered
collections, priority queues, and hash tables. 62 source files across 10 chapters.

## Deliverables

### Review Documents Created (10)

| # | Chap | File | Lines |
|---|------|------|------:|
| 1 | 37 | src/Chap37/analyses/review-against-prose.md | 317 |
| 2 | 38 | src/Chap38/analyses/review-against-prose.md | 180 |
| 3 | 39 | src/Chap39/analyses/review-against-prose.md | 184 |
| 4 | 40 | src/Chap40/analyses/review-against-prose.md | 194 |
| 5 | 41 | src/Chap41/analyses/review-against-prose.md | 208 |
| 6 | 42 | src/Chap42/analyses/review-against-prose.md | 204 |
| 7 | 43 | src/Chap43/analyses/review-against-prose.md | 230 |
| 8 | 44 | src/Chap44/analyses/review-against-prose.md | 223 |
| 9 | 45 | src/Chap45/analyses/review-against-prose.md | 237 |
| 10 | 47 | src/Chap47/analyses/review-against-prose.md | 253 |

### Cost Annotations Added/Updated

27 source files modified with two-line `/// - APAS:` + `/// - Claude-Opus-4.6:` cost
annotations on all exec functions. Total: 964 insertion lines, 195 deletion lines
(conversions from prior one-line format).

| # | Chap | Files Modified | Annotation Count |
|---|------|---------------|:----------------:|
| 1 | 37 | 5 (StEph files) | 172 |
| 2 | 38 | 2 (StEph + MtEph) | 74 |
| 3 | 39 | 0 (already complete) | 156 |
| 4 | 40 | 0 (already complete) | 59 |
| 5 | 41 | 2 (ArraySet files) | 37 |
| 6 | 42 | 3 (all Table files) | 47 |
| 7 | 43 | 9 (all non-Example files) | 520 |
| 8 | 44 | 0 (already complete) | 76 |
| 9 | 45 | 5 (all PQ files) | 104 |
| 10 | 47 | 0 (already complete) | 136 |

## Hole Summary by Chapter

| # | Chap | Topic | Files | Holes | Breakdown | Clean Files |
|---|------|-------|------:|------:|-----------|:-----------:|
| 1 | 37 | BST variants | 19 | 15 | 14 ext_body, 1 trivial spec_wf | 7 |
| 2 | 38 | BST Parallel | 2 | 14 | 4 assume, 1 assume_spec, 9 ext_body | 0 |
| 3 | 39 | Treap | 4 | 18 | 6 assume, 12 ext_body | 1 |
| 4 | 40 | BST Augmented | 3 | 14 | All ext_body | 0 |
| 5 | 41 | Sets | 6 | 22 | 1 assume, 2 unsafe, 19 ext_body | 1 |
| 6 | 42 | Tables | 3 | 15 | All ext_body | 0 |
| 7 | 43 | Ordered Collections | 10 | 99 | 12 assume, 87 ext_body | 0 |
| 8 | 44 | Document Index | 1 | 0 | Clean (vacuously—code outside verus!) | 1 |
| 9 | 45 | Priority Queues | 5 | 4 | All ext_body | 3 |
| 10 | 47 | Hash Tables | 9 | 26 | All ext_body | 3 |
| | | **Total** | **62** | **227** | | **16** |

## Key Findings

### Architectural Patterns

1. **Array-backed ordered collections (Chap42, Chap43 OrderedTable):** Tables use
   ArraySeq<Pair<K,V>>, giving O(n) insert/delete where APAS specifies O(log n). The
   ordered operations (first/last/previous/next/rank/select) use collect-sort-scan at
   O(n log n) where APAS specifies O(log n).

2. **BST-backed sets and ordered sets (Chap41 AVLTreeSet, Chap43 OrderedSet):** These
   achieve APAS O(log n) for base ops since they use AVL trees. But ordering ops still
   degrade to O(n) via to_seq + linear scan.

3. **Persistent cost overhead (Chap45):** Priority queues backed by persistent arrays incur
   O(n) copy costs on insert/delete_min. Only LeftistHeapPQ achieves near-APAS costs because
   its tree structure naturally supports persistence.

4. **Opaque Fn closure blocker (Chap47):** All 24 hash table implementation holes share a
   single root cause: Verus cannot reason about the spec of opaque `Fn(&Key, usize) -> usize`
   closures used for hash functions. Resolving this would likely clear most Chap47 holes.

5. **Chap44 vacuously clean:** DocumentIndex.rs has 0 holes because its code is largely
   outside `verus!`. The implementation works (RTTs pass) but is not formally verified.

### Strongest Implementations

- **BSTAVLStEph.rs (Chap37):** AVL balance invariant fully proven through insert. Zero holes.
- **LeftistHeapPQ.rs (Chap45):** Fully verified meld with heap+leftist+rank invariant
  preservation, Multiset-based view type.
- **BSTPlainStEph.rs (Chap37):** Reference plain BST with all specs fully proven.

### Test Coverage

RTT coverage is excellent across all 10 chapters. All algorithm files have corresponding
test files with comprehensive coverage of ADT operations, edge cases, and persistence
semantics.

PTTs exist only for Chap41 (1 file) and Chap43 (1 file). Most chapters don't need PTTs
since they lack iterators.

## Verification

```
verification results:: 3957 verified, 0 errors
warning: 5 warnings emitted (pre-existing trigger warnings in OrderedSetStEph.rs)
```

## Techniques Used

- 5 parallel subagents, each handling 1-3 chapters
- All 8 phases of review-against-prose procedure executed per chapter
- Cost annotations in two-line `/// - APAS:` + `/// - Claude-Opus-4.6:` format
- Pre-existing annotations in Chap39/40/44/47 preserved (already in correct format)
- Prior one-line annotations in Chap41/42/45 converted to dual format
