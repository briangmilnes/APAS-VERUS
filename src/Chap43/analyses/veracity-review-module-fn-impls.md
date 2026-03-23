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
| 1 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 1 | 3 | 35 | 2 | 35 | 0 | 2 |
| 2 | Chap43 | AugOrderedTableStEph | 31 | 32 | 1 | 2 | 34 | 1 | 34 | 0 | 1 |
| 3 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 2 | 32 | 0 | 32 | 0 | 0 |
| 4 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 5 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 1 | 23 | 0 | 8 | 15 | 0 |
| 6 | Chap43 | OrderedSetStEph | 30 | 32 | 1 | 1 | 32 | 2 | 31 | 1 | 2 |
| 7 | Chap43 | OrderedSetStPer | 30 | 32 | 1 | 1 | 33 | 1 | 32 | 1 | 1 |
| 8 | Chap43 | OrderedTableMtEph | 29 | 32 | 1 | 2 | 31 | 4 | 9 | 22 | 4 |
| 9 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 1 | 20 | 1 | 9 | 10 | 2 |
| 10 | Chap43 | OrderedTableStEph | 40 | 42 | 1 | 5 | 47 | 1 | 46 | 1 | 1 |
| 11 | Chap43 | OrderedTableStPer | 37 | 39 | 1 | 8 | 48 | 0 | 46 | 2 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `recalculate_reduction` |  |  |  | Y | Y |  |  | unknown | 75&#8209;79 |
| 2 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 86&#8209;92 |
| 3 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 118&#8209;121 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 7 | `find` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;154 |
| 8 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;163 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;177 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;184 |
| 12 | `domain` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 13 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;204 |
| 14 | `map` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;211 |
| 15 | `filter` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;219 |
| 16 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;226 |
| 17 | `union` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;233 |
| 18 | `difference` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;238 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;243 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 246&#8209;248 |
| 21 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;253 |
| 22 | `collect` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;257 |
| 23 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;266 |
| 24 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 25 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;284 |
| 26 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 287&#8209;293 |
| 27 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 296&#8209;299 |
| 28 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;304 |
| 29 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 307&#8209;309 |
| 30 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 312&#8209;318 |
| 31 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 321&#8209;328 |
| 32 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;334 |
| 33 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 337&#8209;339 |
| 34 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 342&#8209;344 |
| 35 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | unknown | 347&#8209;352 |
| 36 | `iter` |  |  | Y |  |  | Y | Y |  | 759&#8209;761 |
| 37 | `eq` |  | Y |  |  |  | Y | Y |  | 776&#8209;779 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 71&#8209;81 |
| 39 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 107&#8209;110 |
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 41 | `empty` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 42 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 43 | `find` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;145 |
| 44 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;154 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;175 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 48 | `domain` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 49 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;205 |
| 50 | `map` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;210 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | unknown | 213&#8209;223 |
| 52 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;228 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;245 |
| 54 | `union` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;267 |
| 55 | `difference` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;275 |
| 56 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;283 |
| 57 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;291 |
| 58 | `collect` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;296 |
| 59 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 299&#8209;306 |
| 60 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 309&#8209;316 |
| 61 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;326 |
| 62 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;336 |
| 63 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 339&#8209;348 |
| 64 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 351&#8209;360 |
| 65 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 363&#8209;367 |
| 66 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 370&#8209;376 |
| 67 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 379&#8209;386 |
| 68 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;398 |
| 69 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;402 |
| 70 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;407 |
| 71 | `iter` |  |  | Y |  | Y |  |  | unknown | 816&#8209;821 |
| 72 | `eq` |  | Y |  |  |  | Y | Y |  | 869&#8209;872 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 76&#8209;84 |
| 74 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 110&#8209;113 |
| 75 | `size` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 76 | `empty` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;135 |
| 77 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;142 |
| 78 | `find` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;151 |
| 79 | `insert` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;162 |
| 80 | `delete` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;171 |
| 81 | `domain` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 82 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;193 |
| 83 | `map` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;209 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;224 |
| 85 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;242 |
| 86 | `union` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;264 |
| 87 | `difference` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;276 |
| 88 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;287 |
| 89 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;298 |
| 90 | `collect` | Y | Y |  |  | Y |  |  | unknown | 301&#8209;303 |
| 91 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 306&#8209;313 |
| 92 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;323 |
| 93 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 326&#8209;333 |
| 94 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;343 |
| 95 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 346&#8209;364 |
| 96 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 367&#8209;377 |
| 97 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 380&#8209;387 |
| 98 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;398 |
| 99 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 401&#8209;410 |
| 100 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 413&#8209;426 |
| 101 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 429&#8209;430 |
| 102 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 433&#8209;437 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 910&#8209;915 |
| 104 | `eq` |  |  | Y |  | Y |  |  | unknown | 946&#8209;947 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 105 | `_example_43_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 106 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 107 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 108 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 174&#8209;230 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 109 | `from_st` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 110 | `size` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 111 | `empty` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 112 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 113 | `find` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | hole | 116&#8209;118 |
| 115 | `delete` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 116 | `filter` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;134 |
| 117 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 118 | `union` | Y | Y |  |  | Y |  |  | hole | 141&#8209;144 |
| 119 | `difference` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 120 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 151&#8209;155 |
| 121 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 122 | `first` | Y | Y |  |  | Y |  |  | hole | 165&#8209;171 |
| 123 | `last` | Y | Y |  |  | Y |  |  | hole | 174&#8209;180 |
| 124 | `previous` | Y | Y |  |  | Y |  |  | hole | 183&#8209;189 |
| 125 | `next` | Y | Y |  |  | Y |  |  | hole | 192&#8209;198 |
| 126 | `split` | Y | Y |  |  | Y |  |  | hole | 201&#8209;203 |
| 127 | `join` | Y | Y |  |  | Y |  |  | hole | 206&#8209;209 |
| 128 | `get_range` | Y | Y |  |  | Y |  |  | hole | 212&#8209;213 |
| 129 | `rank` | Y | Y |  |  | Y |  |  | hole | 216&#8209;221 |
| 130 | `select` | Y | Y |  |  | Y |  |  | hole | 224&#8209;230 |
| 131 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 233&#8209;235 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 132 | `size` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 133 | `empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;75 |
| 134 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 135 | `find` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 136 | `insert` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;97 |
| 137 | `delete` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 138 | `filter` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;120 |
| 139 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 140 | `union` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;139 |
| 141 | `difference` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;147 |
| 142 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;156 |
| 143 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;165 |
| 144 | `first` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;176 |
| 145 | `last` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;185 |
| 146 | `previous` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;194 |
| 147 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 197&#8209;203 |
| 148 | `split` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;218 |
| 149 | `join` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;226 |
| 150 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;234 |
| 151 | `rank` | Y | Y |  |  | Y |  |  | unknown | 237&#8209;242 |
| 152 | `select` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;252 |
| 153 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;266 |
| 154 | `first_iter` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;274 |
| 155 | `last_iter` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;282 |
| 156 | `previous_iter` | Y | Y |  |  | Y |  |  | unknown | 284&#8209;290 |
| 157 | `next_iter` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;298 |
| 158 | `split_iter` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;314 |
| 159 | `get_range_iter` | Y | Y |  |  | Y |  |  | unknown | 316&#8209;321 |
| 160 | `rank_iter` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;328 |
| 161 | `split_rank_iter` | Y | Y |  |  | Y |  |  | unknown | 330&#8209;341 |
| 162 | `iter` |  |  | Y |  | Y |  |  | unknown | 1482&#8209;1487 |
| 163 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 1626&#8209;1628 |
| 164 | `default` |  | Y |  |  |  | Y | Y |  | 1653 |
| 165 | `eq` |  | Y |  |  |  | Y | Y |  | 1657&#8209;1667 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 166 | `size` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;77 |
| 167 | `empty` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 168 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 169 | `find` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 170 | `insert` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;97 |
| 171 | `delete` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 172 | `filter` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;115 |
| 173 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;120 |
| 174 | `union` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 175 | `difference` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 176 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |
| 177 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |
| 178 | `first` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;157 |
| 179 | `last` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;167 |
| 180 | `previous` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |
| 181 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 180&#8209;187 |
| 182 | `split` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;205 |
| 183 | `join` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;213 |
| 184 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;223 |
| 185 | `rank` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;232 |
| 186 | `select` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;245 |
| 187 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;260 |
| 188 | `first_iter` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;269 |
| 189 | `last_iter` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;278 |
| 190 | `previous_iter` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;287 |
| 191 | `next_iter` | Y | Y |  |  | Y |  |  | unknown | 289&#8209;296 |
| 192 | `split_iter` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;313 |
| 193 | `get_range_iter` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;322 |
| 194 | `rank_iter` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;330 |
| 195 | `split_rank_iter` | Y | Y |  |  | Y |  |  | unknown | 332&#8209;344 |
| 196 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 1430&#8209;1432 |
| 197 | `iter` |  |  | Y |  | Y |  |  | unknown | 1444&#8209;1449 |
| 198 | `default` |  | Y |  |  | Y |  |  | unknown | 1589&#8209;1590 |
| 199 | `eq` |  | Y |  |  |  | Y | Y |  | 1622&#8209;1624 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 200 | `size` | Y | Y |  |  | Y |  |  | hole | 90&#8209;92 |
| 201 | `empty` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 202 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 203 | `find` | Y | Y |  |  | Y |  |  | hole | 106&#8209;112 |
| 204 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;122 |
| 205 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 206 | `insert` | Y | Y |  |  | Y |  |  | hole | 132&#8209;138 |
| 207 | `delete` | Y | Y |  |  | Y |  |  | hole | 142&#8209;146 |
| 208 | `domain` | Y | Y |  |  | Y |  |  | hole | 150&#8209;151 |
| 209 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;162 |
| 210 | `map` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 211 | `filter` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;181 |
| 212 | `intersection` | Y | Y |  |  | Y |  |  | hole | 185&#8209;187 |
| 213 | `union` | Y | Y |  |  | Y |  |  | hole | 191&#8209;193 |
| 214 | `difference` | Y | Y |  |  | Y |  |  | hole | 197&#8209;198 |
| 215 | `restrict` | Y | Y |  |  | Y |  |  | hole | 202&#8209;203 |
| 216 | `subtract` | Y | Y |  |  | Y |  |  | hole | 207&#8209;208 |
| 217 | `reduce` | Y | Y |  |  | Y |  |  | hole | 212&#8209;214 |
| 218 | `collect` | Y | Y |  |  | Y |  |  | hole | 218&#8209;219 |
| 219 | `first_key` | Y | Y |  |  | Y |  |  | hole | 223&#8209;229 |
| 220 | `last_key` | Y | Y |  |  | Y |  |  | hole | 233&#8209;239 |
| 221 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 243&#8209;249 |
| 222 | `next_key` | Y | Y |  |  | Y |  |  | hole | 253&#8209;259 |
| 223 | `split_key` | Y | Y |  |  | Y |  |  | hole | 263&#8209;265 |
| 224 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 225 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 274&#8209;275 |
| 226 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 279&#8209;285 |
| 227 | `select_key` | Y | Y |  |  | Y |  |  | hole | 289&#8209;296 |
| 228 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 300&#8209;302 |
| 229 | `from_st` |  |  |  | Y | Y |  |  | hole | 766&#8209;768 |
| 230 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 782&#8209;789 |
| 231 | `next` |  | Y |  |  |  | Y | Y |  | 835&#8209;843 |
| 232 | `iter` |  |  | Y |  |  | Y | Y |  | 847&#8209;858 |
| 233 | `eq` |  | Y |  |  |  | Y | Y |  | 871&#8209;878 |
| 234 | `default` |  | Y |  |  |  | Y | Y |  | 882 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 235 | `from_st_table` |  |  |  | Y | Y |  |  | hole | 72&#8209;76 |
| 236 | `size` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 237 | `empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 238 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 239 | `find` | Y | Y |  |  | Y |  | Y |  | 119 |
| 240 | `insert` | Y | Y |  |  | Y |  |  | hole | 123&#8209;125 |
| 241 | `delete` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 242 | `domain` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 243 | `map` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 244 | `filter` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 245 | `first_key` | Y | Y |  |  | Y |  |  | hole | 151&#8209;157 |
| 246 | `last_key` | Y | Y |  |  | Y |  |  | hole | 161&#8209;167 |
| 247 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 171&#8209;177 |
| 248 | `next_key` | Y | Y |  |  | Y |  |  | hole | 181&#8209;187 |
| 249 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 250 | `join_key` | Y | Y |  |  | Y |  |  | hole | 197&#8209;199 |
| 251 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;204 |
| 252 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 208&#8209;213 |
| 253 | `select_key` | Y | Y |  |  | Y |  |  | hole | 217&#8209;223 |
| 254 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 255 | `default` |  | Y |  |  |  | Y | Y |  | 578 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 256 | `size` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 257 | `empty` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 258 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 259 | `find` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;90 |
| 260 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 261 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 262 | `insert` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;121 |
| 263 | `delete` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;128 |
| 264 | `domain` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 265 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;150 |
| 266 | `map` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;158 |
| 267 | `filter` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;177 |
| 268 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 269 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;200 |
| 270 | `union` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;223 |
| 271 | `difference` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;232 |
| 272 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;241 |
| 273 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;250 |
| 274 | `collect` | Y | Y |  |  | Y |  |  | unknown | 253&#8209;255 |
| 275 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;265 |
| 276 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 268&#8209;275 |
| 277 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 278&#8209;285 |
| 278 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 288&#8209;295 |
| 279 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;317 |
| 280 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 320&#8209;330 |
| 281 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;340 |
| 282 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;351 |
| 283 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 354&#8209;363 |
| 284 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 366&#8209;380 |
| 285 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 382&#8209;388 |
| 286 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 390&#8209;404 |
| 287 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 406&#8209;410 |
| 288 | `first_key_iter` | Y | Y |  |  | Y |  |  | unknown | 412&#8209;419 |
| 289 | `last_key_iter` | Y | Y |  |  | Y |  |  | unknown | 421&#8209;428 |
| 290 | `previous_key_iter` | Y | Y |  |  | Y |  |  | unknown | 430&#8209;437 |
| 291 | `next_key_iter` | Y | Y |  |  | Y |  |  | unknown | 439&#8209;446 |
| 292 | `split_key_iter` | Y | Y |  |  | Y |  |  | unknown | 448&#8209;467 |
| 293 | `get_key_range_iter` | Y | Y |  |  | Y |  |  | unknown | 469&#8209;476 |
| 294 | `rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 478&#8209;486 |
| 295 | `split_rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 488&#8209;502 |
| 296 | `lemma_keys_no_dups_implies_no_duplicates` |  |  |  | Y | Y |  |  | unknown | 509&#8209;511 |
| 297 | `avl_seq_length` |  |  |  | Y | Y |  |  | unknown | 530&#8209;532 |
| 298 | `avl_seq_nth` |  |  |  | Y | Y |  |  | unknown | 538&#8209;540 |
| 299 | `key_in_other` |  |  |  | Y | Y |  |  | unknown | 547&#8209;552 |
| 300 | `iter` |  |  | Y |  | Y |  |  | unknown | 3251&#8209;3256 |
| 301 | `next` |  | Y |  |  | Y |  |  | hole | 3284&#8209;3300 |
| 302 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 3396&#8209;3407 |
| 303 | `eq` |  | Y |  |  |  | Y | Y |  | 3450&#8209;3457 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 304 | `lemma_keys_no_dups_implies_no_duplicates` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 305 | `lemma_keys_no_dups_preserved_by_set_eq` |  |  |  | Y | Y |  |  | unknown | 85&#8209;95 |
| 306 | `lemma_keys_no_dups_after_set_remove` |  |  |  | Y | Y |  |  | unknown | 124&#8209;135 |
| 307 | `lemma_entries_to_map_after_remove_pair` |  |  |  | Y | Y |  |  | unknown | 163&#8209;176 |
| 308 | `lemma_keys_no_dups_after_set_insert` |  |  |  | Y | Y |  |  | unknown | 244&#8209;256 |
| 309 | `lemma_entries_to_map_dom_after_insert` |  |  |  | Y | Y |  |  | unknown | 296&#8209;308 |
| 310 | `lemma_entries_to_map_set_determines_map` |  |  |  | Y | Y |  |  | unknown | 351&#8209;360 |
| 311 | `size` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;405 |
| 312 | `empty` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;409 |
| 313 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 412&#8209;414 |
| 314 | `find` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;423 |
| 315 | `insert` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;434 |
| 316 | `delete` | Y | Y |  |  | Y |  |  | unknown | 437&#8209;442 |
| 317 | `domain` | Y | Y |  |  | Y |  |  | unknown | 445&#8209;447 |
| 318 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 450&#8209;463 |
| 319 | `map` | Y | Y |  |  | Y |  |  | unknown | 466&#8209;476 |
| 320 | `filter` | Y | Y |  |  | Y |  |  | unknown | 479&#8209;490 |
| 321 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 493&#8209;507 |
| 322 | `union` | Y | Y |  |  | Y |  |  | unknown | 510&#8209;529 |
| 323 | `difference` | Y | Y |  |  | Y |  |  | unknown | 532&#8209;538 |
| 324 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 541&#8209;547 |
| 325 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 550&#8209;556 |
| 326 | `collect` | Y | Y |  |  | Y |  |  | unknown | 559&#8209;561 |
| 327 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 564&#8209;571 |
| 328 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 574&#8209;581 |
| 329 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 584&#8209;591 |
| 330 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 594&#8209;601 |
| 331 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 604&#8209;622 |
| 332 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 625&#8209;635 |
| 333 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 638&#8209;645 |
| 334 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 648&#8209;656 |
| 335 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 659&#8209;668 |
| 336 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 671&#8209;684 |
| 337 | `find_iter` | Y | Y |  |  | Y |  |  | unknown | 686&#8209;692 |
| 338 | `insert_iter` | Y | Y |  |  | Y |  |  | unknown | 694&#8209;702 |
| 339 | `delete_iter` | Y | Y |  |  | Y |  |  | unknown | 704&#8209;709 |
| 340 | `first_key_iter` | Y | Y |  |  | Y |  |  | unknown | 711&#8209;718 |
| 341 | `last_key_iter` | Y | Y |  |  | Y |  |  | unknown | 720&#8209;727 |
| 342 | `previous_key_iter` | Y | Y |  |  | Y |  |  | unknown | 729&#8209;736 |
| 343 | `next_key_iter` | Y | Y |  |  | Y |  |  | unknown | 738&#8209;745 |
| 344 | `split_key_iter` | Y | Y |  |  | Y |  |  | unknown | 747&#8209;765 |
| 345 | `get_key_range_iter` | Y | Y |  |  | Y |  |  | unknown | 767&#8209;774 |
| 346 | `rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 776&#8209;784 |
| 347 | `split_rank_key_iter` | Y | Y |  |  | Y |  |  | unknown | 786&#8209;799 |
| 348 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 3384&#8209;3393 |
| 349 | `iter` |  |  | Y |  | Y |  |  | unknown | 3418&#8209;3423 |
| 350 | `next` |  | Y |  |  | Y |  |  | hole | 3451&#8209;3467 |
| 351 | `eq` |  | Y |  |  | Y |  |  | hole | 3563&#8209;3564 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
