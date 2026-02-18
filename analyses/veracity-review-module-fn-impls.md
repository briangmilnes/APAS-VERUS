<style>
  body { max-width: 98%; margin: auto; font-size: 16px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 4px 8px; }
</style>

# Module Function Implementations Review

## Specification Summary by Module

| Abbr | Meaning |
|------|---------|
| Tr | declared in a `trait` block |
| IT | in `impl Trait for Type` |
| IBI | in bare `impl Type` |
| ML | module-level free fn |
| V! | inside `verus!` macro |
| -V! | outside `verus!` macro |
| Unk | has requires/ensures (strength not assessed) |
| Hole | contains `assume()`, `admit()`, or `#[verifier::external_body]` |
| NoSpec | no spec |

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap36 | QuickSortMtEph | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 2 | Chap36 | QuickSortMtEphSlice | 6 | 7 | 0 | 0 | 0 | 7 | 0 | 0 | 7 |
| 3 | Chap36 | QuickSortStEph | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 4 | Chap37 | AVLTreeSeq | 20 | 23 | 2 | 10 | 0 | 34 | 0 | 0 | 34 |
| 5 | Chap37 | AVLTreeSeqMtPer | 11 | 14 | 0 | 11 | 0 | 25 | 0 | 0 | 25 |
| 6 | Chap37 | AVLTreeSeqStEph | 18 | 21 | 2 | 9 | 0 | 31 | 0 | 0 | 31 |
| 7 | Chap37 | AVLTreeSeqStPer | 13 | 15 | 1 | 11 | 0 | 27 | 0 | 0 | 27 |
| 8 | Chap37 | BSTAVLMtEph | 0 | 0 | 6 | 8 | 14 | 0 | 6 | 5 | 3 |
| 9 | Chap37 | BSTAVLStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 10 | Chap37 | BSTBBAlphaMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 5 | 3 |
| 11 | Chap37 | BSTBBAlphaStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 12 | Chap37 | BSTPlainMtEph | 0 | 0 | 6 | 5 | 11 | 0 | 3 | 2 | 6 |
| 13 | Chap37 | BSTPlainStEph | 0 | 0 | 0 | 12 | 12 | 0 | 10 | 0 | 2 |
| 14 | Chap37 | BSTRBMtEph | 14 | 16 | 0 | 19 | 0 | 35 | 0 | 0 | 35 |
| 15 | Chap37 | BSTRBStEph | 0 | 0 | 0 | 15 | 15 | 0 | 13 | 0 | 2 |
| 16 | Chap37 | BSTSetAVLMtEph | 20 | 20 | 0 | 3 | 0 | 23 | 0 | 0 | 23 |
| 17 | Chap37 | BSTSetBBAlphaMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 18 | Chap37 | BSTSetPlainMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 19 | Chap37 | BSTSetRBMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 20 | Chap37 | BSTSetSplayMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 21 | Chap37 | BSTSplayMtEph | 14 | 16 | 0 | 14 | 0 | 30 | 0 | 0 | 30 |
| 22 | Chap37 | BSTSplayStEph | 11 | 13 | 0 | 9 | 0 | 22 | 0 | 0 | 22 |
| 23 | Chap38 | BSTParaMtEph | 16 | 16 | 0 | 14 | 0 | 29 | 0 | 0 | 29 |
| 24 | Chap38 | BSTParaStEph | 12 | 12 | 0 | 8 | 0 | 19 | 0 | 0 | 19 |
| 25 | Chap39 | BSTParaTreapMtEph | 16 | 16 | 13 | 4 | 0 | 33 | 0 | 0 | 33 |
| 26 | Chap39 | BSTSetTreapMtEph | 20 | 20 | 3 | 0 | 0 | 23 | 0 | 0 | 23 |
| 27 | Chap39 | BSTTreapMtEph | 11 | 13 | 11 | 0 | 0 | 23 | 0 | 0 | 23 |
| 28 | Chap39 | BSTTreapStEph | 11 | 13 | 0 | 11 | 0 | 24 | 0 | 0 | 24 |
| 29 | Chap40 | BSTKeyValueStEph | 12 | 14 | 0 | 9 | 0 | 23 | 0 | 0 | 23 |
| 30 | Chap40 | BSTReducedStEph | 17 | 19 | 14 | 0 | 0 | 32 | 0 | 0 | 32 |
| 31 | Chap40 | BSTSizeStEph | 13 | 15 | 14 | 0 | 0 | 28 | 0 | 0 | 28 |
| 32 | Chap41 | AVLTreeSetMtEph | 12 | 15 | 0 | 0 | 0 | 15 | 0 | 0 | 15 |
| 33 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 0 | 0 | 16 | 0 | 0 | 16 |
| 34 | Chap41 | AVLTreeSetStEph | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 35 | Chap41 | AVLTreeSetStPer | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 36 | Chap41 | ArraySetEnumMtEph | 13 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 37 | Chap41 | ArraySetStEph | 12 | 13 | 0 | 0 | 0 | 13 | 0 | 0 | 13 |
| 38 | Chap41 | Example41_3 | 3 | 0 | 0 | 4 | 0 | 5 | 0 | 0 | 5 |
| 39 | Chap42 | Example42_1 | 2 | 0 | 0 | 2 | 0 | 3 | 0 | 0 | 3 |
| 40 | Chap42 | TableMtEph | 16 | 16 | 0 | 1 | 0 | 17 | 0 | 0 | 17 |
| 41 | Chap42 | TableStEph | 16 | 16 | 0 | 1 | 0 | 17 | 0 | 0 | 17 |
| 42 | Chap42 | TableStPer | 16 | 16 | 0 | 1 | 0 | 17 | 0 | 0 | 17 |
| 43 | Chap43 | AugOrderedTableMtEph | 32 | 32 | 0 | 2 | 0 | 34 | 0 | 0 | 34 |
| 44 | Chap43 | AugOrderedTableStEph | 31 | 31 | 2 | 0 | 0 | 33 | 0 | 0 | 33 |
| 45 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 0 | 0 | 30 | 0 | 0 | 30 |
| 46 | Chap43 | Example43_1 | 2 | 0 | 0 | 2 | 0 | 3 | 0 | 0 | 3 |
| 47 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 0 | 0 | 22 | 0 | 0 | 22 |
| 48 | Chap43 | OrderedSetStEph | 22 | 22 | 0 | 1 | 0 | 23 | 0 | 0 | 23 |
| 49 | Chap43 | OrderedSetStPer | 22 | 22 | 0 | 1 | 0 | 23 | 0 | 0 | 23 |
| 50 | Chap43 | OrderedTableMtEph | 29 | 29 | 0 | 1 | 0 | 30 | 0 | 0 | 30 |
| 51 | Chap43 | OrderedTableMtPer | 9 | 10 | 0 | 0 | 0 | 10 | 0 | 0 | 10 |
| 52 | Chap43 | OrderedTableStEph | 29 | 29 | 0 | 1 | 0 | 30 | 0 | 0 | 30 |
| 53 | Chap43 | OrderedTableStPer | 26 | 26 | 0 | 1 | 0 | 27 | 0 | 0 | 27 |
| 54 | Chap44 | DocumentIndex | 15 | 15 | 0 | 2 | 0 | 17 | 0 | 0 | 17 |
| 55 | Chap44 | Example44_1 | 0 | 1 | 12 | 8 | 0 | 21 | 0 | 0 | 21 |
| 56 | Chap45 | BalancedTreePQ | 20 | 21 | 6 | 0 | 0 | 27 | 0 | 0 | 27 |
| 57 | Chap45 | BinaryHeapPQ | 10 | 11 | 16 | 0 | 0 | 27 | 0 | 0 | 27 |
| 58 | Chap45 | Example45_2 | 8 | 0 | 0 | 8 | 0 | 8 | 0 | 0 | 8 |
| 59 | Chap45 | HeapsortExample | 20 | 0 | 3 | 20 | 0 | 22 | 0 | 0 | 22 |
| 60 | Chap45 | LeftistHeapPQ | 20 | 20 | 8 | 2 | 0 | 27 | 0 | 0 | 27 |
| 61 | Chap45 | SortedListPQ | 18 | 19 | 0 | 0 | 0 | 19 | 0 | 0 | 19 |
| 62 | Chap45 | UnsortedListPQ | 15 | 16 | 0 | 0 | 0 | 16 | 0 | 0 | 16 |
| 63 | Chap47 | ChainedHashTable | 4 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 4 |
| 64 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 6 | 1 | 0 | 0 | 7 | 0 | 0 | 7 |
| 65 | Chap47 | FlatHashTable | 4 | 4 | 0 | 0 | 0 | 8 | 0 | 0 | 8 |
| 66 | Chap47 | LinProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 67 | Chap47 | LinkedListChainedHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 68 | Chap47 | ParaHashTableStEph | 8 | 0 | 0 | 0 | 0 | 8 | 0 | 0 | 8 |
| 69 | Chap47 | QuadProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 70 | Chap47 | StructChainedHashTable | 0 | 7 | 0 | 0 | 0 | 7 | 0 | 0 | 7 |
| 71 | Chap47 | VecChainedHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 72 | Chap49 | MinEditDistMtEph | 11 | 12 | 1 | 0 | 0 | 13 | 0 | 0 | 13 |
| 73 | Chap49 | MinEditDistMtPer | 6 | 7 | 0 | 1 | 0 | 8 | 0 | 0 | 8 |
| 74 | Chap49 | MinEditDistStEph | 11 | 11 | 0 | 1 | 0 | 12 | 0 | 0 | 12 |
| 75 | Chap49 | MinEditDistStPer | 6 | 6 | 0 | 1 | 0 | 7 | 0 | 0 | 7 |
| 76 | Chap49 | SubsetSumMtEph | 8 | 9 | 0 | 1 | 0 | 10 | 0 | 0 | 10 |
| 77 | Chap49 | SubsetSumMtPer | 5 | 6 | 0 | 1 | 0 | 7 | 0 | 0 | 7 |
| 78 | Chap49 | SubsetSumStEph | 8 | 8 | 0 | 1 | 0 | 9 | 0 | 0 | 9 |
| 79 | Chap49 | SubsetSumStPer | 5 | 5 | 0 | 1 | 0 | 6 | 0 | 0 | 6 |
| 80 | Chap50 | MatrixChainMtEph | 10 | 11 | 3 | 0 | 0 | 14 | 0 | 0 | 14 |
| 81 | Chap50 | MatrixChainMtPer | 7 | 8 | 3 | 0 | 0 | 11 | 0 | 0 | 11 |
| 82 | Chap50 | MatrixChainStEph | 11 | 11 | 2 | 0 | 0 | 13 | 0 | 0 | 13 |
| 83 | Chap50 | MatrixChainStPer | 7 | 7 | 2 | 0 | 0 | 9 | 0 | 0 | 9 |
| 84 | Chap50 | OptBinSearchTreeMtEph | 10 | 11 | 0 | 2 | 0 | 13 | 0 | 0 | 13 |
| 85 | Chap50 | OptBinSearchTreeMtPer | 7 | 8 | 0 | 2 | 0 | 10 | 0 | 0 | 10 |
| 86 | Chap50 | OptBinSearchTreeStEph | 11 | 11 | 1 | 0 | 0 | 12 | 0 | 0 | 12 |
| 87 | Chap50 | OptBinSearchTreeStPer | 7 | 7 | 1 | 0 | 0 | 8 | 0 | 0 | 8 |
| 88 | Chap50 | Probability | 2 | 10 | 4 | 0 | 0 | 14 | 0 | 0 | 14 |
| 89 | Chap51 | BottomUpDPMtEph | 2 | 1 | 10 | 0 | 0 | 12 | 0 | 0 | 12 |
| 90 | Chap51 | BottomUpDPMtPer | 2 | 1 | 8 | 0 | 0 | 10 | 0 | 0 | 10 |
| 91 | Chap51 | BottomUpDPStEph | 2 | 1 | 10 | 0 | 0 | 12 | 0 | 0 | 12 |
| 92 | Chap51 | BottomUpDPStPer | 2 | 1 | 8 | 0 | 0 | 10 | 0 | 0 | 10 |
| 93 | Chap51 | TopDownDPMtEph | 2 | 2 | 15 | 0 | 0 | 18 | 0 | 0 | 18 |
| 94 | Chap51 | TopDownDPMtPer | 2 | 2 | 13 | 0 | 0 | 16 | 0 | 0 | 16 |
| 95 | Chap51 | TopDownDPStEph | 2 | 1 | 13 | 0 | 0 | 15 | 0 | 0 | 15 |
| 96 | Chap51 | TopDownDPStPer | 2 | 1 | 11 | 0 | 0 | 13 | 0 | 0 | 13 |

## Function-by-Function Detail

### Chap36/QuickSortMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 43&#8209;46 |
| 2 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 345&#8209;347 |
| 3 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 353&#8209;355 |
| 4 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 361&#8209;363 |

### Chap36/QuickSortMtEphSlice.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `pivot_mt_first` | Y | Y |  |  |  | Y | Y |  | 16&#8209;18 |
| 6 | `pivot_mt_median3` | Y | Y |  |  |  | Y | Y |  | 19&#8209;21 |
| 7 | `pivot_mt_random` | Y | Y |  |  |  | Y | Y |  | 22&#8209;24 |
| 8 | `quick_sort_mt_first` | Y | Y |  |  |  | Y | Y |  | 25&#8209;27 |
| 9 | `quick_sort_mt_median3` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 10 | `quick_sort_mt_random` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 11 | `sort` x3 |  | Y |  |  |  | Y | Y |  | 64&#8209;94 |

### Chap36/QuickSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 12 | `sort_vec` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 13 | `quick_sort_first` |  |  |  | Y | Y |  |  | unknown | 358&#8209;360 |
| 14 | `quick_sort_median3` |  |  |  | Y | Y |  |  | unknown | 368&#8209;370 |
| 15 | `quick_sort_random` |  |  |  | Y | Y |  |  | unknown | 378&#8209;380 |

### Chap37/AVLTreeSeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `new` x2 | Y | Y | Y |  |  | Y | Y |  | 54&#8209;56 |
| 17 | `empty` | Y | Y |  |  |  | Y | Y |  | 49&#8209;52 |
| 18 | `length` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 19 | `nth` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 20 | `set` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 21 | `singleton` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 22 | `isEmpty` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 23 | `isSingleton` | Y | Y |  |  |  | Y | Y |  | 77 |
| 24 | `subseq_copy` | Y | Y |  |  |  | Y | Y |  | 79&#8209;83 |
| 25 | `new_root` | Y | Y |  |  |  | Y | Y |  | 84 |
| 26 | `update` | Y | Y |  |  |  | Y | Y |  | 85 |
| 27 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 86&#8209;88 |
| 28 | `to_arrayseq` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 29 | `iter` | Y | Y |  |  |  | Y | Y |  | 92 |
| 30 | `push_back` | Y | Y |  |  |  | Y | Y |  | 93 |
| 31 | `contains_value` | Y | Y |  |  |  | Y | Y |  | 94&#8209;96 |
| 32 | `insert_value` | Y | Y |  |  |  | Y | Y |  | 97 |
| 33 | `delete_value` | Y | Y |  |  |  | Y | Y |  | 98&#8209;100 |
| 34 | `is_tree_empty` | Y | Y |  |  |  | Y | Y |  | 101 |
| 35 | `values_in_order` | Y | Y |  |  |  | Y | Y |  | 102&#8209;104 |
| 36 | `eq` |  | Y |  |  |  | Y | Y |  | 263&#8209;273 |
| 37 | `push_left` |  |  | Y |  |  | Y | Y |  | 317&#8209;323 |
| 38 | `next` |  | Y |  |  |  | Y | Y |  | 328&#8209;333 |
| 39 | `h` |  |  |  | Y |  | Y | Y |  | 336 |
| 40 | `size_link` |  |  |  | Y |  | Y | Y |  | 337&#8209;343 |
| 41 | `update_meta` |  |  |  | Y |  | Y | Y |  | 345&#8209;351 |
| 42 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 353&#8209;364 |
| 43 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 366&#8209;377 |
| 44 | `rebalance` |  |  |  | Y |  | Y | Y |  | 379&#8209;400 |
| 45 | `insert_at_link` |  |  |  | Y |  | Y | Y |  | 402&#8209;421 |
| 46 | `nth_link` |  |  |  | Y |  | Y | Y |  | 423&#8209;433 |
| 47 | `set_link` |  |  |  | Y |  | Y | Y |  | 435&#8209;450 |
| 48 | `push_inorder` |  |  |  | Y |  | Y | Y |  | 452&#8209;458 |
| 49 | `default` |  | Y |  |  |  | Y | Y |  | 461 |

