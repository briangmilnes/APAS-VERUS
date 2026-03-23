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
| 5 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 119&#8209;121 |
| 6 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 129&#8209;132 |
| 7 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 148&#8209;151 |
| 8 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 162&#8209;165 |
| 9 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 201&#8209;203 |
| 10 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 211&#8209;218 |
| 11 | `lemma_entries_to_map_subseq_value` |  |  |  | Y | Y |  |  | unknown | 251&#8209;270 |
| 12 | `lemma_entries_to_map_skip_prefix` |  |  |  | Y | Y |  |  | unknown | 395&#8209;406 |
| 13 | `lemma_entries_to_map_ignore_suffix` |  |  |  | Y | Y |  |  | unknown | 448&#8209;458 |
| 14 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 478&#8209;487 |
| 15 | `lemma_entries_to_map_agree_on_key` |  |  |  | Y | Y |  |  | unknown | 497&#8209;510 |
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 550&#8209;552 |
| 17 | `empty` | Y | Y |  |  | Y |  |  | unknown | 555&#8209;556 |
| 18 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 559&#8209;560 |
| 19 | `domain` | Y | Y |  |  | Y |  |  | unknown | 563&#8209;564 |
| 20 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 567&#8209;574 |
| 21 | `map` | Y | Y |  |  | Y |  |  | unknown | 577&#8209;583 |
| 22 | `filter` | Y | Y |  |  | Y |  |  | unknown | 586&#8209;601 |
| 23 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 604&#8209;607 |
| 24 | `union` | Y | Y |  |  | Y |  |  | unknown | 610&#8209;618 |
| 25 | `difference` | Y | Y |  |  | Y |  |  | unknown | 621&#8209;624 |
| 26 | `find` | Y | Y |  |  | Y |  |  | unknown | 627&#8209;633 |
| 27 | `delete` | Y | Y |  |  | Y |  |  | unknown | 636&#8209;637 |
| 28 | `insert` | Y | Y |  |  | Y |  |  | unknown | 640&#8209;651 |
| 29 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 654&#8209;658 |
| 30 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 661&#8209;665 |
| 31 | `entries` | Y | Y |  |  | Y |  |  | unknown | 667&#8209;668 |
| 32 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 2174&#8209;2175 |
| 33 | `eq` |  | Y |  |  | Y |  |  | hole | 2198&#8209;2199 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 122&#8209;125 |
| 35 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 36 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 155&#8209;158 |
| 37 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 194&#8209;196 |
| 38 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 204&#8209;211 |
| 39 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 242&#8209;248 |
| 40 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 261&#8209;270 |
| 41 | `lemma_entries_to_map_finite` |  |  |  | Y | Y |  |  | unknown | 279&#8209;281 |
| 42 | `size` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;301 |
| 43 | `empty` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;305 |
| 44 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;310 |
| 45 | `domain` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;315 |
| 46 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;326 |
| 47 | `map` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;340 |
| 48 | `filter` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;358 |
| 49 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;374 |
| 50 | `union` | Y | Y |  |  | Y |  |  | unknown | 377&#8209;395 |
| 51 | `difference` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;405 |
| 52 | `find` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;414 |
| 53 | `delete` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;419 |
| 54 | `insert` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;436 |
| 55 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 439&#8209;446 |
| 56 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 449&#8209;456 |
| 57 | `entries` | Y | Y |  |  | Y |  |  | unknown | 459&#8209;460 |
| 58 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 2082&#8209;2085 |
| 59 | `default` |  | Y |  |  | Y |  | Y |  | 2100 |
| 60 | `eq` |  | Y |  |  | Y |  |  | hole | 2114&#8209;2115 |

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
