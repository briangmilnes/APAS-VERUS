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
| 1 | Chap59 | JohnsonMtEphI64 | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 2 | Chap59 | JohnsonStEphI64 | 1 | 0 | 0 | 4 | 1 | 3 | 0 | 0 | 4 |

## Function-by-Function Detail

### Chap59/JohnsonMtEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 37 |
| 2 | `parallel_dijkstra_all` |  |  |  | Y |  | Y | Y |  | 79&#8209;138 |
| 3 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 140&#8209;164 |
| 4 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 166&#8209;190 |
| 5 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 192&#8209;205 |

### Chap59/JohnsonStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 34 |
| 7 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 100&#8209;126 |
| 8 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 128&#8209;152 |
| 9 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 154&#8209;167 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