### Chap37/AVLTreeSeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 50 | `height` |  |  |  | Y |  | Y | Y |  | 22 |
| 51 | `size` |  |  |  | Y |  | Y | Y |  | 23 |
| 52 | `mk` |  |  |  | Y |  | Y | Y |  | 25&#8209;36 |
| 53 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 38&#8209;43 |
| 54 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 45&#8209;50 |
| 55 | `rebalance` |  |  |  | Y |  | Y | Y |  | 52&#8209;72 |
| 56 | `nth_ref` |  |  |  | Y |  | Y | Y |  | 74&#8209;87 |
| 57 | `set_rec` |  |  |  | Y |  | Y | Y |  | 89&#8209;111 |
| 58 | `inorder_collect` |  |  |  | Y |  | Y | Y |  | 113&#8209;119 |
| 59 | `build_balanced_from_slice` |  |  |  | Y |  | Y | Y |  | 121&#8209;137 |
| 60 | `rec` |  |  |  | Y |  | Y | Y |  | 122&#8209;135 |
| 61 | `empty` | Y | Y |  |  |  | Y | Y |  | 144&#8209;146 |
| 62 | `new` | Y | Y |  |  |  | Y | Y |  | 147&#8209;148 |
| 63 | `length` | Y | Y |  |  |  | Y | Y |  | 149&#8209;150 |
| 64 | `nth` | Y | Y |  |  |  | Y | Y |  | 151&#8209;152 |
| 65 | `set` | Y | Y |  |  |  | Y | Y |  | 153&#8209;156 |
| 66 | `singleton` | Y | Y |  |  |  | Y | Y |  | 157&#8209;158 |
| 67 | `isEmpty` | Y | Y |  |  |  | Y | Y |  | 159&#8209;160 |
| 68 | `isSingleton` | Y | Y |  |  |  | Y | Y |  | 161&#8209;162 |
| 69 | `subseq_copy` | Y | Y |  |  |  | Y | Y |  | 163&#8209;164 |
| 70 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 165&#8209;166 |
| 71 | `values_in_order` | Y | Y |  |  |  | Y | Y |  | 167&#8209;168 |
| 72 | `default` |  | Y |  |  |  | Y | Y |  | 248 |
| 73 | `next` |  | Y |  |  |  | Y | Y |  | 260&#8209;268 |
| 74 | `eq` |  | Y |  |  |  | Y | Y |  | 284&#8209;294 |

### Chap37/AVLTreeSeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 75 | `new` x2 | Y | Y | Y |  |  | Y | Y |  | 47&#8209;48 |
| 76 | `empty` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 77 | `length` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 78 | `nth` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 79 | `set` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 80 | `singleton` | Y | Y |  |  |  | Y | Y |  | 55&#8209;56 |
| 81 | `isEmpty` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 82 | `isSingleton` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 83 | `subseq_copy` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 84 | `new_root` | Y | Y |  |  |  | Y | Y |  | 63 |
| 85 | `update` | Y | Y |  |  |  | Y | Y |  | 64 |
| 86 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 65 |
| 87 | `to_arrayseq` | Y | Y |  |  |  | Y | Y |  | 66 |
| 88 | `iter` | Y | Y |  |  |  | Y | Y |  | 67 |
| 89 | `push_back` | Y | Y |  |  |  | Y | Y |  | 68 |
| 90 | `contains_value` | Y | Y |  |  |  | Y | Y |  | 69 |
| 91 | `insert_value` | Y | Y |  |  |  | Y | Y |  | 70 |
| 92 | `delete_value` | Y | Y |  |  |  | Y | Y |  | 71 |
| 93 | `push_left` |  |  | Y |  |  | Y | Y |  | 209&#8209;215 |
| 94 | `next` |  | Y |  |  |  | Y | Y |  | 220&#8209;225 |
| 95 | `h` |  |  |  | Y |  | Y | Y |  | 228 |
| 96 | `size_link` |  |  |  | Y |  | Y | Y |  | 230&#8209;236 |
| 97 | `update_meta` |  |  |  | Y |  | Y | Y |  | 238&#8209;244 |
| 98 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 246&#8209;255 |
| 99 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 257&#8209;266 |
| 100 | `rebalance` |  |  |  | Y |  | Y | Y |  | 268&#8209;287 |
| 101 | `insert_at_link` |  |  |  | Y |  | Y | Y |  | 289&#8209;307 |
| 102 | `nth_link` |  |  |  | Y |  | Y | Y |  | 309&#8209;319 |
| 103 | `set_link` |  |  |  | Y |  | Y | Y |  | 321&#8209;336 |
| 104 | `default` |  | Y |  |  |  | Y | Y |  | 339 |
| 105 | `eq` |  | Y |  |  |  | Y | Y |  | 358&#8209;368 |

### Chap37/AVLTreeSeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `height` |  |  |  | Y |  | Y | Y |  | 22 |
| 107 | `size` |  |  |  | Y |  | Y | Y |  | 23 |
| 108 | `mk` |  |  |  | Y |  | Y | Y |  | 25&#8209;36 |
| 109 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 38&#8209;43 |
| 110 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 45&#8209;50 |
| 111 | `rebalance` |  |  |  | Y |  | Y | Y |  | 52&#8209;72 |
| 112 | `nth_ref` |  |  |  | Y |  | Y | Y |  | 74&#8209;87 |
| 113 | `set_rec` |  |  |  | Y |  | Y | Y |  | 89&#8209;112 |
| 114 | `inorder_collect` |  |  |  | Y |  | Y | Y |  | 114&#8209;120 |
| 115 | `build_balanced_from_slice` |  |  |  | Y |  | Y | Y |  | 122&#8209;133 |
| 116 | `rec` |  |  |  | Y |  | Y | Y |  | 123&#8209;131 |
| 117 | `empty` | Y | Y |  |  |  | Y | Y |  | 140&#8209;142 |
| 118 | `new` | Y | Y |  |  |  | Y | Y |  | 143&#8209;144 |
| 119 | `length` | Y | Y |  |  |  | Y | Y |  | 145&#8209;146 |
| 120 | `nth` | Y | Y |  |  |  | Y | Y |  | 147&#8209;148 |
| 121 | `set` | Y | Y |  |  |  | Y | Y |  | 149&#8209;152 |
| 122 | `singleton` | Y | Y |  |  |  | Y | Y |  | 153&#8209;154 |
| 123 | `isEmpty` | Y | Y |  |  |  | Y | Y |  | 155&#8209;156 |
| 124 | `isSingleton` | Y | Y |  |  |  | Y | Y |  | 157&#8209;158 |
| 125 | `subseq_copy` | Y | Y |  |  |  | Y | Y |  | 159&#8209;160 |
| 126 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 161&#8209;162 |
| 127 | `values_in_order` | Y | Y |  |  |  | Y | Y |  | 163&#8209;164 |
| 128 | `to_arrayseq` | Y | Y |  |  |  | Y | Y |  | 165 |
| 129 | `iter` | Y | Y |  |  |  | Y | Y |  | 166 |
| 130 | `eq` |  | Y |  |  |  | Y | Y |  | 223&#8209;233 |
| 131 | `push_left` |  |  | Y |  |  | Y | Y |  | 258&#8209;263 |
| 132 | `next` |  | Y |  |  |  | Y | Y |  | 268&#8209;277 |

### Chap37/BSTAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 39&#8209;71 |
| 134 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 86&#8209;90 |
| 135 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 152&#8209;156 |
| 136 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 219&#8209;226 |
| 137 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 309&#8209;312 |
| 138 | `find_node` |  |  |  | Y | Y |  |  | unknown | 334&#8209;339 |
| 139 | `min_node` |  |  |  | Y | Y |  | Y |  | 361&#8209;362 |
| 140 | `max_node` |  |  |  | Y | Y |  | Y |  | 373&#8209;374 |
| 141 | `new` |  |  | Y |  | Y |  | Y |  | 388 |
| 142 | `insert` |  |  | Y |  | Y |  |  | hole | 396 |
| 143 | `contains` |  |  | Y |  | Y |  |  | hole | 404 |
| 144 | `size` |  |  | Y |  | Y |  |  | hole | 411 |
| 145 | `is_empty` |  |  | Y |  | Y |  |  | hole | 418 |
| 146 | `height` |  |  | Y |  | Y |  |  | hole | 425 |

### Chap37/BSTAVLStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 147 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 54&#8209;86 |
| 148 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 99&#8209;105 |
| 149 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 196&#8209;202 |
| 150 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 293&#8209;300 |
| 151 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 422&#8209;425 |
| 152 | `find_node` |  |  |  | Y | Y |  |  | unknown | 455&#8209;460 |
| 153 | `min_node` |  |  |  | Y | Y |  | Y |  | 490&#8209;491 |
| 154 | `max_node` |  |  |  | Y | Y |  | Y |  | 505&#8209;506 |
| 155 | `avl_new` |  |  |  | Y | Y |  |  | unknown | 520&#8209;523 |
| 156 | `avl_size` |  |  |  | Y | Y |  |  | unknown | 528&#8209;530 |
| 157 | `avl_is_empty` |  |  |  | Y | Y |  |  | unknown | 535&#8209;536 |
| 158 | `avl_height` |  |  |  | Y | Y |  |  | unknown | 541&#8209;543 |
| 159 | `avl_insert` |  |  |  | Y | Y |  |  | unknown | 548&#8209;554 |
| 160 | `avl_contains` |  |  |  | Y | Y |  |  | unknown | 559&#8209;561 |
| 161 | `avl_find` |  |  |  | Y | Y |  |  | unknown | 566&#8209;570 |

### Chap37/BSTBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 162 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 44&#8209;51 |
| 163 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 134&#8209;137 |
| 164 | `find_node` |  |  |  | Y | Y |  |  | unknown | 159&#8209;164 |
| 165 | `min_node` |  |  |  | Y | Y |  | Y |  | 186&#8209;187 |
| 166 | `max_node` |  |  |  | Y | Y |  | Y |  | 198&#8209;199 |
| 167 | `new` |  |  | Y |  | Y |  | Y |  | 213 |
| 168 | `insert` |  |  | Y |  | Y |  |  | hole | 221 |
| 169 | `contains` |  |  | Y |  | Y |  |  | hole | 229 |
| 170 | `size` |  |  | Y |  | Y |  |  | hole | 236 |
| 171 | `is_empty` |  |  | Y |  | Y |  |  | hole | 243 |
| 172 | `height` |  |  | Y |  | Y |  |  | hole | 250 |

### Chap37/BSTBBAlphaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 173 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 58&#8209;65 |
| 174 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 187&#8209;190 |
| 175 | `find_node` |  |  |  | Y | Y |  |  | unknown | 220&#8209;225 |
| 176 | `min_node` |  |  |  | Y | Y |  | Y |  | 255&#8209;256 |
| 177 | `max_node` |  |  |  | Y | Y |  | Y |  | 270&#8209;271 |
| 178 | `bb_new` |  |  |  | Y | Y |  |  | unknown | 285&#8209;288 |
| 179 | `bb_size` |  |  |  | Y | Y |  |  | unknown | 293&#8209;295 |
| 180 | `bb_is_empty` |  |  |  | Y | Y |  |  | unknown | 300&#8209;301 |
| 181 | `bb_height` |  |  |  | Y | Y |  |  | unknown | 306&#8209;308 |
| 182 | `bb_insert` |  |  |  | Y | Y |  |  | unknown | 313&#8209;319 |
| 183 | `bb_contains` |  |  |  | Y | Y |  |  | unknown | 324&#8209;326 |
| 184 | `bb_find` |  |  |  | Y | Y |  |  | unknown | 331&#8209;335 |

### Chap37/BSTPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 185 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 55&#8209;62 |
| 186 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 145&#8209;148 |
| 187 | `find_node` |  |  |  | Y | Y |  |  | unknown | 170&#8209;175 |
| 188 | `min_node` |  |  |  | Y | Y |  | Y |  | 197&#8209;198 |
| 189 | `max_node` |  |  |  | Y | Y |  | Y |  | 209&#8209;210 |
| 190 | `new` |  |  | Y |  | Y |  | Y |  | 224 |
| 191 | `insert` |  |  | Y |  | Y |  | Y |  | 234 |
| 192 | `contains` |  |  | Y |  | Y |  | Y |  | 241 |
| 193 | `is_empty` |  |  | Y |  | Y |  | Y |  | 250 |
| 194 | `size` |  |  | Y |  | Y |  |  | hole | 259 |
| 195 | `height` |  |  | Y |  | Y |  |  | hole | 269 |

### Chap37/BSTPlainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 196 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 63&#8209;70 |
| 197 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 192&#8209;195 |
| 198 | `find_node` |  |  |  | Y | Y |  |  | unknown | 225&#8209;230 |
| 199 | `min_node` |  |  |  | Y | Y |  | Y |  | 260&#8209;261 |
| 200 | `max_node` |  |  |  | Y | Y |  | Y |  | 275&#8209;276 |
| 201 | `bst_new` |  |  |  | Y | Y |  |  | unknown | 290&#8209;293 |
| 202 | `bst_size` |  |  |  | Y | Y |  |  | unknown | 298&#8209;300 |
| 203 | `bst_is_empty` |  |  |  | Y | Y |  |  | unknown | 305&#8209;306 |
| 204 | `bst_height` |  |  |  | Y | Y |  |  | unknown | 311&#8209;313 |
| 205 | `bst_insert` |  |  |  | Y | Y |  |  | unknown | 318&#8209;324 |
| 206 | `bst_contains` |  |  |  | Y | Y |  |  | unknown | 329&#8209;331 |
| 207 | `bst_find` |  |  |  | Y | Y |  |  | unknown | 336&#8209;340 |

