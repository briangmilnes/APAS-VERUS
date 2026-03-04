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
| 1 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 0 | 3 | 35 | 1 | 29 | 4 | 3 |
| 2 | Chap43 | AugOrderedTableStEph | 31 | 32 | 0 | 2 | 33 | 1 | 29 | 4 | 1 |
| 3 | Chap43 | AugOrderedTableStPer | 28 | 29 | 0 | 2 | 30 | 1 | 28 | 2 | 1 |
| 4 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 5 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 0 | 22 | 0 | 0 | 22 | 0 |
| 6 | Chap43 | OrderedSetStEph | 22 | 24 | 0 | 1 | 23 | 2 | 11 | 12 | 2 |
| 7 | Chap43 | OrderedSetStPer | 22 | 24 | 0 | 1 | 23 | 2 | 13 | 10 | 2 |
| 8 | Chap43 | OrderedTableMtEph | 29 | 30 | 0 | 1 | 30 | 1 | 12 | 16 | 3 |
| 9 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 0 | 19 | 1 | 0 | 19 | 1 |
| 10 | Chap43 | OrderedTableStEph | 29 | 30 | 0 | 1 | 30 | 1 | 12 | 18 | 1 |
| 11 | Chap43 | OrderedTableStPer | 26 | 28 | 1 | 1 | 30 | 0 | 26 | 4 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `recalculate_reduction` |  |  |  | Y | Y |  |  | hole | 66&#8209;69 |
| 2 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 75&#8209;80 |
| 3 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 105&#8209;108 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 7 | `find` | Y | Y |  |  | Y |  | Y |  | 122 |
| 8 | `lookup` | Y | Y |  |  | Y |  | Y |  | 123 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 12 | `domain` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 13 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;138 |
| 14 | `map` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 15 | `filter` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 16 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 17 | `union` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 18 | `difference` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 21 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 22 | `collect` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 23 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;158 |
| 24 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 25 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 26 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 27 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;167 |
| 28 | `join_key` | Y | Y |  |  | Y |  |  | hole | 168&#8209;169 |
| 29 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;171 |
| 30 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;173 |
| 31 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;175 |
| 32 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 33 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;180 |
| 34 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 35 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | hole | 183&#8209;184 |
| 36 | `eq` |  | Y |  |  |  | Y | Y |  | 578&#8209;581 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 70&#8209;77 |
| 38 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 102&#8209;105 |
| 39 | `size` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 40 | `empty` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;119 |
| 41 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 42 | `find` | Y | Y |  |  | Y |  |  | hole | 122&#8209;128 |
| 43 | `lookup` | Y | Y |  |  | Y |  |  | hole | 129&#8209;135 |
| 44 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 45 | `insert` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;143 |
| 46 | `delete` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 47 | `domain` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 48 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 49 | `map` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 50 | `filter` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;155 |
| 51 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;157 |
| 52 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;163 |
| 53 | `union` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;170 |
| 54 | `difference` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 55 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 56 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 57 | `collect` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 58 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 59 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 60 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 61 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;189 |
| 62 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 63 | `join_key` | Y | Y |  |  | Y |  |  | hole | 193&#8209;195 |
| 64 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 65 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;199 |
| 66 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 67 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;204 |
| 68 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 69 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 70 | `eq` |  | Y |  |  |  | Y | Y |  | 586&#8209;589 |

### Chap43/AugOrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 71 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 69&#8209;76 |
| 72 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 106&#8209;109 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 74 | `empty` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 75 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;129 |
| 76 | `find` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;136 |
| 77 | `insert` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 78 | `delete` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;146 |
| 79 | `domain` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 80 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 81 | `map` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;154 |
| 82 | `filter` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 83 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;163 |
| 84 | `union` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;170 |
| 85 | `difference` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;173 |
| 86 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 87 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;179 |
| 88 | `collect` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;181 |
| 89 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;183 |
| 90 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;185 |
| 91 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;187 |
| 92 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;189 |
| 93 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;192 |
| 94 | `join_key` | Y | Y |  |  | Y |  |  | hole | 193&#8209;199 |
| 95 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 96 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 97 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;205 |
| 98 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;208 |
| 99 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 100 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;212 |
| 101 | `eq` |  | Y |  |  |  | Y | Y |  | 623&#8209;626 |

### Chap43/Example43_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 102 | `_example_43_1_verified` |  |  |  | Y | Y |  | Y |  | 11 |
| 103 | `run_example43_1` | Y |  |  | Y |  | Y | Y |  | 19&#8209;21 |
| 104 | `demonstrate_ordered_operations` | Y |  |  |  |  | Y | Y |  | 23&#8209;25 |
| 105 | `run_integer_example` |  |  |  | Y |  | Y | Y |  | 174&#8209;230 |

