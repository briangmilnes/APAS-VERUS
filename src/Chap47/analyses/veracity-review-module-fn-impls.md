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
| 1 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 2 | 4 | 1 | 0 | 5 |
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 6 | 1 | 0 | 0 | 7 | 0 | 0 | 7 |
| 3 | Chap47 | FlatHashTable | 4 | 4 | 0 | 0 | 4 | 4 | 0 | 0 | 8 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 6 | 0 | 1 | 1 | 6 | 0 | 0 | 7 |
| 6 | Chap47 | ParaHashTableStEph | 8 | 0 | 0 | 0 | 4 | 4 | 0 | 0 | 8 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 6 | 0 | 0 | 0 | 6 | 0 | 0 | 6 |
| 8 | Chap47 | StructChainedHashTable | 0 | 8 | 0 | 1 | 3 | 6 | 1 | 0 | 8 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 6 | 0 | 1 | 1 | 6 | 0 | 0 | 7 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 25 |
| 2 | `eq` |  | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 3 | `hash_index` | Y |  |  |  |  | Y | Y |  | 74&#8209;77 |
| 4 | `insert_chained` | Y |  |  |  |  | Y | Y |  | 79&#8209;91 |
| 5 | `lookup_chained` | Y |  |  |  |  | Y | Y |  | 93&#8209;103 |
| 6 | `delete_chained` | Y |  |  |  |  | Y | Y |  | 105&#8209;119 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `second_hash` |  |  | Y |  |  | Y | Y |  | 36&#8209;62 |
| 8 | `insert` |  | Y |  |  |  | Y | Y |  | 68&#8209;85 |
| 9 | `lookup` |  | Y |  |  |  | Y | Y |  | 87&#8209;102 |
| 10 | `delete` |  | Y |  |  |  | Y | Y |  | 104&#8209;123 |
| 11 | `resize` |  | Y |  |  |  | Y | Y |  | 125&#8209;156 |
| 12 | `probe` |  | Y |  |  |  | Y | Y |  | 162&#8209;168 |
| 13 | `find_slot` |  | Y |  |  |  | Y | Y |  | 170&#8209;183 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `new` |  | Y |  |  | Y |  | Y |  | 53 |
| 15 | `insert` |  | Y |  |  | Y |  | Y |  | 57 |
| 16 | `lookup` |  | Y |  |  | Y |  | Y |  | 61 |
| 17 | `delete` |  | Y |  |  | Y |  | Y |  | 72 |
| 18 | `probe` | Y |  |  |  |  | Y | Y |  | 97&#8209;100 |
| 19 | `find_slot` | Y |  |  |  |  | Y | Y |  | 102&#8209;105 |
| 20 | `insert_with_probe` | Y |  |  |  |  | Y | Y |  | 107&#8209;115 |
| 21 | `lookup_with_probe` | Y |  |  |  |  | Y | Y |  | 117&#8209;130 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `insert` |  | Y |  |  |  | Y | Y |  | 35&#8209;52 |
| 23 | `lookup` |  | Y |  |  |  | Y | Y |  | 54&#8209;69 |
| 24 | `delete` |  | Y |  |  |  | Y | Y |  | 71&#8209;90 |
| 25 | `resize` |  | Y |  |  |  | Y | Y |  | 92&#8209;123 |
| 26 | `probe` |  | Y |  |  |  | Y | Y |  | 129&#8209;134 |
| 27 | `find_slot` |  | Y |  |  |  | Y | Y |  | 136&#8209;149 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 23 |
| 29 | `new` |  | Y |  |  |  | Y | Y |  | 30&#8209;32 |
| 30 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 34&#8209;44 |
| 31 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 46&#8209;55 |
| 32 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 57&#8209;75 |
| 33 | `resize` |  | Y |  |  |  | Y | Y |  | 101&#8209;132 |
| 34 | `hash_index` |  | Y |  |  |  | Y | Y |  | 138&#8209;142 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `new` | Y |  |  |  | Y |  | Y |  | 49 |
| 36 | `insert` x2 | Y |  |  |  | Y |  | Y |  | 100&#8209;103 |
| 37 | `lookup` x2 | Y |  |  |  | Y |  | Y |  | 105&#8209;108 |
| 38 | `delete` x2 | Y |  |  |  | Y |  | Y |  | 110&#8209;113 |
| 39 | `createTable` | Y |  |  |  |  | Y | Y |  | 81&#8209;98 |
| 40 | `metrics` | Y |  |  |  |  | Y | Y |  | 115&#8209;118 |
| 41 | `loadAndSize` | Y |  |  |  |  | Y | Y |  | 120&#8209;134 |
| 42 | `resize` | Y |  |  |  |  | Y | Y |  | 136&#8209;141 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `insert` |  | Y |  |  |  | Y | Y |  | 36&#8209;53 |
| 44 | `lookup` |  | Y |  |  |  | Y | Y |  | 55&#8209;71 |
| 45 | `delete` |  | Y |  |  |  | Y | Y |  | 73&#8209;93 |
| 46 | `resize` |  | Y |  |  |  | Y | Y |  | 95&#8209;126 |
| 47 | `probe` |  | Y |  |  |  | Y | Y |  | 132&#8209;137 |
| 48 | `find_slot` |  | Y |  |  |  | Y | Y |  | 139&#8209;153 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 27 |
| 50 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 72&#8209;74 |
| 51 | `default` |  | Y |  |  | Y |  | Y |  | 128 |
| 52 | `new` |  | Y |  |  |  | Y | Y |  | 136 |
| 53 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 138&#8209;153 |
| 54 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 155&#8209;164 |
| 55 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 166&#8209;180 |
| 56 | `resize` |  | Y |  |  |  | Y | Y |  | 206&#8209;239 |
| 57 | `hash_index` |  | Y |  |  |  | Y | Y |  | 245&#8209;249 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 22 |
| 59 | `new` |  | Y |  |  |  | Y | Y |  | 31&#8209;33 |
| 60 | `insert` x2 |  | Y |  |  |  | Y | Y |  | 35&#8209;45 |
| 61 | `lookup` x2 |  | Y |  |  |  | Y | Y |  | 47&#8209;56 |
| 62 | `delete` x2 |  | Y |  |  |  | Y | Y |  | 58&#8209;67 |
| 63 | `resize` |  | Y |  |  |  | Y | Y |  | 93&#8209;124 |
| 64 | `hash_index` |  | Y |  |  |  | Y | Y |  | 130&#8209;134 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
