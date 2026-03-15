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
| 2 | Chap42 | TableMtEph | 16 | 17 | 0 | 11 | 28 | 0 | 24 | 4 | 0 |
| 3 | Chap42 | TableStEph | 16 | 18 | 0 | 9 | 27 | 0 | 26 | 0 | 1 |
| 4 | Chap42 | TableStPer | 16 | 17 | 0 | 14 | 31 | 0 | 31 | 0 | 0 |

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
| 5 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 120&#8209;122 |
| 6 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 130&#8209;133 |
| 7 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 149&#8209;152 |
| 8 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 163&#8209;166 |
| 9 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 202&#8209;204 |
| 10 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 212&#8209;219 |
| 11 | `lemma_entries_to_map_subseq_value` |  |  |  | Y | Y |  |  | unknown | 252&#8209;271 |
| 12 | `lemma_entries_to_map_skip_prefix` |  |  |  | Y | Y |  |  | unknown | 396&#8209;407 |
| 13 | `lemma_entries_to_map_ignore_suffix` |  |  |  | Y | Y |  |  | unknown | 449&#8209;459 |
| 14 | `lemma_entries_to_map_agree_on_key` |  |  |  | Y | Y |  |  | unknown | 479&#8209;492 |
| 15 | `size` | Y | Y |  |  | Y |  |  | unknown | 531&#8209;533 |
| 16 | `empty` | Y | Y |  |  | Y |  |  | unknown | 535&#8209;536 |
| 17 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 538&#8209;539 |
| 18 | `domain` | Y | Y |  |  | Y |  |  | unknown | 541&#8209;542 |
| 19 | `tabulate` | Y | Y |  |  | Y |  |  | hole | 544&#8209;546 |
| 20 | `map` | Y | Y |  |  | Y |  |  | hole | 548&#8209;549 |
| 21 | `filter` | Y | Y |  |  | Y |  |  | hole | 551&#8209;554 |
| 22 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 556&#8209;559 |
| 23 | `union` | Y | Y |  |  | Y |  |  | unknown | 561&#8209;569 |
| 24 | `difference` | Y | Y |  |  | Y |  |  | unknown | 571&#8209;574 |
| 25 | `find` | Y | Y |  |  | Y |  |  | unknown | 576&#8209;582 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | unknown | 584&#8209;585 |
| 27 | `insert` | Y | Y |  |  | Y |  |  | hole | 587&#8209;594 |
| 28 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 596&#8209;600 |
| 29 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 602&#8209;606 |
| 30 | `entries` | Y | Y |  |  | Y |  |  | unknown | 608&#8209;609 |
| 31 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1915&#8209;1917 |
| 32 | `eq` |  | Y |  |  | Y |  |  | unknown | 1940&#8209;1941 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 123&#8209;126 |
| 34 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 142&#8209;145 |
| 35 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 156&#8209;159 |
| 36 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 195&#8209;197 |
| 37 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 205&#8209;212 |
| 38 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 243&#8209;249 |
| 39 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 262&#8209;271 |
| 40 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 280&#8209;282 |
| 41 | `size` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;301 |
| 42 | `empty` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;304 |
| 43 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;308 |
| 44 | `domain` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;312 |
| 45 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;316 |
| 46 | `map` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;320 |
| 47 | `filter` | Y | Y |  |  | Y |  |  | unknown | 322&#8209;329 |
| 48 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;336 |
| 49 | `union` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;344 |
| 50 | `difference` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;353 |
| 51 | `find` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;361 |
| 52 | `delete` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;365 |
| 53 | `insert` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;374 |
| 54 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;383 |
| 55 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 385&#8209;392 |
| 56 | `entries` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;396 |
| 57 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1663&#8209;1667 |
| 58 | `default` |  | Y |  |  | Y |  | Y |  | 1682 |
| 59 | `eq` |  | Y |  |  | Y |  |  | unknown | 1696&#8209;1697 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 60 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 150&#8209;152 |
| 61 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 163&#8209;170 |
| 62 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 204&#8209;210 |
| 63 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 223&#8209;226 |
| 64 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 239&#8209;241 |
| 65 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 249&#8209;252 |
| 66 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 268&#8209;271 |
| 67 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 313&#8209;322 |
| 68 | `lemma_spec_collect_domain_step` |  |  |  | Y | Y |  |  | unknown | 332&#8209;335 |
| 69 | `lemma_spec_collect_key_step` |  |  |  | Y | Y |  |  | unknown | 341&#8209;348 |
| 70 | `lemma_spec_collect_key_not_in_domain` |  |  |  | Y | Y |  |  | unknown | 354&#8209;357 |
| 71 | `lemma_spec_collect_key_len_bound` |  |  |  | Y | Y |  |  | unknown | 365&#8209;367 |
| 72 | `size` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;386 |
| 73 | `empty` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;390 |
| 74 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 393&#8209;395 |
| 75 | `domain` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;400 |
| 76 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;405 |
| 77 | `map` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;415 |
| 78 | `filter` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;426 |
| 79 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;437 |
| 80 | `union` | Y | Y |  |  | Y |  |  | unknown | 440&#8209;453 |
| 81 | `difference` | Y | Y |  |  | Y |  |  | unknown | 456&#8209;461 |
| 82 | `find` | Y | Y |  |  | Y |  |  | unknown | 464&#8209;470 |
| 83 | `delete` | Y | Y |  |  | Y |  |  | unknown | 473&#8209;479 |
| 84 | `insert` | Y | Y |  |  | Y |  |  | unknown | 482&#8209;496 |
| 85 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 499&#8209;504 |
| 86 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 507&#8209;512 |
| 87 | `collect` | Y | Y |  |  | Y |  |  | unknown | 515&#8209;516 |
| 88 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1802&#8209;1806 |
| 89 | `collect_by_key` |  |  |  | Y | Y |  |  | unknown | 1820&#8209;1834 |
| 90 | `eq` |  | Y |  |  | Y |  |  | unknown | 1974&#8209;1975 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
