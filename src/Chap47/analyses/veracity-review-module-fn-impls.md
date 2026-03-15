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
| 1 | Chap47 | ChainedHashTable | 4 | 1 | 0 | 1 | 6 | 0 | 5 | 0 | 1 |
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 0 | 7 | 0 | 1 | 0 | 6 |
| 3 | Chap47 | FlatHashTable | 4 | 5 | 0 | 0 | 9 | 0 | 8 | 0 | 1 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 0 | 6 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 5 | 5 | 1 | 8 | 0 | 2 | 0 | 6 |
| 6 | Chap47 | ParaHashTableStEph | 9 | 0 | 0 | 5 | 14 | 0 | 10 | 2 | 2 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 0 | 6 |
| 8 | Chap47 | StructChainedHashTable | 0 | 7 | 5 | 4 | 13 | 0 | 6 | 0 | 7 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 5 | 5 | 1 | 8 | 0 | 3 | 0 | 5 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 42 |
| 2 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 55&#8209;59 |
| 3 | `insert_chained` | Y |  |  |  | Y |  |  | unknown | 64&#8209;70 |
| 4 | `lookup_chained` | Y |  |  |  | Y |  |  | unknown | 85&#8209;89 |
| 5 | `delete_chained` | Y |  |  |  | Y |  |  | unknown | 102&#8209;107 |
| 6 | `eq` |  | Y |  |  | Y |  |  | unknown | 143&#8209;144 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `second_hash` |  |  | Y |  | Y |  |  | unknown | 40&#8209;41 |
| 8 | `insert` |  |  | Y |  | Y |  | Y |  | 53 |
| 9 | `lookup` |  |  | Y |  | Y |  | Y |  | 83 |
| 10 | `delete` |  |  | Y |  | Y |  | Y |  | 107 |
| 11 | `resize` |  |  | Y |  | Y |  | Y |  | 136&#8209;139 |
| 12 | `probe` |  |  | Y |  | Y |  | Y |  | 202 |
| 13 | `find_slot` |  |  | Y |  | Y |  | Y |  | 210 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `probe` | Y |  |  |  | Y |  |  | unknown | 48&#8209;52 |
| 15 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 57&#8209;62 |
| 16 | `insert_with_probe` | Y |  |  |  | Y |  |  | unknown | 67&#8209;70 |
| 17 | `lookup_with_probe` | Y |  |  |  | Y |  |  | unknown | 81&#8209;84 |
| 18 | `new` |  | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 19 | `insert` |  | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 20 | `lookup` |  | Y |  |  | Y |  |  | unknown | 121&#8209;124 |
| 21 | `delete` |  | Y |  |  | Y |  |  | unknown | 136&#8209;139 |
| 22 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 155 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `insert` |  |  | Y |  | Y |  | Y |  | 36 |
| 24 | `lookup` |  |  | Y |  | Y |  | Y |  | 66 |
| 25 | `delete` |  |  | Y |  | Y |  | Y |  | 90 |
| 26 | `resize` |  |  | Y |  | Y |  | Y |  | 119&#8209;122 |
| 27 | `probe` |  |  | Y |  | Y |  | Y |  | 188 |
| 28 | `find_slot` |  |  | Y |  | Y |  | Y |  | 195 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 30 | `new` |  | Y |  |  | Y |  | Y |  | 39 |
| 31 | `insert` |  | Y | Y |  | Y |  |  | unknown | 43&#8209;47 |
| 32 | `lookup` |  | Y | Y |  | Y |  | Y |  | 68 |
| 33 | `delete` |  | Y | Y |  | Y |  |  | unknown | 83&#8209;84 |
| 34 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 103 |
| 35 | `resize` |  |  | Y |  | Y |  | Y |  | 194&#8209;197 |
| 36 | `hash_index` |  |  | Y |  | Y |  | Y |  | 273 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `call_hash_fn` |  |  |  | Y | Y |  |  | hole | 57&#8209;59 |
| 38 | `linear_probe` |  |  |  | Y | Y |  |  | unknown | 65&#8209;67 |
| 39 | `quadratic_probe` |  |  |  | Y | Y |  |  | unknown | 74&#8209;76 |
| 40 | `compute_second_hash` |  |  |  | Y | Y |  |  | hole | 85&#8209;86 |
| 41 | `double_hash_probe` |  |  |  | Y | Y |  |  | unknown | 105&#8209;107 |
| 42 | `new` | Y |  |  |  | Y |  | Y |  | 121 |
| 43 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 174&#8209;182 |
| 44 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 187&#8209;190 |
| 45 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 195&#8209;201 |
| 46 | `clone_entry` | Y |  |  |  | Y |  | Y |  | 132 |
| 47 | `createTable` | Y |  |  |  | Y |  |  | unknown | 141&#8209;147 |
| 48 | `metrics` | Y |  |  |  | Y |  |  | unknown | 206&#8209;207 |
| 49 | `loadAndSize` | Y |  |  |  | Y |  |  | unknown | 214&#8209;217 |
| 50 | `resize` | Y |  |  |  | Y |  |  | unknown | 230&#8209;236 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `insert` |  |  | Y |  | Y |  | Y |  | 38 |
| 52 | `lookup` |  |  | Y |  | Y |  | Y |  | 68 |
| 53 | `delete` |  |  | Y |  | Y |  | Y |  | 92 |
| 54 | `resize` |  |  | Y |  | Y |  | Y |  | 121&#8209;124 |
| 55 | `probe` |  |  | Y |  | Y |  | Y |  | 187 |
| 56 | `find_slot` |  |  | Y |  | Y |  | Y |  | 194 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 57 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 69 |
| 58 | `default` |  | Y |  |  | Y |  | Y |  | 99 |
| 59 | `chain_insert` |  |  |  | Y | Y |  |  | unknown | 103&#8209;111 |
| 60 | `chain_lookup` |  |  |  | Y | Y |  |  | unknown | 136&#8209;142 |
| 61 | `chain_delete` |  |  |  | Y | Y |  |  | unknown | 157&#8209;164 |
| 62 | `new` |  | Y |  |  | Y |  | Y |  | 186 |
| 63 | `insert` |  | Y | Y |  | Y |  |  | unknown | 190&#8209;191 |
| 64 | `lookup` |  | Y | Y |  | Y |  | Y |  | 198 |
| 65 | `delete` |  | Y | Y |  | Y |  |  | unknown | 205&#8209;207 |
| 66 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 215 |
| 67 | `resize` |  |  | Y |  | Y |  | Y |  | 261&#8209;264 |
| 68 | `hash_index` |  |  | Y |  | Y |  | Y |  | 336 |
| 69 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 362&#8209;364 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 32 |
| 71 | `new` |  | Y |  |  | Y |  |  | unknown | 39&#8209;40 |
| 72 | `insert` |  | Y | Y |  | Y |  |  | unknown | 45&#8209;49 |
| 73 | `lookup` |  | Y | Y |  | Y |  | Y |  | 70 |
| 74 | `delete` |  | Y | Y |  | Y |  |  | unknown | 85&#8209;86 |
| 75 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 105 |
| 76 | `resize` |  |  | Y |  | Y |  | Y |  | 198&#8209;201 |
| 77 | `hash_index` |  |  | Y |  | Y |  | Y |  | 274 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
