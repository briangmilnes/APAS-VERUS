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
| 1 | Chap44 | DocumentIndex | 15 | 16 | 0 | 3 | 2 | 17 | 1 | 0 | 18 |
| 2 | Chap44 | Example44_1 | 0 | 1 | 12 | 9 | 1 | 21 | 0 | 0 | 22 |

## Function-by-Function Detail

### Chap44/DocumentIndex.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `_document_index_verified` |  |  |  | Y | Y |  | Y |  | 25 |
| 2 | `eq` |  | Y |  |  | Y |  |  | unknown | 41&#8209;42 |
| 3 | `make_index` | Y | Y |  |  |  | Y | Y |  | 53&#8209;55 |
| 4 | `find` x3 | Y | Y |  |  |  | Y | Y |  | 285&#8209;287 |
| 5 | `query_and` | Y | Y |  |  |  | Y | Y |  | 61&#8209;63 |
| 6 | `query_or` | Y | Y |  |  |  | Y | Y |  | 65&#8209;67 |
| 7 | `query_and_not` | Y | Y |  |  |  | Y | Y |  | 69&#8209;71 |
| 8 | `size` | Y | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 9 | `to_seq` | Y | Y |  |  |  | Y | Y |  | 77&#8209;79 |
| 10 | `empty` | Y | Y |  |  |  | Y | Y |  | 81&#8209;83 |
| 11 | `get_all_words` | Y | Y |  |  |  | Y | Y |  | 85&#8209;87 |
| 12 | `word_count` | Y | Y |  |  |  | Y | Y |  | 89&#8209;91 |
| 13 | `tokens` |  |  |  | Y |  | Y | Y |  | 195&#8209;222 |
| 14 | `create_finder` |  |  |  | Y |  | Y | Y |  | 224&#8209;229 |
| 15 | `new` | Y | Y |  |  |  | Y | Y |  | 281&#8209;283 |
| 16 | `and` | Y | Y |  |  |  | Y | Y |  | 289&#8209;291 |
| 17 | `or` | Y | Y |  |  |  | Y | Y |  | 293&#8209;295 |
| 18 | `and_not` | Y | Y |  |  |  | Y | Y |  | 297&#8209;299 |
| 19 | `complex_query` | Y | Y |  |  |  | Y | Y |  | 301&#8209;303 |

### Chap44/Example44_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `_example_44_1_verified` |  |  |  | Y | Y |  | Y |  | 14 |
| 21 | `create_tweet_collection` |  |  |  | Y |  | Y | Y |  | 17&#8209;27 |
| 22 | `create_tweet_index` |  |  |  | Y |  | Y | Y |  | 29&#8209;35 |
| 23 | `create_tweet_finder` |  |  |  | Y |  | Y | Y |  | 37&#8209;44 |
| 24 | `default` |  | Y |  |  |  | Y | Y |  | 56 |
| 25 | `new` |  |  | Y |  |  | Y | Y |  | 60&#8209;68 |
| 26 | `search_fun` |  |  | Y |  |  | Y | Y |  | 70&#8209;73 |
| 27 | `search_club` |  |  | Y |  |  | Y | Y |  | 75&#8209;78 |
| 28 | `search_food` |  |  | Y |  |  | Y | Y |  | 80&#8209;83 |
| 29 | `search_chess` |  |  | Y |  |  | Y | Y |  | 85&#8209;88 |
| 30 | `complex_query_fun_and_food_or_chess` |  |  | Y |  |  | Y | Y |  | 90&#8209;104 |
| 31 | `count_fun_but_not_chess` |  |  | Y |  |  | Y | Y |  | 106&#8209;117 |
| 32 | `search_food_or_fun` |  |  | Y |  |  | Y | Y |  | 119&#8209;127 |
| 33 | `search_party_and_food` |  |  | Y |  |  | Y | Y |  | 129&#8209;137 |
| 34 | `get_all_words` |  |  | Y |  |  | Y | Y |  | 139&#8209;142 |
| 35 | `get_word_count` |  |  | Y |  |  | Y | Y |  | 144&#8209;147 |
| 36 | `query_builder_example` |  |  | Y |  |  | Y | Y |  | 149&#8209;162 |
| 37 | `doc_set_to_sorted_vec` |  |  |  | Y |  | Y | Y |  | 165&#8209;178 |
| 38 | `verify_textbook_examples` |  |  |  | Y |  | Y | Y |  | 180&#8209;220 |
| 39 | `performance_comparison_demo` |  |  |  | Y |  | Y | Y |  | 222&#8209;236 |
| 40 | `tokenization_demo` |  |  |  | Y |  | Y | Y |  | 238&#8209;244 |
| 41 | `index_statistics` |  |  |  | Y |  | Y | Y |  | 246&#8209;265 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
