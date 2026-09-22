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
| 1 | Chap41 | AVLTreeSetMtEph | 13 | 14 | 0 | 1 | 15 | 0 | 14 | 0 | 1 |
| 2 | Chap41 | AVLTreeSetMtPer | 12 | 16 | 0 | 3 | 19 | 0 | 16 | 2 | 1 |
| 3 | Chap41 | AVLTreeSetStEph | 25 | 28 | 1 | 5 | 34 | 0 | 32 | 0 | 2 |
| 4 | Chap41 | AVLTreeSetStPer | 20 | 23 | 1 | 4 | 28 | 0 | 26 | 0 | 2 |
| 5 | Chap41 | ArraySetEnumMtEph | 13 | 14 | 0 | 5 | 19 | 0 | 19 | 0 | 0 |
| 6 | Chap41 | ArraySetStEph | 12 | 14 | 1 | 7 | 22 | 0 | 21 | 0 | 1 |
| 7 | Chap41 | Example41_3 | 3 | 3 | 0 | 8 | 9 | 0 | 0 | 1 | 8 |
| 8 | Chap41 | OrdKeyMap | 29 | 29 | 0 | 32 | 61 | 0 | 61 | 0 | 0 |

## Function-by-Function Detail

### Chap41/AVLTreeSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `size` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 2 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;115 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;123 |
| 5 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;135 |
| 6 | `filter` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;158 |
| 7 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;171 |
| 8 | `difference` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;184 |
| 9 | `union` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;198 |
| 10 | `find` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;221 |
| 12 | `insert` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;234 |
| 13 | `iter` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;239 |
| 14 | `from_vec_dc` |  |  |  | Y | Y |  |  | unknown | 247&#8209;255 |
| 15 | `default` |  | Y |  |  | Y |  | Y |  | 551 |

### Chap41/AVLTreeSetMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `size` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 17 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;111 |
| 18 | `empty` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;116 |
| 19 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 20 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;134 |
| 21 | `filter` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;157 |
| 22 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;168 |
| 23 | `difference` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;179 |
| 24 | `union` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;191 |
| 25 | `find` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;201 |
| 26 | `delete` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;211 |
| 27 | `insert` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;222 |
| 28 | `assert_avltreesetmtper_always_wf` |  |  |  | Y | Y |  |  | unknown | 239&#8209;240 |
| 29 | `assert_avltreesetmtper_bounded_size` |  |  |  | Y | Y |  |  | unknown | 247&#8209;253 |
| 30 | `from_vec_dc_per` |  |  |  | Y | Y |  |  | unknown | 261&#8209;269 |
| 31 | `partial_cmp` |  | Y |  |  | Y |  |  | hole | 512 |
| 32 | `cmp` |  | Y |  |  | Y |  |  | hole | 519 |
| 33 | `default` |  | Y |  |  | Y |  | Y |  | 560 |
| 34 | `eq` |  | Y |  |  | Y |  |  | unknown | 572&#8209;573 |

### Chap41/AVLTreeSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `lemma_wf_implies_len_bound` |  |  |  | Y | Y |  |  | unknown | 110&#8209;115 |
| 36 | `lemma_inorder_values_maps_to_views` |  |  |  | Y | Y |  |  | unknown | 128&#8209;130 |
| 37 | `lemma_empty_set_is_sorted` |  |  |  | Y | Y |  |  | unknown | 146&#8209;151 |
| 38 | `lemma_push_sorted` |  |  |  | Y | Y |  |  | unknown | 155&#8209;160 |
| 39 | `lemma_subseq_sorted` |  |  |  | Y | Y |  |  | unknown | 178&#8209;183 |
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;200 |
| 41 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;210 |
| 42 | `empty` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;217 |
| 43 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;225 |
| 44 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;237 |
| 45 | `filter` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;259 |
| 46 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;271 |
| 47 | `difference` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;283 |
| 48 | `union` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;296 |
| 49 | `find` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;305 |
| 50 | `delete` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;316 |
| 51 | `insert` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;328 |
| 52 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;336 |
| 53 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;347 |
| 54 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 350&#8209;357 |
| 55 | `filter_iter` | Y | Y |  |  | Y |  |  | unknown | 360&#8209;378 |
| 56 | `intersection_iter` | Y | Y |  |  | Y |  |  | unknown | 381&#8209;389 |
| 57 | `union_iter` | Y | Y |  |  | Y |  |  | unknown | 392&#8209;401 |
| 58 | `difference_iter` | Y | Y |  |  | Y |  |  | unknown | 404&#8209;412 |
| 59 | `insert_sorted` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;432 |
| 60 | `delete_sorted` | Y | Y |  |  | Y |  |  | unknown | 435&#8209;444 |
| 61 | `filter_sorted` | Y | Y |  |  | Y |  |  | unknown | 447&#8209;467 |
| 62 | `intersection_sorted` | Y | Y |  |  | Y |  |  | unknown | 470&#8209;480 |
| 63 | `difference_sorted` | Y | Y |  |  | Y |  |  | unknown | 483&#8209;493 |
| 64 | `union_sorted` | Y | Y |  |  | Y |  |  | unknown | 496&#8209;507 |
| 65 | `iter` |  |  | Y |  | Y |  |  | unknown | 519&#8209;524 |
| 66 | `clone_wf` |  | Y |  |  | Y |  | Y |  | 810 |
| 67 | `default` |  | Y |  |  | Y |  | Y |  | 847 |
| 68 | `eq` |  | Y |  |  | Y |  |  | unknown | 859&#8209;860 |

