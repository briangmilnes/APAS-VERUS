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
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 25 | 2 | 24 | 0 | 3 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 9 | 25 | 1 | 24 | 1 | 1 |

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
| 5 | `size` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 6 | `empty` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 7 | `singleton` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 8 | `domain` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 9 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 10 | `map` | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 11 | `filter` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 12 | `intersection` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 13 | `union` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 14 | `difference` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 15 | `find` | Y | Y |  |  | Y |  |  | hole | 120&#8209;125 |
| 16 | `delete` | Y | Y |  |  | Y |  |  | hole | 127&#8209;128 |
| 17 | `insert` | Y | Y |  |  | Y |  |  | hole | 130&#8209;131 |
| 18 | `restrict` | Y | Y |  |  | Y |  |  | hole | 133&#8209;134 |
| 19 | `subtract` | Y | Y |  |  | Y |  |  | hole | 136&#8209;137 |
| 20 | `entries` | Y | Y |  |  | Y |  | Y |  | 139 |
| 21 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 753&#8209;754 |
| 22 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 763&#8209;765 |
| 23 | `eq` |  | Y |  |  |  | Y | Y |  | 777&#8209;779 |

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
| 34 | `domain` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 35 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;274 |
| 36 | `map` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;278 |
| 37 | `filter` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;282 |
| 38 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;289 |
| 39 | `union` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;297 |
| 40 | `difference` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;301 |
| 41 | `find` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;309 |
| 42 | `delete` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;313 |
| 43 | `insert` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;320 |
| 44 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;324 |
| 45 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;328 |
| 46 | `entries` | Y | Y |  |  | Y |  | Y |  | 331 |
| 47 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1090&#8209;1093 |
| 48 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 1102&#8209;1104 |
| 49 | `default` |  | Y |  |  |  | Y | Y |  | 1130&#8209;1132 |
| 50 | `eq` |  | Y |  |  |  | Y | Y |  | 1136&#8209;1138 |

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
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;305 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;310 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;318 |
| 67 | `union` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;327 |
| 68 | `difference` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;332 |
| 69 | `find` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;341 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;350 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;363 |
| 72 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;368 |
| 73 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;373 |
| 74 | `collect` | Y | Y |  |  | Y |  |  | hole | 376&#8209;377 |
| 75 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1339&#8209;1342 |
| 76 | `eq` |  | Y |  |  |  | Y | Y |  | 1371&#8209;1373 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
