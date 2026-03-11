# Agent2 Work Plan: spec_wf and multi_struct Updates

Date: 2026-03-11
Branch: agent2/ready
Chapters: 38, 39, 40, 43, 44, 45, 47

## Current State Summary

| # | Chap | File | Has wf? | wf Name | Multi-Struct Issue | Work Needed |
|---|------|------|---------|---------|--------------------|-------------|
| 1 | 38 | BSTParaStEph | No | — | None | Add wf |
| 2 | 38 | BSTParaMtEph | No | — | None | Add wf |
| 3 | 39 | BSTTreapStEph | Yes | spec_bsttreapsteph_wf | free spec_contains_link | Review only |
| 4 | 39 | BSTTreapMtEph | Yes (free fn) | spec_bsttreapmteph_wf | free spec_contains_link | Move wf into trait |
| 5 | 39 | BSTParaTreapMtEph | No | — | None (clean) | Add wf |
| 6 | 39 | BSTSetTreapMtEph | No | — | None | Add wf |
| 7 | 40 | BSTSizeStEph | Yes | spec_bstsizesteph_wf | 4 free spec fns on Link | Partial cleanup |
| 8 | 40 | BSTReducedStEph | Yes | spec_bstreducedsteph_wf | 4 free spec fns on Link | Partial cleanup |
| 9 | 40 | BSTKeyValueStEph | Yes | spec_bstkeyvaluesteph_wf | 5 free spec fns on Link | Partial cleanup |
| 10 | 43 | AugOrderedTableStEph | Yes | spec_augorderedtablesteph_wf | None | No work |
| 11 | 43 | AugOrderedTableStPer | Yes | spec_augorderedtablestper_wf | None | No work |
| 12 | 43 | AugOrderedTableMtEph | No | — | None | Add wf |
| 13 | 43 | OrderedSetStEph | No | — | None | Add wf (*) |
| 14 | 43 | OrderedSetStPer | No | — | None | Add wf (*) |
| 15 | 43 | OrderedSetMtEph | No | — | None | Add wf |
| 16 | 43 | OrderedTableStEph | Yes | spec_orderedtablesteph_wf | None | No work |
| 17 | 43 | OrderedTableStPer | Yes | spec_orderedtablestper_wf | None | No work |
| 18 | 43 | OrderedTableMtEph | No | — | None | Add wf |
| 19 | 43 | OrderedTableMtPer | No | — | None | Add wf |
| 20 | 44 | DocumentIndex | No | — | None | Add wf |
| 21 | 45 | BinaryHeapPQ | No | — | None | Add wf |
| 22 | 45 | LeftistHeapPQ | No | — | None (has multi-struct) | Add wf |
| 23 | 45 | SortedListPQ | No | — | None | Add wf |
| 24 | 45 | UnsortedListPQ | No | — | None | Add wf |
| 25 | 45 | BalancedTreePQ | Yes | spec_balancedtreepq_wf | None | No work |
| 26 | 47 | ParaHashTableStEph | No | — | None | Add wf |
| 27 | 47 | LinProbFlatHashTableStEph | No | — | None | Add wf |
| 28 | 47 | QuadProbFlatHashTableStEph | No | — | None | Add wf |
| 29 | 47 | DoubleHashFlatHashTableStEph | No | — | bare impl block | Add wf |
| 30 | 47 | LinkedListChainedHashTableStEph | No | — | None | Add wf |
| 31 | 47 | VecChainedHashTableStEph | No | — | None | Add wf |
| 32 | 47 | StructChainedHashTable | No | — | free spec_chain_to_map | Add wf + cleanup |

(*) Not in original plan but discovered missing during exploration.

## Work Groups

### Group A: spec_wf additions (19 files)

Mechanical work: add `spec_<module>_wf` abstract in trait, open in impl, thread into
requires/ensures per the standard.

| Batch | Files | wf Name |
|-------|-------|---------|
| A1 Chap38 | BSTParaStEph, BSTParaMtEph | spec_bstparasteph_wf, spec_bstparamteph_wf |
| A2 Chap39 | BSTParaTreapMtEph, BSTSetTreapMtEph | spec_bstparatreapmteph_wf, spec_bstsettreapmteph_wf |
| A3 Chap43 | AugOrderedTableMtEph | spec_augorderedtablemteph_wf |
| A4 Chap43 | OrderedSetStEph, OrderedSetStPer, OrderedSetMtEph | spec_orderedsetsteph_wf, spec_orderedsetstper_wf, spec_orderedsetmteph_wf |
| A5 Chap43 | OrderedTableMtEph, OrderedTableMtPer | spec_orderedtablemteph_wf, spec_orderedtablemtper_wf |
| A6 Chap44 | DocumentIndex | spec_documentindex_wf |
| A7 Chap45 | BinaryHeapPQ, LeftistHeapPQ, SortedListPQ, UnsortedListPQ | spec_binaryheappq_wf, spec_leftistheappq_wf, spec_sortedlistpq_wf, spec_unsortedlistpq_wf |
| A8 Chap47 | ParaHashTableStEph, LinProb..., QuadProb..., DoubleHash..., LinkedList..., Vec... | spec_parahashtablesteph_wf, etc. |