### Chap43/OrderedSetMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `size` | Y | Y |  |  | Y |  |  | hole | 54&#8209;55 |
| 107 | `empty` | Y | Y |  |  | Y |  |  | hole | 57&#8209;58 |
| 108 | `singleton` | Y | Y |  |  | Y |  |  | hole | 60&#8209;61 |
| 109 | `find` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 110 | `insert` | Y | Y |  |  | Y |  |  | hole | 66&#8209;67 |
| 111 | `delete` | Y | Y |  |  | Y |  |  | hole | 69&#8209;70 |
| 112 | `filter` | Y | Y |  |  | Y |  |  | hole | 72&#8209;73 |
| 113 | `intersection` | Y | Y |  |  | Y |  |  | hole | 75&#8209;76 |
| 114 | `union` | Y | Y |  |  | Y |  |  | hole | 78&#8209;79 |
| 115 | `difference` | Y | Y |  |  | Y |  |  | hole | 81&#8209;82 |
| 116 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 84&#8209;85 |
| 117 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 118 | `first` | Y | Y |  |  | Y |  |  | hole | 92&#8209;93 |
| 119 | `last` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 120 | `previous` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 121 | `next` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 122 | `split` | Y | Y |  |  | Y |  |  | hole | 104&#8209;106 |
| 123 | `join` | Y | Y |  |  | Y |  |  | hole | 108&#8209;109 |
| 124 | `get_range` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 125 | `rank` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 126 | `select` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 127 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 120&#8209;122 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 128 | `size` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 129 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 130 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 131 | `find` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 132 | `insert` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 133 | `delete` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;73 |
| 134 | `filter` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;76 |
| 135 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 136 | `union` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 137 | `difference` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 138 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 139 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 90&#8209;91 |
| 140 | `first` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 141 | `last` | Y | Y |  |  | Y |  |  | hole | 98&#8209;99 |
| 142 | `previous` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 143 | `next` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 144 | `split` | Y | Y |  |  | Y |  |  | hole | 107&#8209;109 |
| 145 | `join` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 146 | `get_range` | Y | Y |  |  | Y |  |  | hole | 114&#8209;115 |
| 147 | `rank` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 148 | `select` | Y | Y |  |  | Y |  |  | hole | 120&#8209;121 |
| 149 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 123&#8209;125 |
| 150 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 408&#8209;409 |
| 151 | `default` |  | Y |  |  |  | Y | Y |  | 433 |
| 152 | `eq` |  | Y |  |  |  | Y | Y |  | 437&#8209;447 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 153 | `size` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;57 |
| 154 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 155 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;63 |
| 156 | `find` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 157 | `insert` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 158 | `delete` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 159 | `filter` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 160 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 161 | `union` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 162 | `difference` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 163 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 164 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 165 | `first` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 166 | `last` | Y | Y |  |  | Y |  |  | hole | 97&#8209;98 |
| 167 | `previous` | Y | Y |  |  | Y |  |  | hole | 100&#8209;101 |
| 168 | `next` | Y | Y |  |  | Y |  |  | hole | 103&#8209;104 |
| 169 | `split` | Y | Y |  |  | Y |  |  | hole | 106&#8209;108 |
| 170 | `join` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 171 | `get_range` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 172 | `rank` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 173 | `select` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 174 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 122&#8209;124 |
| 175 | `from_sorted_elements` |  |  |  | Y | Y |  |  | hole | 387&#8209;388 |
| 176 | `default` |  | Y |  |  |  | Y | Y |  | 412 |
| 177 | `eq` |  | Y |  |  |  | Y | Y |  | 416&#8209;426 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 178 | `size` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;59 |
| 179 | `empty` | Y | Y |  |  | Y |  |  | unknown | 61&#8209;62 |
| 180 | `singleton` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 181 | `find` | Y | Y |  |  | Y |  | Y |  | 67 |
| 182 | `lookup` | Y | Y |  |  | Y |  | Y |  | 69 |
| 183 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 184 | `insert` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 185 | `delete` | Y | Y |  |  | Y |  |  | hole | 77&#8209;78 |
| 186 | `domain` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;81 |
| 187 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 188 | `map` | Y | Y |  |  | Y |  |  | hole | 86&#8209;87 |
| 189 | `filter` | Y | Y |  |  | Y |  |  | hole | 89&#8209;90 |
| 190 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 191 | `union` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;96 |
| 192 | `difference` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 193 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 194 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 195 | `reduce` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 196 | `collect` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 197 | `first_key` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 198 | `last_key` | Y | Y |  |  | Y |  |  | hole | 116&#8209;117 |
| 199 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 200 | `next_key` | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 201 | `split_key` | Y | Y |  |  | Y |  |  | hole | 125&#8209;127 |
| 202 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 203 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 132&#8209;133 |
| 204 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 135&#8209;136 |
| 205 | `select_key` | Y | Y |  |  | Y |  |  | hole | 138&#8209;139 |
| 206 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 141&#8209;143 |
| 207 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 511&#8209;512 |
| 208 | `eq` |  | Y |  |  |  | Y | Y |  | 529&#8209;531 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 209 | `size` | Y | Y |  |  | Y |  |  | hole | 59&#8209;60 |
| 210 | `empty` | Y | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 211 | `singleton` | Y | Y |  |  | Y |  |  | hole | 65&#8209;66 |
| 212 | `find` | Y | Y |  |  | Y |  |  | hole | 68 |
| 213 | `insert` | Y | Y |  |  | Y |  |  | hole | 70&#8209;71 |
| 214 | `delete` | Y | Y |  |  | Y |  |  | hole | 73&#8209;74 |
| 215 | `domain` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 216 | `map` | Y | Y |  |  | Y |  |  | hole | 79&#8209;80 |
| 217 | `filter` | Y | Y |  |  | Y |  |  | hole | 82&#8209;83 |
| 218 | `first_key` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 219 | `last_key` | Y | Y |  |  | Y |  |  | hole | 88&#8209;89 |
| 220 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 91&#8209;92 |
| 221 | `next_key` | Y | Y |  |  | Y |  |  | hole | 94&#8209;95 |
| 222 | `split_key` | Y | Y |  |  | Y |  |  | hole | 97&#8209;99 |
| 223 | `join_key` | Y | Y |  |  | Y |  |  | hole | 101&#8209;102 |
| 224 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 225 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 107&#8209;108 |
| 226 | `select_key` | Y | Y |  |  | Y |  |  | hole | 110&#8209;111 |
| 227 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 113&#8209;115 |
| 228 | `default` |  | Y |  |  |  | Y | Y |  | 375 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 229 | `size` | Y | Y |  |  | Y |  |  | hole | 58&#8209;59 |
| 230 | `empty` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;61 |
| 231 | `singleton` | Y | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 232 | `find` | Y | Y |  |  | Y |  |  | hole | 64&#8209;65 |
| 233 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 234 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 235 | `insert` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 236 | `delete` | Y | Y |  |  | Y |  |  | hole | 76&#8209;77 |
| 237 | `domain` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 238 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 239 | `map` | Y | Y |  |  | Y |  |  | hole | 83&#8209;84 |
| 240 | `filter` | Y | Y |  |  | Y |  |  | hole | 85&#8209;86 |
| 241 | `reduce` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 242 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;94 |
| 243 | `union` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;101 |
| 244 | `difference` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;104 |
| 245 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 246 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 247 | `collect` | Y | Y |  |  | Y |  |  | hole | 111&#8209;112 |
| 248 | `first_key` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 249 | `last_key` | Y | Y |  |  | Y |  |  | hole | 115&#8209;116 |
| 250 | `previous_key` | Y | Y |  |  | Y |  |  | hole | 117&#8209;118 |
| 251 | `next_key` | Y | Y |  |  | Y |  |  | hole | 119&#8209;120 |
| 252 | `split_key` | Y | Y |  |  | Y |  |  | hole | 121&#8209;123 |
| 253 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 254 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 127&#8209;128 |
| 255 | `rank_key` | Y | Y |  |  | Y |  |  | hole | 129&#8209;130 |
| 256 | `select_key` | Y | Y |  |  | Y |  |  | hole | 131&#8209;132 |
| 257 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 133&#8209;135 |
| 258 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 497&#8209;499 |
| 259 | `eq` |  | Y |  |  |  | Y | Y |  | 513&#8209;515 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 260 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 261 | `empty` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 262 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;71 |
| 263 | `find` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;78 |
| 264 | `insert` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 265 | `delete` | Y | Y |  |  | Y |  |  | hole | 82&#8209;88 |
| 266 | `domain` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;90 |
| 267 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 268 | `map` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 269 | `filter` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 270 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 271 | `union` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;112 |
| 272 | `difference` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;115 |
| 273 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 274 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 275 | `collect` | Y | Y |  |  | Y |  |  | hole | 122&#8209;123 |
| 276 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 277 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 278 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 279 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 280 | `split_key` | Y | Y |  |  | Y |  |  | hole | 132&#8209;134 |
| 281 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;140 |
| 282 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 283 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 284 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 285 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 286 | `from_sorted_entries` |  |  |  | Y | Y |  |  | hole | 515&#8209;518 |
| 287 | `iter` |  |  | Y |  | Y |  |  | unknown | 543&#8209;547 |
| 288 | `next` |  | Y |  |  | Y |  |  | unknown | 571&#8209;587 |
| 289 | `eq` |  | Y |  |  | Y |  |  | unknown | 675&#8209;676 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
