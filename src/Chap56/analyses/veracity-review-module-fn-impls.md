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
| 2 | Chap56 | AllPairsResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 0 | 2 |
| 3 | Chap56 | AllPairsResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 4 | Chap56 | AllPairsResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 5 | 0 | 2 |
| 5 | Chap56 | Example56_1 | 3 | 0 | 0 | 3 | 3 | 0 | 0 | 3 | 0 |
| 6 | Chap56 | Example56_3 | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 2 | 0 |
| 7 | Chap56 | PathWeightUtilsStEph | 4 | 0 | 0 | 4 | 4 | 0 | 0 | 0 | 4 |
| 8 | Chap56 | PathWeightUtilsStPer | 4 | 0 | 0 | 4 | 4 | 0 | 0 | 0 | 4 |
| 9 | Chap56 | SSSPResultStEphF64 | 0 | 0 | 7 | 0 | 6 | 1 | 1 | 0 | 6 |
| 10 | Chap56 | SSSPResultStEphI64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 11 | Chap56 | SSSPResultStPerF64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |
| 12 | Chap56 | SSSPResultStPerI64 | 7 | 7 | 0 | 0 | 7 | 0 | 3 | 0 | 4 |

## Function-by-Function Detail

### Chap56/AllPairsResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 36 |
| 2 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 38 |
| 3 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 40 |
| 4 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 42 |
| 5 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 44 |
| 6 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 46 |
| 7 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 48 |

### Chap56/AllPairsResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `new` | Y | Y |  |  | Y |  |  | unknown | 26 |
| 9 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 28 |
| 10 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 30 |
| 11 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 32 |
| 12 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 34 |
| 13 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 36 |
| 14 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 38 |

### Chap56/AllPairsResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `new` | Y | Y |  |  | Y |  |  | unknown | 35 |
| 16 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 37 |
| 17 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 39 |
| 18 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 41 |
| 19 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 43 |
| 20 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 45 |
| 21 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 47 |

### Chap56/AllPairsResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 22 | `new` | Y | Y |  |  | Y |  |  | unknown | 25 |
| 23 | `get_distance` | Y | Y |  |  | Y |  |  | unknown | 27 |
| 24 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 29 |
| 25 | `get_predecessor` | Y | Y |  |  | Y |  |  | unknown | 31 |
| 26 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 33 |
| 27 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 35 |
| 28 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 37 |

### Chap56/Example56_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `example_path_weight_int` | Y |  |  | Y | Y |  |  | hole | 28 |
| 30 | `example_path_weight_float` | Y |  |  | Y | Y |  |  | hole | 32 |
| 31 | `example_negative_weights` | Y |  |  | Y | Y |  |  | hole | 36 |

### Chap56/Example56_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `example_negative_cycle` | Y |  |  | Y | Y |  |  | hole | 28 |
| 33 | `example_undefined_shortest_path` | Y |  |  | Y | Y |  |  | hole | 32 |

### Chap56/PathWeightUtilsStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `path_weight_int` | Y |  |  | Y | Y |  | Y |  | 37 |
| 35 | `path_weight_float` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 36 | `validate_subpath_property_int` | Y |  |  | Y | Y |  | Y |  | 44&#8209;48 |
| 37 | `validate_subpath_property_float` | Y |  |  | Y | Y |  | Y |  | 50&#8209;54 |

### Chap56/PathWeightUtilsStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `path_weight_int` | Y |  |  | Y | Y |  | Y |  | 36 |
| 39 | `path_weight_float` | Y |  |  | Y | Y |  | Y |  | 38&#8209;41 |
| 40 | `validate_subpath_property_int` | Y |  |  | Y | Y |  | Y |  | 43&#8209;47 |
| 41 | `validate_subpath_property_float` | Y |  |  | Y | Y |  | Y |  | 49&#8209;53 |

### Chap56/SSSPResultStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `new` |  |  | Y |  | Y |  |  | unknown | 36&#8209;37 |
| 43 | `get_distance` |  |  | Y |  | Y |  | Y |  | 62 |
| 44 | `set_distance` |  |  | Y |  | Y |  | Y |  | 69 |
| 45 | `get_predecessor` |  |  | Y |  | Y |  | Y |  | 75 |
| 46 | `set_predecessor` |  |  | Y |  | Y |  | Y |  | 83 |
| 47 | `is_reachable` |  |  | Y |  | Y |  | Y |  | 89 |
| 48 | `extract_path` |  |  | Y |  |  | Y | Y |  | 97&#8209;112 |

### Chap56/SSSPResultStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 27&#8209;28 |
| 50 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 30 |
| 51 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 32 |
| 52 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 34 |
| 53 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 36 |
| 54 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 38 |
| 55 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 40 |

### Chap56/SSSPResultStPerF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `new` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;54 |
| 57 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 56 |
| 58 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 58 |
| 59 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 60 |
| 60 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 62 |
| 61 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 64 |
| 62 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 66 |

### Chap56/SSSPResultStPerI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 63 | `new` | Y | Y |  |  | Y |  |  | unknown | 25&#8209;26 |
| 64 | `get_distance` | Y | Y |  |  | Y |  | Y |  | 28 |
| 65 | `set_distance` | Y | Y |  |  | Y |  |  | unknown | 30 |
| 66 | `get_predecessor` | Y | Y |  |  | Y |  | Y |  | 32 |
| 67 | `set_predecessor` | Y | Y |  |  | Y |  |  | unknown | 34 |
| 68 | `is_reachable` | Y | Y |  |  | Y |  | Y |  | 36 |
| 69 | `extract_path` | Y | Y |  |  | Y |  | Y |  | 38 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
