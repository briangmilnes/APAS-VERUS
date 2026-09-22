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
| 1 | Chap56 | AllPairsResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 2 | Chap56 | AllPairsResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 3 | Chap56 | AllPairsResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 4 | Chap56 | AllPairsResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 5 | Chap56 | Example56_1 | 3 | 3 | 0 | 0 | 3 | 0 | 0 | 0 | 3 |
| 6 | Chap56 | Example56_3 | 2 | 2 | 0 | 0 | 2 | 0 | 0 | 0 | 2 |
| 7 | Chap56 | PathWeightUtilsStEph | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 8 | Chap56 | PathWeightUtilsStPer | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 9 | Chap56 | SSSPResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 10 | Chap56 | SSSPResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 11 | Chap56 | SSSPResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 12 | Chap56 | SSSPResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap56/AllPairsResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;60 |
| 2 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 3 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;73 |
| 4 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 5 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;86 |
| 6 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 7 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |

### Chap56/AllPairsResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `new` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;80 |
| 9 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;88 |
| 10 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;114 |
| 11 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;123 |
| 12 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;149 |
| 13 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;158 |
| 14 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;176 |

### Chap56/AllPairsResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;59 |
| 16 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 17 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;72 |
| 18 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 19 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;85 |
| 20 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 21 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |

### Chap56/AllPairsResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `new` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;79 |
| 23 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 24 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;113 |
| 25 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 26 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;148 |
| 27 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;157 |
| 28 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;175 |

### Chap56/Example56_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `example_path_weight_int` | Y | Y |  |  | Y |  | Y |  | 32 |
| 30 | `example_path_weight_i64` | Y | Y |  |  | Y |  | Y |  | 35 |
| 31 | `example_negative_weights` | Y | Y |  |  | Y |  | Y |  | 38 |

### Chap56/Example56_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `example_negative_cycle` | Y | Y |  |  | Y |  | Y |  | 33 |
| 33 | `example_undefined_shortest_path` | Y | Y |  |  | Y |  | Y |  | 36 |

### Chap56/PathWeightUtilsStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 35 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 69&#8209;72 |
| 36 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;87 |
| 37 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 90&#8209;94 |

### Chap56/PathWeightUtilsStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 39 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 72&#8209;75 |
| 40 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;83 |
| 41 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 86&#8209;90 |

### Chap56/SSSPResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `new` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;74 |
| 43 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 44 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;94 |
| 45 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 46 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;115 |
| 47 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;123 |
| 48 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |

### Chap56/SSSPResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;67 |
| 50 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;74 |
| 51 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;84 |
| 52 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 53 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;102 |
| 54 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;109 |
| 55 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;124 |

### Chap56/SSSPResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `new` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;89 |
| 57 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;97 |
| 58 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;109 |
| 59 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 60 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;130 |
| 61 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 62 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |

### Chap56/SSSPResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 63 | `new` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;66 |
| 64 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 65 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;82 |
| 66 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 67 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 68 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;106 |
| 69 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;121 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
