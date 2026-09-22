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
| 2 | Chap05 | MappingStEph | 13 | 15 | 0 | 2 | 17 | 0 | 16 | 0 | 1 |
| 3 | Chap05 | RelationStEph | 9 | 11 | 0 | 0 | 11 | 0 | 10 | 0 | 1 |
| 4 | Chap05 | SetMtEph | 18 | 20 | 0 | 7 | 27 | 0 | 25 | 2 | 0 |
| 5 | Chap05 | SetStEph | 18 | 20 | 0 | 7 | 27 | 0 | 26 | 1 | 0 |

## Function-by-Function Detail

### Chap05/KleeneStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_star_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 87&#8209;92 |
| 2 | `lemma_plus_closed_under_concat` |  |  |  | Y | Y |  |  | unknown | 97&#8209;102 |
| 3 | `ptt_star_contains_empty` |  |  |  | Y | Y |  |  | unknown | 108&#8209;109 |
| 4 | `ptt_plus_rejects_empty` |  |  |  | Y | Y |  |  | unknown | 114&#8209;115 |
| 5 | `ptt_singleton_in_star_and_plus` |  |  |  | Y | Y |  |  | unknown | 120&#8209;124 |
| 6 | `ptt_plus_subset_of_star` |  |  |  | Y | Y |  |  | unknown | 129&#8209;131 |
| 7 | `ptt_star_property_transfer` |  |  |  | Y | Y |  |  | unknown | 137&#8209;146 |
| 8 | `ptt_star_concat_plus_is_plus` |  |  |  | Y | Y |  |  | unknown | 152&#8209;157 |
| 9 | `ptt_plus_concat_star_is_plus` |  |  |  | Y | Y |  |  | unknown | 163&#8209;168 |
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 11 | `mem_star` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;191 |
| 12 | `mem_plus` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 13 | `alphabet` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;203 |

### Chap05/MappingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_view_contains_pair` |  |  |  | Y | Y |  |  | unknown | 135&#8209;139 |
| 15 | `lemma_view_kv_pairs` |  |  |  | Y | Y |  |  | unknown | 157&#8209;161 |
| 16 | `is_functional_vec` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 17 | `is_functional_vec_at` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 18 | `is_functional_SetStEph_at` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 19 | `is_functional_SetStEph` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 20 | `is_functional_RelationStEph` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 21 | `empty` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;221 |
| 22 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 23 | `from_relation` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;239 |
| 24 | `size` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;245 |
| 25 | `domain` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 26 | `range` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;260 |
| 27 | `mem` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;269 |
| 28 | `iter` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;280 |
| 29 | `hash` |  | Y |  |  | Y |  | Y |  | 609 |
| 30 | `eq` |  | Y |  |  | Y |  |  | unknown | 615&#8209;616 |

### Chap05/RelationStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 32 | `from_set` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 33 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 34 | `size` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 35 | `domain` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 36 | `range` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 37 | `mem` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 38 | `relates` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 39 | `iter` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;158 |
| 40 | `hash` |  | Y |  |  | Y |  | Y |  | 299 |
| 41 | `eq` |  | Y |  |  | Y |  |  | unknown | 305&#8209;306 |

### Chap05/SetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 125&#8209;130 |
| 43 | `lemma_viewed_contains` |  |  |  | Y | Y |  |  | unknown | 143&#8209;147 |
| 44 | `lemma_viewed_mem` |  |  |  | Y | Y |  |  | unknown | 152&#8209;156 |
| 45 | `lemma_viewed_insert` |  |  |  | Y | Y |  |  | unknown | 166&#8209;168 |
| 46 | `lemma_viewed_empty` |  |  |  | Y | Y |  |  | unknown | 174&#8209;176 |
| 47 | `lemma_viewed_len` |  |  |  | Y | Y |  |  | unknown | 182&#8209;186 |
| 48 | `lemma_iter_keys_view` |  |  |  | Y | Y |  |  | unknown | 194&#8209;203 |
| 49 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;236 |
| 50 | `iter` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;248 |
| 51 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;255 |
| 52 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 1105&#8209;1107 |
| 53 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 54 | `size` x3 | Y | Y |  |  | Y |  |  | unknown | 1110&#8209;1111 |
| 55 | `mem` x3 | Y | Y |  |  | Y |  |  | unknown | 1114&#8209;1115 |
| 56 | `insert` x3 | Y | Y |  |  | Y |  |  | hole | 1118&#8209;1121 |
| 57 | `union` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;296 |
| 58 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;309 |
| 59 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;317 |
| 60 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;327 |
| 61 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;339 |
| 62 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;347 |
| 63 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;361 |
| 64 | `partition` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;379 |
| 65 | `split` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;396 |
| 66 | `choose` x3 | Y | Y |  |  | Y |  |  | unknown | 1124&#8209;1126 |
| 67 | `hash` |  | Y |  |  | Y |  |  | hole | 1236 |
| 68 | `eq` |  | Y |  |  | Y |  |  | unknown | 1250&#8209;1251 |

### Chap05/SetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_singleton_choose` |  |  |  | Y | Y |  |  | unknown | 113&#8209;118 |
| 70 | `lemma_viewed_contains` |  |  |  | Y | Y |  |  | unknown | 131&#8209;135 |
| 71 | `lemma_viewed_mem` |  |  |  | Y | Y |  |  | unknown | 140&#8209;144 |
| 72 | `lemma_viewed_insert` |  |  |  | Y | Y |  |  | unknown | 154&#8209;156 |
| 73 | `lemma_viewed_empty` |  |  |  | Y | Y |  |  | unknown | 162&#8209;164 |
| 74 | `lemma_viewed_len` |  |  |  | Y | Y |  |  | unknown | 170&#8209;174 |
| 75 | `lemma_iter_keys_view` |  |  |  | Y | Y |  |  | unknown | 182&#8209;191 |
| 76 | `from_vec` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 77 | `iter` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;236 |
| 78 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;243 |
| 79 | `empty` | Y | Y |  |  | Y |  |  | unknown | 247&#8209;249 |
| 80 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;255 |
| 81 | `size` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;261 |
| 82 | `mem` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;267 |
| 83 | `insert` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;276 |
| 84 | `union` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;284 |
| 85 | `disjoint_union` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;297 |
| 86 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;305 |
| 87 | `elt_cross_set` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;315 |
| 88 | `cartesian_product` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;326 |
| 89 | `all_nonempty` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;334 |
| 90 | `partition_on_elt` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;348 |
| 91 | `partition` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;366 |
| 92 | `split` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;383 |
| 93 | `choose` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;394 |
| 94 | `hash` |  | Y |  |  | Y |  |  | hole | 968 |
| 95 | `eq` |  | Y |  |  | Y |  |  | unknown | 982&#8209;983 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
