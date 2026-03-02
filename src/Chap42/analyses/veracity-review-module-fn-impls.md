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
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 25 | 2 | 12 | 12 | 3 |
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
| 24 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 94&#8209;97 |
| 25 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 113&#8209;116 |
| 26 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 127&#8209;130 |
| 27 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 166&#8209;168 |
| 28 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 176&#8209;183 |
| 29 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 214&#8209;220 |
| 30 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 233&#8209;242 |
| 31 | `size` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;260 |
| 32 | `empty` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;263 |
| 33 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 34 | `domain` | Y | Y |  |  | Y |  |  | hole | 269&#8209;270 |
| 35 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 272&#8209;273 |
| 36 | `map` | Y | Y |  |  | Y |  |  | hole | 275&#8209;276 |
| 37 | `filter` | Y | Y |  |  | Y |  |  | hole | 278&#8209;280 |
| 38 | `intersection` | Y | Y |  |  | Y |  |  | hole | 282&#8209;283 |
| 39 | `union` | Y | Y |  |  | Y |  |  | hole | 285&#8209;286 |
| 40 | `difference` | Y | Y |  |  | Y |  |  | hole | 288&#8209;289 |
| 41 | `find` | Y | Y |  |  | Y |  |  | hole | 291&#8209;297 |
| 42 | `delete` | Y | Y |  |  | Y |  |  | hole | 299&#8209;300 |
| 43 | `insert` | Y | Y |  |  | Y |  |  | hole | 302&#8209;303 |
| 44 | `restrict` | Y | Y |  |  | Y |  |  | hole | 305&#8209;306 |
| 45 | `subtract` | Y | Y |  |  | Y |  |  | hole | 308&#8209;309 |
| 46 | `entries` | Y | Y |  |  | Y |  | Y |  | 312 |
| 47 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 719&#8209;722 |
| 48 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 731&#8209;733 |
| 49 | `default` |  | Y |  |  |  | Y | Y |  | 759&#8209;761 |
| 50 | `eq` |  | Y |  |  |  | Y | Y |  | 765&#8209;767 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 59&#8209;61 |
| 52 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 105&#8209;112 |
| 53 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 146&#8209;152 |
| 54 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 165&#8209;168 |
| 55 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 181&#8209;183 |
| 56 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 191&#8209;194 |
| 57 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 210&#8209;213 |
| 58 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 255&#8209;264 |
| 59 | `size` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 60 | `empty` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;286 |
| 61 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;291 |
| 62 | `domain` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;295 |
| 63 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;300 |
| 64 | `map` | Y | Y |  |  | Y |  |  | hole | 303&#8209;305 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | hole | 308&#8209;310 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | hole | 313&#8209;315 |
| 67 | `union` | Y | Y |  |  | Y |  |  | hole | 318&#8209;320 |
| 68 | `difference` | Y | Y |  |  | Y |  |  | hole | 323&#8209;324 |
| 69 | `find` | Y | Y |  |  | Y |  |  | hole | 327&#8209;333 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | hole | 336&#8209;338 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | hole | 341&#8209;343 |
| 72 | `restrict` | Y | Y |  |  | Y |  |  | hole | 346&#8209;347 |
| 73 | `subtract` | Y | Y |  |  | Y |  |  | hole | 350&#8209;351 |
| 74 | `collect` | Y | Y |  |  | Y |  | Y |  | 354 |
| 75 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 988&#8209;991 |
| 76 | `eq` |  | Y |  |  |  | Y | Y |  | 1020&#8209;1022 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