### Group B: wf trait migration (1 file)

BSTTreapMtEph has `spec_bsttreapmteph_wf` as a free function. Move it into
`BSTTreapMtEphTrait` as abstract in trait, open in impl.

### Group C: multi_struct partial cleanup (4 files)

Chap40 files have free spec fns operating on `Link = Option<Box<Node<T>>>`:
- `spec_size_link`, `spec_height_link`, `spec_content_link`, `spec_link_size_wf`

**Question for user**: These free fns operate on `Option<Box<Node<T>>>`, which can't have
trait methods. Options:
1. Move them into NodeTrait as associated spec fns (e.g., `spec_size_of_link(link)`)
2. Leave as free fns — acceptable for Option-wrapped types
3. Create a `LinkTrait` on a newtype wrapper

I recommend option 1 (move into NodeTrait as associated functions) or option 2 (leave as-is
since they can't be trait methods on Option). Need user guidance.

StructChainedHashTable.rs has `spec_chain_to_map` as a free spec fn that could move into
`ChainListTrait`.

### Group D: No work needed (6 files)

Already compliant: AugOrderedTableStEph, AugOrderedTableStPer, OrderedTableStEph,
OrderedTableStPer, BalancedTreePQ. BSTTreapStEph has wf already.

## Execution Order

1. **Chap43** first — has existing wf patterns to mirror (St files have wf, add to Mt files).
2. **Chap38** — 2 files, standalone.
3. **Chap39** — wf additions + BSTTreapMtEph trait migration.
4. **Chap45** — 4 PQ files, independent of each other.
5. **Chap44** — 1 file (DocumentIndex).
6. **Chap47** — 6 hash table files, validate as batch.
7. **Chap40** — multi_struct cleanup (pending user decision on approach).

Validate after each batch. Fix verification errors before moving to next batch.

## Naming Convention Summary

Module name → remove underscores → lowercase → prepend `spec_` → append `_wf`.

| Module | wf Name |
|--------|---------|
| BSTParaStEph | spec_bstparasteph_wf |
| BSTParaMtEph | spec_bstparamteph_wf |
| BSTParaTreapMtEph | spec_bstparatreapmteph_wf |
| BSTSetTreapMtEph | spec_bstsettreapmteph_wf |
| AugOrderedTableMtEph | spec_augorderedtablemteph_wf |
| OrderedSetStEph | spec_orderedsetsteph_wf |
| OrderedSetStPer | spec_orderedsetstper_wf |
| OrderedSetMtEph | spec_orderedsetmteph_wf |
| OrderedTableMtEph | spec_orderedtablemteph_wf |
| OrderedTableMtPer | spec_orderedtablemtper_wf |
| DocumentIndex | spec_documentindex_wf |
| BinaryHeapPQ | spec_binaryheappq_wf |
| LeftistHeapPQ | spec_leftistheappq_wf |
| SortedListPQ | spec_sortedlistpq_wf |
| UnsortedListPQ | spec_unsortedlistpq_wf |
| ParaHashTableStEph | spec_parahashtablesteph_wf |
| LinProbFlatHashTableStEph | spec_linprobflathashtablesteph_wf |
| QuadProbFlatHashTableStEph | spec_quadprobflathashtablesteph_wf |
| DoubleHashFlatHashTableStEph | spec_doublehashflathashtablesteph_wf |
| LinkedListChainedHashTableStEph | spec_linkedlistchainedhashtablesteph_wf |
| VecChainedHashTableStEph | spec_vecchainedhashtablesteph_wf |
| StructChainedHashTable | spec_structchainedhashtable_wf |

## Pattern for Each wf Addition

Per `src/standards/spec_wf_standard.rs`:

**In trait** (abstract):
```rust
spec fn spec_<module>_wf(&self) -> bool;
```

**In impl** (open):
```rust
open spec fn spec_<module>_wf(&self) -> bool {
    // Real invariant — never just `true`.
}
```

**Threading**: Every `&self` method gets `self.spec_<module>_wf()` in requires.
Every `&mut self` method gets `old(self).spec_<module>_wf()` in requires,
`self.spec_<module>_wf()` in ensures. Every `Self` return gets `out.spec_<module>_wf()`
in ensures.

## Risks

- Chap47 hash tables use a parametric trait hierarchy (ParaHashTable → FlatHashTable/
  ChainedHashTable → concrete variants). wf on the parametric base trait may propagate
  to all variants. Need to check if base trait already has a wf-like predicate.
- Chap43 Mt files mirror St files. Ensure wf body matches the St counterpart's semantics.
- LeftistHeapPQ already has `spec_is_valid_leftist_heap()` — may need to incorporate this
  into the new `spec_leftistheappq_wf` rather than replace it.