### Chap37/BSTRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 208 | `new_node` |  |  |  | Y |  | Y | Y |  | 29&#8209;37 |
| 209 | `is_red` |  |  |  | Y |  | Y | Y |  | 39 |
| 210 | `size_link` |  |  |  | Y |  | Y | Y |  | 41 |
| 211 | `update` |  |  |  | Y |  | Y | Y |  | 43 |
| 212 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 45&#8209;62 |
| 213 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 64&#8209;81 |
| 214 | `flip_colors` |  |  |  | Y |  | Y | Y |  | 83&#8209;102 |
| 215 | `fix_up` |  |  |  | Y |  | Y | Y |  | 104&#8209;138 |
| 216 | `insert_link` |  |  |  | Y |  | Y | Y |  | 140&#8209;154 |
| 217 | `find_link` |  |  |  | Y |  | Y | Y |  | 156&#8209;169 |
| 218 | `min_link` |  |  |  | Y |  | Y | Y |  | 171&#8209;179 |
| 219 | `max_link` |  |  |  | Y |  | Y | Y |  | 181&#8209;189 |
| 220 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 191&#8209;197 |
| 221 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 199&#8209;205 |
| 222 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 209&#8209;224 |
| 223 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 226&#8209;241 |
| 224 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 243&#8209;263 |
| 225 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 265&#8209;290 |
| 226 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 292&#8209;314 |
| 227 | `new` | Y | Y |  |  |  | Y | Y |  | 324&#8209;325 |
| 228 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 326&#8209;327 |
| 229 | `insert` | Y | Y |  |  |  | Y | Y |  | 328&#8209;329 |
| 230 | `find` | Y | Y |  |  |  | Y | Y |  | 330&#8209;331 |
| 231 | `contains` | Y | Y |  |  |  | Y | Y |  | 332 |
| 232 | `size` | Y | Y |  |  |  | Y | Y |  | 333 |
| 233 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 334 |
| 234 | `height` | Y | Y |  |  |  | Y | Y |  | 335 |
| 235 | `minimum` | Y | Y |  |  |  | Y | Y |  | 336 |
| 236 | `maximum` | Y | Y |  |  |  | Y | Y |  | 337 |
| 237 | `in_order` | Y | Y |  |  |  | Y | Y |  | 338&#8209;339 |
| 238 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 340&#8209;341 |
| 239 | `filter` | Y | Y |  |  |  | Y | Y |  | 342&#8209;345 |
| 240 | `reduce` | Y | Y |  |  |  | Y | Y |  | 346&#8209;349 |
| 241 | `height_rec` |  | Y |  |  |  | Y | Y |  | 382&#8209;387 |
| 242 | `default` |  | Y |  |  |  | Y | Y |  | 441 |

### Chap37/BSTRBStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 243 | `lemma_bst_deep` |  |  |  | Y | Y |  |  | unknown | 38&#8209;70 |
| 244 | `rotate_right` |  |  |  | Y | Y |  |  | unknown | 84&#8209;90 |
| 245 | `rotate_left` |  |  |  | Y | Y |  |  | unknown | 182&#8209;188 |
| 246 | `insert_node` |  |  |  | Y | Y |  |  | unknown | 279&#8209;286 |
| 247 | `contains_node` |  |  |  | Y | Y |  |  | unknown | 408&#8209;411 |
| 248 | `find_node` |  |  |  | Y | Y |  |  | unknown | 441&#8209;446 |
| 249 | `min_node` |  |  |  | Y | Y |  | Y |  | 476&#8209;477 |
| 250 | `max_node` |  |  |  | Y | Y |  | Y |  | 491&#8209;492 |
| 251 | `rb_new` |  |  |  | Y | Y |  |  | unknown | 506&#8209;509 |
| 252 | `rb_size` |  |  |  | Y | Y |  |  | unknown | 514&#8209;516 |
| 253 | `rb_is_empty` |  |  |  | Y | Y |  |  | unknown | 521&#8209;522 |
| 254 | `rb_height` |  |  |  | Y | Y |  |  | unknown | 527&#8209;529 |
| 255 | `rb_insert` |  |  |  | Y | Y |  |  | unknown | 534&#8209;540 |
| 256 | `rb_contains` |  |  |  | Y | Y |  |  | unknown | 545&#8209;547 |
| 257 | `rb_find` |  |  |  | Y | Y |  |  | unknown | 552&#8209;556 |

### Chap37/BSTSetAVLMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 258 | `values_vec` |  |  |  | Y |  | Y | Y |  | 19 |
| 259 | `rebuild_from_vec` |  |  |  | Y |  | Y | Y |  | 21&#8209;27 |
| 260 | `from_sorted_iter` |  |  |  | Y |  | Y | Y |  | 29&#8209;38 |
| 261 | `empty` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 262 | `singleton` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 263 | `size` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 264 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 265 | `find` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 266 | `contains` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 267 | `minimum` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 268 | `maximum` | Y | Y |  |  |  | Y | Y |  | 55&#8209;56 |
| 269 | `insert` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 270 | `delete` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 271 | `union` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 272 | `intersection` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 273 | `difference` | Y | Y |  |  |  | Y | Y |  | 65&#8209;66 |
| 274 | `split` | Y | Y |  |  |  | Y | Y |  | 67&#8209;68 |
| 275 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 69&#8209;70 |
| 276 | `join_m` | Y | Y |  |  |  | Y | Y |  | 71&#8209;72 |
| 277 | `filter` | Y | Y |  |  |  | Y | Y |  | 73&#8209;74 |
| 278 | `reduce` | Y | Y |  |  |  | Y | Y |  | 75&#8209;76 |
| 279 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 77&#8209;78 |
| 280 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 79&#8209;80 |

### Chap37/BSTSetBBAlphaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 281 | `empty` | Y | Y |  |  |  | Y | Y |  | 20&#8209;21 |
| 282 | `singleton` | Y | Y |  |  |  | Y | Y |  | 22&#8209;23 |
| 283 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 284 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 285 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 286 | `contains` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 287 | `minimum` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 288 | `maximum` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 289 | `insert` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 290 | `delete` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 291 | `union` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 292 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 293 | `difference` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 294 | `split` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 295 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 296 | `join_m` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 297 | `filter` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 298 | `reduce` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 299 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 300 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 301 | `values_vec` |  |  | Y |  |  | Y | Y |  | 63 |
| 302 | `rebuild_from_vec` |  |  | Y |  |  | Y | Y |  | 64&#8209;70 |
| 303 | `from_sorted_iter` |  |  | Y |  |  | Y | Y |  | 71&#8209;80 |

### Chap37/BSTSetPlainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 304 | `empty` | Y | Y |  |  |  | Y | Y |  | 20&#8209;21 |
| 305 | `singleton` | Y | Y |  |  |  | Y | Y |  | 22&#8209;23 |
| 306 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 307 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 308 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 309 | `contains` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 310 | `minimum` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 311 | `maximum` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 312 | `insert` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 313 | `delete` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 314 | `union` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 315 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 316 | `difference` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 317 | `split` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 318 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 319 | `join_m` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 320 | `filter` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 321 | `reduce` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 322 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 323 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 324 | `values_vec` |  |  | Y |  |  | Y | Y |  | 63 |
| 325 | `rebuild_from_vec` |  |  | Y |  |  | Y | Y |  | 64&#8209;70 |
| 326 | `from_sorted_iter` |  |  | Y |  |  | Y | Y |  | 71&#8209;80 |

### Chap37/BSTSetRBMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 327 | `empty` | Y | Y |  |  |  | Y | Y |  | 20&#8209;21 |
| 328 | `singleton` | Y | Y |  |  |  | Y | Y |  | 22&#8209;23 |
| 329 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 330 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 331 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 332 | `contains` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 333 | `minimum` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 334 | `maximum` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 335 | `insert` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 336 | `delete` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 337 | `union` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 338 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 339 | `difference` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 340 | `split` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 341 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 342 | `join_m` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 343 | `filter` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 344 | `reduce` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 345 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 346 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 347 | `values_vec` |  |  | Y |  |  | Y | Y |  | 63 |
| 348 | `rebuild_from_vec` |  |  | Y |  |  | Y | Y |  | 64&#8209;70 |
| 349 | `from_sorted_iter` |  |  | Y |  |  | Y | Y |  | 71&#8209;80 |

### Chap37/BSTSetSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 350 | `empty` | Y | Y |  |  |  | Y | Y |  | 20&#8209;21 |
| 351 | `singleton` | Y | Y |  |  |  | Y | Y |  | 22&#8209;23 |
| 352 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 353 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 354 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 355 | `contains` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 356 | `minimum` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 357 | `maximum` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 358 | `insert` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 359 | `delete` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 360 | `union` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 361 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 362 | `difference` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 363 | `split` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 364 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 365 | `join_m` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 366 | `filter` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 367 | `reduce` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 368 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 369 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 370 | `values_vec` |  |  | Y |  |  | Y | Y |  | 63 |
| 371 | `rebuild_from_vec` |  |  | Y |  |  | Y | Y |  | 64&#8209;70 |
| 372 | `from_sorted_iter` |  |  | Y |  |  | Y | Y |  | 71&#8209;80 |

### Chap37/BSTSplayMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 373 | `new_node` |  |  |  | Y |  | Y | Y |  | 22&#8209;29 |
| 374 | `size_link` |  |  |  | Y |  | Y | Y |  | 31 |
| 375 | `update` |  |  |  | Y |  | Y | Y |  | 33 |
| 376 | `insert_link` |  |  |  | Y |  | Y | Y |  | 35&#8209;55 |
| 377 | `find_link` |  |  |  | Y |  | Y | Y |  | 57&#8209;70 |
| 378 | `min_link` |  |  |  | Y |  | Y | Y |  | 72&#8209;80 |
| 379 | `max_link` |  |  |  | Y |  | Y | Y |  | 82&#8209;90 |
| 380 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 92&#8209;98 |
| 381 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 100&#8209;106 |
| 382 | `in_order_parallel` |  |  |  | Y |  | Y | Y |  | 110&#8209;125 |
| 383 | `pre_order_parallel` |  |  |  | Y |  | Y | Y |  | 127&#8209;142 |
| 384 | `build_balanced` |  |  |  | Y |  | Y | Y |  | 144&#8209;163 |
| 385 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 165&#8209;190 |
| 386 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 192&#8209;214 |
| 387 | `new` | Y | Y |  |  |  | Y | Y |  | 224&#8209;225 |
| 388 | `from_sorted_slice` | Y | Y |  |  |  | Y | Y |  | 226&#8209;227 |
| 389 | `insert` | Y | Y |  |  |  | Y | Y |  | 228&#8209;229 |
| 390 | `find` | Y | Y |  |  |  | Y | Y |  | 230&#8209;231 |
| 391 | `contains` | Y | Y |  |  |  | Y | Y |  | 232&#8209;233 |
| 392 | `size` | Y | Y |  |  |  | Y | Y |  | 234&#8209;235 |
| 393 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 236&#8209;237 |
| 394 | `height` | Y | Y |  |  |  | Y | Y |  | 238&#8209;239 |
| 395 | `minimum` | Y | Y |  |  |  | Y | Y |  | 240&#8209;241 |
| 396 | `maximum` | Y | Y |  |  |  | Y | Y |  | 242&#8209;243 |
| 397 | `in_order` | Y | Y |  |  |  | Y | Y |  | 244&#8209;245 |
| 398 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 246&#8209;247 |
| 399 | `filter` | Y | Y |  |  |  | Y | Y |  | 248&#8209;251 |
| 400 | `reduce` | Y | Y |  |  |  | Y | Y |  | 252&#8209;255 |
| 401 | `height_rec` |  | Y |  |  |  | Y | Y |  | 285&#8209;290 |
| 402 | `default` |  | Y |  |  |  | Y | Y |  | 344 |

### Chap37/BSTSplayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 403 | `new_node` |  |  |  | Y |  | Y | Y |  | 20&#8209;27 |
| 404 | `size_link` |  |  |  | Y |  | Y | Y |  | 29 |
| 405 | `update` |  |  |  | Y |  | Y | Y |  | 31 |
| 406 | `insert_link` |  |  |  | Y |  | Y | Y |  | 33&#8209;53 |
| 407 | `find_link` |  |  |  | Y |  | Y | Y |  | 55&#8209;68 |
| 408 | `min_link` |  |  |  | Y |  | Y | Y |  | 70&#8209;78 |
| 409 | `max_link` |  |  |  | Y |  | Y | Y |  | 80&#8209;88 |
| 410 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 90&#8209;96 |
| 411 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 98&#8209;104 |
| 412 | `new` | Y | Y |  |  |  | Y | Y |  | 114&#8209;115 |
| 413 | `size` | Y | Y |  |  |  | Y | Y |  | 116&#8209;117 |
| 414 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 118&#8209;119 |
| 415 | `height` | Y | Y |  |  |  | Y | Y |  | 120&#8209;121 |
| 416 | `insert` | Y | Y |  |  |  | Y | Y |  | 122&#8209;123 |
| 417 | `find` | Y | Y |  |  |  | Y | Y |  | 124&#8209;125 |
| 418 | `contains` | Y | Y |  |  |  | Y | Y |  | 126&#8209;127 |
| 419 | `minimum` | Y | Y |  |  |  | Y | Y |  | 128&#8209;129 |
| 420 | `maximum` | Y | Y |  |  |  | Y | Y |  | 130&#8209;131 |
| 421 | `in_order` | Y | Y |  |  |  | Y | Y |  | 132&#8209;133 |
| 422 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 134&#8209;135 |
| 423 | `height_rec` |  | Y |  |  |  | Y | Y |  | 146&#8209;151 |
| 424 | `default` |  | Y |  |  |  | Y | Y |  | 179 |

### Chap38/BSTParaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 425 | `new` | Y | Y |  |  |  | Y | Y |  | 32&#8209;34 |
| 426 | `expose` | Y | Y |  |  |  | Y | Y |  | 35&#8209;37 |
| 427 | `join_mid` | Y | Y |  | Y |  | Y | Y |  | 38&#8209;40 |
| 428 | `size` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 429 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 430 | `insert` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 431 | `delete` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 432 | `find` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 433 | `split` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 434 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 435 | `union` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 436 | `intersect` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 437 | `difference` | Y | Y |  |  |  | Y | Y |  | 68&#8209;70 |
| 438 | `filter` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 439 | `reduce` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 440 | `in_order` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 441 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 82&#8209;90 |
| 442 | `split_inner` |  |  |  | Y |  | Y | Y |  | 108&#8209;135 |
| 443 | `join_m` |  |  |  | Y |  | Y | Y |  | 137&#8209;141 |
| 444 | `min_key` |  |  |  | Y |  | Y | Y |  | 143&#8209;153 |
| 445 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 155&#8209;166 |
| 446 | `union_inner` |  |  |  | Y |  | Y | Y |  | 168&#8209;180 |
| 447 | `intersect_inner` |  |  |  | Y |  | Y | Y |  | 182&#8209;200 |
| 448 | `difference_inner` |  |  |  | Y |  | Y | Y |  | 202&#8209;222 |
| 449 | `filter_inner` |  |  |  | Y |  | Y | Y |  | 224&#8209;248 |
| 450 | `filter_parallel` |  |  |  | Y |  | Y | Y |  | 250&#8209;258 |
| 451 | `reduce_inner` |  |  |  | Y |  | Y | Y |  | 260&#8209;283 |
| 452 | `reduce_parallel` |  |  |  | Y |  | Y | Y |  | 285&#8209;294 |
| 453 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 296&#8209;307 |

### Chap38/BSTParaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 454 | `new` | Y | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 455 | `expose` | Y | Y |  |  |  | Y | Y |  | 37&#8209;39 |
| 456 | `join_mid` | Y | Y |  | Y |  | Y | Y |  | 40&#8209;42 |
| 457 | `size` | Y | Y |  |  |  | Y | Y |  | 43&#8209;45 |
| 458 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 46&#8209;48 |
| 459 | `insert` | Y | Y |  |  |  | Y | Y |  | 49&#8209;51 |
| 460 | `delete` | Y | Y |  |  |  | Y | Y |  | 52&#8209;54 |
| 461 | `find` | Y | Y |  |  |  | Y | Y |  | 55&#8209;57 |
| 462 | `split` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 463 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 464 | `union` | Y | Y |  |  |  | Y | Y |  | 64&#8209;66 |
| 465 | `in_order` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 466 | `expose_internal` |  |  |  | Y |  | Y | Y |  | 72&#8209;80 |
| 467 | `split_inner` |  |  |  | Y |  | Y | Y |  | 98&#8209;125 |
| 468 | `join_m` |  |  |  | Y |  | Y | Y |  | 127&#8209;131 |
| 469 | `min_key` |  |  |  | Y |  | Y | Y |  | 133&#8209;143 |
| 470 | `join_pair_inner` |  |  |  | Y |  | Y | Y |  | 145&#8209;156 |
| 471 | `union_inner` |  |  |  | Y |  | Y | Y |  | 158&#8209;170 |
| 472 | `collect_in_order` |  |  |  | Y |  | Y | Y |  | 172&#8209;183 |