### Chap41/AVLTreeSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 69 | `lemma_inorder_values_maps_to_views_per` |  |  |  | Y | Y |  |  | unknown | 111&#8209;113 |
| 70 | `lemma_push_sorted_per` |  |  |  | Y | Y |  |  | unknown | 126&#8209;131 |
| 71 | `lemma_map_view_feq_implies_ext_eq_per` |  |  |  | Y | Y |  |  | unknown | 150&#8209;155 |
| 72 | `lemma_subseq_sorted_per` |  |  |  | Y | Y |  |  | unknown | 170&#8209;175 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 74 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;202 |
| 75 | `empty` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;209 |
| 76 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;217 |
| 77 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;229 |
| 78 | `filter` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;251 |
| 79 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;263 |
| 80 | `difference` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;275 |
| 81 | `union` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;288 |
| 82 | `find` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;297 |
| 83 | `delete` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;308 |
| 84 | `insert` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;320 |
| 85 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 86 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;339 |
| 87 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;349 |
| 88 | `filter_iter` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;370 |
| 89 | `intersection_iter` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;381 |
| 90 | `union_iter` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;393 |
| 91 | `difference_iter` | Y | Y |  |  | Y |  |  | unknown | 396&#8209;404 |
| 92 | `insert_sorted_per` | Y | Y |  |  | Y |  |  | unknown | 414&#8209;424 |
| 93 | `iter` |  |  | Y |  | Y |  |  | unknown | 433&#8209;438 |
| 94 | `clone_wf` |  | Y |  |  | Y |  | Y |  | 689 |
| 95 | `default` |  | Y |  |  | Y |  | Y |  | 726 |
| 96 | `eq` |  | Y |  |  | Y |  |  | unknown | 738&#8209;739 |

### Chap41/ArraySetEnumMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 97 | `zero_bit_false` |  |  |  | Y | Y |  |  | unknown | 110&#8209;112 |
| 98 | `set_bit64_proof` |  |  |  | Y | Y |  |  | unknown | 116&#8209;123 |
| 99 | `bit_or_64_proof` |  |  |  | Y | Y |  |  | unknown | 127&#8209;130 |
| 100 | `bit_and_64_proof` |  |  |  | Y | Y |  |  | unknown | 134&#8209;137 |
| 101 | `bit_andnot_64_proof` |  |  |  | Y | Y |  |  | unknown | 141&#8209;144 |
| 102 | `new` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;199 |
| 103 | `size` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 104 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;215 |
| 105 | `empty` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;223 |
| 106 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;233 |
| 107 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;241 |
| 108 | `filter` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;253 |
| 109 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;266 |
| 110 | `difference` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;279 |
| 111 | `union` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;292 |
| 112 | `find` | Y | Y |  |  | Y |  |  | unknown | 297&#8209;299 |
| 113 | `delete` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;309 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;320 |
| 115 | `eq` |  | Y |  |  | Y |  |  | unknown | 946&#8209;947 |

