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
| 1 | Chap43 | AugOrderedTableMtEph | 32 | 33 | 1 | 3 | 36 | 1 | 34 | 2 | 1 |
| 2 | Chap43 | AugOrderedTableStEph | 31 | 32 | 1 | 2 | 34 | 1 | 32 | 2 | 1 |
| 3 | Chap43 | AugOrderedTableStPer | 28 | 28 | 2 | 2 | 32 | 0 | 30 | 2 | 0 |
| 4 | Chap43 | Example43_1 | 2 | 0 | 0 | 3 | 1 | 3 | 0 | 0 | 4 |
| 5 | Chap43 | OrderedSetMtEph | 22 | 22 | 0 | 1 | 23 | 0 | 16 | 7 | 0 |
| 6 | Chap43 | OrderedSetStEph | 22 | 24 | 1 | 1 | 24 | 2 | 16 | 8 | 2 |
| 7 | Chap43 | OrderedSetStPer | 22 | 24 | 1 | 1 | 25 | 1 | 23 | 2 | 1 |
| 8 | Chap43 | OrderedTableMtEph | 29 | 31 | 1 | 1 | 32 | 1 | 32 | 0 | 1 |
| 9 | Chap43 | OrderedTableMtPer | 19 | 20 | 0 | 1 | 20 | 1 | 17 | 2 | 2 |
| 10 | Chap43 | OrderedTableStEph | 29 | 31 | 1 | 1 | 32 | 1 | 28 | 4 | 1 |
| 11 | Chap43 | OrderedTableStPer | 26 | 28 | 1 | 1 | 30 | 0 | 26 | 4 | 0 |

## Function-by-Function Detail

### Chap43/AugOrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `recalculate_reduction` |  |  |  | Y | Y |  |  | unknown | 69&#8209;72 |
| 2 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 83&#8209;88 |
| 3 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 113&#8209;116 |
| 4 | `size` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 7 | `find` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;139 |
| 8 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;146 |
| 9 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 11 | `delete` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;158 |
| 12 | `domain` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;160 |
| 13 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;168 |
| 14 | `map` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 15 | `filter` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;174 |
| 16 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 17 | `union` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 18 | `difference` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;182 |
| 19 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 183&#8209;184 |
| 20 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;186 |
| 21 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 22 | `collect` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;191 |
| 23 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;193 |
| 24 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;195 |
| 25 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 26 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;199 |
| 27 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;202 |
| 28 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;204 |
| 29 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;206 |
| 30 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;208 |
| 31 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;210 |
| 32 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 33 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 34 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;217 |
| 35 | `reduce_range_parallel` | Y | Y |  |  | Y |  |  | hole | 218&#8209;219 |
| 36 | `iter` |  |  | Y |  | Y |  |  | unknown | 589&#8209;593 |
| 37 | `eq` |  | Y |  |  |  | Y | Y |  | 634&#8209;637 |

### Chap43/AugOrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `calculate_reduction` |  |  |  | Y | Y |  |  | hole | 71&#8209;78 |
| 39 | `lemma_aug_view` |  |  |  | Y | Y |  |  | unknown | 103&#8209;106 |
| 40 | `size` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;121 |
| 41 | `empty` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 42 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 43 | `find` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;133 |
| 44 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;140 |
| 45 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 46 | `insert` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;149 |
| 47 | `delete` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;156 |
| 48 | `domain` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 49 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 50 | `map` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 51 | `filter` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 52 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 53 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;177 |
| 54 | `union` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;184 |
| 55 | `difference` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 56 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;190 |
| 57 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;193 |
| 58 | `collect` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;195 |
| 59 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;197 |
| 60 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;199 |
| 61 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 62 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;203 |
| 63 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 64 | `join_key` | Y | Y |  |  | Y |  |  | hole | 207&#8209;209 |
| 65 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;211 |
| 66 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 212&#8209;213 |
| 67 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;215 |
| 68 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 69 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 219&#8209;220 |
| 70 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;222 |
| 71 | `iter` |  |  | Y |  | Y |  |  | unknown | 579&#8209;583 |
| 72 | `eq` |  | Y |  |  |  | Y | Y |  | 629&#8209;632 |

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
| 81 | `domain` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 82 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 83 | `map` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;162 |
| 85 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 86 | `union` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |
| 87 | `difference` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;180 |
| 88 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 89 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 90 | `collect` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;188 |
| 91 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;193 |
| 92 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;198 |
| 93 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;202 |
| 94 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;206 |
| 95 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 207&#8209;214 |
| 96 | `join_key` | Y | Y |  |  | Y |  |  | hole | 215&#8209;221 |
| 97 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;225 |
| 98 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 226&#8209;229 |
| 99 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;234 |
| 100 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;242 |
| 101 | `reduce_val` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;244 |
| 102 | `reduce_range` | Y | Y |  |  | Y |  |  | unknown | 245&#8209;246 |
| 103 | `iter` |  |  | Y |  | Y |  |  | unknown | 672&#8209;676 |
| 104 | `eq` |  |  | Y |  | Y |  |  | unknown | 706&#8209;707 |

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
| 109 | `from_st` |  |  |  | Y | Y |  |  | unknown | 67&#8209;69 |
| 110 | `size` | Y | Y |  |  | Y |  |  | hole | 95&#8209;96 |
| 111 | `empty` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 112 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 113 | `find` | Y | Y |  |  | Y |  |  | hole | 104&#8209;105 |
| 114 | `insert` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;108 |
| 115 | `delete` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;111 |
| 116 | `filter` | Y | Y |  |  | Y |  |  | hole | 113&#8209;114 |
| 117 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 118 | `union` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 119 | `difference` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 120 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 125&#8209;129 |
| 121 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 122 | `first` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 123 | `last` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 124 | `previous` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;143 |
| 125 | `next` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 126 | `split` | Y | Y |  |  | Y |  |  | hole | 148&#8209;150 |
| 127 | `join` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 128 | `get_range` | Y | Y |  |  | Y |  |  | hole | 155&#8209;156 |
| 129 | `rank` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;159 |
| 130 | `select` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;162 |
| 131 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 164&#8209;166 |