### Chap39/BSTParaTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 473 | `priority_for` |  |  |  | Y |  | Y | Y |  | 34&#8209;42 |
| 474 | `tree_priority` |  |  |  | Y |  | Y | Y |  | 44&#8209;49 |
| 475 | `tree_size` |  |  |  | Y |  | Y | Y |  | 51&#8209;56 |
| 476 | `make_node` |  |  |  | Y |  | Y | Y |  | 58&#8209;71 |
| 477 | `new` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 478 | `expose` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 479 | `join_mid` | Y | Y |  |  |  | Y | Y |  | 80&#8209;82 |
| 480 | `size` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 481 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 86&#8209;88 |
| 482 | `insert` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 483 | `delete` | Y | Y |  |  |  | Y | Y |  | 92&#8209;94 |
| 484 | `find` | Y | Y |  |  |  | Y | Y |  | 95&#8209;97 |
| 485 | `split` | Y | Y |  |  |  | Y | Y |  | 98&#8209;100 |
| 486 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 101&#8209;103 |
| 487 | `union` | Y | Y |  |  |  | Y | Y |  | 104&#8209;106 |
| 488 | `intersect` | Y | Y |  |  |  | Y | Y |  | 107&#8209;109 |
| 489 | `difference` | Y | Y |  |  |  | Y | Y |  | 110&#8209;112 |
| 490 | `filter` | Y | Y |  |  |  | Y | Y |  | 113&#8209;115 |
| 491 | `reduce` | Y | Y |  |  |  | Y | Y |  | 116&#8209;120 |
| 492 | `in_order` | Y | Y |  |  |  | Y | Y |  | 121&#8209;123 |
| 493 | `expose_internal` |  |  | Y |  |  | Y | Y |  | 127&#8209;135 |
| 494 | `expose_with_priority` |  |  | Y |  |  | Y | Y |  | 137&#8209;144 |
| 495 | `join_with_priority` |  |  | Y |  |  | Y | Y |  | 146&#8209;167 |
| 496 | `split_inner` |  |  | Y |  |  | Y | Y |  | 169&#8209;188 |
| 497 | `join_pair_inner` |  |  | Y |  |  | Y | Y |  | 190&#8209;202 |
| 498 | `union_inner` |  |  | Y |  |  | Y | Y |  | 204&#8209;218 |
| 499 | `intersect_inner` |  |  | Y |  |  | Y | Y |  | 220&#8209;238 |
| 500 | `difference_inner` |  |  | Y |  |  | Y | Y |  | 240&#8209;258 |
| 501 | `filter_inner` |  |  | Y |  |  | Y | Y |  | 260&#8209;279 |
| 502 | `filter_parallel` |  |  | Y |  |  | Y | Y |  | 281&#8209;286 |
| 503 | `reduce_inner` |  |  | Y |  |  | Y | Y |  | 288&#8209;310 |
| 504 | `reduce_parallel` |  |  | Y |  |  | Y | Y |  | 312&#8209;320 |
| 505 | `collect_in_order` |  |  | Y |  |  | Y | Y |  | 322&#8209;333 |

### Chap39/BSTSetTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 506 | `empty` | Y | Y |  |  |  | Y | Y |  | 40&#8209;42 |
| 507 | `singleton` | Y | Y |  |  |  | Y | Y |  | 43&#8209;45 |
| 508 | `size` | Y | Y |  |  |  | Y | Y |  | 46&#8209;48 |
| 509 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 49&#8209;51 |
| 510 | `find` | Y | Y |  |  |  | Y | Y |  | 52&#8209;54 |
| 511 | `contains` | Y | Y |  |  |  | Y | Y |  | 55&#8209;57 |
| 512 | `minimum` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 513 | `maximum` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 514 | `insert` | Y | Y |  |  |  | Y | Y |  | 64&#8209;66 |
| 515 | `delete` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 516 | `union` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 517 | `intersection` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 518 | `difference` | Y | Y |  |  |  | Y | Y |  | 76&#8209;78 |
| 519 | `split` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 520 | `join_pair` | Y | Y |  |  |  | Y | Y |  | 82&#8209;84 |
| 521 | `join_m` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 522 | `filter` | Y | Y |  |  |  | Y | Y |  | 88&#8209;90 |
| 523 | `reduce` | Y | Y |  |  |  | Y | Y |  | 91&#8209;93 |
| 524 | `iter_in_order` | Y | Y |  |  |  | Y | Y |  | 94&#8209;96 |
| 525 | `as_tree` | Y | Y |  |  |  | Y | Y |  | 97&#8209;99 |
| 526 | `values_vec` |  |  | Y |  |  | Y | Y |  | 103&#8209;105 |
| 527 | `rebuild_from_vec` |  |  | Y |  |  | Y | Y |  | 106&#8209;114 |
| 528 | `from_sorted_iter` |  |  | Y |  |  | Y | Y |  | 115&#8209;126 |

### Chap39/BSTTreapMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 529 | `new` | Y | Y | Y |  |  | Y | Y |  | 44&#8209;46 |
| 530 | `insert` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 531 | `find` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 532 | `contains` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 533 | `size` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 534 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 535 | `height` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 536 | `minimum` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 537 | `maximum` | Y | Y |  |  |  | Y | Y |  | 68&#8209;70 |
| 538 | `in_order` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 539 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 540 | `size_link` |  |  | Y |  |  | Y | Y |  | 81&#8209;83 |
| 541 | `update` |  |  | Y |  |  | Y | Y |  | 85&#8209;87 |
| 542 | `rotate_left` |  |  | Y |  |  | Y | Y |  | 89&#8209;103 |
| 543 | `rotate_right` |  |  | Y |  |  | Y | Y |  | 105&#8209;119 |
| 544 | `insert_link` |  |  | Y |  |  | Y | Y |  | 121&#8209;142 |
| 545 | `find_link` |  |  | Y |  |  | Y | Y |  | 144&#8209;159 |
| 546 | `min_link` |  |  | Y |  |  | Y | Y |  | 161&#8209;171 |
| 547 | `max_link` |  |  | Y |  |  | Y | Y |  | 173&#8209;183 |
| 548 | `in_order_collect` |  |  | Y |  |  | Y | Y |  | 185&#8209;193 |
| 549 | `pre_order_collect` |  |  | Y |  |  | Y | Y |  | 195&#8209;203 |
| 550 | `height_rec` |  | Y |  |  |  | Y | Y |  | 234&#8209;239 |
| 551 | `default` |  | Y |  |  |  | Y | Y |  | 271 |

### Chap39/BSTTreapStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 552 | `new_node` |  |  |  | Y |  | Y | Y |  | 22&#8209;32 |
| 553 | `new` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 554 | `size` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 555 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 48&#8209;50 |
| 556 | `height` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 557 | `insert` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 558 | `find` | Y | Y |  |  |  | Y | Y |  | 57&#8209;59 |
| 559 | `contains` | Y | Y |  |  |  | Y | Y |  | 60&#8209;62 |
| 560 | `minimum` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 561 | `maximum` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 562 | `in_order` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 563 | `pre_order` | Y | Y |  |  |  | Y | Y |  | 72&#8209;74 |
| 564 | `size_link` |  |  |  | Y |  | Y | Y |  | 77&#8209;79 |
| 565 | `update` |  |  |  | Y |  | Y | Y |  | 81&#8209;83 |
| 566 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 85&#8209;99 |
| 567 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 101&#8209;115 |
| 568 | `insert_link` |  |  |  | Y |  | Y | Y |  | 117&#8209;138 |
| 569 | `find_link` |  |  |  | Y |  | Y | Y |  | 140&#8209;155 |
| 570 | `min_link` |  |  |  | Y |  | Y | Y |  | 157&#8209;167 |
| 571 | `max_link` |  |  |  | Y |  | Y | Y |  | 169&#8209;179 |
| 572 | `in_order_collect` |  |  |  | Y |  | Y | Y |  | 181&#8209;189 |
| 573 | `pre_order_collect` |  |  |  | Y |  | Y | Y |  | 191&#8209;199 |
| 574 | `height_rec` |  | Y |  |  |  | Y | Y |  | 209&#8209;214 |
| 575 | `default` |  | Y |  |  |  | Y | Y |  | 245 |

### Chap40/BSTKeyValueStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 576 | `new_node` |  |  |  | Y |  | Y | Y |  | 22&#8209;32 |
| 577 | `new` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 578 | `size` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 579 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 580 | `height` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 581 | `insert` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 582 | `find` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 583 | `contains` | Y | Y |  |  |  | Y | Y |  | 55&#8209;56 |
| 584 | `get` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 585 | `keys` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 586 | `values` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 587 | `minimum_key` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 588 | `maximum_key` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 589 | `rotate_left` |  |  |  | Y |  | Y | Y |  | 71&#8209;83 |
| 590 | `rotate_right` |  |  |  | Y |  | Y | Y |  | 85&#8209;97 |
| 591 | `insert_link` |  |  |  | Y |  | Y | Y |  | 99&#8209;124 |
| 592 | `find_link` |  |  |  | Y |  | Y | Y |  | 126&#8209;141 |
| 593 | `min_key_link` |  |  |  | Y |  | Y | Y |  | 143&#8209;153 |
| 594 | `max_key_link` |  |  |  | Y |  | Y | Y |  | 155&#8209;165 |
| 595 | `collect_keys` |  |  |  | Y |  | Y | Y |  | 167&#8209;175 |
| 596 | `collect_values` |  |  |  | Y |  | Y | Y |  | 177&#8209;185 |
| 597 | `height_rec` |  | Y |  |  |  | Y | Y |  | 195&#8209;200 |
| 598 | `default` |  | Y |  |  |  | Y | Y |  | 236 |

### Chap40/BSTReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 599 | `identity` x2 | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 600 | `combine` x2 | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 601 | `lift` x2 | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 602 | `new` | Y | Y | Y |  |  | Y | Y |  | 83&#8209;84 |
| 603 | `size` | Y | Y |  |  |  | Y | Y |  | 85&#8209;86 |
| 604 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 87&#8209;88 |
| 605 | `height` | Y | Y |  |  |  | Y | Y |  | 89&#8209;90 |
| 606 | `insert` | Y | Y |  |  |  | Y | Y |  | 91&#8209;92 |
| 607 | `find` | Y | Y |  |  |  | Y | Y |  | 93&#8209;94 |
| 608 | `contains` | Y | Y |  |  |  | Y | Y |  | 95&#8209;96 |
| 609 | `get` | Y | Y |  |  |  | Y | Y |  | 97&#8209;98 |
| 610 | `keys` | Y | Y |  |  |  | Y | Y |  | 99&#8209;100 |
| 611 | `values` | Y | Y |  |  |  | Y | Y |  | 101&#8209;102 |
| 612 | `minimum_key` | Y | Y |  |  |  | Y | Y |  | 103&#8209;105 |
| 613 | `maximum_key` | Y | Y |  |  |  | Y | Y |  | 106&#8209;108 |
| 614 | `reduced_value` | Y | Y |  |  |  | Y | Y |  | 109&#8209;111 |
| 615 | `range_reduce` | Y | Y |  |  |  | Y | Y |  | 112&#8209;114 |
| 616 | `default` |  | Y |  |  |  | Y | Y |  | 118 |
| 617 | `size_link` |  |  | Y |  |  | Y | Y |  | 122&#8209;124 |
| 618 | `reduced_value_link` |  |  | Y |  |  | Y | Y |  | 126&#8209;131 |
| 619 | `update_node` |  |  | Y |  |  | Y | Y |  | 133&#8209;144 |
| 620 | `make_node` |  |  | Y |  |  | Y | Y |  | 146&#8209;155 |
| 621 | `rotate_left` |  |  | Y |  |  | Y | Y |  | 157&#8209;171 |
| 622 | `rotate_right` |  |  | Y |  |  | Y | Y |  | 173&#8209;187 |
| 623 | `insert_link` |  |  | Y |  |  | Y | Y |  | 189&#8209;214 |
| 624 | `find_link` |  |  | Y |  |  | Y | Y |  | 216&#8209;231 |
| 625 | `min_key_link` |  |  | Y |  |  | Y | Y |  | 233&#8209;243 |
| 626 | `max_key_link` |  |  | Y |  |  | Y | Y |  | 245&#8209;255 |
| 627 | `collect_keys` |  |  | Y |  |  | Y | Y |  | 257&#8209;265 |
| 628 | `collect_values` |  |  | Y |  |  | Y | Y |  | 267&#8209;275 |
| 629 | `range_reduce_link` |  |  | Y |  |  | Y | Y |  | 277&#8209;303 |
| 630 | `height_rec` |  | Y |  |  |  | Y | Y |  | 321&#8209;326 |

