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
| 1 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 6 | 0 | 2 | 3 | 1 |
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 | 0 |
| 3 | Chap47 | FlatHashTable | 4 | 4 | 0 | 0 | 8 | 0 | 6 | 2 | 0 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 6 | 0 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 6 | 0 | 1 | 1 | 6 | 0 | 0 | 7 |
| 6 | Chap47 | ParaHashTableStEph | 8 | 0 | 0 | 0 | 8 | 0 | 4 | 3 | 1 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 6 | 0 |
| 8 | Chap47 | StructChainedHashTable | 0 | 6 | 5 | 4 | 12 | 0 | 4 | 5 | 3 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 4 | 5 | 1 | 7 | 0 | 0 | 5 | 2 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 42 |
| 2 | `eq` |  | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 3 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 85&#8209;89 |
| 4 | `insert_chained` | Y |  |  |  | Y |  |  | hole | 95 |
| 5 | `lookup_chained` | Y |  |  |  | Y |  |  | hole | 110 |
| 6 | `delete_chained` | Y |  |  |  | Y |  |  | hole | 123 |

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
| 14 | `new` |  | Y |  |  | Y |  |  | unknown | 42&#8209;43 |
| 15 | `insert` |  | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 16 | `lookup` |  | Y |  |  | Y |  |  | unknown | 54&#8209;57 |
| 17 | `delete` |  | Y |  |  | Y |  |  | unknown | 69&#8209;72 |
| 18 | `probe` | Y |  |  |  | Y |  |  | unknown | 111&#8209;115 |
| 19 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 120&#8209;124 |
| 20 | `insert_with_probe` | Y |  |  |  | Y |  |  | hole | 130 |
| 21 | `lookup_with_probe` | Y |  |  |  | Y |  |  | hole | 141 |

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
| 28 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 31 |
| 29 | `new` |  | Y |  |  |  | Y | Y |  | 35&#8209;37 |
| 30 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 39&#8209;49 |
| 31 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 51&#8209;60 |
| 32 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 62&#8209;80 |
| 33 | `resize` |  | Y |  |  |  | Y | Y |  | 107&#8209;136 |
| 34 | `hash_index` |  | Y |  |  |  | Y | Y |  | 143&#8209;147 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `new` | Y |  |  |  | Y |  | Y |  | 52 |
| 36 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 93&#8209;96 |
| 37 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 101&#8209;104 |
| 38 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 109&#8209;112 |
| 39 | `createTable` | Y |  |  |  | Y |  |  | hole | 71&#8209;76 |
| 40 | `metrics` | Y |  |  |  | Y |  |  | hole | 118 |
| 41 | `loadAndSize` | Y |  |  |  | Y |  |  | hole | 125&#8209;126 |
| 42 | `resize` | Y |  |  |  | Y |  |  | unknown | 144&#8209;149 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `insert` |  |  | Y |  | Y |  |  | hole | 38 |
| 44 | `lookup` |  |  | Y |  | Y |  |  | hole | 58 |
| 45 | `delete` |  |  | Y |  | Y |  |  | hole | 77 |
| 46 | `resize` |  |  | Y |  | Y |  |  | hole | 100&#8209;103 |
| 47 | `probe` |  |  | Y |  | Y |  |  | hole | 137 |
| 48 | `find_slot` |  |  | Y |  | Y |  |  | hole | 145 |

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
| 58 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 236&#8209;238 |
| 59 | `resize` |  |  | Y |  | Y |  |  | hole | 315&#8209;318 |
| 60 | `hash_index` |  |  | Y |  | Y |  |  | hole | 354 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 31 |
| 62 | `new` |  | Y |  |  | Y |  | Y |  | 38 |
| 63 | `insert` |  | Y | Y |  | Y |  |  | hole | 42 |
| 64 | `lookup` |  | Y | Y |  | Y |  |  | hole | 59 |
| 65 | `delete` |  | Y | Y |  | Y |  |  | hole | 74 |
| 66 | `resize` |  |  | Y |  | Y |  |  | hole | 119&#8209;122 |
| 67 | `hash_index` |  |  | Y |  | Y |  |  | hole | 156 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
