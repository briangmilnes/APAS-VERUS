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
| 2 | Chap05 | MappingStEph | 13 | 16 | 0 | 0 | 16 | 0 | 14 | 1 | 1 |
| 3 | Chap05 | RelationStEph | 9 | 12 | 0 | 0 | 12 | 0 | 11 | 0 | 1 |
| 4 | Chap05 | SetMtEph | 17 | 20 | 0 | 1 | 21 | 0 | 15 | 5 | 1 |
| 5 | Chap05 | SetStEph | 18 | 21 | 0 | 1 | 22 | 0 | 20 | 1 | 1 |

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
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 11 | `mem_star` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;204 |
| 12 | `mem_plus` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;211 |
| 13 | `alphabet` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |

### Chap05/MappingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `is_functional_vec` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 15 | `is_functional_vec_at` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 16 | `is_functional_SetStEph_at` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;150 |
| 17 | `is_functional_SetStEph` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 18 | `is_functional_RelationStEph` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 19 | `empty` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;170 |
| 20 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 21 | `from_relation` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;188 |
| 22 | `size` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 23 | `domain` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;200 |
| 24 | `range` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;210 |
| 25 | `mem` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;219 |
| 26 | `iter` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;229 |
| 27 | `next` |  | Y |  |  | Y |  |  | unknown | 488&#8209;504 |
| 28 | `hash` |  | Y |  |  | Y |  | Y |  | 598 |
| 29 | `eq` |  | Y |  |  | Y |  |  | hole | 604&#8209;605 |

### Chap05/RelationStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 30 | `empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 31 | `from_set` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 32 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 33 | `size` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 34 | `domain` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 35 | `range` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 36 | `mem` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 37 | `relates` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 38 | `iter` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;155 |
| 39 | `next` |  | Y |  |  | Y |  |  | unknown | 300&#8209;316 |
| 40 | `hash` |  | Y |  |  | Y |  | Y |  | 410 |
| 41 | `eq` |  | Y |  |  | Y |  |  | unknown | 416&#8209;417 |

### Chap05/SetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 107&#8209;113 |
| 43 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 44 | `iter` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;147 |
| 45 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;155 |
| 46 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 1004&#8209;1006 |
| 47 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 48 | `size` x3 | Y | Y |  |  | Y |  |  | hole | 1008&#8209;1009 |
| 49 | `mem` x3 | Y | Y |  |  | Y |  |  | hole | 1011&#8209;1012 |
| 50 | `insert` x3 | Y | Y |  |  | Y |  |  | hole | 1014&#8209;1018 |
| 51 | `union` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;196 |
| 52 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;209 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;217 |
| 54 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;228 |
| 55 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;240 |
| 56 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;249 |
| 57 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;264 |
| 58 | `partition` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;282 |
| 59 | `choose` x3 | Y | Y |  |  | Y |  |  | hole | 1020&#8209;1022 |
| 60 | `next` |  | Y |  |  | Y |  |  | unknown | 868&#8209;884 |
| 61 | `hash` |  | Y |  |  | Y |  | Y |  | 1083 |
| 62 | `eq` |  | Y |  |  | Y |  |  | hole | 1089&#8209;1090 |

### Chap05/SetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 63 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 99&#8209;105 |
| 64 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 65 | `iter` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;135 |
| 66 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 67 | `empty` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 68 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 69 | `size` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 70 | `mem` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 71 | `insert` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;176 |
| 72 | `union` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;184 |
| 73 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;197 |
| 74 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;205 |
| 75 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;216 |
| 76 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;227 |
| 77 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;236 |
| 78 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;251 |
| 79 | `partition` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;269 |
| 80 | `split` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;286 |
| 81 | `choose` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;297 |
| 82 | `next` |  | Y |  |  | Y |  |  | unknown | 793&#8209;809 |
| 83 | `hash` |  | Y |  |  | Y |  | Y |  | 887 |
| 84 | `eq` |  | Y |  |  | Y |  |  | hole | 893&#8209;894 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