### Chap40/BSTSizeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 631 | `new` | Y | Y | Y |  |  | Y | Y |  | 42&#8209;43 |
| 632 | `size` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 633 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 634 | `height` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 635 | `insert` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 636 | `find` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 637 | `contains` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 638 | `minimum` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 639 | `maximum` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 640 | `in_order` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 641 | `rank` | Y | Y |  |  |  | Y | Y |  | 62&#8209;63 |
| 642 | `select` | Y | Y |  |  |  | Y | Y |  | 64&#8209;66 |
| 643 | `split_rank` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 644 | `size_link` |  |  | Y |  |  | Y | Y |  | 73&#8209;75 |
| 645 | `update_size` |  |  | Y |  |  | Y | Y |  | 77&#8209;81 |
| 646 | `make_node` |  |  | Y |  |  | Y | Y |  | 83&#8209;91 |
| 647 | `rotate_left` |  |  | Y |  |  | Y | Y |  | 93&#8209;107 |
| 648 | `rotate_right` |  |  | Y |  |  | Y | Y |  | 109&#8209;123 |
| 649 | `insert_link` |  |  | Y |  |  | Y | Y |  | 125&#8209;146 |
| 650 | `find_link` |  |  | Y |  |  | Y | Y |  | 148&#8209;163 |
| 651 | `min_link` |  |  | Y |  |  | Y | Y |  | 165&#8209;175 |
| 652 | `max_link` |  |  | Y |  |  | Y | Y |  | 177&#8209;187 |
| 653 | `in_order_collect` |  |  | Y |  |  | Y | Y |  | 189&#8209;197 |
| 654 | `rank_link` |  |  | Y |  |  | Y | Y |  | 199&#8209;215 |
| 655 | `select_link` |  |  | Y |  |  | Y | Y |  | 217&#8209;233 |
| 656 | `split_rank_link` |  |  | Y |  |  | Y | Y |  | 235&#8209;253 |
| 657 | `height_rec` |  | Y |  |  |  | Y | Y |  | 264&#8209;269 |
| 658 | `default` |  | Y |  |  |  | Y | Y |  | 315 |

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 659 | `size` | Y | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 660 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 36&#8209;38 |
| 661 | `empty` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 662 | `singleton` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 663 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 664 | `filter` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 665 | `intersection` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 666 | `difference` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 667 | `union` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 668 | `find` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 669 | `delete` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 670 | `insert` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 671 | `parallel_filter` |  | Y |  |  |  | Y | Y |  | 114&#8209;140 |
| 672 | `parallel_intersect` |  | Y |  |  |  | Y | Y |  | 171&#8209;202 |
| 673 | `default` |  | Y |  |  |  | Y | Y |  | 266 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 674 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 675 | `cmp` |  | Y |  |  |  | Y | Y |  | 40&#8209;58 |
| 676 | `size` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 677 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 678 | `empty` | Y | Y |  |  |  | Y | Y |  | 68&#8209;70 |
| 679 | `singleton` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 680 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 74&#8209;75 |
| 681 | `filter` | Y | Y |  |  |  | Y | Y |  | 76&#8209;78 |
| 682 | `intersection` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 683 | `difference` | Y | Y |  |  |  | Y | Y |  | 82&#8209;84 |
| 684 | `union` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 685 | `find` | Y | Y |  |  |  | Y | Y |  | 88&#8209;90 |
| 686 | `delete` | Y | Y |  |  |  | Y | Y |  | 91&#8209;93 |
| 687 | `insert` | Y | Y |  |  |  | Y | Y |  | 94&#8209;96 |
| 688 | `parallel_sort` |  | Y |  |  |  | Y | Y |  | 119&#8209;149 |
| 689 | `default` |  | Y |  |  |  | Y | Y |  | 327 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 690 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 691 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 692 | `empty` | Y | Y |  |  |  | Y | Y |  | 27&#8209;29 |
| 693 | `singleton` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 694 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 695 | `filter` | Y | Y |  |  |  | Y | Y |  | 35&#8209;37 |
| 696 | `intersection` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 697 | `difference` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 698 | `union` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 699 | `find` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 700 | `delete` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 701 | `insert` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 702 | `default` |  | Y |  |  |  | Y | Y |  | 191 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 703 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 704 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 705 | `empty` | Y | Y |  |  |  | Y | Y |  | 27&#8209;29 |
| 706 | `singleton` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 707 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 708 | `filter` | Y | Y |  |  |  | Y | Y |  | 35&#8209;37 |
| 709 | `intersection` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 710 | `difference` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 711 | `union` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 712 | `find` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 713 | `delete` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 714 | `insert` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 715 | `default` |  | Y |  |  |  | Y | Y |  | 201 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 716 | `new` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 717 | `size` | Y | Y |  |  |  | Y | Y |  | 26&#8209;28 |
| 718 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 29&#8209;31 |
| 719 | `empty` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 720 | `singleton` | Y | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 721 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 722 | `filter` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 723 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 724 | `difference` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 725 | `union` | Y | Y |  |  |  | Y | Y |  | 48&#8209;50 |
| 726 | `find` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 727 | `delete` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 728 | `insert` | Y | Y |  |  |  | Y | Y |  | 57&#8209;59 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 729 | `size` | Y | Y |  |  |  | Y | Y |  | 17&#8209;18 |
| 730 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 19&#8209;20 |
| 731 | `empty` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 732 | `singleton` | Y | Y |  |  |  | Y | Y |  | 23&#8209;24 |
| 733 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 25&#8209;26 |
| 734 | `filter` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 735 | `intersection` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 736 | `difference` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 737 | `union` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 738 | `find` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 739 | `delete` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 740 | `insert` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 741 | `default` |  | Y |  |  |  | Y | Y |  | 176 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 742 | `example_41_1_array_set` | Y |  |  | Y |  | Y | Y |  | 15&#8209;17 |
| 743 | `example_41_1_avl_set` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 744 | `demonstrate_set_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 745 | `example_41_3_from_seq_demonstration` |  |  |  | Y |  | Y | Y |  | 128&#8209;172 |
| 746 | `additional_set_operations` |  |  |  | Y |  | Y | Y |  | 174&#8209;215 |

### Chap42/Example42_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 747 | `example_42_1` | Y |  |  | Y |  | Y | Y |  | 15&#8209;17 |
| 748 | `demonstrate_table_operations` | Y |  |  |  |  | Y | Y |  | 19&#8209;21 |
| 749 | `performance_comparison` |  |  |  | Y |  | Y | Y |  | 145&#8209;196 |

### Chap42/TableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 750 | `size` | Y | Y |  |  |  | Y | Y |  | 32&#8209;34 |
| 751 | `empty` | Y | Y |  |  |  | Y | Y |  | 35&#8209;37 |
| 752 | `singleton` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 753 | `domain` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 754 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 755 | `map` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 756 | `filter` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 757 | `intersection` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 758 | `union` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 759 | `difference` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 760 | `find` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 761 | `delete` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 762 | `insert` | Y | Y |  |  |  | Y | Y |  | 68&#8209;70 |
| 763 | `restrict` | Y | Y |  |  |  | Y | Y |  | 71&#8209;73 |
| 764 | `subtract` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 765 | `collect` | Y | Y |  |  |  | Y | Y |  | 78&#8209;80 |
| 766 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 669&#8209;674 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 767 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 768 | `empty` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 769 | `singleton` | Y | Y |  |  |  | Y | Y |  | 27&#8209;29 |
| 770 | `domain` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 771 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 33&#8209;35 |
| 772 | `map` | Y | Y |  |  |  | Y | Y |  | 36&#8209;38 |
| 773 | `filter` | Y | Y |  |  |  | Y | Y |  | 39&#8209;41 |
| 774 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 775 | `union` | Y | Y |  |  |  | Y | Y |  | 45&#8209;47 |
| 776 | `difference` | Y | Y |  |  |  | Y | Y |  | 48&#8209;50 |
| 777 | `find` | Y | Y |  |  |  | Y | Y |  | 51&#8209;53 |
| 778 | `delete` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 779 | `insert` | Y | Y |  |  |  | Y | Y |  | 57&#8209;59 |
| 780 | `restrict` | Y | Y |  |  |  | Y | Y |  | 60&#8209;62 |
| 781 | `subtract` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 782 | `collect` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 783 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 330&#8209;335 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 784 | `size` | Y | Y |  |  |  | Y | Y |  | 22&#8209;24 |
| 785 | `empty` | Y | Y |  |  |  | Y | Y |  | 26&#8209;28 |
| 786 | `singleton` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 787 | `domain` | Y | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 788 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 789 | `map` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 790 | `filter` | Y | Y |  |  |  | Y | Y |  | 46&#8209;48 |
| 791 | `intersection` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 792 | `union` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 793 | `difference` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 794 | `find` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 795 | `delete` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 796 | `insert` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 797 | `restrict` | Y | Y |  |  |  | Y | Y |  | 74&#8209;76 |
| 798 | `subtract` | Y | Y |  |  |  | Y | Y |  | 78&#8209;80 |
| 799 | `collect` | Y | Y |  |  |  | Y | Y |  | 82&#8209;84 |
| 800 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 362&#8209;367 |

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 801 | `size` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 802 | `empty` | Y | Y |  |  |  | Y | Y |  | 34 |
| 803 | `singleton` | Y | Y |  |  |  | Y | Y |  | 35 |
| 804 | `find` | Y | Y |  |  |  | Y | Y |  | 36 |
| 805 | `lookup` | Y | Y |  |  |  | Y | Y |  | 37 |
| 806 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 38 |
| 807 | `insert` | Y | Y |  |  |  | Y | Y |  | 39 |
| 808 | `delete` | Y | Y |  |  |  | Y | Y |  | 40 |
| 809 | `domain` | Y | Y |  |  |  | Y | Y |  | 41 |
| 810 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 42&#8209;47 |
| 811 | `map` | Y | Y |  |  |  | Y | Y |  | 48 |
| 812 | `filter` | Y | Y |  |  |  | Y | Y |  | 49 |
| 813 | `intersection` | Y | Y |  |  |  | Y | Y |  | 50 |
| 814 | `union` | Y | Y |  |  |  | Y | Y |  | 51 |
| 815 | `difference` | Y | Y |  |  |  | Y | Y |  | 52 |
| 816 | `restrict` | Y | Y |  |  |  | Y | Y |  | 53 |
| 817 | `subtract` | Y | Y |  |  |  | Y | Y |  | 54 |
| 818 | `reduce` | Y | Y |  |  |  | Y | Y |  | 55 |
| 819 | `collect` | Y | Y |  |  |  | Y | Y |  | 56 |
| 820 | `first_key` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 821 | `last_key` | Y | Y |  |  |  | Y | Y |  | 60 |
| 822 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 61 |
| 823 | `next_key` | Y | Y |  |  |  | Y | Y |  | 62 |
| 824 | `split_key` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 825 | `join_key` | Y | Y |  |  |  | Y | Y |  | 66 |
| 826 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 67 |
| 827 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 68 |
| 828 | `select_key` | Y | Y |  |  |  | Y | Y |  | 69 |
| 829 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 830 | `reduce_val` | Y | Y |  |  |  | Y | Y |  | 74&#8209;77 |
| 831 | `reduce_range` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 832 | `reduce_range_parallel` | Y | Y |  |  |  | Y | Y |  | 83&#8209;85 |
| 833 | `recalculate_reduction` |  |  |  | Y |  | Y | Y |  | 372&#8209;374 |
| 834 | `calculate_reduction` |  |  |  | Y |  | Y | Y |  | 376&#8209;400 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 835 | `size` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 836 | `empty` | Y | Y |  |  |  | Y | Y |  | 35 |
| 837 | `singleton` | Y | Y |  |  |  | Y | Y |  | 36 |
| 838 | `find` | Y | Y |  |  |  | Y | Y |  | 37 |
| 839 | `lookup` | Y | Y |  |  |  | Y | Y |  | 38 |
| 840 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 39 |
| 841 | `insert` | Y | Y |  |  |  | Y | Y |  | 40 |
| 842 | `delete` | Y | Y |  |  |  | Y | Y |  | 41 |
| 843 | `domain` | Y | Y |  |  |  | Y | Y |  | 42 |
| 844 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 43 |
| 845 | `map` | Y | Y |  |  |  | Y | Y |  | 44 |
| 846 | `filter` | Y | Y |  |  |  | Y | Y |  | 45 |
| 847 | `reduce` | Y | Y |  |  |  | Y | Y |  | 46 |
| 848 | `intersection` | Y | Y |  |  |  | Y | Y |  | 47 |
| 849 | `union` | Y | Y |  |  |  | Y | Y |  | 48 |
| 850 | `difference` | Y | Y |  |  |  | Y | Y |  | 49 |
| 851 | `restrict` | Y | Y |  |  |  | Y | Y |  | 50 |
| 852 | `subtract` | Y | Y |  |  |  | Y | Y |  | 51 |
| 853 | `collect` | Y | Y |  |  |  | Y | Y |  | 52 |
| 854 | `first_key` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 855 | `last_key` | Y | Y |  |  |  | Y | Y |  | 56 |
| 856 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 57 |
| 857 | `next_key` | Y | Y |  |  |  | Y | Y |  | 58 |
| 858 | `split_key` | Y | Y |  |  |  | Y | Y |  | 59&#8209;61 |
| 859 | `join_key` | Y | Y |  |  |  | Y | Y |  | 62 |
| 860 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 63 |
| 861 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 64 |
| 862 | `select_key` | Y | Y |  |  |  | Y | Y |  | 65 |
| 863 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 864 | `reduce_val` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 865 | `reduce_range` | Y | Y |  |  |  | Y | Y |  | 75&#8209;77 |
| 866 | `recalculate_reduction` |  |  | Y |  |  | Y | Y |  | 328&#8209;330 |
| 867 | `calculate_reduction` |  |  | Y |  |  | Y | Y |  | 332&#8209;352 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 868 | `size` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 869 | `empty` | Y | Y |  |  |  | Y | Y |  | 35 |
| 870 | `singleton` | Y | Y |  |  |  | Y | Y |  | 36 |
| 871 | `find` | Y | Y |  |  |  | Y | Y |  | 37 |
| 872 | `insert` | Y | Y |  |  |  | Y | Y |  | 38 |
| 873 | `delete` | Y | Y |  |  |  | Y | Y |  | 39 |
| 874 | `domain` | Y | Y |  |  |  | Y | Y |  | 40 |
| 875 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 41 |
| 876 | `map` | Y | Y |  |  |  | Y | Y |  | 42 |
| 877 | `filter` | Y | Y |  |  |  | Y | Y |  | 43 |
| 878 | `intersection` | Y | Y |  |  |  | Y | Y |  | 44 |
| 879 | `union` | Y | Y |  |  |  | Y | Y |  | 45 |
| 880 | `difference` | Y | Y |  |  |  | Y | Y |  | 46 |
| 881 | `restrict` | Y | Y |  |  |  | Y | Y |  | 47 |
| 882 | `subtract` | Y | Y |  |  |  | Y | Y |  | 48 |
| 883 | `collect` | Y | Y |  |  |  | Y | Y |  | 49 |
| 884 | `first_key` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 885 | `last_key` | Y | Y |  |  |  | Y | Y |  | 53 |
| 886 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 54 |
| 887 | `next_key` | Y | Y |  |  |  | Y | Y |  | 55 |
| 888 | `split_key` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 889 | `join_key` | Y | Y |  |  |  | Y | Y |  | 59 |
| 890 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 60 |
| 891 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 61 |
| 892 | `select_key` | Y | Y |  |  |  | Y | Y |  | 62 |
| 893 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 63&#8209;65 |
| 894 | `reduce_val` | Y | Y |  |  |  | Y | Y |  | 67&#8209;70 |
| 895 | `reduce_range` | Y | Y |  |  |  | Y | Y |  | 72&#8209;74 |
| 896 | `recalculate_reduction` |  |  | Y |  |  | Y | Y |  | 363&#8209;365 |
| 897 | `calculate_reduction` |  |  | Y |  |  | Y | Y |  | 367&#8209;387 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 898 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 13&#8209;15 |
| 899 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 17&#8209;19 |
| 900 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 168&#8209;224 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 901 | `size` | Y | Y |  |  |  | Y | Y |  | 18&#8209;20 |
| 902 | `empty` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 903 | `singleton` | Y | Y |  |  |  | Y | Y |  | 23&#8209;24 |
| 904 | `find` | Y | Y |  |  |  | Y | Y |  | 25&#8209;26 |
| 905 | `insert` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 906 | `delete` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 907 | `filter` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 908 | `intersection` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 909 | `union` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 910 | `difference` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 911 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 912 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 913 | `first` | Y | Y |  |  |  | Y | Y |  | 44&#8209;46 |
| 914 | `last` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 915 | `previous` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 916 | `next` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 917 | `split` | Y | Y |  |  |  | Y | Y |  | 53&#8209;56 |
| 918 | `join` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 919 | `get_range` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 920 | `rank` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 921 | `select` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 922 | `split_rank` | Y | Y |  |  |  | Y | Y |  | 65&#8209;68 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 923 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 924 | `empty` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 925 | `singleton` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 926 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 927 | `insert` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 928 | `delete` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 929 | `filter` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 930 | `intersection` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 931 | `union` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 932 | `difference` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 933 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 934 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 935 | `first` | Y | Y |  |  |  | Y | Y |  | 47&#8209;49 |
| 936 | `last` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 937 | `previous` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 938 | `next` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 939 | `split` | Y | Y |  |  |  | Y | Y |  | 56&#8209;59 |
| 940 | `join` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 941 | `get_range` | Y | Y |  |  |  | Y | Y |  | 62&#8209;63 |
| 942 | `rank` | Y | Y |  |  |  | Y | Y |  | 64&#8209;65 |
| 943 | `select` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 944 | `split_rank` | Y | Y |  |  |  | Y | Y |  | 68&#8209;71 |
| 945 | `from_sorted_elements` |  |  |  | Y |  | Y | Y |  | 326&#8209;329 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 946 | `size` | Y | Y |  |  |  | Y | Y |  | 20&#8209;22 |
| 947 | `empty` | Y | Y |  |  |  | Y | Y |  | 23&#8209;24 |
| 948 | `singleton` | Y | Y |  |  |  | Y | Y |  | 25&#8209;26 |
| 949 | `find` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 950 | `insert` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 951 | `delete` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 952 | `filter` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 953 | `intersection` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 954 | `union` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 955 | `difference` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 956 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 957 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 958 | `first` | Y | Y |  |  |  | Y | Y |  | 46&#8209;48 |
| 959 | `last` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 960 | `previous` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 961 | `next` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 962 | `split` | Y | Y |  |  |  | Y | Y |  | 55&#8209;58 |
| 963 | `join` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 964 | `get_range` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 965 | `rank` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 966 | `select` | Y | Y |  |  |  | Y | Y |  | 65&#8209;66 |
| 967 | `split_rank` | Y | Y |  |  |  | Y | Y |  | 67&#8209;70 |
| 968 | `from_sorted_elements` |  |  |  | Y |  | Y | Y |  | 304&#8209;307 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 969 | `size` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 970 | `empty` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 971 | `singleton` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 972 | `find` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 973 | `lookup` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 974 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 34&#8209;36 |
| 975 | `insert` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 976 | `delete` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 977 | `domain` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 978 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 979 | `map` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 980 | `filter` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 981 | `intersection` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 982 | `union` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 983 | `difference` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 984 | `restrict` | Y | Y |  |  |  | Y | Y |  | 55&#8209;56 |
| 985 | `subtract` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 986 | `reduce` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 987 | `collect` | Y | Y |  |  |  | Y | Y |  | 61&#8209;62 |
| 988 | `first_key` | Y | Y |  |  |  | Y | Y |  | 64&#8209;66 |
| 989 | `last_key` | Y | Y |  |  |  | Y | Y |  | 67&#8209;68 |
| 990 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 69&#8209;70 |
| 991 | `next_key` | Y | Y |  |  |  | Y | Y |  | 71&#8209;72 |
| 992 | `split_key` | Y | Y |  |  |  | Y | Y |  | 73&#8209;76 |
| 993 | `join_key` | Y | Y |  |  |  | Y | Y |  | 77 |
| 994 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 78 |
| 995 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 79 |
| 996 | `select_key` | Y | Y |  |  |  | Y | Y |  | 80 |
| 997 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 81&#8209;83 |
| 998 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 394&#8209;403 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 999 | `size` | Y | Y |  |  |  | Y | Y |  | 22&#8209;23 |
| 1000 | `empty` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 1001 | `singleton` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 1002 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1003 | `insert` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 1004 | `delete` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1005 | `domain` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 1006 | `map` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 1007 | `filter` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1008 | `default` |  | Y |  |  |  | Y | Y |  | 127 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1009 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 1010 | `empty` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 1011 | `singleton` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 1012 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1013 | `lookup` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 1014 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 1015 | `insert` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 1016 | `delete` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 1017 | `domain` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1018 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 1019 | `map` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 1020 | `filter` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1021 | `reduce` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 1022 | `intersection` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1023 | `union` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1024 | `difference` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 1025 | `restrict` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 1026 | `subtract` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 1027 | `collect` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 1028 | `first_key` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 1029 | `last_key` | Y | Y |  |  |  | Y | Y |  | 64&#8209;65 |
| 1030 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 66&#8209;67 |
| 1031 | `next_key` | Y | Y |  |  |  | Y | Y |  | 68&#8209;69 |
| 1032 | `split_key` | Y | Y |  |  |  | Y | Y |  | 70&#8209;73 |
| 1033 | `join_key` | Y | Y |  |  |  | Y | Y |  | 74&#8209;75 |
| 1034 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 76 |
| 1035 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 77 |
| 1036 | `select_key` | Y | Y |  |  |  | Y | Y |  | 78 |
| 1037 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 79&#8209;81 |
| 1038 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 366&#8209;376 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1039 | `size` | Y | Y |  |  |  | Y | Y |  | 21&#8209;23 |
| 1040 | `empty` | Y | Y |  |  |  | Y | Y |  | 24&#8209;25 |
| 1041 | `singleton` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 1042 | `find` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1043 | `insert` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 1044 | `delete` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1045 | `domain` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 1046 | `tabulate` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 1047 | `map` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1048 | `filter` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 1049 | `intersection` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 1050 | `union` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1051 | `difference` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 1052 | `restrict` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1053 | `subtract` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1054 | `collect` | Y | Y |  |  |  | Y | Y |  | 52&#8209;53 |
| 1055 | `first_key` | Y | Y |  |  |  | Y | Y |  | 55&#8209;57 |
| 1056 | `last_key` | Y | Y |  |  |  | Y | Y |  | 58&#8209;59 |
| 1057 | `previous_key` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 1058 | `next_key` | Y | Y |  |  |  | Y | Y |  | 62&#8209;63 |
| 1059 | `split_key` | Y | Y |  |  |  | Y | Y |  | 64&#8209;67 |
| 1060 | `join_key` | Y | Y |  |  |  | Y | Y |  | 68&#8209;69 |
| 1061 | `get_key_range` | Y | Y |  |  |  | Y | Y |  | 70&#8209;71 |
| 1062 | `rank_key` | Y | Y |  |  |  | Y | Y |  | 72&#8209;73 |
| 1063 | `select_key` | Y | Y |  |  |  | Y | Y |  | 74&#8209;75 |
| 1064 | `split_rank_key` | Y | Y |  |  |  | Y | Y |  | 76&#8209;78 |
| 1065 | `from_sorted_entries` |  |  |  | Y |  | Y | Y |  | 350&#8209;359 |

### Chap44/DocumentIndex.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1066 | `make_index` | Y | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 1067 | `find` x3 | Y | Y |  |  |  | Y | Y |  | 288&#8209;290 |
| 1068 | `query_and` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1069 | `query_or` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1070 | `query_and_not` | Y | Y |  |  |  | Y | Y |  | 46&#8209;48 |
| 1071 | `size` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 1072 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 1073 | `empty` | Y | Y |  |  |  | Y | Y |  | 58&#8209;60 |
| 1074 | `get_all_words` | Y | Y |  |  |  | Y | Y |  | 62&#8209;64 |
| 1075 | `word_count` | Y | Y |  |  |  | Y | Y |  | 66&#8209;68 |
| 1076 | `tokens` |  |  |  | Y |  | Y | Y |  | 200&#8209;227 |
| 1077 | `create_finder` |  |  |  | Y |  | Y | Y |  | 229&#8209;234 |
| 1078 | `new` | Y | Y |  |  |  | Y | Y |  | 284&#8209;286 |
| 1079 | `and` | Y | Y |  |  |  | Y | Y |  | 292&#8209;294 |
| 1080 | `or` | Y | Y |  |  |  | Y | Y |  | 296&#8209;298 |
| 1081 | `and_not` | Y | Y |  |  |  | Y | Y |  | 300&#8209;302 |
| 1082 | `complex_query` | Y | Y |  |  |  | Y | Y |  | 304&#8209;306 |

### Chap44/Example44_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1083 | `create_tweet_collection` |  |  |  | Y |  | Y | Y |  | 11&#8209;21 |
| 1084 | `create_tweet_index` |  |  |  | Y |  | Y | Y |  | 23&#8209;29 |
| 1085 | `create_tweet_finder` |  |  |  | Y |  | Y | Y |  | 31&#8209;38 |
| 1086 | `default` |  | Y |  |  |  | Y | Y |  | 47 |
| 1087 | `new` |  |  | Y |  |  | Y | Y |  | 51&#8209;59 |
| 1088 | `search_fun` |  |  | Y |  |  | Y | Y |  | 61&#8209;64 |
| 1089 | `search_club` |  |  | Y |  |  | Y | Y |  | 66&#8209;69 |
| 1090 | `search_food` |  |  | Y |  |  | Y | Y |  | 71&#8209;74 |
| 1091 | `search_chess` |  |  | Y |  |  | Y | Y |  | 76&#8209;79 |
| 1092 | `complex_query_fun_and_food_or_chess` |  |  | Y |  |  | Y | Y |  | 81&#8209;95 |
| 1093 | `count_fun_but_not_chess` |  |  | Y |  |  | Y | Y |  | 97&#8209;108 |
| 1094 | `search_food_or_fun` |  |  | Y |  |  | Y | Y |  | 110&#8209;118 |
| 1095 | `search_party_and_food` |  |  | Y |  |  | Y | Y |  | 120&#8209;128 |
| 1096 | `get_all_words` |  |  | Y |  |  | Y | Y |  | 130&#8209;133 |
| 1097 | `get_word_count` |  |  | Y |  |  | Y | Y |  | 135&#8209;138 |
| 1098 | `query_builder_example` |  |  | Y |  |  | Y | Y |  | 140&#8209;153 |
| 1099 | `doc_set_to_sorted_vec` |  |  |  | Y |  | Y | Y |  | 156&#8209;169 |
| 1100 | `verify_textbook_examples` |  |  |  | Y |  | Y | Y |  | 171&#8209;211 |
| 1101 | `performance_comparison_demo` |  |  |  | Y |  | Y | Y |  | 213&#8209;227 |
| 1102 | `tokenization_demo` |  |  |  | Y |  | Y | Y |  | 229&#8209;235 |
| 1103 | `index_statistics` |  |  |  | Y |  | Y | Y |  | 237&#8209;256 |

### Chap45/BalancedTreePQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1104 | `empty` | Y | Y |  |  |  | Y | Y |  | 18&#8209;19 |
| 1105 | `singleton` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 1106 | `find_min` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 1107 | `insert` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 1108 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 32&#8209;36 |
| 1109 | `meld` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1110 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1111 | `size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 1112 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47 |
| 1113 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 48 |
| 1114 | `find_max` | Y | Y |  |  |  | Y | Y |  | 49 |
| 1115 | `delete_max` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 1116 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 53 |
| 1117 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 54 |
| 1118 | `contains` | Y | Y |  |  |  | Y | Y |  | 55 |
| 1119 | `remove` | Y | Y |  |  |  | Y | Y |  | 56&#8209;58 |
| 1120 | `range` | Y | Y |  |  |  | Y | Y |  | 59 |
| 1121 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 60 |
| 1122 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 61 |
| 1123 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 62 |
| 1124 | `default` |  | Y |  |  |  | Y | Y |  | 306 |
| 1125 | `is_sorted` |  |  | Y |  |  | Y | Y |  | 339&#8209;349 |
| 1126 | `height` |  |  | Y |  |  | Y | Y |  | 350&#8209;359 |
| 1127 | `split` |  |  | Y |  |  | Y | Y |  | 364&#8209;384 |
| 1128 | `join` |  |  | Y |  |  | Y | Y |  | 386&#8209;387 |
| 1129 | `filter` |  |  | Y |  |  | Y | Y |  | 389&#8209;404 |
| 1130 | `map` |  |  | Y |  |  | Y | Y |  | 406&#8209;421 |

