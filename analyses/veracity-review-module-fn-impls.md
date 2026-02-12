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
| 1 | Chap18 | ArraySeq | 23 | 28 | 3 | 13 | 43 | 1 | 39 | 1 | 4 |
| 2 | Chap18 | ArraySeqMtEph | 20 | 25 | 5 | 0 | 29 | 1 | 27 | 1 | 2 |
| 3 | Chap18 | ArraySeqMtPer | 19 | 24 | 5 | 0 | 28 | 1 | 26 | 1 | 2 |
| 4 | Chap18 | ArraySeqStEph | 20 | 25 | 2 | 0 | 26 | 1 | 24 | 1 | 2 |
| 5 | Chap18 | ArraySeqStPer | 19 | 24 | 2 | 0 | 25 | 1 | 22 | 1 | 3 |
| 6 | Chap18 | LinkedListStEph | 19 | 24 | 2 | 0 | 25 | 1 | 22 | 1 | 3 |
| 7 | Chap18 | LinkedListStPer | 18 | 23 | 2 | 0 | 24 | 1 | 21 | 1 | 3 |

## Function-by-Function Detail

### Chap18/ArraySeq.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | strong | 191&#8209;198 |
| 2 | `set` | Y | Y |  |  | Y |  |  | strong | 202&#8209;207 |
| 3 | `length` | Y | Y |  |  | Y |  |  | strong | 211&#8209;212 |
| 4 | `nth` | Y | Y |  |  | Y |  |  | strong | 216&#8209;218 |
| 5 | `empty` | Y | Y |  |  | Y |  |  | strong | 222&#8209;223 |
| 6 | `singleton` | Y | Y |  |  | Y |  |  | strong | 227&#8209;230 |
| 7 | `subseq` | Y | Y |  |  | Y |  |  | strong | 234&#8209;242 |
| 8 | `append` | Y | Y |  |  | Y |  |  | strong | 246&#8209;254 |
| 9 | `filter` | Y | Y |  |  | Y |  |  | strong | 260&#8209;271 |
| 10 | `update` | Y | Y |  |  | Y |  |  | strong | 275&#8209;283 |
| 11 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 287&#8209;288 |
| 12 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 292&#8209;293 |
| 13 | `iterate` | Y | Y |  |  | Y |  |  | strong | 298&#8209;303 |
| 14 | `reduce` | Y | Y |  |  | Y |  |  | strong | 308&#8209;316 |
| 15 | `scan` | Y | Y |  |  | Y |  |  | strong | 321&#8209;335 |
| 16 | `inject` | Y | Y |  |  | Y |  |  | strong | 340&#8209;349 |
| 17 | `scan_inclusive` | Y | Y |  |  | Y |  |  | strong | 354&#8209;364 |
| 18 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 368&#8209;376 |
| 19 | `remove` | Y | Y |  |  | Y |  |  | strong | 380&#8209;387 |
| 20 | `insert` | Y | Y |  |  | Y |  |  | strong | 391&#8209;398 |
| 21 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 402&#8209;405 |
| 22 | `find_key` | Y | Y |  |  | Y |  |  | strong | 408&#8209;420 |
| 23 | `collect` | Y | Y |  |  | Y |  |  | strong | 426&#8209;438 |
| 24 | `map` |  |  |  | Y | Y |  |  | strong | 1082&#8209;1086 |
| 25 | `tabulate` |  |  |  | Y | Y |  |  | strong | 1111&#8209;1117 |
| 26 | `flatten` |  |  |  | Y | Y |  |  | strong | 1138&#8209;1142 |
| 27 | `iterate_prefixes` |  |  |  | Y | Y |  |  | strong | 1196&#8209;1211 |
| 28 | `lemma_deep_view_len` |  |  |  | Y | Y |  |  | strong | 1277&#8209;1279 |
| 29 | `lemma_deep_view_key` |  |  |  | Y | Y |  |  | strong | 1284&#8209;1289 |
| 30 | `lemma_find_key_index_bounds` |  |  |  | Y | Y |  |  | strong | 1294&#8209;1299 |
| 31 | `lemma_find_key_index_found` |  |  |  | Y | Y |  |  | strong | 1309&#8209;1320 |
| 32 | `lemma_find_key_index_not_found` |  |  |  | Y | Y |  |  | strong | 1329&#8209;1337 |
| 33 | `lemma_spec_collect_step_some` |  |  |  | Y | Y |  |  | strong | 1346&#8209;1358 |
| 34 | `lemma_spec_collect_step_none` |  |  |  | Y | Y |  |  | strong | 1369&#8209;1380 |
| 35 | `lemma_find_key_some` |  |  |  | Y | Y |  |  | strong | 1390&#8209;1397 |
| 36 | `lemma_find_key_none` |  |  |  | Y | Y |  |  | strong | 1408&#8209;1413 |
| 37 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 1425&#8209;1427 |
| 38 | `iter` |  |  | Y |  | Y |  |  | strong | 1431&#8209;1435 |
| 39 | `iter_mut` |  |  | Y |  | Y |  | Y | none | 1443 |
| 40 | `next` |  | Y |  |  | Y |  |  | strong | 1494&#8209;1510 |
| 41 | `into_iter` x3 |  | Y |  |  | Y |  | Y | none | 1566 |
| 42 | `clone` |  | Y |  |  | Y |  | Y | none | 1586 |
| 43 | `eq` |  | Y |  |  | Y |  |  | strong | 1594&#8209;1595 |
| 44 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 1608&#8209;1610 |

