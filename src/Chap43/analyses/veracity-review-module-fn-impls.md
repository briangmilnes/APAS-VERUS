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
| 1 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 1 | 3 | 36 | 1 | 30 | 4 | 3 |
| 2 | Chap43 | AugOrderedTableStEph | 31 | 32 | 1 | 2 | 34 | 1 | 30 | 4 | 1 |
| 3 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 2 | 32 | 0 | 30 | 2 | 0 |
| 4 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 5 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 0 | 22 | 0 | 0 | 22 | 0 |
| 6 | Chap43 | OrderedSetStEph | 22 | 24 | 1 | 1 | 24 | 2 | 11 | 13 | 2 |
| 7 | Chap43 | OrderedSetStPer | 22 | 24 | 1 | 1 | 25 | 1 | 14 | 11 | 1 |
| 8 | Chap43 | OrderedTableMtEph | 29 | 31 | 1 | 1 | 32 | 1 | 14 | 16 | 3 |
| 9 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 0 | 19 | 1 | 0 | 19 | 1 |
| 10 | Chap43 | OrderedTableStEph | 29 | 31 | 1 | 1 | 32 | 1 | 14 | 18 | 1 |
| 11 | Chap43 | OrderedTableStPer | 26 | 28 | 1 | 1 | 30 | 0 | 18 | 12 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `recalculate_reduction` |  |  |  | Y | Y |  |  | hole | 67&#8209;70 |
| 2 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 76&#8209;81 |
| 3 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 106&#8209;109 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 7 | `find` | Y | Y |  |  | Y |  | Y |  | 123 |
| 8 | `lookup` | Y | Y |  |  | Y |  | Y |  | 124 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 12 | `domain` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 13 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;139 |
| 14 | `map` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 15 | `filter` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 16 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 17 | `union` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 18 | `difference` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 21 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 22 | `collect` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 23 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 24 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 25 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;163 |
| 26 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;165 |
| 27 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 28 | `join_key` | Y | Y |  |  | Y |  |  | hole | 169&#8209;170 |
| 29 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;172 |
| 30 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;174 |
| 31 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;176 |
| 32 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 33 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 34 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 35 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | hole | 184&#8209;185 |
| 36 | `iter` |  |  | Y |  | Y |  |  | unknown | 562&#8209;566 |
| 37 | `eq` |  | Y |  |  |  | Y | Y |  | 606&#8209;609 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 71&#8209;78 |
| 39 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 103&#8209;106 |
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 41 | `empty` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 42 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 43 | `find` | Y | Y |  |  | Y |  |  | hole | 123&#8209;129 |
| 44 | `lookup` | Y | Y |  |  | Y |  |  | hole | 130&#8209;136 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 48 | `domain` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 49 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 50 | `map` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 52 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;158 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;164 |
| 54 | `union` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;171 |
| 55 | `difference` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;174 |
| 56 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 57 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 58 | `collect` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 59 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 60 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 61 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 62 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;190 |
| 63 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 64 | `join_key` | Y | Y |  |  | Y |  |  | hole | 194&#8209;196 |
| 65 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;198 |
| 66 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;200 |
| 67 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 201&#8209;202 |
| 68 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 69 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;207 |
| 70 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;209 |
| 71 | `iter` |  |  | Y |  | Y |  |  | unknown | 564&#8209;568 |
| 72 | `eq` |  | Y |  |  |  | Y | Y |  | 614&#8209;617 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 73 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 74&#8209;81 |
| 74 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 110&#8209;113 |
| 75 | `size` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 76 | `empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 77 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 78 | `find` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;140 |
| 79 | `insert` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 80 | `delete` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;150 |
| 81 | `domain` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 82 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 83 | `map` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 85 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;167 |
| 86 | `union` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;174 |
| 87 | `difference` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 88 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 89 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 90 | `collect` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 91 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;190 |
| 92 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;195 |
| 93 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;199 |
| 94 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;203 |
| 95 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;211 |
| 96 | `join_key` | Y | Y |  |  | Y |  |  | hole | 212&#8209;218 |
| 97 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;222 |
| 98 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;226 |
| 99 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 227&#8209;231 |
| 100 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 232&#8209;239 |
| 101 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 240&#8209;241 |
| 102 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 242&#8209;243 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 669&#8209;673 |
| 104 | `eq` |  |  | Y |  | Y |  |  | unknown | 703&#8209;704 |

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
| 109 | `size` | Y | Y |  |  | Y |  |  | hole | 54&#8209;55 |
| 110 | `empty` | Y | Y |  |  | Y |  |  | hole | 57&#8209;58 |
| 111 | `singleton` | Y | Y |  |  | Y |  |  | hole | 60&#8209;61 |
| 112 | `find` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 113 | `insert` | Y | Y |  |  | Y |  |  | hole | 66&#8209;67 |
| 114 | `delete` | Y | Y |  |  | Y |  |  | hole | 69&#8209;70 |
| 115 | `filter` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 116 | `intersection` | Y | Y |  |  | Y |  |  | hole | 75&#8209;76 |
| 117 | `union` | Y | Y |  |  | Y |  |  | hole | 78&#8209;79 |
| 118 | `difference` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 119 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 120 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 121 | `first` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 122 | `last` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 123 | `previous` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 124 | `next` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 125 | `split` | Y | Y |  |  | Y |  |  | hole | 104&#8209;106 |
| 126 | `join` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 127 | `get_range` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 128 | `rank` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 129 | `select` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 130 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 120&#8209;122 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 131 | `size` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 132 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 133 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 134 | `find` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 135 | `insert` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 136 | `delete` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 137 | `filter` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 138 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 139 | `union` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 140 | `difference` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;86 |
| 141 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 142 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 143 | `first` | Y | Y |  |  | Y |  |  | hole | 96&#8209;97 |
| 144 | `last` | Y | Y |  |  | Y |  |  | hole | 99&#8209;100 |
| 145 | `previous` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 146 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 105&#8209;106 |
| 147 | `split` | Y | Y |  |  | Y |  |  | hole | 108&#8209;110 |
| 148 | `join` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;113 |
| 149 | `get_range` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 150 | `rank` | Y | Y |  |  | Y |  |  | hole | 118&#8209;119 |
| 151 | `select` | Y | Y |  |  | Y |  |  | hole | 121&#8209;122 |
| 152 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 124&#8209;126 |
| 153 | `iter` |  |  | Y |  | Y |  |  | hole | 401&#8209;405 |
| 154 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 545&#8209;546 |
| 155 | `default` |  | Y |  |  |  | Y | Y |  | 570 |
| 156 | `eq` |  | Y |  |  |  | Y | Y |  | 574&#8209;584 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 157 | `size` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 158 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 159 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 160 | `find` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 161 | `insert` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 162 | `delete` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 163 | `filter` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 164 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 165 | `union` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 166 | `difference` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 167 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 168 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 169 | `first` | Y | Y |  |  | Y |  |  | hole | 97&#8209;101 |
| 170 | `last` | Y | Y |  |  | Y |  |  | hole | 103&#8209;107 |
| 171 | `previous` | Y | Y |  |  | Y |  |  | hole | 109&#8209;112 |
| 172 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 114&#8209;117 |
| 173 | `split` | Y | Y |  |  | Y |  |  | hole | 119&#8209;128 |
| 174 | `join` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 175 | `get_range` | Y | Y |  |  | Y |  |  | hole | 133&#8209;137 |
| 176 | `rank` | Y | Y |  |  | Y |  |  | hole | 139&#8209;142 |
| 177 | `select` | Y | Y |  |  | Y |  |  | hole | 144&#8209;148 |
| 178 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 150&#8209;157 |
| 179 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 438&#8209;439 |
| 180 | `iter` |  |  | Y |  | Y |  |  | hole | 450&#8209;454 |
| 181 | `default` |  | Y |  |  | Y |  |  | unknown | 594&#8209;595 |
| 182 | `eq` |  | Y |  |  |  | Y | Y |  | 627&#8209;629 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 183 | `size` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 184 | `empty` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 185 | `singleton` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 186 | `find` | Y | Y |  |  | Y |  | Y |  | 68 |
| 187 | `lookup` | Y | Y |  |  | Y |  | Y |  | 70 |
| 188 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 189 | `insert` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 190 | `delete` | Y | Y |  |  | Y |  |  | hole | 78&#8209;79 |
| 191 | `domain` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 192 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 193 | `map` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 194 | `filter` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 195 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;94 |
| 196 | `union` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 197 | `difference` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;100 |
| 198 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;103 |
| 199 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 200 | `reduce` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 201 | `collect` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 202 | `first_key` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 203 | `last_key` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 204 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 205 | `next_key` | Y | Y |  |  | Y |  |  | hole | 123&#8209;124 |
| 206 | `split_key` | Y | Y |  |  | Y |  |  | hole | 126&#8209;128 |
| 207 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 208 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 133&#8209;134 |
| 209 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 136&#8209;137 |
| 210 | `select_key` | Y | Y |  |  | Y |  |  | hole | 139&#8209;140 |
| 211 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 142&#8209;144 |
| 212 | `iter` |  |  | Y |  | Y |  |  | unknown | 502&#8209;506 |
| 213 | `next` |  | Y |  |  | Y |  |  | unknown | 530&#8209;546 |
| 214 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 637&#8209;638 |
| 215 | `eq` |  | Y |  |  |  | Y | Y |  | 655&#8209;657 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 216 | `size` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 217 | `empty` | Y | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 218 | `singleton` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 219 | `find` | Y | Y |  |  | Y |  |  | hole | 68 |
| 220 | `insert` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 221 | `delete` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 222 | `domain` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 223 | `map` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 224 | `filter` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 225 | `first_key` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 226 | `last_key` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 227 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 228 | `next_key` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 229 | `split_key` | Y | Y |  |  | Y |  |  | hole | 97&#8209;99 |
| 230 | `join_key` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 231 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 232 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 233 | `select_key` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 234 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 113&#8209;115 |
| 235 | `default` |  | Y |  |  |  | Y | Y |  | 375 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 236 | `size` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 237 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 238 | `singleton` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 239 | `find` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 240 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 241 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 242 | `insert` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;76 |
| 243 | `delete` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 244 | `domain` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 245 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 246 | `map` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 247 | `filter` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 248 | `reduce` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 249 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;95 |
| 250 | `union` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;102 |
| 251 | `difference` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 252 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 253 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 254 | `collect` | Y | Y |  |  | Y |  |  | hole | 112&#8209;113 |
| 255 | `first_key` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 256 | `last_key` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 257 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 118&#8209;119 |
| 258 | `next_key` | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 259 | `split_key` | Y | Y |  |  | Y |  |  | hole | 122&#8209;124 |
| 260 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 261 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 128&#8209;129 |
| 262 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 130&#8209;131 |
| 263 | `select_key` | Y | Y |  |  | Y |  |  | hole | 132&#8209;133 |
| 264 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 134&#8209;136 |
| 265 | `iter` |  |  | Y |  | Y |  |  | unknown | 491&#8209;495 |
| 266 | `next` |  | Y |  |  | Y |  |  | unknown | 519&#8209;535 |
| 267 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 623&#8209;625 |
| 268 | `eq` |  | Y |  |  |  | Y | Y |  | 639&#8209;641 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 269 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 270 | `empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 271 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 272 | `find` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 273 | `insert` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 274 | `delete` | Y | Y |  |  | Y |  |  | hole | 83&#8209;89 |
| 275 | `domain` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 276 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 277 | `map` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 278 | `filter` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 279 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;106 |
| 280 | `union` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;113 |
| 281 | `difference` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 282 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 283 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 284 | `collect` | Y | Y |  |  | Y |  |  | hole | 123&#8209;124 |
| 285 | `first_key` | Y | Y |  |  | Y |  |  | hole | 126&#8209;130 |
| 286 | `last_key` | Y | Y |  |  | Y |  |  | hole | 132&#8209;136 |
| 287 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 138&#8209;141 |
| 288 | `next_key` | Y | Y |  |  | Y |  |  | hole | 143&#8209;146 |
| 289 | `split_key` | Y | Y |  |  | Y |  |  | hole | 148&#8209;155 |
| 290 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;162 |
| 291 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 164&#8209;167 |
| 292 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 169&#8209;172 |
| 293 | `select_key` | Y | Y |  |  | Y |  |  | hole | 174&#8209;178 |
| 294 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 180&#8209;187 |
| 295 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 542&#8209;545 |
| 296 | `iter` |  |  | Y |  | Y |  |  | unknown | 570&#8209;574 |
| 297 | `next` |  | Y |  |  | Y |  |  | unknown | 598&#8209;614 |
| 298 | `eq` |  | Y |  |  | Y |  |  | unknown | 702&#8209;703 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