### Chap41/ArraySetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 116 | `lemma_filter_remove` |  |  |  | Y | Y |  |  | unknown | 94&#8209;97 |
| 117 | `lemma_push_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 117&#8209;122 |
| 118 | `lemma_filter_in_original` |  |  |  | Y | Y |  |  | unknown | 128&#8209;130 |
| 119 | `lemma_filter_preserves_no_dups` |  |  |  | Y | Y |  |  | unknown | 143&#8209;146 |
| 120 | `lemma_filter_to_set_intersect` |  |  |  | Y | Y |  |  | unknown | 161&#8209;164 |
| 121 | `lemma_filter_to_set_difference` |  |  |  | Y | Y |  |  | unknown | 185&#8209;188 |
| 122 | `lemma_subseq_no_dups_subset` |  |  |  | Y | Y |  |  | unknown | 210&#8209;216 |
| 123 | `size` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;231 |
| 124 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;240 |
| 125 | `empty` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 126 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;250 |
| 127 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;258 |
| 128 | `filter` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;279 |
| 129 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;290 |
| 130 | `difference` | Y | Y |  |  | Y |  |  | unknown | 295&#8209;301 |
| 131 | `union` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;312 |
| 132 | `find` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;318 |
| 133 | `delete` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 134 | `insert` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;338 |
| 135 | `iter` |  |  | Y |  | Y |  |  | unknown | 352&#8209;357 |
| 136 | `default` |  | Y |  |  | Y |  | Y |  | 1064 |
| 137 | `eq` |  | Y |  |  | Y |  |  | unknown | 1076&#8209;1077 |

### Chap41/Example41_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 138 | `example_41_1_array_set` | Y | Y |  | Y | Y |  | Y |  | 23 |
| 139 | `example_41_1_avl_set` | Y | Y |  | Y | Y |  | Y |  | 26 |
| 140 | `demonstrate_set_operations` | Y | Y |  |  | Y |  | Y |  | 29 |
| 141 | `example_41_1_array_set_impl` |  |  |  | Y | Y |  | Y |  | 32 |
| 142 | `example_41_1_avl_set_impl` |  |  |  | Y | Y |  |  | hole | 70 |
| 143 | `example_41_3_from_seq_demonstration_impl` |  |  |  | Y | Y |  | Y |  | 115 |
| 144 | `additional_set_operations_impl` |  |  |  | Y | Y |  | Y |  | 141 |
| 145 | `example_41_3_from_seq_demonstration` |  |  |  | Y | Y |  | Y |  | 187 |
| 146 | `additional_set_operations` |  |  |  | Y | Y |  | Y |  | 189 |

### Chap41/OrdKeyMap.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 147 | `lemma_view_gen_subset` |  |  |  | Y | Y |  |  | unknown | 118&#8209;126 |
| 148 | `lemma_view_gen_insert` |  |  |  | Y | Y |  |  | unknown | 131&#8209;138 |
| 149 | `lemma_view_gen_union` |  |  |  | Y | Y |  |  | unknown | 143&#8209;151 |
| 150 | `lemma_pair_set_to_map_len` |  |  |  | Y | Y |  |  | unknown | 179&#8209;181 |
| 151 | `lemma_pair_in_set_map_contains` |  |  |  | Y | Y |  |  | unknown | 219&#8209;225 |
| 152 | `lemma_pair_set_to_map_dom_contains` |  |  |  | Y | Y |  |  | unknown | 237&#8209;239 |
| 153 | `lemma_map_contains_pair_in_set` |  |  |  | Y | Y |  |  | unknown | 253&#8209;255 |
| 154 | `lemma_key_unique_insert` |  |  |  | Y | Y |  |  | unknown | 261&#8209;266 |
| 155 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 286&#8209;292 |
| 156 | `lemma_sorted_keys_pairwise_distinct` |  |  |  | Y | Y |  |  | unknown | 300&#8209;312 |
| 157 | `lemma_key_unique_remove` |  |  |  | Y | Y |  |  | unknown | 340&#8209;342 |
| 158 | `lemma_key_unique_subset` |  |  |  | Y | Y |  |  | unknown | 347&#8209;352 |
| 159 | `lemma_key_unique_empty` |  |  |  | Y | Y |  |  | unknown | 357&#8209;358 |
| 160 | `lemma_key_unique_disjoint_union` |  |  |  | Y | Y |  |  | unknown | 363&#8209;378 |
| 161 | `lemma_set_to_map_insert` |  |  |  | Y | Y |  |  | unknown | 410&#8209;416 |
| 162 | `lemma_set_to_map_remove_pair` |  |  |  | Y | Y |  |  | unknown | 460&#8209;466 |
| 163 | `lemma_set_to_map_union_root` |  |  |  | Y | Y |  |  | unknown | 494&#8209;519 |
| 164 | `lemma_set_to_map_empty` |  |  |  | Y | Y |  |  | unknown | 581&#8209;582 |
| 165 | `lemma_view_gen_empty` |  |  |  | Y | Y |  |  | unknown | 587&#8209;588 |
| 166 | `lemma_freshness_from_sorted` |  |  |  | Y | Y |  |  | unknown | 594&#8209;607 |
| 167 | `lemma_map_dom_preserved_by_superset` |  |  |  | Y | Y |  |  | unknown | 617&#8209;623 |
| 168 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 634&#8209;640 |
| 169 | `lemma_post_insert_invariants` |  |  |  | Y | Y |  |  | unknown | 649&#8209;663 |
| 170 | `lemma_ordkeymap_wf_type_axioms` |  |  |  | Y | Y |  |  | unknown | 674&#8209;683 |
| 171 | `lemma_values_preserved_from_subset` |  |  |  | Y | Y |  |  | unknown | 689&#8209;699 |
| 172 | `lemma_loop_init_sorted` |  |  |  | Y | Y |  |  | unknown | 714&#8209;727 |
| 173 | `new` | Y | Y |  |  | Y |  |  | unknown | 742&#8209;753 |
| 174 | `size` | Y | Y |  |  | Y |  |  | unknown | 756&#8209;758 |
| 175 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 761&#8209;763 |
| 176 | `find` | Y | Y |  |  | Y |  |  | unknown | 766&#8209;772 |
| 177 | `insert` | Y | Y |  |  | Y |  |  | unknown | 775&#8209;785 |
| 178 | `delete` | Y | Y |  |  | Y |  |  | unknown | 788&#8209;795 |
| 179 | `split` | Y | Y |  |  | Y |  |  | unknown | 798&#8209;814 |
| 180 | `union` | Y | Y |  |  | Y |  |  | unknown | 817&#8209;828 |
| 181 | `intersect` | Y | Y |  |  | Y |  |  | unknown | 831&#8209;838 |
| 182 | `union_with` | Y | Y |  |  | Y |  |  | unknown | 841&#8209;858 |
| 183 | `intersect_with` | Y | Y |  |  | Y |  |  | unknown | 861&#8209;873 |
| 184 | `difference` | Y | Y |  |  | Y |  |  | unknown | 876&#8209;883 |
| 185 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 886&#8209;897 |
| 186 | `prev_key` | Y | Y |  |  | Y |  |  | unknown | 900&#8209;908 |
| 187 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 911&#8209;920 |
| 188 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 923&#8209;933 |
| 189 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 936&#8209;942 |
| 190 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 945&#8209;951 |
| 191 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 954&#8209;959 |
| 192 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 962&#8209;971 |
| 193 | `collect` | Y | Y |  |  | Y |  |  | unknown | 974&#8209;977 |
| 194 | `filter` | Y | Y |  |  | Y |  |  | unknown | 980&#8209;995 |
| 195 | `map_values` | Y | Y |  |  | Y |  |  | unknown | 998&#8209;1005 |
| 196 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 1008&#8209;1011 |
| 197 | `domain` | Y | Y |  |  | Y |  |  | unknown | 1014&#8209;1016 |
| 198 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 1019&#8209;1037 |
| 199 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 1040&#8209;1045 |
| 200 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 1048&#8209;1053 |
| 201 | `iter` | Y | Y |  |  | Y |  |  | unknown | 1055&#8209;1059 |
| 202 | `ordkeymap_find` |  |  |  | Y | Y |  |  | unknown | 1066&#8209;1086 |
| 203 | `ordkeymap_split` |  |  |  | Y | Y |  |  | unknown | 1165&#8209;1199 |
| 204 | `ordkeymap_next` |  |  |  | Y | Y |  |  | unknown | 1322&#8209;1345 |
| 205 | `ordkeymap_prev` |  |  |  | Y | Y |  |  | unknown | 1575&#8209;1598 |
| 206 | `ordkeymap_rank` |  |  |  | Y | Y |  |  | unknown | 1837&#8209;1857 |
| 207 | `ordkeymap_select` |  |  |  | Y | Y |  |  | unknown | 2067&#8209;2088 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
