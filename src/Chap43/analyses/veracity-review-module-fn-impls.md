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
| 1 | Chap43 | AugOrderedTableMtEph | 33 | 34 | 0 | 3 | 36 | 1 | 35 | 0 | 2 |
| 2 | Chap43 | AugOrderedTableStEph | 31 | 32 | 1 | 2 | 34 | 1 | 34 | 0 | 1 |
| 3 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 2 | 32 | 0 | 32 | 0 | 0 |
| 4 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 5 | Chap43 | OrderedSetMtEph | 23 | 23 | 0 | 1 | 24 | 0 | 9 | 15 | 0 |
| 6 | Chap43 | OrderedSetStEph | 30 | 32 | 1 | 6 | 37 | 2 | 37 | 0 | 2 |
| 7 | Chap43 | OrderedSetStPer | 30 | 32 | 1 | 6 | 38 | 1 | 38 | 0 | 1 |
| 8 | Chap43 | OrderedSpecsAndLemmas | 0 | 0 | 0 | 11 | 11 | 0 | 11 | 0 | 0 |
| 9 | Chap43 | OrderedTableMtEph | 30 | 32 | 0 | 2 | 32 | 2 | 10 | 22 | 2 |
| 10 | Chap43 | OrderedTableMtPer | 22 | 23 | 0 | 1 | 23 | 1 | 23 | 0 | 1 |
| 11 | Chap43 | OrderedTableStEph | 40 | 42 | 1 | 1 | 43 | 1 | 42 | 0 | 2 |
| 12 | Chap43 | OrderedTableStPer | 39 | 40 | 1 | 1 | 42 | 0 | 42 | 0 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 96&#8209;99 |
| 2 | `size` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 3 | `empty` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;129 |
| 4 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;144 |
| 5 | `find` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;154 |
| 6 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 7 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;170 |
| 8 | `insert` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 9 | `delete` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;187 |
| 10 | `domain` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;192 |
| 11 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;214 |
| 12 | `map` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;222 |
| 13 | `filter` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;230 |
| 14 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;240 |
| 15 | `union` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;250 |
| 16 | `difference` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;255 |
| 17 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;260 |
| 18 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;265 |
| 19 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 20 | `collect` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;276 |
| 21 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;286 |
| 22 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;296 |
| 23 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;306 |
| 24 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;316 |
| 25 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;322 |
| 26 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;327 |
| 27 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;336 |
| 28 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;345 |
| 29 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;355 |
| 30 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;361 |
| 31 | `reduce_val` | Y | Y |  |  | Y |  | Y |  | 364 |
| 32 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;369 |
| 33 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;377 |
| 34 | `iter` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;383 |
| 35 | `recalculate_reduction` |  |  |  | Y | Y |  |  | unknown | 392&#8209;395 |
| 36 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 404&#8209;409 |
| 37 | `eq` |  | Y |  |  |  | Y | Y |  | 936&#8209;939 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 91&#8209;94 |
| 39 | `size` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 40 | `empty` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;127 |
| 41 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;141 |
| 42 | `find` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;151 |
| 43 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;161 |
| 44 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 45 | `insert` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;183 |
| 46 | `delete` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;191 |
| 47 | `domain` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;197 |
| 48 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;221 |
| 49 | `map` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;227 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;240 |
| 51 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;245 |
| 52 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;262 |
| 53 | `union` | Y | Y |  |  | Y |  |  | unknown | 266&#8209;284 |
| 54 | `difference` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;292 |
| 55 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;300 |
| 56 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 304&#8209;308 |
| 57 | `collect` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;314 |
| 58 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 318&#8209;324 |
| 59 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 328&#8209;334 |
| 60 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;344 |
| 61 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;354 |
| 62 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 358&#8209;363 |
| 63 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;375 |
| 64 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;382 |
| 65 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;391 |
| 66 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 395&#8209;401 |
| 67 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;410 |
| 68 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;414 |
| 69 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 418&#8209;419 |
| 70 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 428&#8209;437 |
| 71 | `iter` |  |  | Y |  | Y |  |  | unknown | 919&#8209;924 |
| 72 | `eq` |  | Y |  |  |  | Y | Y |  | 1001&#8209;1004 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 74 | `size` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 75 | `empty` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;128 |
| 76 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;143 |
| 77 | `find` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;153 |
| 78 | `insert` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;164 |
| 79 | `delete` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;174 |
| 80 | `domain` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 81 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;204 |
| 82 | `map` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;220 |
| 83 | `filter` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;235 |
| 84 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;253 |
| 85 | `union` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;275 |
| 86 | `difference` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;287 |
| 87 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;298 |
| 88 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;309 |
| 89 | `collect` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;315 |
| 90 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;325 |
| 91 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;335 |
| 92 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;345 |
| 93 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;355 |
| 94 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;374 |
| 95 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 378&#8209;387 |
| 96 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 391&#8209;397 |
| 97 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;408 |
| 98 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 412&#8209;420 |
| 99 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 424&#8209;434 |
| 100 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;438 |
| 101 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 442&#8209;444 |
| 102 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 454&#8209;461 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 990&#8209;995 |
| 104 | `eq` |  |  | Y |  | Y |  |  | unknown | 1037&#8209;1038 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 105 | `_example_43_1_verified` |  |  |  | Y | Y |  | Y |  | 12 |
| 106 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 20&#8209;22 |
| 107 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 24&#8209;26 |
| 108 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 175&#8209;231 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 109 | `from_st` |  |  |  | Y | Y |  |  | unknown | 75&#8209;77 |
| 110 | `size` | Y | Y |  |  | Y |  |  | hole | 116&#8209;118 |
| 111 | `empty` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;126 |
| 112 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 113 | `find` | Y | Y |  |  | Y |  |  | hole | 138&#8209;140 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | hole | 144&#8209;146 |
| 115 | `delete` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 116 | `filter` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;164 |
| 117 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;170 |
| 118 | `union` | Y | Y |  |  | Y |  |  | hole | 174&#8209;179 |
| 119 | `difference` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 120 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 189&#8209;193 |
| 121 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;201 |
| 122 | `first` | Y | Y |  |  | Y |  |  | hole | 206&#8209;212 |
| 123 | `last` | Y | Y |  |  | Y |  |  | hole | 215&#8209;221 |
| 124 | `previous` | Y | Y |  |  | Y |  |  | hole | 224&#8209;231 |
| 125 | `next` | Y | Y |  |  | Y |  |  | hole | 234&#8209;241 |
| 126 | `split` | Y | Y |  |  | Y |  |  | hole | 244&#8209;248 |
| 127 | `join` | Y | Y |  |  | Y |  |  | hole | 251&#8209;255 |
| 128 | `get_range` | Y | Y |  |  | Y |  |  | hole | 258&#8209;262 |
| 129 | `rank` | Y | Y |  |  | Y |  |  | hole | 265&#8209;268 |
| 130 | `select` | Y | Y |  |  | Y |  |  | hole | 271&#8209;275 |
| 131 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 278&#8209;282 |
| 132 | `iter` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;288 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 84&#8209;88 |
| 134 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 95&#8209;100 |
| 135 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 107&#8209;112 |
| 136 | `size` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 137 | `empty` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 138 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;150 |
| 139 | `find` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 140 | `insert` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;164 |
| 141 | `delete` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;171 |
| 142 | `filter` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;190 |
| 143 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;197 |
| 144 | `union` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;207 |
| 145 | `difference` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;214 |
| 146 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;222 |
| 147 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;232 |
| 148 | `first` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;243 |
| 149 | `last` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;252 |
| 150 | `previous` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;262 |
| 151 | `next` | Y | Y |  |  | Y |  |  | unknown | 265&#8209;272 |
| 152 | `split` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;287 |
| 153 | `join` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 154 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;303 |
| 155 | `rank` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;309 |
| 156 | `select` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;316 |
| 157 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;328 |
| 158 | `first_iter` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;337 |
| 159 | `last_iter` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;346 |
| 160 | `previous_iter` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;356 |
| 161 | `next_iter` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;366 |
| 162 | `split_iter` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;381 |
| 163 | `get_range_iter` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;389 |
| 164 | `rank_iter` | Y | Y |  |  | Y |  |  | unknown | 392&#8209;395 |
| 165 | `split_rank_iter` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;407 |
| 166 | `tree_max_key` |  |  |  | Y | Y |  |  | unknown | 415&#8209;424 |
| 167 | `tree_select` |  |  |  | Y | Y |  |  | unknown | 473&#8209;481 |
| 168 | `iter` |  |  | Y |  | Y |  |  | unknown | 944&#8209;949 |
| 169 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 958&#8209;963 |
| 170 | `default` |  | Y |  |  |  | Y | Y |  | 1022 |
| 171 | `eq` |  | Y |  |  |  | Y | Y |  | 1026&#8209;1036 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 172 | `lemma_cmp_antisymmetry` |  |  |  | Y | Y |  |  | unknown | 86&#8209;90 |
| 173 | `lemma_cmp_transitivity` |  |  |  | Y | Y |  |  | unknown | 97&#8209;102 |
| 174 | `lemma_cmp_equal_congruent` |  |  |  | Y | Y |  |  | unknown | 109&#8209;114 |
| 175 | `size` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 176 | `empty` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;141 |
| 177 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;148 |
| 178 | `find` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 179 | `insert` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;160 |
| 180 | `delete` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 181 | `filter` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;178 |
| 182 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 183 | `union` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;191 |
| 184 | `difference` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;196 |
| 185 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;204 |
| 186 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;213 |
| 187 | `first` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;224 |
| 188 | `last` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;233 |
| 189 | `previous` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;243 |
| 190 | `next` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;253 |
| 191 | `split` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;268 |
| 192 | `join` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;276 |
| 193 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;284 |
| 194 | `rank` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;290 |
| 195 | `select` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;297 |
| 196 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;309 |
| 197 | `first_iter` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;319 |
| 198 | `last_iter` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;327 |
| 199 | `previous_iter` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;336 |
| 200 | `next_iter` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;345 |
| 201 | `split_iter` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;359 |
| 202 | `get_range_iter` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;366 |
| 203 | `rank_iter` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;371 |
| 204 | `split_rank_iter` | Y | Y |  |  | Y |  |  | unknown | 373&#8209;382 |
| 205 | `tree_max_key` |  |  |  | Y | Y |  |  | unknown | 390&#8209;399 |
| 206 | `tree_select` |  |  |  | Y | Y |  |  | unknown | 442&#8209;450 |
| 207 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 1011&#8209;1016 |
| 208 | `iter` |  |  | Y |  | Y |  |  | unknown | 1027&#8209;1032 |
| 209 | `default` |  | Y |  |  | Y |  |  | unknown | 1056&#8209;1057 |
| 210 | `eq` |  | Y |  |  |  | Y | Y |  | 1090&#8209;1092 |