### Chap43/OrderedSetStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 132 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 133 | `empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;71 |
| 134 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;77 |
| 135 | `find` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 136 | `insert` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;88 |
| 137 | `delete` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;95 |
| 138 | `filter` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;103 |
| 139 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;110 |
| 140 | `union` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 141 | `difference` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;124 |
| 142 | `to_seq` | Y | Y |  |  | Y |  |  | hole | 126&#8209;131 |
| 143 | `from_seq` | Y | Y |  |  | Y |  |  | hole | 133&#8209;136 |
| 144 | `first` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;145 |
| 145 | `last` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;152 |
| 146 | `previous` | Y | Y |  |  | Y |  |  | hole | 154&#8209;157 |
| 147 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 159&#8209;162 |
| 148 | `split` | Y | Y |  |  | Y |  |  | hole | 164&#8209;174 |
| 149 | `join` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 150 | `get_range` | Y | Y |  |  | Y |  |  | hole | 180&#8209;184 |
| 151 | `rank` | Y | Y |  |  | Y |  |  | hole | 186&#8209;189 |
| 152 | `select` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;196 |
| 153 | `split_rank` | Y | Y |  |  | Y |  |  | hole | 198&#8209;206 |
| 154 | `iter` |  |  | Y |  | Y |  |  | unknown | 548&#8209;553 |
| 155 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 692&#8209;693 |
| 156 | `default` |  | Y |  |  |  | Y | Y |  | 717 |
| 157 | `eq` |  | Y |  |  |  | Y | Y |  | 721&#8209;731 |

### Chap43/OrderedSetStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 158 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 159 | `empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 160 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 161 | `find` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;76 |
| 162 | `insert` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;80 |
| 163 | `delete` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 164 | `filter` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 165 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 166 | `union` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 167 | `difference` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 168 | `to_seq` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;108 |
| 169 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 170 | `first` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;121 |
| 171 | `last` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 172 | `previous` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 173 | `next` x2 | Y | Y |  |  | Y |  |  | hole | 136&#8209;140 |
| 174 | `split` | Y | Y |  |  | Y |  |  | hole | 142&#8209;152 |
| 175 | `join` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 176 | `get_range` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;163 |
| 177 | `rank` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;169 |
| 178 | `select` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;176 |
| 179 | `split_rank` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;186 |
| 180 | `from_sorted_elements` |  |  |  | Y | Y |  |  | unknown | 610&#8209;612 |
| 181 | `iter` |  |  | Y |  | Y |  |  | unknown | 622&#8209;627 |
| 182 | `default` |  | Y |  |  | Y |  |  | unknown | 767&#8209;768 |
| 183 | `eq` |  | Y |  |  |  | Y | Y |  | 800&#8209;802 |

