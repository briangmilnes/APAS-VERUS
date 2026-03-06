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
| 2 | Chap42 | TableMtEph | 16 | 17 | 0 | 2 | 19 | 0 | 3 | 16 | 0 |
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 27 | 0 | 24 | 2 | 1 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 10 | 27 | 0 | 24 | 3 | 0 |

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
| 5 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 115&#8209;117 |
| 6 | `size` | Y | Y |  |  | Y |  |  | hole | 132&#8209;133 |
| 7 | `empty` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 8 | `singleton` | Y | Y |  |  | Y |  |  | hole | 138&#8209;139 |
| 9 | `domain` | Y | Y |  |  | Y |  |  | hole | 141&#8209;142 |
| 10 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 144&#8209;146 |
| 11 | `map` | Y | Y |  |  | Y |  |  | hole | 148&#8209;149 |
| 12 | `filter` | Y | Y |  |  | Y |  |  | hole | 151&#8209;154 |
| 13 | `intersection` | Y | Y |  |  | Y |  |  | hole | 156&#8209;157 |
| 14 | `union` | Y | Y |  |  | Y |  |  | hole | 159&#8209;165 |
| 15 | `difference` | Y | Y |  |  | Y |  |  | hole | 167&#8209;170 |
| 16 | `find` | Y | Y |  |  | Y |  |  | hole | 172&#8209;177 |
| 17 | `delete` | Y | Y |  |  | Y |  |  | hole | 179&#8209;180 |
| 18 | `insert` | Y | Y |  |  | Y |  |  | hole | 182&#8209;187 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | hole | 189&#8209;193 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | hole | 195&#8209;199 |
| 21 | `entries` | Y | Y |  |  | Y |  |  | hole | 201&#8209;202 |
| 22 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 816&#8209;817 |
| 23 | `eq` |  | Y |  |  | Y |  |  | hole | 840&#8209;841 |

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
| 35 | `domain` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;310 |
| 36 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;314 |
| 37 | `map` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;318 |
| 38 | `filter` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;327 |
| 39 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;334 |
| 40 | `union` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;342 |
| 41 | `difference` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;351 |
| 42 | `find` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;359 |
| 43 | `delete` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;363 |
| 44 | `insert` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;372 |
| 45 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;381 |
| 46 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;390 |
| 47 | `entries` | Y | Y |  |  | Y |  |  | hole | 393&#8209;394 |
| 48 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1661&#8209;1664 |
| 49 | `default` |  | Y |  |  | Y |  | Y |  | 1679 |
| 50 | `eq` |  | Y |  |  | Y |  |  | hole | 1693&#8209;1694 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 148&#8209;150 |
| 52 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 161&#8209;168 |
| 53 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 202&#8209;208 |
| 54 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 221&#8209;224 |
| 55 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 237&#8209;239 |
| 56 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 247&#8209;250 |
| 57 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 266&#8209;269 |
| 58 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 311&#8209;320 |
| 59 | `size` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;341 |
| 60 | `empty` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;345 |
| 61 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;350 |
| 62 | `domain` | Y | Y |  |  | Y |  |  | unknown | 353&#8209;355 |
| 63 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;360 |
| 64 | `map` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;370 |
| 65 | `filter` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;381 |
| 66 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;392 |
| 67 | `union` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;408 |
| 68 | `difference` | Y | Y |  |  | Y |  |  | unknown | 411&#8209;416 |
| 69 | `find` | Y | Y |  |  | Y |  |  | unknown | 419&#8209;425 |
| 70 | `delete` | Y | Y |  |  | Y |  |  | unknown | 428&#8209;434 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;448 |
| 72 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 451&#8209;456 |
| 73 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 459&#8209;464 |
| 74 | `collect` | Y | Y |  |  | Y |  |  | hole | 467&#8209;468 |
| 75 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1736&#8209;1739 |
| 76 | `collect_by_key` |  |  |  | Y | Y |  |  | hole | 1754&#8209;1762 |
| 77 | `eq` |  | Y |  |  | Y |  |  | hole | 1794&#8209;1795 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
