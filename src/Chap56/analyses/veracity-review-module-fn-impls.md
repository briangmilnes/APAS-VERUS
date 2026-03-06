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
| 1 | Chap56 | AllPairsResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 2 | Chap56 | AllPairsResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 3 | Chap56 | AllPairsResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 4 | Chap56 | AllPairsResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 5 | Chap56 | Example56_1 | 3 | 3 | 0 | 0 | 3 | 0 | 0 | 3 | 0 |
| 6 | Chap56 | Example56_3 | 2 | 2 | 0 | 0 | 2 | 0 | 0 | 2 | 0 |
| 7 | Chap56 | PathWeightUtilsStEph | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 8 | Chap56 | PathWeightUtilsStPer | 4 | 4 | 0 | 0 | 4 | 0 | 2 | 0 | 2 |
| 9 | Chap56 | SSSPResultStEphF64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 1 | 1 |
| 10 | Chap56 | SSSPResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |
| 11 | Chap56 | SSSPResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 1 | 1 |
| 12 | Chap56 | SSSPResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 7 | 0 | 0 |

## Function-by-Function Detail

### Chap56/AllPairsResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 38&#8209;39 |
| 2 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 41 |
| 3 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;45 |
| 4 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 47 |
| 5 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;51 |
| 6 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 53 |
| 7 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 55 |

### Chap56/AllPairsResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `new` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;63 |
| 9 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;69 |
| 10 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;93 |
| 11 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 12 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;124 |
| 13 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;131 |
| 14 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;147 |

### Chap56/AllPairsResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;38 |
| 16 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 40 |
| 17 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;43 |
| 18 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 45 |
| 19 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 20 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 50 |
| 21 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 52 |

### Chap56/AllPairsResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `new` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;62 |
| 23 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;68 |
| 24 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;92 |
| 25 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;99 |
| 26 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;123 |
| 27 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;130 |
| 28 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;146 |

### Chap56/Example56_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `example_path_weight_int` | Y | Y |  |  | Y |  |  | hole | 32 |
| 30 | `example_path_weight_float` | Y | Y |  |  | Y |  |  | hole | 35 |
| 31 | `example_negative_weights` | Y | Y |  |  | Y |  |  | hole | 38 |

### Chap56/Example56_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `example_negative_cycle` | Y | Y |  |  | Y |  |  | hole | 32 |
| 33 | `example_undefined_shortest_path` | Y | Y |  |  | Y |  |  | hole | 35 |

### Chap56/PathWeightUtilsStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 47&#8209;48 |
| 35 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 50&#8209;53 |
| 36 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;67 |
| 37 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 69&#8209;73 |

### Chap56/PathWeightUtilsStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `path_weight_int` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;52 |
| 39 | `path_weight_float` | Y | Y |  |  | Y |  | Y |  | 54&#8209;57 |
| 40 | `validate_subpath_property_int` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;64 |
| 41 | `validate_subpath_property_float` | Y | Y |  |  | Y |  | Y |  | 66&#8209;70 |

### Chap56/SSSPResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `new` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;48 |
| 43 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 50&#8209;53 |
| 44 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;61 |
| 45 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 46 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;75 |
| 47 | `is_reachable` | Y | Y |  |  | Y |  |  | hole | 77&#8209;80 |
| 48 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 82 |

### Chap56/SSSPResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 41&#8209;50 |
| 50 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;55 |
| 51 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;63 |
| 52 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;69 |
| 53 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 54 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;82 |
| 55 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;95 |

### Chap56/SSSPResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `new` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;64 |
| 57 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 58 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 59 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 60 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;91 |
| 61 | `is_reachable` | Y | Y |  |  | Y |  |  | hole | 93&#8209;96 |
| 62 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 98 |

### Chap56/SSSPResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 63 | `new` | Y | Y |  |  | Y |  |  | unknown | 40&#8209;49 |
| 64 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;54 |
| 65 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;61 |
| 66 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 67 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;74 |
| 68 | `is_reachable` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;79 |
| 69 | `extract_path` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;92 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
