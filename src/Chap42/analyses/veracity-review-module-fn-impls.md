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
| 2 | Chap42 | TableMtEph | 16 | 17 | 0 | 12 | 29 | 0 | 28 | 1 | 0 |
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 27 | 0 | 25 | 1 | 1 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 14 | 31 | 0 | 30 | 1 | 0 |

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
| 5 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 116&#8209;118 |
| 6 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 126&#8209;129 |
| 7 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 145&#8209;148 |
| 8 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 159&#8209;162 |
| 9 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |
| 10 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 208&#8209;215 |
| 11 | `lemma_entries_to_map_subseq_value` |  |  |  | Y | Y |  |  | unknown | 248&#8209;267 |
| 12 | `lemma_entries_to_map_skip_prefix` |  |  |  | Y | Y |  |  | unknown | 392&#8209;403 |
| 13 | `lemma_entries_to_map_ignore_suffix` |  |  |  | Y | Y |  |  | unknown | 445&#8209;455 |
| 14 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 475&#8209;484 |
| 15 | `lemma_entries_to_map_agree_on_key` |  |  |  | Y | Y |  |  | unknown | 494&#8209;507 |
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 546&#8209;548 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 551&#8209;552 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 555&#8209;556 |
| 19 | `domain` | Y | Y |  |  | Y |  |  | unknown | 559&#8209;560 |
| 20 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 563&#8209;570 |
| 21 | `map` | Y | Y |  |  | Y |  |  | unknown | 573&#8209;579 |
| 22 | `filter` | Y | Y |  |  | Y |  |  | unknown | 582&#8209;596 |
| 23 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 599&#8209;602 |
| 24 | `union` | Y | Y |  |  | Y |  |  | unknown | 605&#8209;613 |
| 25 | `difference` | Y | Y |  |  | Y |  |  | unknown | 616&#8209;619 |
| 26 | `find` | Y | Y |  |  | Y |  |  | unknown | 622&#8209;628 |
| 27 | `delete` | Y | Y |  |  | Y |  |  | unknown | 631&#8209;632 |
| 28 | `insert` | Y | Y |  |  | Y |  |  | unknown | 635&#8209;645 |
| 29 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 648&#8209;652 |
| 30 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 655&#8209;659 |
| 31 | `entries` | Y | Y |  |  | Y |  |  | unknown | 661&#8209;662 |
| 32 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 2176&#8209;2177 |
| 33 | `eq` |  | Y |  |  | Y |  |  | hole | 2197&#8209;2198 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 117&#8209;120 |
| 35 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 136&#8209;139 |
| 36 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 150&#8209;153 |
| 37 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 189&#8209;191 |
| 38 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 199&#8209;206 |
| 39 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 237&#8209;243 |
| 40 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 256&#8209;265 |
| 41 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 274&#8209;276 |
| 42 | `size` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;295 |
| 43 | `empty` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;299 |
| 44 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;304 |
| 45 | `domain` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;309 |
| 46 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;320 |
| 47 | `map` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;334 |
| 48 | `filter` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;351 |
| 49 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;367 |
| 50 | `union` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;387 |
| 51 | `difference` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;396 |
| 52 | `find` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;405 |
| 53 | `delete` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;410 |
| 54 | `insert` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;426 |
| 55 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;435 |
| 56 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;444 |
| 57 | `entries` | Y | Y |  |  | Y |  |  | unknown | 447&#8209;448 |
| 58 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 2073&#8209;2076 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 2088 |
| 60 | `eq` |  | Y |  |  | Y |  |  | hole | 2102&#8209;2103 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 61 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 149&#8209;151 |
| 62 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 162&#8209;169 |
| 63 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 203&#8209;209 |
| 64 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 222&#8209;225 |
| 65 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 238&#8209;240 |
| 66 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 248&#8209;251 |
| 67 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 267&#8209;270 |
| 68 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 312&#8209;321 |
| 69 | `lemma_spec_collect_domain_step` |  |  |  | Y | Y |  |  | unknown | 331&#8209;334 |
| 70 | `lemma_spec_collect_key_step` |  |  |  | Y | Y |  |  | unknown | 340&#8209;347 |
| 71 | `lemma_spec_collect_key_not_in_domain` |  |  |  | Y | Y |  |  | unknown | 353&#8209;356 |
| 72 | `lemma_spec_collect_key_len_bound` |  |  |  | Y | Y |  |  | unknown | 364&#8209;366 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;386 |
| 74 | `empty` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;391 |
| 75 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;397 |
| 76 | `domain` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;403 |
| 77 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 407&#8209;415 |
| 78 | `map` | Y | Y |  |  | Y |  |  | unknown | 419&#8209;431 |
| 79 | `filter` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;451 |
| 80 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;469 |
| 81 | `union` | Y | Y |  |  | Y |  |  | unknown | 473&#8209;491 |
| 82 | `difference` | Y | Y |  |  | Y |  |  | unknown | 495&#8209;500 |
| 83 | `find` | Y | Y |  |  | Y |  |  | unknown | 504&#8209;510 |
| 84 | `delete` | Y | Y |  |  | Y |  |  | unknown | 514&#8209;520 |
| 85 | `insert` | Y | Y |  |  | Y |  |  | unknown | 524&#8209;538 |
| 86 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 542&#8209;547 |
| 87 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 551&#8209;556 |
| 88 | `collect` | Y | Y |  |  | Y |  |  | unknown | 560&#8209;561 |
| 89 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 2018&#8209;2021 |
| 90 | `collect_by_key` |  |  |  | Y | Y |  |  | unknown | 2035&#8209;2050 |
| 91 | `eq` |  | Y |  |  | Y |  |  | hole | 2190&#8209;2191 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
