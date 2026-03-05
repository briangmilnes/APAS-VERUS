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
| 1 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 6 | 0 | 3 | 2 | 1 |
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 | 0 |
| 3 | Chap47 | FlatHashTable | 4 | 4 | 0 | 0 | 8 | 0 | 6 | 2 | 0 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 6 | 0 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 4 | 5 | 1 | 7 | 0 | 0 | 5 | 2 |
| 6 | Chap47 | ParaHashTableStEph | 8 | 0 | 0 | 0 | 8 | 0 | 6 | 1 | 1 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 6 | 0 |
| 8 | Chap47 | StructChainedHashTable | 0 | 6 | 5 | 4 | 12 | 0 | 4 | 5 | 3 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 4 | 5 | 1 | 7 | 0 | 1 | 5 | 1 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 42 |
| 2 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 55&#8209;59 |
| 3 | `insert_chained` | Y |  |  |  | Y |  |  | hole | 65&#8209;71 |
| 4 | `lookup_chained` | Y |  |  |  | Y |  |  | unknown | 86&#8209;89 |
| 5 | `delete_chained` | Y |  |  |  | Y |  |  | hole | 103&#8209;108 |
| 6 | `eq` |  | Y |  |  | Y |  |  | unknown | 142&#8209;143 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `second_hash` |  |  | Y |  | Y |  |  | hole | 41 |
| 8 | `insert` |  |  | Y |  | Y |  |  | hole | 71 |
| 9 | `lookup` |  |  | Y |  | Y |  |  | hole | 91 |
| 10 | `delete` |  |  | Y |  | Y |  |  | hole | 109 |
| 11 | `resize` |  |  | Y |  | Y |  |  | hole | 131&#8209;134 |
| 12 | `probe` |  |  | Y |  | Y |  |  | hole | 168 |
| 13 | `find_slot` |  |  | Y |  | Y |  |  | hole | 177 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `probe` | Y |  |  |  | Y |  |  | unknown | 48&#8209;52 |
| 15 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 57&#8209;61 |
| 16 | `insert_with_probe` | Y |  |  |  | Y |  |  | hole | 67&#8209;70 |
| 17 | `lookup_with_probe` | Y |  |  |  | Y |  |  | hole | 82&#8209;85 |
| 18 | `new` |  | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 19 | `insert` |  | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 20 | `lookup` |  | Y |  |  | Y |  |  | unknown | 116&#8209;119 |
| 21 | `delete` |  | Y |  |  | Y |  |  | unknown | 131&#8209;134 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `insert` |  |  | Y |  | Y |  |  | hole | 37 |
| 23 | `lookup` |  |  | Y |  | Y |  |  | hole | 57 |
| 24 | `delete` |  |  | Y |  | Y |  |  | hole | 75 |
| 25 | `resize` |  |  | Y |  | Y |  |  | hole | 97&#8209;100 |
| 26 | `probe` |  |  | Y |  | Y |  |  | hole | 134 |
| 27 | `find_slot` |  |  | Y |  | Y |  |  | hole | 142 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 29 | `new` |  | Y |  |  | Y |  | Y |  | 39 |
| 30 | `insert` |  | Y | Y |  | Y |  |  | hole | 43&#8209;47 |
| 31 | `lookup` |  | Y | Y |  | Y |  |  | hole | 68 |
| 32 | `delete` |  | Y | Y |  | Y |  |  | hole | 83&#8209;84 |
| 33 | `resize` |  |  | Y |  | Y |  |  | hole | 131&#8209;134 |
| 34 | `hash_index` |  |  | Y |  | Y |  |  | hole | 168 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `new` | Y |  |  |  | Y |  | Y |  | 52 |
| 36 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 102&#8209;108 |
| 37 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 113&#8209;116 |
| 38 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 121&#8209;126 |
| 39 | `createTable` | Y |  |  |  | Y |  |  | unknown | 70&#8209;75 |
| 40 | `metrics` | Y |  |  |  | Y |  |  | unknown | 131&#8209;132 |
| 41 | `loadAndSize` | Y |  |  |  | Y |  |  | hole | 140&#8209;141 |
| 42 | `resize` | Y |  |  |  | Y |  |  | unknown | 159&#8209;164 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `insert` |  |  | Y |  | Y |  |  | hole | 39 |
| 44 | `lookup` |  |  | Y |  | Y |  |  | hole | 59 |
| 45 | `delete` |  |  | Y |  | Y |  |  | hole | 78 |
| 46 | `resize` |  |  | Y |  | Y |  |  | hole | 101&#8209;104 |
| 47 | `probe` |  |  | Y |  | Y |  |  | hole | 138 |
| 48 | `find_slot` |  |  | Y |  | Y |  |  | hole | 146 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 69 |
| 50 | `default` |  | Y |  |  | Y |  | Y |  | 99 |
| 51 | `chain_insert` |  |  |  | Y | Y |  |  | unknown | 103&#8209;111 |
| 52 | `chain_lookup` |  |  |  | Y | Y |  |  | unknown | 136&#8209;142 |
| 53 | `chain_delete` |  |  |  | Y | Y |  |  | unknown | 157&#8209;164 |
| 54 | `new` |  | Y |  |  | Y |  | Y |  | 186 |
| 55 | `insert` |  | Y | Y |  | Y |  |  | hole | 190&#8209;191 |
| 56 | `lookup` |  | Y | Y |  | Y |  |  | hole | 198 |
| 57 | `delete` |  | Y | Y |  | Y |  |  | hole | 205&#8209;207 |
| 58 | `resize` |  |  | Y |  | Y |  |  | hole | 245&#8209;248 |
| 59 | `hash_index` |  |  | Y |  | Y |  |  | hole | 284 |
| 60 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 310&#8209;312 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 62 | `new` |  | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 63 | `insert` |  | Y | Y |  | Y |  |  | hole | 45&#8209;49 |
| 64 | `lookup` |  | Y | Y |  | Y |  |  | hole | 70 |
| 65 | `delete` |  | Y | Y |  | Y |  |  | hole | 85&#8209;86 |
| 66 | `resize` |  |  | Y |  | Y |  |  | hole | 135&#8209;138 |
| 67 | `hash_index` |  |  | Y |  | Y |  |  | hole | 172 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
