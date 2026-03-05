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
| 2 | Chap42 | TableMtEph | 16 | 17 | 0 | 2 | 19 | 0 | 3 | 15 | 1 |
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 26 | 1 | 24 | 1 | 2 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 9 | 26 | 0 | 24 | 2 | 0 |

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
| 5 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 113&#8209;115 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 130&#8209;131 |
| 7 | `empty` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | hole | 136&#8209;137 |
| 9 | `domain` | Y | Y |  |  | Y |  |  | hole | 139&#8209;140 |
| 10 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 142&#8209;143 |
| 11 | `map` | Y | Y |  |  | Y |  |  | hole | 145&#8209;146 |
| 12 | `filter` | Y | Y |  |  | Y |  |  | hole | 148&#8209;149 |
| 13 | `intersection` | Y | Y |  |  | Y |  |  | hole | 151&#8209;152 |
| 14 | `union` | Y | Y |  |  | Y |  |  | hole | 154&#8209;155 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 157&#8209;158 |
| 16 | `find` | Y | Y |  |  | Y |  |  | hole | 160&#8209;165 |
| 17 | `delete` | Y | Y |  |  | Y |  |  | hole | 167&#8209;168 |
| 18 | `insert` | Y | Y |  |  | Y |  |  | hole | 170&#8209;173 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | hole | 175&#8209;177 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | hole | 179&#8209;181 |
| 21 | `entries` | Y | Y |  |  | Y |  | Y |  | 183 |
| 22 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 795&#8209;796 |
| 23 | `eq` |  | Y |  |  | Y |  |  | hole | 830&#8209;831 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 121&#8209;124 |
| 25 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 140&#8209;143 |
| 26 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 154&#8209;157 |
| 27 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 193&#8209;195 |
| 28 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 203&#8209;210 |
| 29 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 241&#8209;247 |
| 30 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 260&#8209;269 |
| 31 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 278&#8209;280 |
| 32 | `size` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;299 |
| 33 | `empty` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;302 |
| 34 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;306 |
| 35 | `domain` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;309 |
| 36 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;313 |
| 37 | `map` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;317 |
| 38 | `filter` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;321 |
| 39 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 40 | `union` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;336 |
| 41 | `difference` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;340 |
| 42 | `find` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;348 |
| 43 | `delete` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;352 |
| 44 | `insert` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;361 |
| 45 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;365 |
| 46 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;369 |
| 47 | `entries` | Y | Y |  |  | Y |  | Y |  | 372 |
| 48 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1416&#8209;1419 |
| 49 | `eq` |  | Y |  |  | Y |  |  | hole | 1442&#8209;1443 |
| 50 | `default` |  | Y |  |  |  | Y | Y |  | 1481&#8209;1483 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 119&#8209;121 |
| 52 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 132&#8209;139 |
| 53 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 173&#8209;179 |
| 54 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 192&#8209;195 |
| 55 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 208&#8209;210 |
| 56 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 218&#8209;221 |
| 57 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 237&#8209;240 |
| 58 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 282&#8209;291 |
| 59 | `size` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;312 |
| 60 | `empty` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;316 |
| 61 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;321 |
| 62 | `domain` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;325 |
| 63 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;330 |
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;335 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;340 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;348 |
| 67 | `union` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;357 |
| 68 | `difference` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;362 |
| 69 | `find` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;371 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;380 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;393 |
| 72 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;398 |
| 73 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;403 |
| 74 | `collect` | Y | Y |  |  | Y |  |  | hole | 406&#8209;407 |
| 75 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1360&#8209;1363 |
| 76 | `eq` |  | Y |  |  | Y |  |  | hole | 1386&#8209;1387 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
