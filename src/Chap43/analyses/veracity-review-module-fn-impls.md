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
| 6 | Chap43 | OrderedSetStEph | 22 | 24 | 1 | 1 | 24 | 2 | 22 | 2 | 2 |
| 7 | Chap43 | OrderedSetStPer | 22 | 24 | 1 | 1 | 25 | 1 | 23 | 2 | 1 |
| 8 | Chap43 | OrderedTableMtEph | 29 | 32 | 1 | 2 | 31 | 4 | 9 | 22 | 4 |
| 9 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 1 | 20 | 1 | 9 | 10 | 2 |
| 10 | Chap43 | OrderedTableStEph | 29 | 31 | 1 | 5 | 36 | 1 | 35 | 1 | 1 |
| 11 | Chap43 | OrderedTableStPer | 26 | 28 | 1 | 8 | 37 | 0 | 35 | 2 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `recalculate_reduction` |  |  |  | Y | Y |  |  | unknown | 71&#8209;75 |
| 2 | `calculate_reduction` |  |  |  | Y | Y |  |  | unknown | 82&#8209;88 |
| 3 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 114&#8209;117 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 7 | `find` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;150 |
| 8 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;174 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;182 |
| 12 | `domain` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 13 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;202 |
| 14 | `map` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;209 |
| 15 | `filter` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;217 |
| 16 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;224 |
| 17 | `union` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;231 |
| 18 | `difference` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;236 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;241 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;246 |
| 21 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 249&#8209;251 |
| 22 | `collect` | Y | Y |  |  | Y |  |  | unknown | 254&#8209;255 |
| 23 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;264 |
| 24 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |
| 25 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 276&#8209;282 |
| 26 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 285&#8209;291 |
| 27 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 294&#8209;297 |
| 28 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;302 |
| 29 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 305&#8209;307 |
| 30 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;316 |
| 31 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 319&#8209;326 |
| 32 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 329&#8209;332 |
| 33 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;337 |
| 34 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 340&#8209;342 |
| 35 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;351 |
| 36 | `iter` |  |  | Y |  |  | Y | Y |  | 750&#8209;752 |
| 37 | `eq` |  | Y |  |  |  | Y | Y |  | 767&#8209;770 |

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
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;176 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;185 |
| 48 | `domain` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 49 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;208 |
| 50 | `map` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;226 |
| 52 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;231 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;248 |
| 54 | `union` | Y | Y |  |  | Y |  |  | unknown | 251&#8209;271 |
| 55 | `difference` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;279 |
| 56 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;287 |
| 57 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;295 |
| 58 | `collect` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;300 |
| 59 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 303&#8209;310 |
| 60 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 313&#8209;320 |
| 61 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 323&#8209;330 |
| 62 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 333&#8209;340 |
| 63 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 343&#8209;352 |
| 64 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;365 |
| 65 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 368&#8209;372 |
| 66 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;381 |
| 67 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 384&#8209;391 |
| 68 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 394&#8209;403 |
| 69 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 406&#8209;407 |
| 70 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;412 |
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
| 79 | `insert` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;163 |
| 80 | `delete` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;173 |
| 81 | `domain` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 82 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;195 |
| 83 | `map` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;212 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | unknown | 215&#8209;228 |
| 85 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;247 |
| 86 | `union` | Y | Y |  |  | Y |  |  | unknown | 250&#8209;270 |
| 87 | `difference` | Y | Y |  |  | Y |  |  | unknown | 273&#8209;283 |
| 88 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 286&#8209;295 |
| 89 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;307 |
| 90 | `collect` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;312 |
| 91 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;322 |
| 92 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 325&#8209;332 |
| 93 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;342 |
| 94 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 345&#8209;352 |
| 95 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;373 |
| 96 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 376&#8209;386 |
| 97 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 389&#8209;396 |
| 98 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 399&#8209;407 |
| 99 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;419 |
| 100 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 422&#8209;435 |
| 101 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;439 |
| 102 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 442&#8209;446 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 908&#8209;913 |
| 104 | `eq` |  |  | Y |  | Y |  |  | unknown | 944&#8209;945 |

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
| 110 | `size` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 111 | `empty` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;104 |
| 112 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 113 | `find` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | hole | 115&#8209;117 |
| 115 | `delete` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 116 | `filter` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;133 |
| 117 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 118 | `union` | Y | Y |  |  | Y |  |  | hole | 140&#8209;143 |
| 119 | `difference` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 120 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 150&#8209;154 |
| 121 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 122 | `first` | Y | Y |  |  | Y |  |  | hole | 164&#8209;170 |
| 123 | `last` | Y | Y |  |  | Y |  |  | hole | 173&#8209;179 |
| 124 | `previous` | Y | Y |  |  | Y |  |  | hole | 182&#8209;188 |
| 125 | `next` | Y | Y |  |  | Y |  |  | hole | 191&#8209;197 |
| 126 | `split` | Y | Y |  |  | Y |  |  | hole | 200&#8209;202 |
| 127 | `join` | Y | Y |  |  | Y |  |  | hole | 205&#8209;208 |
| 128 | `get_range` | Y | Y |  |  | Y |  |  | hole | 211&#8209;212 |
| 129 | `rank` | Y | Y |  |  | Y |  |  | hole | 215&#8209;220 |
| 130 | `select` | Y | Y |  |  | Y |  |  | hole | 223&#8209;229 |
| 131 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 232&#8209;234 |

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
| 144 | `first` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |
| 145 | `last` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;187 |
| 146 | `previous` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;197 |
| 147 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 200&#8209;207 |
| 148 | `split` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;222 |
| 149 | `join` | Y | Y |  |  | Y |  |  | unknown | 225&#8209;230 |
| 150 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 233&#8209;238 |
| 151 | `rank` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;247 |
| 152 | `select` | Y | Y |  |  | Y |  |  | hole | 250&#8209;257 |
| 153 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 260&#8209;271 |
| 154 | `iter` |  |  | Y |  | Y |  |  | unknown | 1275&#8209;1280 |
| 155 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 1419&#8209;1421 |
| 156 | `default` |  | Y |  |  |  | Y | Y |  | 1445 |
| 157 | `eq` |  | Y |  |  |  | Y | Y |  | 1449&#8209;1459 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `size` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;70 |
| 159 | `empty` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 160 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 161 | `find` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 162 | `insert` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 163 | `delete` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 164 | `filter` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;108 |
| 165 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 166 | `union` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;121 |
| 167 | `difference` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 168 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 169 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 170 | `first` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;150 |
| 171 | `last` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;160 |
| 172 | `previous` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;170 |
| 173 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 173&#8209;180 |
| 174 | `split` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;198 |
| 175 | `join` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;206 |
| 176 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;216 |
| 177 | `rank` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;225 |
| 178 | `select` | Y | Y |  |  | Y |  |  | hole | 228&#8209;235 |
| 179 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;250 |
| 180 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 1200&#8209;1202 |
| 181 | `iter` |  |  | Y |  | Y |  |  | unknown | 1213&#8209;1218 |
| 182 | `default` |  | Y |  |  | Y |  |  | unknown | 1358&#8209;1359 |
| 183 | `eq` |  | Y |  |  |  | Y | Y |  | 1391&#8209;1393 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 184 | `size` | Y | Y |  |  | Y |  |  | hole | 88&#8209;90 |
| 185 | `empty` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 186 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 187 | `find` | Y | Y |  |  | Y |  |  | hole | 104&#8209;110 |
| 188 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;120 |
| 189 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 190 | `insert` | Y | Y |  |  | Y |  |  | hole | 130&#8209;137 |
| 191 | `delete` | Y | Y |  |  | Y |  |  | hole | 141&#8209;146 |
| 192 | `domain` | Y | Y |  |  | Y |  |  | hole | 150&#8209;151 |
| 193 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;162 |
| 194 | `map` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 195 | `filter` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;181 |
| 196 | `intersection` | Y | Y |  |  | Y |  |  | hole | 185&#8209;187 |
| 197 | `union` | Y | Y |  |  | Y |  |  | hole | 191&#8209;193 |
| 198 | `difference` | Y | Y |  |  | Y |  |  | hole | 197&#8209;198 |
| 199 | `restrict` | Y | Y |  |  | Y |  |  | hole | 202&#8209;203 |
| 200 | `subtract` | Y | Y |  |  | Y |  |  | hole | 207&#8209;208 |
| 201 | `reduce` | Y | Y |  |  | Y |  |  | hole | 212&#8209;214 |
| 202 | `collect` | Y | Y |  |  | Y |  |  | hole | 218&#8209;219 |
| 203 | `first_key` | Y | Y |  |  | Y |  |  | hole | 223&#8209;229 |
| 204 | `last_key` | Y | Y |  |  | Y |  |  | hole | 233&#8209;239 |
| 205 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 243&#8209;249 |
| 206 | `next_key` | Y | Y |  |  | Y |  |  | hole | 253&#8209;259 |
| 207 | `split_key` | Y | Y |  |  | Y |  |  | hole | 263&#8209;265 |
| 208 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;270 |
| 209 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 274&#8209;275 |
| 210 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 279&#8209;285 |
| 211 | `select_key` | Y | Y |  |  | Y |  |  | hole | 289&#8209;296 |
| 212 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 300&#8209;302 |
| 213 | `from_st` |  |  |  | Y | Y |  |  | hole | 756&#8209;758 |
| 214 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 769&#8209;776 |
| 215 | `next` |  | Y |  |  |  | Y | Y |  | 820&#8209;828 |
| 216 | `iter` |  |  | Y |  |  | Y | Y |  | 832&#8209;843 |
| 217 | `eq` |  | Y |  |  |  | Y | Y |  | 856&#8209;863 |
| 218 | `default` |  | Y |  |  |  | Y | Y |  | 867 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 219 | `from_st_table` |  |  |  | Y | Y |  |  | hole | 72&#8209;76 |
| 220 | `size` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 221 | `empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;110 |
| 222 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 223 | `find` | Y | Y |  |  | Y |  | Y |  | 119 |
| 224 | `insert` | Y | Y |  |  | Y |  |  | hole | 123&#8209;125 |
| 225 | `delete` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 226 | `domain` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 227 | `map` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;141 |
| 228 | `filter` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 229 | `first_key` | Y | Y |  |  | Y |  |  | hole | 151&#8209;157 |
| 230 | `last_key` | Y | Y |  |  | Y |  |  | hole | 161&#8209;167 |
| 231 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 171&#8209;177 |
| 232 | `next_key` | Y | Y |  |  | Y |  |  | hole | 181&#8209;187 |
| 233 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 234 | `join_key` | Y | Y |  |  | Y |  |  | hole | 197&#8209;199 |
| 235 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;204 |
| 236 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 208&#8209;213 |
| 237 | `select_key` | Y | Y |  |  | Y |  |  | hole | 217&#8209;223 |
| 238 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;229 |
| 239 | `default` |  | Y |  |  |  | Y | Y |  | 578 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 240 | `size` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 241 | `empty` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 242 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 243 | `find` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;90 |
| 244 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 245 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 246 | `insert` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;122 |
| 247 | `delete` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;131 |
| 248 | `domain` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 249 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;153 |
| 250 | `map` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;161 |
| 251 | `filter` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;180 |
| 252 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;185 |
| 253 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;203 |
| 254 | `union` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;227 |
| 255 | `difference` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;236 |
| 256 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;245 |
| 257 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;254 |
| 258 | `collect` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;259 |
| 259 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 262&#8209;269 |
| 260 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 272&#8209;279 |
| 261 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 282&#8209;289 |
| 262 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;299 |
| 263 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 302&#8209;321 |
| 264 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 324&#8209;335 |
| 265 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 338&#8209;345 |
| 266 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;356 |
| 267 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 359&#8209;368 |
| 268 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 371&#8209;385 |
| 269 | `lemma_keys_no_dups_implies_no_duplicates` |  |  |  | Y | Y |  |  | unknown | 392&#8209;394 |
| 270 | `avl_seq_length` |  |  |  | Y | Y |  |  | unknown | 413&#8209;415 |
| 271 | `avl_seq_nth` |  |  |  | Y | Y |  |  | unknown | 421&#8209;423 |
| 272 | `key_in_other` |  |  |  | Y | Y |  |  | unknown | 430&#8209;435 |
| 273 | `iter` |  |  | Y |  | Y |  |  | unknown | 3085&#8209;3090 |
| 274 | `next` |  | Y |  |  | Y |  |  | hole | 3118&#8209;3134 |
| 275 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 3229&#8209;3240 |
| 276 | `eq` |  | Y |  |  |  | Y | Y |  | 3282&#8209;3289 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 277 | `lemma_keys_no_dups_implies_no_duplicates` |  |  |  | Y | Y |  |  | unknown | 70&#8209;72 |
| 278 | `lemma_keys_no_dups_preserved_by_set_eq` |  |  |  | Y | Y |  |  | unknown | 85&#8209;95 |
| 279 | `lemma_keys_no_dups_after_set_remove` |  |  |  | Y | Y |  |  | unknown | 124&#8209;135 |
| 280 | `lemma_entries_to_map_after_remove_pair` |  |  |  | Y | Y |  |  | unknown | 163&#8209;176 |
| 281 | `lemma_keys_no_dups_after_set_insert` |  |  |  | Y | Y |  |  | unknown | 244&#8209;256 |
| 282 | `lemma_entries_to_map_dom_after_insert` |  |  |  | Y | Y |  |  | unknown | 296&#8209;308 |
| 283 | `lemma_entries_to_map_set_determines_map` |  |  |  | Y | Y |  |  | unknown | 351&#8209;360 |
| 284 | `size` | Y | Y |  |  | Y |  |  | unknown | 403&#8209;405 |
| 285 | `empty` | Y | Y |  |  | Y |  |  | unknown | 408&#8209;409 |
| 286 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 412&#8209;414 |
| 287 | `find` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;423 |
| 288 | `insert` | Y | Y |  |  | Y |  |  | unknown | 426&#8209;435 |
| 289 | `delete` | Y | Y |  |  | Y |  |  | unknown | 438&#8209;444 |
| 290 | `domain` | Y | Y |  |  | Y |  |  | unknown | 447&#8209;449 |
| 291 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 452&#8209;465 |
| 292 | `map` | Y | Y |  |  | Y |  |  | unknown | 468&#8209;478 |
| 293 | `filter` | Y | Y |  |  | Y |  |  | unknown | 481&#8209;493 |
| 294 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 496&#8209;511 |
| 295 | `union` | Y | Y |  |  | Y |  |  | unknown | 514&#8209;534 |
| 296 | `difference` | Y | Y |  |  | Y |  |  | unknown | 537&#8209;543 |
| 297 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 546&#8209;552 |
| 298 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 555&#8209;561 |
| 299 | `collect` | Y | Y |  |  | Y |  |  | unknown | 564&#8209;566 |
| 300 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 569&#8209;576 |
| 301 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 579&#8209;586 |
| 302 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 589&#8209;596 |
| 303 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 599&#8209;606 |
| 304 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 609&#8209;627 |
| 305 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 630&#8209;640 |
| 306 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 643&#8209;650 |
| 307 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 653&#8209;661 |
| 308 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 664&#8209;673 |
| 309 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 676&#8209;689 |
| 310 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 3211&#8209;3220 |
| 311 | `iter` |  |  | Y |  | Y |  |  | unknown | 3242&#8209;3247 |
| 312 | `next` |  | Y |  |  | Y |  |  | hole | 3275&#8209;3291 |
| 313 | `eq` |  | Y |  |  | Y |  |  | hole | 3387&#8209;3388 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