### Chap18/ArraySeqMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `new` | Y | Y |  |  | Y |  |  | strong | 86&#8209;93 |
| 46 | `set` | Y | Y |  |  | Y |  |  | strong | 97&#8209;102 |
| 47 | `length` | Y | Y |  |  | Y |  |  | strong | 106&#8209;107 |
| 48 | `nth` | Y | Y |  |  | Y |  |  | strong | 111&#8209;113 |
| 49 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 117&#8209;125 |
| 50 | `subseq` | Y | Y |  |  | Y |  |  | strong | 129&#8209;137 |
| 51 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 141&#8209;144 |
| 52 | `empty` | Y | Y |  |  | Y |  |  | strong | 152&#8209;153 |
| 53 | `singleton` | Y | Y |  |  | Y |  |  | strong | 157&#8209;160 |
| 54 | `append` | Y | Y |  |  | Y |  |  | strong | 164&#8209;172 |
| 55 | `filter` | Y | Y |  |  | Y |  |  | strong | 176&#8209;183 |
| 56 | `update` | Y | Y |  |  | Y |  |  | strong | 187&#8209;195 |
| 57 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 199&#8209;200 |
| 58 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 204&#8209;205 |
| 59 | `iterate` | Y | Y |  |  | Y |  |  | partial | 209&#8209;210 |
| 60 | `reduce` | Y | Y |  |  | Y |  |  | partial | 214&#8209;216 |
| 61 | `scan` | Y | Y |  |  | Y |  |  | partial | 220&#8209;223 |
| 62 | `map` | Y | Y |  |  | Y |  |  | strong | 227&#8209;232 |
| 63 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 236&#8209;242 |
| 64 | `flatten` | Y | Y |  |  | Y |  |  | strong | 246&#8209;251 |
| 65 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 636&#8209;638 |
| 66 | `iter` |  |  | Y |  | Y |  |  | strong | 642&#8209;646 |
| 67 | `map_par` |  |  | Y |  | Y |  |  | partial | 651&#8209;662 |
| 68 | `filter_par` |  |  | Y |  | Y |  |  | partial | 701&#8209;711 |
| 69 | `reduce_par` |  |  | Y |  | Y |  |  | none | 754&#8209;764 |
| 70 | `next` |  | Y |  |  | Y |  |  | strong | 814&#8209;830 |
| 71 | `into_iter` x2 |  | Y |  |  | Y |  |  | strong | 894&#8209;898 |
| 72 | `clone` |  | Y |  |  | Y |  | Y | none | 920 |
| 73 | `eq` |  | Y |  |  | Y |  |  | strong | 926&#8209;927 |
| 74 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 942&#8209;944 |

