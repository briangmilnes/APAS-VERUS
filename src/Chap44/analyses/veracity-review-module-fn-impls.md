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
| 1 | Chap44 | DocumentIndex | 15 | 16 | 0 | 2 | 17 | 1 | 16 | 1 | 1 |
| 2 | Chap44 | Example44_1 | 0 | 1 | 12 | 9 | 1 | 21 | 0 | 0 | 22 |

## Function-by-Function Detail

### Chap44/DocumentIndex.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `make_index` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;101 |
| 2 | `find` x3 | Y | Y |  |  | Y |  |  | unknown | 481&#8209;489 |
| 3 | `query_and` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;126 |
| 4 | `query_or` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;140 |
| 5 | `query_and_not` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;153 |
| 6 | `size` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 7 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 8 | `empty` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;168 |
| 9 | `get_all_words` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 10 | `word_count` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;177 |
| 11 | `new` | Y | Y |  |  | Y |  |  | unknown | 475&#8209;477 |
| 12 | `and` | Y | Y |  |  | Y |  |  | unknown | 492&#8209;501 |
| 13 | `or` | Y | Y |  |  | Y |  |  | unknown | 504&#8209;514 |
| 14 | `and_not` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;526 |
| 15 | `complex_query` | Y | Y |  |  | Y |  |  | unknown | 529&#8209;536 |
| 16 | `tokens` |  |  |  | Y | Y |  |  | unknown | 598&#8209;599 |
| 17 | `eq` |  | Y |  |  | Y |  |  | hole | 650 |
| 18 | `create_finder` |  |  |  | Y |  | Y | Y |  | 686&#8209;690 |

### Chap44/Example44_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `_example_44_1_verified` |  |  |  | Y | Y |  | Y |  | 17 |
| 20 | `create_tweet_collection` |  |  |  | Y |  | Y | Y |  | 20&#8209;29 |
| 21 | `create_tweet_index` |  |  |  | Y |  | Y | Y |  | 31&#8209;36 |
| 22 | `create_tweet_finder` |  |  |  | Y |  | Y | Y |  | 38&#8209;44 |
| 23 | `default` |  | Y |  |  |  | Y | Y |  | 54 |
| 24 | `new` |  |  | Y |  |  | Y | Y |  | 58&#8209;65 |
| 25 | `search_fun` |  |  | Y |  |  | Y | Y |  | 67&#8209;69 |
| 26 | `search_club` |  |  | Y |  |  | Y | Y |  | 71&#8209;73 |
| 27 | `search_food` |  |  | Y |  |  | Y | Y |  | 75&#8209;77 |
| 28 | `search_chess` |  |  | Y |  |  | Y | Y |  | 79&#8209;81 |
| 29 | `complex_query_fun_and_food_or_chess` |  |  | Y |  |  | Y | Y |  | 83&#8209;96 |
| 30 | `count_fun_but_not_chess` |  |  | Y |  |  | Y | Y |  | 98&#8209;108 |
| 31 | `search_food_or_fun` |  |  | Y |  |  | Y | Y |  | 110&#8209;117 |
| 32 | `search_party_and_food` |  |  | Y |  |  | Y | Y |  | 119&#8209;126 |
| 33 | `get_all_words` |  |  | Y |  |  | Y | Y |  | 128&#8209;130 |
| 34 | `get_word_count` |  |  | Y |  |  | Y | Y |  | 132&#8209;134 |
| 35 | `query_builder_example` |  |  | Y |  |  | Y | Y |  | 136&#8209;148 |
| 36 | `doc_set_to_sorted_vec` |  |  |  | Y |  | Y | Y |  | 151&#8209;163 |
| 37 | `verify_textbook_examples` |  |  |  | Y |  | Y | Y |  | 165&#8209;204 |
| 38 | `performance_comparison_demo` |  |  |  | Y |  | Y | Y |  | 206&#8209;219 |
| 39 | `tokenization_demo` |  |  |  | Y |  | Y | Y |  | 235&#8209;240 |
| 40 | `index_statistics` |  |  |  | Y |  | Y | Y |  | 242&#8209;260 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
