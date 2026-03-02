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
| 1 | Chap05 | KleeneStPer | 4 | 4 | 0 | 9 | 13 | 0 | 13 | 0 | 0 |
| 2 | Chap05 | MappingStEph | 13 | 16 | 0 | 0 | 16 | 0 | 15 | 0 | 1 |
| 3 | Chap05 | RelationStEph | 9 | 12 | 0 | 0 | 12 | 0 | 11 | 0 | 1 |
| 4 | Chap05 | SetMtEph | 17 | 20 | 0 | 1 | 21 | 0 | 20 | 0 | 1 |
| 5 | Chap05 | SetStEph | 18 | 21 | 0 | 1 | 22 | 0 | 21 | 0 | 1 |

## Function-by-Function Detail

### Chap05/KleeneStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_star_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 83&#8209;88 |
| 2 | `lemma_plus_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 101&#8209;106 |
| 3 | `ptt_star_contains_empty` |  |  |  | Y | Y |  |  | unknown | 113&#8209;114 |
| 4 | `ptt_plus_rejects_empty` |  |  |  | Y | Y |  |  | unknown | 119&#8209;120 |
| 5 | `ptt_singleton_in_star_and_plus` |  |  |  | Y | Y |  |  | unknown | 125&#8209;129 |
| 6 | `ptt_plus_subset_of_star` |  |  |  | Y | Y |  |  | unknown | 134&#8209;136 |
| 7 | `ptt_star_property_transfer` |  |  |  | Y | Y |  |  | unknown | 142&#8209;151 |
| 8 | `ptt_star_concat_plus_is_plus` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 9 | `ptt_plus_concat_star_is_plus` |  |  |  | Y | Y |  |  | unknown | 171&#8209;176 |
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 11 | `mem_star` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 12 | `mem_plus` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 13 | `alphabet` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;214 |

### Chap05/MappingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `is_functional_vec` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;142 |
| 15 | `is_functional_vec_at` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 16 | `is_functional_SetStEph_at` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;154 |
| 17 | `is_functional_SetStEph` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 18 | `is_functional_RelationStEph` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 19 | `empty` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;175 |
| 20 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;181 |
| 21 | `from_relation` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 22 | `size` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;192 |
| 23 | `domain` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;198 |
| 24 | `range` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 25 | `mem` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;217 |
| 26 | `iter` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;227 |
| 27 | `next` |  | Y |  |  | Y |  |  | unknown | 452&#8209;468 |
| 28 | `hash` |  | Y |  |  | Y |  | Y |  | 563 |
| 29 | `eq` |  | Y |  |  | Y |  |  | unknown | 569&#8209;570 |

### Chap05/RelationStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 31 | `from_set` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 32 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 33 | `size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 34 | `domain` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 35 | `range` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 36 | `mem` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 37 | `relates` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 38 | `iter` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;152 |
| 39 | `next` |  | Y |  |  | Y |  |  | unknown | 288&#8209;304 |
| 40 | `hash` |  | Y |  |  | Y |  | Y |  | 398 |
| 41 | `eq` |  | Y |  |  | Y |  |  | unknown | 404&#8209;405 |

### Chap05/SetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 106&#8209;112 |
| 43 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 44 | `iter` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 45 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;152 |
| 46 | `empty` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 47 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 48 | `size` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;169 |
| 49 | `mem` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 50 | `insert` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 51 | `union` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;190 |
| 52 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;202 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 54 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;219 |
| 55 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;231 |
| 56 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;240 |
| 57 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;255 |
| 58 | `partition` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;273 |
| 59 | `choose` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;283 |
| 60 | `next` |  | Y |  |  | Y |  |  | unknown | 849&#8209;865 |
| 61 | `hash` |  | Y |  |  | Y |  | Y |  | 958 |
| 62 | `eq` |  | Y |  |  | Y |  |  | unknown | 964&#8209;965 |

### Chap05/SetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 63 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 101&#8209;107 |
| 64 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 65 | `iter` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;139 |
| 66 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;147 |
| 67 | `empty` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 68 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 69 | `size` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 70 | `mem` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;170 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 72 | `union` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;185 |
| 73 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;197 |
| 74 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;203 |
| 75 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;214 |
| 76 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;225 |
| 77 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;234 |
| 78 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;249 |
| 79 | `partition` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;267 |
| 80 | `split` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;284 |
| 81 | `choose` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 82 | `next` |  | Y |  |  | Y |  |  | unknown | 774&#8209;790 |
| 83 | `hash` |  | Y |  |  | Y |  | Y |  | 869 |
| 84 | `eq` |  | Y |  |  | Y |  |  | unknown | 875&#8209;876 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