### Chap45/BinaryHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1131 | `empty` | Y | Y |  |  |  | Y | Y |  | 18&#8209;19 |
| 1132 | `singleton` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 1133 | `find_min` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 1134 | `insert` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 1135 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 32&#8209;36 |
| 1136 | `meld` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1137 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1138 | `size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 1139 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47 |
| 1140 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 48 |
| 1141 | `left_child` |  |  | Y |  |  | Y | Y |  | 52 |
| 1142 | `right_child` |  |  | Y |  |  | Y | Y |  | 53 |
| 1143 | `parent` |  |  | Y |  |  | Y | Y |  | 54 |
| 1144 | `is_heap` |  |  | Y |  |  | Y | Y |  | 56&#8209;70 |
| 1145 | `bubble_up` |  |  | Y |  |  | Y | Y |  | 72&#8209;91 |
| 1146 | `bubble_down` |  |  | Y |  |  | Y | Y |  | 93&#8209;121 |
| 1147 | `swap_elements` |  |  | Y |  |  | Y | Y |  | 123&#8209;141 |
| 1148 | `heapify` |  |  | Y |  |  | Y | Y |  | 143&#8209;159 |
| 1149 | `insert_all` |  |  | Y |  |  | Y | Y |  | 267&#8209;275 |
| 1150 | `extract_all_sorted` |  |  | Y |  |  | Y | Y |  | 277&#8209;292 |
| 1151 | `is_valid_heap` |  |  | Y |  |  | Y | Y |  | 294&#8209;295 |
| 1152 | `height` |  |  | Y |  |  | Y | Y |  | 297&#8209;304 |
| 1153 | `level_elements` |  |  | Y |  |  | Y | Y |  | 306&#8209;321 |
| 1154 | `default` |  | Y |  |  |  | Y | Y |  | 325 |
| 1155 | `from_vec` |  |  | Y |  |  | Y | Y |  | 358&#8209;366 |
| 1156 | `to_vec` |  |  | Y |  |  | Y | Y |  | 368&#8209;375 |
| 1157 | `to_sorted_vec` |  |  | Y |  |  | Y | Y |  | 377&#8209;385 |

### Chap45/Example45_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1158 | `example_45_2_textbook_example` | Y |  |  | Y |  | Y | Y |  | 12&#8209;14 |
| 1159 | `example_45_2_reverse_sorted` | Y |  |  | Y |  | Y | Y |  | 16&#8209;18 |
| 1160 | `example_45_2_already_sorted` | Y |  |  | Y |  | Y | Y |  | 20&#8209;22 |
| 1161 | `example_45_2_duplicates` | Y |  |  | Y |  | Y | Y |  | 24&#8209;26 |
| 1162 | `example_45_2_single_element` | Y |  |  | Y |  | Y | Y |  | 28&#8209;30 |
| 1163 | `example_45_2_empty` | Y |  |  | Y |  | Y | Y |  | 32&#8209;34 |
| 1164 | `example_45_2_efficiency_demonstration` | Y |  |  | Y |  | Y | Y |  | 36&#8209;38 |
| 1165 | `run_example_45_2` | Y |  |  | Y |  | Y | Y |  | 40&#8209;42 |

### Chap45/HeapsortExample.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1166 | `heapsort_unsorted_list` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 1167 | `heapsort_sorted_list` | Y |  |  | Y |  | Y | Y |  | 23&#8209;25 |
| 1168 | `heapsort_balanced_tree` | Y |  |  | Y |  | Y | Y |  | 27&#8209;29 |
| 1169 | `heapsort_binary_heap` | Y |  |  | Y |  | Y | Y |  | 31&#8209;33 |
| 1170 | `heapsort_leftist_heap` | Y |  |  | Y |  | Y | Y |  | 35&#8209;37 |
| 1171 | `compare_all_heapsorts` | Y |  |  | Y |  | Y | Y |  | 39&#8209;41 |
| 1172 | `textbook_example` | Y |  |  | Y |  | Y | Y |  | 158&#8209;159 |
| 1173 | `reverse_sorted_example` | Y |  |  | Y |  | Y | Y |  | 160&#8209;161 |
| 1174 | `already_sorted_example` | Y |  |  | Y |  | Y | Y |  | 162&#8209;163 |
| 1175 | `duplicates_example` | Y |  |  | Y |  | Y | Y |  | 164&#8209;165 |
| 1176 | `single_element_example` | Y |  |  | Y |  | Y | Y |  | 166&#8209;167 |
| 1177 | `empty_example` | Y |  |  | Y |  | Y | Y |  | 168&#8209;169 |
| 1178 | `large_example` | Y |  |  | Y |  | Y | Y |  | 170&#8209;171 |
| 1179 | `efficiency_demonstration` | Y |  |  | Y |  | Y | Y |  | 172&#8209;173 |
| 1180 | `complexity_analysis` | Y |  |  | Y |  | Y | Y |  | 177&#8209;178 |
| 1181 | `correctness_verification` | Y |  |  | Y |  | Y | Y |  | 179&#8209;180 |
| 1182 | `vec_to_array_seq` | Y |  |  | Y |  | Y | Y |  | 184&#8209;185 |
| 1183 | `vec_to_avl_seq` | Y |  |  | Y |  | Y | Y |  | 186&#8209;187 |
| 1184 | `is_sorted` | Y |  | Y | Y |  | Y | Y |  | 188&#8209;189 |
| 1185 | `generate_test_sequences` | Y |  |  | Y |  | Y | Y |  | 190&#8209;191 |
| 1186 | `all_results_match` |  |  | Y |  |  | Y | Y |  | 195&#8209;202 |
| 1187 | `all_results_sorted` |  |  | Y |  |  | Y | Y |  | 204&#8209;213 |

### Chap45/LeftistHeapPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1188 | `empty` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 1189 | `singleton` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 1190 | `find_min` | Y | Y |  |  |  | Y | Y |  | 36&#8209;38 |
| 1191 | `insert` | Y | Y |  |  |  | Y | Y |  | 40&#8209;42 |
| 1192 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 44&#8209;48 |
| 1193 | `meld` | Y | Y |  |  |  | Y | Y |  | 50&#8209;52 |
| 1194 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 54&#8209;56 |
| 1195 | `size` | Y | Y | Y |  |  | Y | Y |  | 58 |
| 1196 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 59 |
| 1197 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 60 |
| 1198 | `height` | Y | Y | Y |  |  | Y | Y |  | 61 |
| 1199 | `root_rank` | Y | Y |  |  |  | Y | Y |  | 62 |
| 1200 | `is_valid_leftist_heap` | Y | Y |  |  |  | Y | Y |  | 63 |
| 1201 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 64 |
| 1202 | `to_vec` | Y | Y | Y |  |  | Y | Y |  | 65 |
| 1203 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 66 |
| 1204 | `meld_multiple` | Y | Y |  |  |  | Y | Y |  | 67&#8209;69 |
| 1205 | `split` | Y | Y |  |  |  | Y | Y |  | 70&#8209;72 |
| 1206 | `efficient_multi_way_merge` | Y |  |  | Y |  | Y | Y |  | 76&#8209;78 |
| 1207 | `parallel_heap_construction` | Y |  |  | Y |  | Y | Y |  | 79&#8209;80 |
| 1208 | `rank` |  |  | Y |  |  | Y | Y |  | 84&#8209;90 |
| 1209 | `make_node` |  |  | Y |  |  | Y | Y |  | 92&#8209;112 |
| 1210 | `meld_nodes` |  |  | Y |  |  | Y | Y |  | 114&#8209;163 |
| 1211 | `is_leftist` |  |  | Y |  |  | Y | Y |  | 181&#8209;189 |
| 1212 | `is_heap` |  |  | Y |  |  | Y | Y |  | 191&#8209;207 |
| 1213 | `default` |  | Y |  |  |  | Y | Y |  | 377 |
| 1214 | `format_node` |  | Y |  |  |  | Y | Y |  | 382&#8209;393 |

### Chap45/SortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1215 | `empty` | Y | Y |  |  |  | Y | Y |  | 18&#8209;19 |
| 1216 | `singleton` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 1217 | `find_min` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 1218 | `insert` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 1219 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 32&#8209;36 |
| 1220 | `meld` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1221 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1222 | `size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 1223 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47 |
| 1224 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 48 |
| 1225 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 49 |
| 1226 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 50 |
| 1227 | `find_max` | Y | Y |  |  |  | Y | Y |  | 51 |
| 1228 | `delete_max` | Y | Y |  |  |  | Y | Y |  | 52&#8209;54 |
| 1229 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 55 |
| 1230 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 56 |
| 1231 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 57 |
| 1232 | `is_sorted` | Y | Y |  |  |  | Y | Y |  | 58 |
| 1233 | `default` |  | Y |  |  |  | Y | Y |  | 285 |

