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
| 2 | Chap42 | TableMtEph | 17 | 18 | 0 | 3 | 21 | 0 | 20 | 0 | 1 |
| 3 | Chap42 | TableSpecsAndLemmas | 0 | 0 | 0 | 14 | 14 | 0 | 14 | 0 | 0 |
| 4 | Chap42 | TableStEph | 19 | 21 | 2 | 1 | 24 | 0 | 22 | 0 | 2 |
| 5 | Chap42 | TableStPer | 19 | 20 | 2 | 6 | 28 | 0 | 27 | 0 | 1 |

## Function-by-Function Detail

### Chap42/Example42_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_example_42_1_verified` |  |  |  | Y | Y |  | Y |  | 12 |
| 2 | `example_42_1` | Y |  |  | Y |  | Y | Y |  | 22&#8209;24 |
| 3 | `demonstrate_table_operations` | Y |  |  |  |  | Y | Y |  | 26&#8209;28 |
| 4 | `performance_comparison` |  |  |  | Y |  | Y | Y |  | 152&#8209;203 |

### Chap42/TableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `size` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 6 | `empty` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 7 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 8 | `domain` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 9 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;133 |
| 10 | `map` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;148 |
| 11 | `filter` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;166 |
| 12 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;183 |
| 13 | `union` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;204 |
| 14 | `difference` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;214 |
| 15 | `find` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;223 |
| 16 | `delete` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 17 | `insert` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;245 |
| 18 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;254 |
| 19 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;263 |
| 20 | `entries` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;267 |
| 21 | `iter` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;274 |
| 22 | `map_table_dc` |  |  |  | Y | Y |  |  | unknown | 284&#8209;298 |
| 23 | `tabulate_table_dc` |  |  |  | Y | Y |  |  | unknown | 423&#8209;437 |
| 24 | `from_sorted_entries` |  |  |  | Y | Y |  | Y |  | 2423 |
| 25 | `eq` |  | Y |  |  | Y |  |  | unknown | 2460&#8209;2461 |

### Chap42/TableSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `lemma_entries_to_map_key_in_seq` |  |  |  | Y | Y |  |  | unknown | 64&#8209;67 |
| 27 | `lemma_entries_to_map_contains_key` |  |  |  | Y | Y |  |  | unknown | 81&#8209;84 |
| 28 | `lemma_entries_to_map_len` |  |  |  | Y | Y |  |  | unknown | 95&#8209;98 |
| 29 | `lemma_entries_to_map_no_key` |  |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 30 | `lemma_entries_to_map_get` |  |  |  | Y | Y |  |  | unknown | 127&#8209;134 |
| 31 | `lemma_entries_to_map_dom_subset` |  |  |  | Y | Y |  |  | unknown | 149&#8209;155 |
| 32 | `lemma_entries_to_map_dom_same_keys` |  |  |  | Y | Y |  |  | unknown | 169&#8209;178 |
| 33 | `lemma_entries_to_map_subseq_value` |  |  |  | Y | Y |  |  | unknown | 190&#8209;209 |
| 34 | `lemma_entries_to_map_skip_prefix` |  |  |  | Y | Y |  |  | unknown | 340&#8209;351 |
| 35 | `lemma_entries_to_map_ignore_suffix` |  |  |  | Y | Y |  |  | unknown | 396&#8209;406 |
| 36 | `lemma_entries_to_map_agree_on_key` |  |  |  | Y | Y |  |  | unknown | 430&#8209;443 |
| 37 | `lemma_subseq_no_dups` |  |  |  | Y | Y |  |  | unknown | 479&#8209;492 |
| 38 | `lemma_subseq_dom_forward` |  |  |  | Y | Y |  |  | unknown | 509&#8209;520 |
| 39 | `lemma_subseq_value_agrees` |  |  |  | Y | Y |  |  | unknown | 537&#8209;555 |

### Chap42/TableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 41 | `empty` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 42 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 43 | `domain` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 44 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;126 |
| 45 | `map` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;141 |
| 46 | `filter` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;160 |
| 47 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;177 |
| 48 | `union` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;198 |
| 49 | `difference` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;209 |
| 50 | `find` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;218 |
| 51 | `find_ref` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;230 |
| 52 | `delete` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;239 |
| 53 | `insert` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;260 |
| 54 | `insert_wf` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;291 |
| 55 | `delete_wf` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;307 |
| 56 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;317 |
| 57 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;327 |
| 58 | `entries` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;332 |
| 59 | `lemma_spec_stored_value_view` |  |  | Y |  | Y |  |  | unknown | 348&#8209;350 |
| 60 | `iter` |  |  | Y |  | Y |  |  | unknown | 376&#8209;380 |
| 61 | `from_sorted_entries` |  |  |  | Y | Y |  | Y |  | 2533&#8209;2535 |
| 62 | `default` |  | Y |  |  | Y |  | Y |  | 2558 |
| 63 | `eq` |  | Y |  |  | Y |  |  | unknown | 2572&#8209;2573 |

### Chap42/TableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_spec_collect_domain_step` |  |  |  | Y | Y |  |  | unknown | 118&#8209;121 |
| 65 | `lemma_spec_collect_key_step` |  |  |  | Y | Y |  |  | unknown | 128&#8209;135 |
| 66 | `lemma_spec_collect_key_not_in_domain` |  |  |  | Y | Y |  |  | unknown | 142&#8209;145 |
| 67 | `lemma_spec_collect_key_len_bound` |  |  |  | Y | Y |  |  | unknown | 153&#8209;155 |
| 68 | `size` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 69 | `empty` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 70 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 71 | `domain` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 72 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;205 |
| 73 | `map` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;221 |
| 74 | `filter` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;241 |
| 75 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;259 |
| 76 | `union` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;281 |
| 77 | `difference` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;290 |
| 78 | `find` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;300 |
| 79 | `find_ref` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;313 |
| 80 | `delete` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;323 |
| 81 | `insert` | Y | Y |  |  | Y |  |  | unknown | 327&#8209;344 |
| 82 | `insert_wf` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;374 |
| 83 | `delete_wf` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;391 |
| 84 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;400 |
| 85 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;409 |
| 86 | `collect` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;414 |
| 87 | `lemma_spec_stored_value_view` |  |  | Y |  | Y |  |  | unknown | 430&#8209;432 |
| 88 | `iter` |  |  | Y |  | Y |  |  | unknown | 450&#8209;454 |
| 89 | `from_sorted_entries` |  |  |  | Y | Y |  | Y |  | 2586&#8209;2588 |
| 90 | `collect_by_key` |  |  |  | Y | Y |  |  | unknown | 2597&#8209;2613 |
| 91 | `eq` |  | Y |  |  | Y |  |  | unknown | 2782&#8209;2783 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