### Chap18/ArraySeqMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 75 | `new` | Y | Y |  |  | Y |  |  | strong | 87&#8209;94 |
| 76 | `length` | Y | Y |  |  | Y |  |  | strong | 98&#8209;99 |
| 77 | `nth` | Y | Y |  |  | Y |  |  | strong | 103&#8209;105 |
| 78 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 109&#8209;117 |
| 79 | `subseq` | Y | Y |  |  | Y |  |  | strong | 121&#8209;129 |
| 80 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 133&#8209;136 |
| 81 | `empty` | Y | Y |  |  | Y |  |  | strong | 144&#8209;145 |
| 82 | `singleton` | Y | Y |  |  | Y |  |  | strong | 149&#8209;152 |
| 83 | `append` | Y | Y |  |  | Y |  |  | strong | 156&#8209;164 |
| 84 | `filter` | Y | Y |  |  | Y |  |  | strong | 168&#8209;175 |
| 85 | `update` | Y | Y |  |  | Y |  |  | strong | 179&#8209;187 |
| 86 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 191&#8209;192 |
| 87 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 196&#8209;197 |
| 88 | `iterate` | Y | Y |  |  | Y |  |  | partial | 201&#8209;202 |
| 89 | `reduce` | Y | Y |  |  | Y |  |  | partial | 206&#8209;208 |
| 90 | `scan` | Y | Y |  |  | Y |  |  | partial | 212&#8209;215 |
| 91 | `map` | Y | Y |  |  | Y |  |  | strong | 219&#8209;224 |
| 92 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 228&#8209;234 |
| 93 | `flatten` | Y | Y |  |  | Y |  |  | strong | 238&#8209;243 |
| 94 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 619&#8209;621 |
| 95 | `iter` |  |  | Y |  | Y |  |  | strong | 625&#8209;629 |
| 96 | `map_par` |  |  | Y |  | Y |  |  | partial | 634&#8209;644 |
| 97 | `filter_par` |  |  | Y |  | Y |  |  | partial | 689&#8209;698 |
| 98 | `reduce_par` |  |  | Y |  | Y |  |  | none | 745&#8209;755 |
| 99 | `next` |  | Y |  |  | Y |  |  | strong | 819&#8209;835 |
| 100 | `into_iter` x2 |  | Y |  |  | Y |  |  | strong | 899&#8209;903 |
| 101 | `clone` |  | Y |  |  | Y |  | Y | none | 925 |
| 102 | `eq` |  | Y |  |  | Y |  |  | strong | 931&#8209;932 |
| 103 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 946&#8209;948 |

### Chap18/ArraySeqStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 104 | `new` | Y | Y |  |  | Y |  |  | strong | 83&#8209;90 |
| 105 | `set` | Y | Y |  |  | Y |  |  | strong | 94&#8209;99 |
| 106 | `length` | Y | Y |  |  | Y |  |  | strong | 103&#8209;104 |
| 107 | `nth` | Y | Y |  |  | Y |  |  | strong | 108&#8209;110 |
| 108 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 114&#8209;122 |
| 109 | `subseq` | Y | Y |  |  | Y |  |  | strong | 126&#8209;134 |
| 110 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 138&#8209;141 |
| 111 | `empty` | Y | Y |  |  | Y |  |  | strong | 149&#8209;150 |
| 112 | `singleton` | Y | Y |  |  | Y |  |  | strong | 154&#8209;157 |
| 113 | `append` | Y | Y |  |  | Y |  |  | strong | 161&#8209;169 |
| 114 | `filter` | Y | Y |  |  | Y |  |  | strong | 173&#8209;180 |
| 115 | `update` | Y | Y |  |  | Y |  |  | strong | 184&#8209;192 |
| 116 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 196&#8209;197 |
| 117 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 201&#8209;202 |
| 118 | `iterate` | Y | Y |  |  | Y |  |  | partial | 206&#8209;207 |
| 119 | `reduce` | Y | Y |  |  | Y |  |  | partial | 211&#8209;213 |
| 120 | `scan` | Y | Y |  |  | Y |  |  | partial | 217&#8209;220 |
| 121 | `map` | Y | Y |  |  | Y |  |  | strong | 224&#8209;229 |
| 122 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 233&#8209;239 |
| 123 | `flatten` | Y | Y |  |  | Y |  |  | strong | 243&#8209;248 |
| 124 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 633&#8209;635 |
| 125 | `iter` |  |  | Y |  | Y |  |  | strong | 639&#8209;643 |
| 126 | `next` |  | Y |  |  | Y |  |  | strong | 690&#8209;706 |
| 127 | `into_iter` x2 |  | Y |  |  | Y |  |  | strong | 757&#8209;761 |
| 128 | `clone` |  | Y |  |  | Y |  | Y | none | 783 |
| 129 | `eq` |  | Y |  |  | Y |  |  | strong | 789&#8209;790 |
| 130 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 804&#8209;806 |