### Chap45/UnsortedListPQ.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1234 | `empty` | Y | Y |  |  |  | Y | Y |  | 18&#8209;19 |
| 1235 | `singleton` | Y | Y |  |  |  | Y | Y |  | 21&#8209;22 |
| 1236 | `find_min` | Y | Y |  |  |  | Y | Y |  | 24&#8209;26 |
| 1237 | `insert` | Y | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 1238 | `delete_min` | Y | Y |  |  |  | Y | Y |  | 32&#8209;36 |
| 1239 | `meld` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1240 | `from_seq` | Y | Y |  |  |  | Y | Y |  | 42&#8209;44 |
| 1241 | `size` | Y | Y |  |  |  | Y | Y |  | 46 |
| 1242 | `is_empty` | Y | Y |  |  |  | Y | Y |  | 47 |
| 1243 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 48 |
| 1244 | `insert_all` | Y | Y |  |  |  | Y | Y |  | 49 |
| 1245 | `extract_all_sorted` | Y | Y |  |  |  | Y | Y |  | 50 |
| 1246 | `from_vec` | Y | Y |  |  |  | Y | Y |  | 51 |
| 1247 | `to_vec` | Y | Y |  |  |  | Y | Y |  | 52 |
| 1248 | `to_sorted_vec` | Y | Y |  |  |  | Y | Y |  | 53 |
| 1249 | `default` |  | Y |  |  |  | Y | Y |  | 209 |

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1250 | `hash_index` | Y |  |  |  |  | Y | Y |  | 26&#8209;29 |
| 1251 | `insert_chained` | Y |  |  |  |  | Y | Y |  | 31&#8209;39 |
| 1252 | `lookup_chained` | Y |  |  |  |  | Y | Y |  | 41&#8209;51 |
| 1253 | `delete_chained` | Y |  |  |  |  | Y | Y |  | 53&#8209;63 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1254 | `second_hash` |  |  | Y |  |  | Y | Y |  | 18&#8209;56 |
| 1255 | `insert` |  | Y |  |  |  | Y | Y |  | 62&#8209;79 |
| 1256 | `lookup` |  | Y |  |  |  | Y | Y |  | 81&#8209;96 |
| 1257 | `delete` |  | Y |  |  |  | Y | Y |  | 98&#8209;117 |
| 1258 | `resize` |  | Y |  |  |  | Y | Y |  | 119&#8209;153 |
| 1259 | `probe` |  | Y |  |  |  | Y | Y |  | 159&#8209;167 |
| 1260 | `find_slot` |  | Y |  |  |  | Y | Y |  | 169&#8209;182 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1261 | `new` |  | Y |  |  |  | Y | Y |  | 23&#8209;25 |
| 1262 | `insert` |  | Y |  |  |  | Y | Y |  | 26&#8209;28 |
| 1263 | `lookup` |  | Y |  |  |  | Y | Y |  | 30&#8209;37 |
| 1264 | `delete` |  | Y |  |  |  | Y | Y |  | 39&#8209;49 |
| 1265 | `probe` | Y |  |  |  |  | Y | Y |  | 58&#8209;61 |
| 1266 | `find_slot` | Y |  |  |  |  | Y | Y |  | 63&#8209;66 |
| 1267 | `insert_with_probe` | Y |  |  |  |  | Y | Y |  | 68&#8209;76 |
| 1268 | `lookup_with_probe` | Y |  |  |  |  | Y | Y |  | 78&#8209;91 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1269 | `insert` |  | Y |  |  |  | Y | Y |  | 18&#8209;38 |
| 1270 | `lookup` |  | Y |  |  |  | Y | Y |  | 40&#8209;56 |
| 1271 | `delete` |  | Y |  |  |  | Y | Y |  | 58&#8209;79 |
| 1272 | `resize` |  | Y |  |  |  | Y | Y |  | 81&#8209;115 |
| 1273 | `probe` |  | Y |  |  |  | Y | Y |  | 121&#8209;128 |
| 1274 | `find_slot` |  | Y |  |  |  | Y | Y |  | 130&#8209;145 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1275 | `new` |  | Y |  |  |  | Y | Y |  | 15&#8209;17 |
| 1276 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 19&#8209;30 |
| 1277 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 32&#8209;41 |
| 1278 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 43&#8209;61 |
| 1279 | `resize` |  | Y |  |  |  | Y | Y |  | 88&#8209;122 |
| 1280 | `hash_index` |  | Y |  |  |  | Y | Y |  | 128&#8209;134 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1281 | `new` | Y |  |  |  |  | Y | Y |  | 44&#8209;46 |
| 1282 | `insert` x2 | Y |  |  |  |  | Y | Y |  | 79&#8209;82 |
| 1283 | `lookup` x2 | Y |  |  |  |  | Y | Y |  | 84&#8209;87 |
| 1284 | `delete` x2 | Y |  |  |  |  | Y | Y |  | 89&#8209;92 |
| 1285 | `createTable` | Y |  |  |  |  | Y | Y |  | 60&#8209;77 |
| 1286 | `metrics` | Y |  |  |  |  | Y | Y |  | 94&#8209;97 |
| 1287 | `loadAndSize` | Y |  |  |  |  | Y | Y |  | 99&#8209;113 |
| 1288 | `resize` | Y |  |  |  |  | Y | Y |  | 115&#8209;120 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1289 | `insert` |  | Y |  |  |  | Y | Y |  | 20&#8209;39 |
| 1290 | `lookup` |  | Y |  |  |  | Y | Y |  | 41&#8209;57 |
| 1291 | `delete` |  | Y |  |  |  | Y | Y |  | 59&#8209;79 |
| 1292 | `resize` |  | Y |  |  |  | Y | Y |  | 81&#8209;115 |
| 1293 | `probe` |  | Y |  |  |  | Y | Y |  | 121&#8209;128 |
| 1294 | `find_slot` |  | Y |  |  |  | Y | Y |  | 130&#8209;144 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1295 | `new` |  | Y |  |  |  | Y | Y |  | 28&#8209;30 |
| 1296 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 32&#8209;51 |
| 1297 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 53&#8209;64 |
| 1298 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 66&#8209;82 |
| 1299 | `default` |  | Y |  |  |  | Y | Y |  | 86&#8209;88 |
| 1300 | `resize` |  | Y |  |  |  | Y | Y |  | 115&#8209;151 |
| 1301 | `hash_index` |  | Y |  |  |  | Y | Y |  | 157&#8209;163 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1302 | `new` |  | Y |  |  |  | Y | Y |  | 14&#8209;16 |
| 1303 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 18&#8209;29 |
| 1304 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 31&#8209;40 |
| 1305 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 42&#8209;51 |
| 1306 | `resize` |  | Y |  |  |  | Y | Y |  | 78&#8209;112 |
| 1307 | `hash_index` |  | Y |  |  |  | Y | Y |  | 118&#8209;124 |

### Chap49/MinEditDistMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1308 | `new` | Y | Y |  |  |  | Y | Y |  | 24&#8209;27 |
| 1309 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 1310 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 32&#8209;36 |
| 1311 | `source` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1312 | `target` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 1313 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1314 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 1315 | `set_source` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1316 | `set_target` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 1317 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 1318 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 59&#8209;60 |
| 1319 | `min_edit_distance_rec` |  |  | Y |  |  | Y | Y |  | 64&#8209;112 |
| 1320 | `eq` |  | Y |  |  |  | Y | Y |  | 185 |

### Chap49/MinEditDistMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1321 | `new` | Y | Y |  |  |  | Y | Y |  | 23&#8209;26 |
| 1322 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1323 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 31&#8209;35 |
| 1324 | `source` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 1325 | `target` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 1326 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 1327 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 47&#8209;96 |
| 1328 | `eq` |  | Y |  |  |  | Y | Y |  | 145 |

### Chap49/MinEditDistStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1329 | `new` | Y | Y |  |  |  | Y | Y |  | 23&#8209;26 |
| 1330 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1331 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 1332 | `source` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1333 | `target` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1334 | `source_mut` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 1335 | `target_mut` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1336 | `set_source` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 1337 | `set_target` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1338 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 1339 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 56&#8209;57 |
| 1340 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 60&#8209;91 |

### Chap49/MinEditDistStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1341 | `new` | Y | Y |  |  |  | Y | Y |  | 23&#8209;26 |
| 1342 | `from_sequences` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1343 | `min_edit_distance` | Y | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 1344 | `source` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1345 | `target` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1346 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 1347 | `min_edit_distance_rec` |  |  |  | Y |  | Y | Y |  | 45&#8209;76 |

### Chap49/SubsetSumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1348 | `new` | Y | Y |  |  |  | Y | Y |  | 23&#8209;26 |
| 1349 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 28&#8209;29 |
| 1350 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 31&#8209;35 |
| 1351 | `multiset` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 1352 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 40&#8209;41 |
| 1353 | `set` | Y | Y |  |  |  | Y | Y |  | 43&#8209;44 |
| 1354 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 46&#8209;47 |
| 1355 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 49&#8209;50 |
| 1356 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 53&#8209;100 |
| 1357 | `eq` |  | Y |  |  |  | Y | Y |  | 161 |

### Chap49/SubsetSumMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1358 | `new` | Y | Y |  |  |  | Y | Y |  | 22&#8209;25 |
| 1359 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 27&#8209;28 |
| 1360 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 30&#8209;34 |
| 1361 | `multiset` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 1362 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 39&#8209;40 |
| 1363 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 43&#8209;90 |
| 1364 | `eq` |  | Y |  |  |  | Y | Y |  | 137 |

### Chap49/SubsetSumStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1365 | `new` | Y | Y |  |  |  | Y | Y |  | 21&#8209;24 |
| 1366 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 1367 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 29&#8209;33 |
| 1368 | `multiset` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1369 | `multiset_mut` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1370 | `set` | Y | Y |  |  |  | Y | Y |  | 41&#8209;42 |
| 1371 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1372 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 1373 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 51&#8209;77 |

### Chap49/SubsetSumStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1374 | `new` | Y | Y |  |  |  | Y | Y |  | 21&#8209;24 |
| 1375 | `from_multiset` | Y | Y |  |  |  | Y | Y |  | 26&#8209;27 |
| 1376 | `subset_sum` | Y | Y |  |  |  | Y | Y |  | 29&#8209;33 |
| 1377 | `multiset` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1378 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1379 | `subset_sum_rec` |  |  |  | Y |  | Y | Y |  | 42&#8209;68 |

### Chap50/MatrixChainMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1380 | `new` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 1381 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1382 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1383 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1384 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 1385 | `set_dimension` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1386 | `update_dimension` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1387 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 1388 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 1389 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 1390 | `multiply_cost` |  |  | Y |  |  | Y | Y |  | 62&#8209;70 |
| 1391 | `parallel_min_reduction` |  |  | Y |  |  | Y | Y |  | 72&#8209;98 |
| 1392 | `matrix_chain_rec` |  |  | Y |  |  | Y | Y |  | 100&#8209;135 |
| 1393 | `eq` |  | Y |  |  |  | Y | Y |  | 229&#8209;234 |

### Chap50/MatrixChainMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1394 | `new` | Y | Y |  |  |  | Y | Y |  | 31&#8209;32 |
| 1395 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 34&#8209;35 |
| 1396 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 37&#8209;38 |
| 1397 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 40&#8209;42 |
| 1398 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 44&#8209;45 |
| 1399 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 1400 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1401 | `multiply_cost` |  |  | Y |  |  | Y | Y |  | 55&#8209;62 |
| 1402 | `parallel_min_reduction` |  |  | Y |  |  | Y | Y |  | 64&#8209;90 |
| 1403 | `matrix_chain_rec` |  |  | Y |  |  | Y | Y |  | 92&#8209;127 |
| 1404 | `eq` |  | Y |  |  |  | Y | Y |  | 185 |

### Chap50/MatrixChainStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1405 | `new` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 1406 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1407 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1408 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1409 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 1410 | `dimensions_mut` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1411 | `set_dimension` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1412 | `update_dimension` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 1413 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 1414 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 1415 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 1416 | `multiply_cost` |  |  | Y |  |  | Y | Y |  | 65&#8209;72 |
| 1417 | `matrix_chain_rec` |  |  | Y |  |  | Y | Y |  | 74&#8209;101 |

