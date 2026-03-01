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
| 1 | Chap42 | Example42_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 2 | Chap42 | TableMtEph | 16 | 17 | 0 | 2 | 18 | 1 | 3 | 14 | 2 |
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 2 | 18 | 2 | 3 | 14 | 3 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 9 | 25 | 1 | 14 | 10 | 2 |

## Function-by-Function Detail

### Chap42/Example42_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_example_42_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 2 | `example_42_1` | Y |  |  | Y |  | Y | Y |  | 21&#8209;23 |
| 3 | `demonstrate_table_operations` | Y |  |  |  |  | Y | Y |  | 25&#8209;27 |
| 4 | `performance_comparison` |  |  |  | Y |  | Y | Y |  | 151&#8209;202 |

### Chap42/TableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `size` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 6 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 7 | `singleton` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 8 | `domain` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 9 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 10 | `map` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 11 | `filter` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 12 | `intersection` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 14 | `difference` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 15 | `find` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 16 | `delete` | Y | Y |  |  | Y |  |  | hole | 106&#8209;107 |
| 17 | `insert` | Y | Y |  |  | Y |  |  | hole | 109&#8209;110 |
| 18 | `restrict` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 19 | `subtract` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 20 | `entries` | Y | Y |  |  | Y |  | Y |  | 118 |
| 21 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 728&#8209;729 |
| 22 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 738&#8209;740 |
| 23 | `eq` |  | Y |  |  |  | Y | Y |  | 752&#8209;754 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `size` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 25 | `empty` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 26 | `singleton` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 27 | `domain` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 28 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 29 | `map` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 30 | `filter` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 31 | `intersection` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 32 | `union` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 33 | `difference` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 34 | `find` | Y | Y |  |  | Y |  |  | hole | 103&#8209;108 |
| 35 | `delete` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 36 | `insert` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 37 | `restrict` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 38 | `subtract` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 39 | `entries` | Y | Y |  |  | Y |  | Y |  | 123 |
| 40 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 423&#8209;426 |
| 41 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 435&#8209;437 |
| 42 | `default` |  | Y |  |  |  | Y | Y |  | 463&#8209;465 |
| 43 | `eq` |  | Y |  |  |  | Y | Y |  | 469&#8209;471 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 44 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 59&#8209;61 |
| 45 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 105&#8209;112 |
| 46 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 146&#8209;152 |
| 47 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 165&#8209;168 |
| 48 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 181&#8209;183 |
| 49 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 191&#8209;194 |
| 50 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 210&#8209;213 |
| 51 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 255&#8209;264 |
| 52 | `size` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 53 | `empty` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;286 |
| 54 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 55 | `domain` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 56 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;300 |
| 57 | `map` | Y | Y |  |  | Y |  |  | hole | 303&#8209;305 |
| 58 | `filter` | Y | Y |  |  | Y |  |  | hole | 308&#8209;310 |
| 59 | `intersection` | Y | Y |  |  | Y |  |  | hole | 313&#8209;315 |
| 60 | `union` | Y | Y |  |  | Y |  |  | hole | 318&#8209;320 |
| 61 | `difference` | Y | Y |  |  | Y |  |  | hole | 323&#8209;324 |
| 62 | `find` | Y | Y |  |  | Y |  |  | hole | 327&#8209;333 |
| 63 | `delete` | Y | Y |  |  | Y |  |  | hole | 336&#8209;338 |
| 64 | `insert` | Y | Y |  |  | Y |  |  | hole | 341&#8209;343 |
| 65 | `restrict` | Y | Y |  |  | Y |  |  | hole | 346&#8209;347 |
| 66 | `subtract` | Y | Y |  |  | Y |  |  | hole | 350&#8209;351 |
| 67 | `collect` | Y | Y |  |  | Y |  | Y |  | 354 |
| 68 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 988&#8209;991 |
| 69 | `eq` |  | Y |  |  |  | Y | Y |  | 1020&#8209;1022 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