### Chap18/ArraySeqStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 131 | `new` | Y | Y |  |  | Y |  |  | strong | 84&#8209;91 |
| 132 | `length` | Y | Y |  |  | Y |  |  | strong | 95&#8209;96 |
| 133 | `nth` | Y | Y |  |  | Y |  |  | strong | 100&#8209;102 |
| 134 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 106&#8209;114 |
| 135 | `subseq` | Y | Y |  |  | Y |  |  | strong | 118&#8209;126 |
| 136 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 130&#8209;133 |
| 137 | `empty` | Y | Y |  |  | Y |  |  | strong | 141&#8209;142 |
| 138 | `singleton` | Y | Y |  |  | Y |  |  | strong | 146&#8209;149 |
| 139 | `append` | Y | Y |  |  | Y |  |  | strong | 153&#8209;161 |
| 140 | `filter` | Y | Y |  |  | Y |  |  | strong | 165&#8209;172 |
| 141 | `update` | Y | Y |  |  | Y |  |  | strong | 176&#8209;184 |
| 142 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 188&#8209;189 |
| 143 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 193&#8209;194 |
| 144 | `iterate` | Y | Y |  |  | Y |  |  | partial | 198&#8209;199 |
| 145 | `reduce` | Y | Y |  |  | Y |  |  | partial | 203&#8209;205 |
| 146 | `scan` | Y | Y |  |  | Y |  |  | partial | 209&#8209;212 |
| 147 | `map` | Y | Y |  |  | Y |  |  | strong | 216&#8209;221 |
| 148 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 225&#8209;231 |
| 149 | `flatten` | Y | Y |  |  | Y |  |  | strong | 235&#8209;240 |
| 150 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 616&#8209;618 |
| 151 | `iter` |  |  | Y |  | Y |  |  | strong | 622&#8209;626 |
| 152 | `next` |  | Y |  |  | Y |  |  | strong | 660&#8209;676 |
| 153 | `into_iter` x2 |  | Y |  |  | Y |  | Y | none | 743 |
| 154 | `clone` |  | Y |  |  | Y |  | Y | none | 756 |
| 155 | `eq` |  | Y |  |  | Y |  |  | strong | 764&#8209;765 |
| 156 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 779&#8209;781 |