### Chap50/MatrixChainStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1418 | `new` | Y | Y |  |  |  | Y | Y |  | 29&#8209;30 |
| 1419 | `from_dimensions` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1420 | `from_dim_pairs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1421 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 38&#8209;40 |
| 1422 | `dimensions` | Y | Y |  |  |  | Y | Y |  | 42&#8209;43 |
| 1423 | `num_matrices` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1424 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1425 | `multiply_cost` |  |  | Y |  |  | Y | Y |  | 53&#8209;60 |
| 1426 | `matrix_chain_rec` |  |  | Y |  |  | Y | Y |  | 62&#8209;89 |

### Chap50/OptBinSearchTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1427 | `new` | Y | Y |  |  |  | Y | Y |  | 30&#8209;31 |
| 1428 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 33&#8209;34 |
| 1429 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 36&#8209;37 |
| 1430 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 39&#8209;43 |
| 1431 | `keys` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1432 | `set_key_prob` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1433 | `update_prob` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 1434 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 1435 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 1436 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 1437 | `parallel_min_reduction` |  |  |  | Y |  | Y | Y |  | 64&#8209;90 |
| 1438 | `obst_rec` |  |  |  | Y |  | Y | Y |  | 92&#8209;136 |
| 1439 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 229&#8209;234 |

### Chap50/OptBinSearchTreeMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1440 | `new` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1441 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1442 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1443 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 41&#8209;45 |
| 1444 | `keys` | Y | Y |  |  |  | Y | Y |  | 47&#8209;48 |
| 1445 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 50&#8209;51 |
| 1446 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 53&#8209;54 |
| 1447 | `parallel_min_reduction` |  |  |  | Y |  | Y | Y |  | 57&#8209;83 |
| 1448 | `obst_rec` |  |  |  | Y |  | Y | Y |  | 85&#8209;126 |
| 1449 | `eq` x2 |  | Y |  |  |  | Y | Y |  | 184 |

### Chap50/OptBinSearchTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1450 | `new` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1451 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1452 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1453 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 1454 | `keys` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1455 | `keys_mut` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1456 | `set_key_prob` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 1457 | `update_prob` | Y | Y |  |  |  | Y | Y |  | 54&#8209;55 |
| 1458 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 57&#8209;58 |
| 1459 | `clear_memo` | Y | Y |  |  |  | Y | Y |  | 60&#8209;61 |
| 1460 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 63&#8209;64 |
| 1461 | `obst_rec` |  |  | Y |  |  | Y | Y |  | 68&#8209;100 |

### Chap50/OptBinSearchTreeStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1462 | `new` | Y | Y |  |  |  | Y | Y |  | 32&#8209;33 |
| 1463 | `from_keys_probs` | Y | Y |  |  |  | Y | Y |  | 35&#8209;36 |
| 1464 | `from_key_probs` | Y | Y |  |  |  | Y | Y |  | 38&#8209;39 |
| 1465 | `optimal_cost` | Y | Y |  |  |  | Y | Y |  | 41&#8209;43 |
| 1466 | `keys` | Y | Y |  |  |  | Y | Y |  | 45&#8209;46 |
| 1467 | `num_keys` | Y | Y |  |  |  | Y | Y |  | 48&#8209;49 |
| 1468 | `memo_size` | Y | Y |  |  |  | Y | Y |  | 51&#8209;52 |
| 1469 | `obst_rec` |  |  | Y |  |  | Y | Y |  | 56&#8209;88 |

### Chap50/Probability.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1470 | `new` | Y |  | Y |  |  | Y | Y |  | 19&#8209;21 |
| 1471 | `value` | Y |  | Y |  |  | Y | Y |  | 23&#8209;25 |
| 1472 | `infinity` |  |  | Y |  |  | Y | Y |  | 44&#8209;46 |
| 1473 | `zero` |  |  | Y |  |  | Y | Y |  | 48&#8209;50 |
| 1474 | `default` |  | Y |  |  |  | Y | Y |  | 54 |
| 1475 | `eq` |  | Y |  |  |  | Y | Y |  | 58&#8209;61 |
| 1476 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 67 |
| 1477 | `cmp` |  | Y |  |  |  | Y | Y |  | 71&#8209;87 |
| 1478 | `hash` |  | Y |  |  |  | Y | Y |  | 91 |
| 1479 | `from` x2 |  | Y |  |  |  | Y | Y |  | 103 |
| 1480 | `add` |  | Y |  |  |  | Y | Y |  | 114 |
| 1481 | `sub` |  | Y |  |  |  | Y | Y |  | 120 |
| 1482 | `mul` |  | Y |  |  |  | Y | Y |  | 126 |
| 1483 | `div` |  | Y |  |  |  | Y | Y |  | 132 |

### Chap51/BottomUpDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1484 | `new` | Y |  | Y |  |  | Y | Y |  | 19&#8209;21 |
| 1485 | `solve` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1486 | `med_bottom_up_parallel` |  |  | Y |  |  | Y | Y |  | 41&#8209;59 |
| 1487 | `initialize_base_cases` |  |  | Y |  |  | Y | Y |  | 61&#8209;80 |
| 1488 | `compute_diagonal_parallel` |  |  | Y |  |  | Y | Y |  | 82&#8209;121 |
| 1489 | `compute_cell_value_static` |  |  | Y |  |  | Y | Y |  | 123&#8209;148 |
| 1490 | `s_length` |  |  | Y |  |  | Y | Y |  | 150&#8209;152 |
| 1491 | `t_length` |  |  | Y |  |  | Y | Y |  | 154&#8209;156 |
| 1492 | `is_empty` |  |  | Y |  |  | Y | Y |  | 158&#8209;160 |
| 1493 | `set_s` |  |  | Y |  |  | Y | Y |  | 162&#8209;164 |
| 1494 | `set_t` |  |  | Y |  |  | Y | Y |  | 166&#8209;168 |
| 1495 | `default` |  | Y |  |  |  | Y | Y |  | 172&#8209;176 |

### Chap51/BottomUpDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1496 | `new` | Y |  | Y |  |  | Y | Y |  | 19&#8209;21 |
| 1497 | `solve` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1498 | `med_bottom_up_parallel` |  |  | Y |  |  | Y | Y |  | 41&#8209;59 |
| 1499 | `initialize_base_cases` |  |  | Y |  |  | Y | Y |  | 61&#8209;80 |
| 1500 | `compute_diagonal_parallel` |  |  | Y |  |  | Y | Y |  | 82&#8209;121 |
| 1501 | `compute_cell_value_static` |  |  | Y |  |  | Y | Y |  | 123&#8209;147 |
| 1502 | `s_length` |  |  | Y |  |  | Y | Y |  | 149&#8209;151 |
| 1503 | `t_length` |  |  | Y |  |  | Y | Y |  | 153&#8209;155 |
| 1504 | `is_empty` |  |  | Y |  |  | Y | Y |  | 157&#8209;159 |
| 1505 | `default` |  | Y |  |  |  | Y | Y |  | 163&#8209;167 |

### Chap51/BottomUpDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1506 | `new` | Y |  | Y |  |  | Y | Y |  | 17&#8209;20 |
| 1507 | `solve` | Y |  |  |  |  | Y | Y |  | 22&#8209;25 |
| 1508 | `med_bottom_up` |  |  | Y |  |  | Y | Y |  | 41&#8209;58 |
| 1509 | `initialize_base_cases` |  |  | Y |  |  | Y | Y |  | 60&#8209;79 |
| 1510 | `compute_diagonal` |  |  | Y |  |  | Y | Y |  | 81&#8209;98 |
| 1511 | `compute_cell_value` |  |  | Y |  |  | Y | Y |  | 100&#8209;116 |
| 1512 | `s_length` |  |  | Y |  |  | Y | Y |  | 118&#8209;120 |
| 1513 | `t_length` |  |  | Y |  |  | Y | Y |  | 122&#8209;124 |
| 1514 | `is_empty` |  |  | Y |  |  | Y | Y |  | 126&#8209;128 |
| 1515 | `set_s` |  |  | Y |  |  | Y | Y |  | 130&#8209;132 |
| 1516 | `set_t` |  |  | Y |  |  | Y | Y |  | 134&#8209;136 |
| 1517 | `default` |  | Y |  |  |  | Y | Y |  | 140&#8209;144 |

### Chap51/BottomUpDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1518 | `new` | Y |  | Y |  |  | Y | Y |  | 17&#8209;19 |
| 1519 | `solve` | Y |  |  |  |  | Y | Y |  | 21&#8209;23 |
| 1520 | `med_bottom_up` |  |  | Y |  |  | Y | Y |  | 39&#8209;56 |
| 1521 | `initialize_base_cases` |  |  | Y |  |  | Y | Y |  | 58&#8209;77 |
| 1522 | `compute_diagonal` |  |  | Y |  |  | Y | Y |  | 79&#8209;98 |
| 1523 | `compute_cell_value` |  |  | Y |  |  | Y | Y |  | 100&#8209;116 |
| 1524 | `s_length` |  |  | Y |  |  | Y | Y |  | 118&#8209;120 |
| 1525 | `t_length` |  |  | Y |  |  | Y | Y |  | 122&#8209;124 |
| 1526 | `is_empty` |  |  | Y |  |  | Y | Y |  | 126&#8209;128 |
| 1527 | `default` |  | Y |  |  |  | Y | Y |  | 132&#8209;136 |

### Chap51/TopDownDPMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1528 | `new` | Y |  | Y |  |  | Y | Y |  | 19&#8209;21 |
| 1529 | `solve` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1530 | `med_memoized_concurrent` |  |  | Y |  |  | Y | Y |  | 49&#8209;57 |
| 1531 | `med_recursive_concurrent` |  |  | Y |  |  | Y | Y |  | 59&#8209;100 |
| 1532 | `med_memoized_parallel` |  |  | Y |  |  | Y | Y |  | 102&#8209;110 |
| 1533 | `med_recursive_parallel` |  |  | Y |  |  | Y | Y |  | 112&#8209;163 |
| 1534 | `memo_size` |  |  | Y |  |  | Y | Y |  | 165&#8209;170 |
| 1535 | `is_memoized` |  |  | Y |  |  | Y | Y |  | 172&#8209;177 |
| 1536 | `get_memoized` |  |  | Y |  |  | Y | Y |  | 179&#8209;184 |
| 1537 | `insert_memo` |  |  | Y |  |  | Y | Y |  | 186&#8209;191 |
| 1538 | `s_length` |  |  | Y |  |  | Y | Y |  | 193&#8209;195 |
| 1539 | `t_length` |  |  | Y |  |  | Y | Y |  | 197&#8209;199 |
| 1540 | `is_empty` |  |  | Y |  |  | Y | Y |  | 201&#8209;203 |
| 1541 | `clear_memo` |  |  | Y |  |  | Y | Y |  | 205&#8209;210 |
| 1542 | `set_s` |  |  | Y |  |  | Y | Y |  | 212&#8209;217 |
| 1543 | `set_t` |  |  | Y |  |  | Y | Y |  | 219&#8209;224 |
| 1544 | `eq` |  | Y |  |  |  | Y | Y |  | 228&#8209;232 |
| 1545 | `default` |  | Y |  |  |  | Y | Y |  | 236&#8209;240 |

### Chap51/TopDownDPMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1546 | `new` | Y |  | Y |  |  | Y | Y |  | 19&#8209;21 |
| 1547 | `solve` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 1548 | `med_memoized_concurrent` |  |  | Y |  |  | Y | Y |  | 49&#8209;57 |
| 1549 | `med_recursive_concurrent` |  |  | Y |  |  | Y | Y |  | 59&#8209;102 |
| 1550 | `med_memoized_parallel` |  |  | Y |  |  | Y | Y |  | 104&#8209;112 |
| 1551 | `med_recursive_parallel` |  |  | Y |  |  | Y | Y |  | 114&#8209;165 |
| 1552 | `with_memo_table` |  |  | Y |  |  | Y | Y |  | 167&#8209;175 |
| 1553 | `memo_size` |  |  | Y |  |  | Y | Y |  | 177&#8209;182 |
| 1554 | `is_memoized` |  |  | Y |  |  | Y | Y |  | 184&#8209;189 |
| 1555 | `get_memoized` |  |  | Y |  |  | Y | Y |  | 191&#8209;196 |
| 1556 | `s_length` |  |  | Y |  |  | Y | Y |  | 198&#8209;200 |
| 1557 | `t_length` |  |  | Y |  |  | Y | Y |  | 202&#8209;204 |
| 1558 | `is_empty` |  |  | Y |  |  | Y | Y |  | 206&#8209;208 |
| 1559 | `clear_memo` |  |  | Y |  |  | Y | Y |  | 210&#8209;218 |
| 1560 | `eq` |  | Y |  |  |  | Y | Y |  | 222&#8209;226 |
| 1561 | `default` |  | Y |  |  |  | Y | Y |  | 230&#8209;234 |

### Chap51/TopDownDPStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1562 | `new` | Y |  | Y |  |  | Y | Y |  | 17&#8209;19 |
| 1563 | `solve` | Y |  |  |  |  | Y | Y |  | 21&#8209;23 |
| 1564 | `med_memoized` |  |  | Y |  |  | Y | Y |  | 47&#8209;55 |
| 1565 | `med_recursive` |  |  | Y |  |  | Y | Y |  | 57&#8209;91 |
| 1566 | `memo_size` |  |  | Y |  |  | Y | Y |  | 93&#8209;95 |
| 1567 | `is_memoized` |  |  | Y |  |  | Y | Y |  | 97&#8209;99 |
| 1568 | `get_memoized` |  |  | Y |  |  | Y | Y |  | 101&#8209;103 |
| 1569 | `insert_memo` |  |  | Y |  |  | Y | Y |  | 105&#8209;107 |
| 1570 | `s_length` |  |  | Y |  |  | Y | Y |  | 109&#8209;111 |
| 1571 | `t_length` |  |  | Y |  |  | Y | Y |  | 113&#8209;115 |
| 1572 | `is_empty` |  |  | Y |  |  | Y | Y |  | 117&#8209;119 |
| 1573 | `clear_memo` |  |  | Y |  |  | Y | Y |  | 121&#8209;123 |
| 1574 | `set_s` |  |  | Y |  |  | Y | Y |  | 125&#8209;130 |
| 1575 | `set_t` |  |  | Y |  |  | Y | Y |  | 132&#8209;137 |
| 1576 | `default` |  | Y |  |  |  | Y | Y |  | 141&#8209;145 |

### Chap51/TopDownDPStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1577 | `new` | Y |  | Y |  |  | Y | Y |  | 17&#8209;19 |
| 1578 | `solve` | Y |  |  |  |  | Y | Y |  | 21&#8209;23 |
| 1579 | `med_memoized` |  |  | Y |  |  | Y | Y |  | 47&#8209;58 |
| 1580 | `med_recursive` |  |  | Y |  |  | Y | Y |  | 60&#8209;94 |
| 1581 | `with_memo_table` |  |  | Y |  |  | Y | Y |  | 96&#8209;104 |
| 1582 | `memo_size` |  |  | Y |  |  | Y | Y |  | 106&#8209;108 |
| 1583 | `is_memoized` |  |  | Y |  |  | Y | Y |  | 110&#8209;112 |
| 1584 | `get_memoized` |  |  | Y |  |  | Y | Y |  | 114&#8209;116 |
| 1585 | `s_length` |  |  | Y |  |  | Y | Y |  | 118&#8209;120 |
| 1586 | `t_length` |  |  | Y |  |  | Y | Y |  | 122&#8209;124 |
| 1587 | `is_empty` |  |  | Y |  |  | Y | Y |  | 126&#8209;128 |
| 1588 | `clear_memo` |  |  | Y |  |  | Y | Y |  | 130&#8209;138 |
| 1589 | `default` |  | Y |  |  |  | Y | Y |  | 142&#8209;146 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