### Chap43/OrderedSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 211 | `lemma_view_gen_insert` |  |  |  | Y | Y |  |  | unknown | 69&#8209;76 |
| 212 | `lemma_pair_set_to_map_dom_contains` |  |  |  | Y | Y |  |  | unknown | 95&#8209;97 |
| 213 | `lemma_pair_set_to_map_len` |  |  |  | Y | Y |  |  | unknown | 137&#8209;139 |
| 214 | `lemma_pair_in_set_map_contains` |  |  |  | Y | Y |  |  | unknown | 180&#8209;186 |
| 215 | `lemma_map_contains_pair_in_set` |  |  |  | Y | Y |  |  | unknown | 201&#8209;203 |
| 216 | `lemma_key_unique_insert` |  |  |  | Y | Y |  |  | unknown | 209&#8209;214 |
| 217 | `lemma_sorted_keys_pairwise_distinct` |  |  |  | Y | Y |  |  | unknown | 239&#8209;251 |
| 218 | `lemma_key_unique_subset` |  |  |  | Y | Y |  |  | unknown | 281&#8209;286 |
| 219 | `lemma_key_unique_empty` |  |  |  | Y | Y |  |  | unknown | 292&#8209;293 |
| 220 | `lemma_set_to_map_insert` |  |  |  | Y | Y |  |  | unknown | 299&#8209;305 |
| 221 | `lemma_set_to_map_empty` |  |  |  | Y | Y |  |  | unknown | 360&#8209;361 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 222 | `size` | Y | Y |  |  | Y |  |  | hole | 123&#8209;125 |
| 223 | `empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;137 |
| 224 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;149 |
| 225 | `find` | Y | Y |  |  | Y |  |  | hole | 154&#8209;160 |
| 226 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;171 |
| 227 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 228 | `insert` | Y | Y |  |  | Y |  |  | hole | 183&#8209;188 |
| 229 | `delete` | Y | Y |  |  | Y |  |  | hole | 193&#8209;197 |
| 230 | `domain` | Y | Y |  |  | Y |  |  | hole | 202&#8209;203 |
| 231 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;220 |
| 232 | `map` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;226 |
| 233 | `filter` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;239 |
| 234 | `intersection` | Y | Y |  |  | Y |  |  | hole | 244&#8209;245 |
| 235 | `union` | Y | Y |  |  | Y |  |  | hole | 250&#8209;251 |
| 236 | `difference` | Y | Y |  |  | Y |  |  | hole | 256&#8209;257 |
| 237 | `restrict` | Y | Y |  |  | Y |  |  | hole | 262&#8209;263 |
| 238 | `subtract` | Y | Y |  |  | Y |  |  | hole | 268&#8209;269 |
| 239 | `reduce` | Y | Y |  |  | Y |  |  | hole | 274&#8209;275 |
| 240 | `collect` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;281 |
| 241 | `first_key` | Y | Y |  |  | Y |  |  | hole | 286&#8209;292 |
| 242 | `last_key` | Y | Y |  |  | Y |  |  | hole | 297&#8209;303 |
| 243 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 308&#8209;314 |
| 244 | `next_key` | Y | Y |  |  | Y |  |  | hole | 319&#8209;325 |
| 245 | `split_key` | Y | Y |  |  | Y |  |  | hole | 330&#8209;332 |
| 246 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;338 |
| 247 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 343&#8209;348 |
| 248 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 353&#8209;358 |
| 249 | `select_key` | Y | Y |  |  | Y |  |  | hole | 363&#8209;369 |
| 250 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 374&#8209;376 |
| 251 | `iter` | Y | Y |  |  | Y |  |  | hole | 378&#8209;382 |
| 252 | `from_st` |  |  |  | Y | Y |  |  | hole | 820&#8209;821 |
| 253 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 839&#8209;852 |
| 254 | `eq` |  | Y |  |  |  | Y | Y |  | 951&#8209;958 |
| 255 | `default` |  | Y |  |  |  | Y | Y |  | 962 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 256 | `from_st_table` |  |  |  | Y | Y |  |  | unknown | 72&#8209;76 |
| 257 | `size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 258 | `empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;131 |
| 259 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;144 |
| 260 | `find` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;154 |
| 261 | `insert` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 262 | `insert_wf` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;173 |
| 263 | `delete` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;181 |
| 264 | `delete_wf` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;189 |
| 265 | `domain` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;195 |
| 266 | `map` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;209 |
| 267 | `filter` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 268 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;226 |
| 269 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;237 |
| 270 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;248 |
| 271 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;259 |
| 272 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 264&#8209;266 |
| 273 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;277 |
| 274 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;283 |
| 275 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;293 |
| 276 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;304 |
| 277 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 278 | `iter` | Y | Y |  |  | Y |  |  | unknown | 314&#8209;318 |
| 279 | `default` |  | Y |  |  |  | Y | Y |  | 880 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 280 | `size` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 281 | `empty` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;111 |
| 282 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;123 |
| 283 | `find` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;133 |
| 284 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;143 |
| 285 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 286 | `insert` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;166 |
| 287 | `delete` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;175 |
| 288 | `domain` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;181 |
| 289 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;204 |
| 290 | `map` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;213 |
| 291 | `filter` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;232 |
| 292 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;237 |
| 293 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;255 |
| 294 | `union` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;278 |
| 295 | `difference` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;287 |
| 296 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 291&#8209;296 |
| 297 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;305 |
| 298 | `collect` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;311 |
| 299 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;321 |
| 300 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;330 |
| 301 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 334&#8209;339 |
| 302 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;348 |
| 303 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 352&#8209;367 |
| 304 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;380 |
| 305 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;390 |
| 306 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 394&#8209;401 |
| 307 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;413 |
| 308 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;427 |
| 309 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;436 |
| 310 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 439&#8209;452 |
| 311 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 455&#8209;460 |
| 312 | `first_key_iter` | Y | Y |  |  | Y |  |  | unknown | 463&#8209;468 |
| 313 | `last_key_iter` | Y | Y |  |  | Y |  |  | unknown | 471&#8209;476 |
| 314 | `previous_key_iter` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;484 |
| 315 | `next_key_iter` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;492 |
| 316 | `split_key_iter` | Y | Y |  |  | Y |  |  | unknown | 495&#8209;510 |
| 317 | `get_key_range_iter` | Y | Y |  |  | Y |  |  | unknown | 513&#8209;519 |
| 318 | `rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 522&#8209;529 |
| 319 | `split_rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 532&#8209;542 |
| 320 | `union_bypassed_r158` |  | Y |  |  | Y |  | Y |  | 750 |
| 321 | `iter` |  |  | Y |  | Y |  |  | unknown | 1597&#8209;1603 |
| 322 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1613&#8209;1631 |
| 323 | `eq` |  | Y |  |  |  | Y | Y |  | 1782&#8209;1784 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 324 | `size` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 325 | `empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;108 |
| 326 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;119 |
| 327 | `find` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;128 |
| 328 | `insert` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;140 |
| 329 | `insert_wf` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;152 |
| 330 | `delete` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;160 |
| 331 | `delete_wf` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;171 |
| 332 | `domain` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 333 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;196 |
| 334 | `map` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;208 |
| 335 | `filter` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;221 |
| 336 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;237 |
| 337 | `union` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;258 |
| 338 | `difference` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 339 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;274 |
| 340 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 277&#8209;282 |
| 341 | `collect` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;287 |
| 342 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;296 |
| 343 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;305 |
| 344 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 308&#8209;314 |
| 345 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 317&#8209;323 |
| 346 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;341 |
| 347 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 344&#8209;353 |
| 348 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 356&#8209;362 |
| 349 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 365&#8209;372 |
| 350 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;383 |
| 351 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 386&#8209;396 |
| 352 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 398&#8209;404 |
| 353 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 406&#8209;413 |
| 354 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 415&#8209;420 |
| 355 | `first_key_iter` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;428 |
| 356 | `last_key_iter` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;436 |
| 357 | `previous_key_iter` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;444 |
| 358 | `next_key_iter` | Y | Y |  |  | Y |  |  | unknown | 446&#8209;452 |
| 359 | `split_key_iter` | Y | Y |  |  | Y |  |  | unknown | 454&#8209;469 |
| 360 | `get_key_range_iter` | Y | Y |  |  | Y |  |  | unknown | 471&#8209;477 |
| 361 | `rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;486 |
| 362 | `split_rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;498 |
| 363 | `iter` |  |  | Y |  | Y |  |  | unknown | 1214&#8209;1220 |
| 364 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 1228&#8209;1247 |
| 365 | `eq` |  | Y |  |  | Y |  |  | unknown | 1402&#8209;1403 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