### Chap18/LinkedListStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 157 | `new` | Y | Y |  |  | Y |  |  | strong | 84&#8209;91 |
| 158 | `set` | Y | Y |  |  | Y |  |  | strong | 94&#8209;99 |
| 159 | `length` | Y | Y |  |  | Y |  |  | strong | 102&#8209;103 |
| 160 | `nth` | Y | Y |  |  | Y |  |  | strong | 106&#8209;108 |
| 161 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 111&#8209;119 |
| 162 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 122&#8209;125 |
| 163 | `empty` | Y | Y |  |  | Y |  |  | strong | 132&#8209;133 |
| 164 | `singleton` | Y | Y |  |  | Y |  |  | strong | 136&#8209;139 |
| 165 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 142&#8209;148 |
| 166 | `map` | Y | Y |  |  | Y |  |  | strong | 151&#8209;156 |
| 167 | `append` | Y | Y |  |  | Y |  |  | strong | 159&#8209;167 |
| 168 | `filter` | Y | Y |  |  | Y |  |  | strong | 170&#8209;177 |
| 169 | `flatten` | Y | Y |  |  | Y |  |  | strong | 180&#8209;185 |
| 170 | `update` | Y | Y |  |  | Y |  |  | strong | 188&#8209;196 |
| 171 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 199&#8209;200 |
| 172 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 203&#8209;204 |
| 173 | `iterate` | Y | Y |  |  | Y |  |  | strong | 207&#8209;212 |
| 174 | `reduce` | Y | Y |  |  | Y |  |  | strong | 215&#8209;223 |
| 175 | `scan` | Y | Y |  |  | Y |  |  | strong | 226&#8209;238 |
| 176 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 665&#8209;667 |
| 177 | `iter` |  |  | Y |  | Y |  |  | strong | 671&#8209;675 |
| 178 | `next` |  | Y |  |  | Y |  |  | strong | 709&#8209;725 |
| 179 | `into_iter` x2 |  | Y |  |  | Y |  | Y | none | 784 |
| 180 | `clone` |  | Y |  |  | Y |  | Y | none | 797 |
| 181 | `eq` |  | Y |  |  | Y |  |  | strong | 803&#8209;804 |
| 182 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 818&#8209;820 |

### Chap18/LinkedListStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 183 | `new` | Y | Y |  |  | Y |  |  | strong | 85&#8209;92 |
| 184 | `length` | Y | Y |  |  | Y |  |  | strong | 95&#8209;96 |
| 185 | `nth` | Y | Y |  |  | Y |  |  | strong | 99&#8209;101 |
| 186 | `subseq_copy` | Y | Y |  |  | Y |  |  | strong | 104&#8209;112 |
| 187 | `from_vec` | Y | Y |  |  | Y |  |  | strong | 115&#8209;118 |
| 188 | `empty` | Y | Y |  |  | Y |  |  | strong | 125&#8209;126 |
| 189 | `singleton` | Y | Y |  |  | Y |  |  | strong | 129&#8209;132 |
| 190 | `tabulate` | Y | Y |  |  | Y |  |  | strong | 135&#8209;141 |
| 191 | `map` | Y | Y |  |  | Y |  |  | strong | 144&#8209;149 |
| 192 | `append` | Y | Y |  |  | Y |  |  | strong | 152&#8209;160 |
| 193 | `filter` | Y | Y |  |  | Y |  |  | strong | 163&#8209;170 |
| 194 | `flatten` | Y | Y |  |  | Y |  |  | strong | 173&#8209;178 |
| 195 | `update` | Y | Y |  |  | Y |  |  | strong | 181&#8209;189 |
| 196 | `is_empty` | Y | Y |  |  | Y |  |  | strong | 192&#8209;193 |
| 197 | `is_singleton` | Y | Y |  |  | Y |  |  | strong | 196&#8209;197 |
| 198 | `iterate` | Y | Y |  |  | Y |  |  | strong | 200&#8209;205 |
| 199 | `reduce` | Y | Y |  |  | Y |  |  | strong | 208&#8209;216 |
| 200 | `scan` | Y | Y |  |  | Y |  |  | strong | 219&#8209;231 |
| 201 | `lemma_spec_index` |  |  | Y |  | Y |  |  | strong | 649&#8209;651 |
| 202 | `iter` |  |  | Y |  | Y |  |  | strong | 655&#8209;659 |
| 203 | `next` |  | Y |  |  | Y |  |  | strong | 693&#8209;709 |
| 204 | `into_iter` x2 |  | Y |  |  | Y |  | Y | none | 768 |
| 205 | `clone` |  | Y |  |  | Y |  | Y | none | 781 |
| 206 | `eq` |  | Y |  |  | Y |  |  | strong | 787&#8209;788 |
| 207 | `fmt` x2 |  | Y |  |  |  | Y | Y | none | 802&#8209;804 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.