### Chap43/OrderedTableMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 184 | `size` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;66 |
| 185 | `empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 186 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 187 | `find` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;80 |
| 188 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;88 |
| 189 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 190 | `insert` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 191 | `delete` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 192 | `domain` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 193 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;110 |
| 194 | `map` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 195 | `filter` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 196 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 197 | `union` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 198 | `difference` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 199 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 200 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;135 |
| 201 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 202 | `collect` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 203 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;145 |
| 204 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;148 |
| 205 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 206 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;154 |
| 207 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 208 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;161 |
| 209 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;164 |
| 210 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;167 |
| 211 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;170 |
| 212 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;174 |
| 213 | `iter` |  |  | Y |  | Y |  |  | unknown | 638&#8209;642 |
| 214 | `next` |  | Y |  |  | Y |  |  | unknown | 666&#8209;682 |
| 215 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 773&#8209;775 |
| 216 | `eq` |  | Y |  |  |  | Y | Y |  | 803&#8209;805 |

### Chap43/OrderedTableMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 217 | `from_st_table` |  |  |  | Y | Y |  |  | hole | 72&#8209;76 |
| 218 | `size` | Y | Y |  |  | Y |  |  | hole | 102&#8209;103 |
| 219 | `empty` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 220 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 221 | `find` | Y | Y |  |  | Y |  | Y |  | 111 |
| 222 | `insert` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;114 |
| 223 | `delete` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 224 | `domain` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 225 | `map` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 226 | `filter` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 227 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 228 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 229 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 230 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;140 |
| 231 | `split_key` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 232 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 233 | `get_key_range` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;150 |
| 234 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 235 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;156 |
| 236 | `split_rank_key` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;160 |
| 237 | `default` |  | Y |  |  |  | Y | Y |  | 463 |

### Chap43/OrderedTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 238 | `size` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;64 |
| 239 | `empty` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 240 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;69 |
| 241 | `find` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;76 |
| 242 | `lookup` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;83 |
| 243 | `is_empty` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 244 | `insert` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;92 |
| 245 | `delete` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 246 | `domain` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;102 |
| 247 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;105 |
| 248 | `map` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;108 |
| 249 | `filter` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;111 |
| 250 | `reduce` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 251 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;120 |
| 252 | `union` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;127 |
| 253 | `difference` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 254 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 255 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;136 |
| 256 | `collect` | Y | Y |  |  | Y |  |  | hole | 137&#8209;138 |
| 257 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 258 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;150 |
| 259 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;155 |
| 260 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;160 |
| 261 | `split_key` | Y | Y |  |  | Y |  |  | hole | 162&#8209;170 |
| 262 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;174 |
| 263 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 176&#8209;179 |
| 264 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;184 |
| 265 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;190 |
| 266 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 192&#8209;200 |
| 267 | `iter` |  |  | Y |  | Y |  |  | unknown | 665&#8209;669 |
| 268 | `next` |  | Y |  |  | Y |  |  | unknown | 693&#8209;709 |
| 269 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 796&#8209;800 |
| 270 | `eq` |  | Y |  |  |  | Y | Y |  | 827&#8209;829 |

### Chap43/OrderedTableStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 271 | `size` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;67 |
| 272 | `empty` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 273 | `singleton` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 274 | `find` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 275 | `insert` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 276 | `delete` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;89 |
| 277 | `domain` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 278 | `tabulate` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 279 | `map` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;98 |
| 280 | `filter` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |
| 281 | `intersection` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 282 | `union` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;116 |
| 283 | `difference` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 284 | `restrict` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 285 | `subtract` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 286 | `collect` | Y | Y |  |  | Y |  |  | hole | 126&#8209;127 |
| 287 | `first_key` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;133 |
| 288 | `last_key` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;139 |
| 289 | `previous_key` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;144 |
| 290 | `next_key` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;149 |
| 291 | `split_key` | Y | Y |  |  | Y |  |  | hole | 151&#8209;158 |
| 292 | `join_key` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;166 |
| 293 | `get_key_range` | Y | Y |  |  | Y |  |  | hole | 168&#8209;171 |
| 294 | `rank_key` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;176 |
| 295 | `select_key` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;182 |
| 296 | `split_rank_key` | Y | Y |  |  | Y |  |  | hole | 184&#8209;191 |
| 297 | `from_sorted_entries` |  |  |  | Y | Y |  |  | unknown | 592&#8209;596 |
| 298 | `iter` |  |  | Y |  | Y |  |  | unknown | 620&#8209;624 |
| 299 | `next` |  | Y |  |  | Y |  |  | unknown | 648&#8209;664 |
| 300 | `eq` |  | Y |  |  | Y |  |  | unknown | 752&#8209;753 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
