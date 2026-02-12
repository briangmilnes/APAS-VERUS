# Chapter 18 Review

## Specification Summary by Module

| # | Module | Trait | ITT | BI | ML | V! | Strong | Weak | Assume | None |
|---|--------|:-----:|:---:|:--:|:--:|:--:|:------:|:----:|:------:|:----:|
| 1 | ArraySeq | 23 | 31 | 2 | 4 | 35 | 29 | 0 | 1 | 7 |
| 2 | ArraySeqStEph | 20 | 27 | 1 | 0 | 26 | 22 | 0 | 1 | 5 |
| 3 | ArraySeqStPer | 19 | 26 | 1 | 0 | 25 | 21 | 0 | 1 | 5 |
| 4 | ArraySeqMtEph | 20 | 27 | 4 | 0 | 29 | 25 | 0 | 1 | 5 |
| 5 | ArraySeqMtPer | 19 | 26 | 4 | 0 | 28 | 24 | 0 | 1 | 5 |
| 6 | LLStEph | 19 | 7 | 18 | 0 | 25 | 17 | 2 | 1 | 7 |
| 7 | LLStPer | 18 | 7 | 17 | 0 | 24 | 16 | 2 | 1 | 7 |

## Function-by-Function Detail

Ordered: trait-spec'd first (canonical order), then impl-struct, then module-level, then std trait impls.

Columns: **Trait** = declared in a `trait` block; **ITT** = in `impl Trait for Type`; **BI** = in bare `impl Type`; **ML** = module-level free fn; **V!** = inside `verus!`; **Spec** = spec strength.

| # | File | Function | Trait | ITT | BI | ML | V! | Spec |
|---|------|----------|:-----:|:---:|:--:|:--:|:--:|------|
| | **ArraySeq.rs** | | | | | | | |
| 1 | ArraySeq | `new` | Y | Y | | | Y | strong |
| 2 | ArraySeq | `set` | Y | Y | | | Y | strong |
| 3 | ArraySeq | `length` | Y | Y | | | Y | strong |
| 4 | ArraySeq | `nth` | Y | Y | | | Y | strong |
| 5 | ArraySeq | `empty` | Y | Y | | | Y | strong |
| 6 | ArraySeq | `singleton` | Y | Y | | | Y | strong |
| 7 | ArraySeq | `subseq` | Y | Y | | | Y | strong |
| 8 | ArraySeq | `subseq_copy` | Y | Y | | | Y | strong |
| 9 | ArraySeq | `append` | Y | Y | | | Y | strong |
| 10 | ArraySeq | `filter` | Y | Y | | | Y | strong |
| 11 | ArraySeq | `update` | Y | Y | | | Y | strong |
| 12 | ArraySeq | `is_empty` | Y | Y | | | Y | strong |
| 13 | ArraySeq | `is_singleton` | Y | Y | | | Y | strong |
| 14 | ArraySeq | `iterate` | Y | Y | | | Y | strong |
| 15 | ArraySeq | `reduce` | Y | Y | | | Y | strong |
| 16 | ArraySeq | `scan` | Y | Y | | | Y | strong |
| 17 | ArraySeq | `scan_inclusive` | Y | Y | | | Y | strong |
| 18 | ArraySeq | `inject` | Y | Y | | | Y | strong |
| 19 | ArraySeq | `remove` | Y | Y | | | Y | strong |
| 20 | ArraySeq | `insert` | Y | Y | | | Y | strong |
| 21 | ArraySeq | `from_vec` | Y | Y | | | Y | strong |
| 22 | ArraySeq | `find_key` | Y | Y | | | Y | strong |
| 23 | ArraySeq | `collect` | Y | Y | | | Y | strong |
| 24 | ArraySeq | `iter` | | | Y | | Y | strong |
| 25 | ArraySeq | `iter_mut` | | | Y | | Y | none (ext) |
| 26 | ArraySeq | `map` | | | | Y | Y | strong |
| 27 | ArraySeq | `tabulate` | | | | Y | Y | strong |
| 28 | ArraySeq | `flatten` | | | | Y | Y | strong |
| 29 | ArraySeq | `iterate_prefixes` | | | | Y | Y | strong |
| 30 | ArraySeq | `clone` | | Y | | | Y | none |
| 31 | ArraySeq | `eq` | | Y | | | Y | assume |
| 32 | ArraySeq | `next` | | Y | | | Y | strong |
| 33 | ArraySeq | `into_iter` x3 | | Y | | | Y | none |
| 34 | ArraySeq | `fmt` x2 | | Y | | | | none |
| | **ArraySeqStEph.rs** | | | | | | | |
| 35 | StEph | `new` | Y | Y | | | Y | strong |
| 36 | StEph | `set` | Y | Y | | | Y | strong |
| 37 | StEph | `length` | Y | Y | | | Y | strong |
| 38 | StEph | `nth` | Y | Y | | | Y | strong |
| 39 | StEph | `empty` | Y | Y | | | Y | strong |
| 40 | StEph | `singleton` | Y | Y | | | Y | strong |
| 41 | StEph | `subseq` | Y | Y | | | Y | strong |
| 42 | StEph | `subseq_copy` | Y | Y | | | Y | strong |
| 43 | StEph | `append` | Y | Y | | | Y | strong |
| 44 | StEph | `filter` | Y | Y | | | Y | strong |
| 45 | StEph | `update` | Y | Y | | | Y | strong |
| 46 | StEph | `is_empty` | Y | Y | | | Y | strong |
| 47 | StEph | `is_singleton` | Y | Y | | | Y | strong |
| 48 | StEph | `iterate` | Y | Y | | | Y | strong |
| 49 | StEph | `reduce` | Y | Y | | | Y | strong |
| 50 | StEph | `scan` | Y | Y | | | Y | strong |
| 51 | StEph | `map` | Y | Y | | | Y | strong |
| 52 | StEph | `tabulate` | Y | Y | | | Y | strong |
| 53 | StEph | `flatten` | Y | Y | | | Y | strong |
| 54 | StEph | `from_vec` | Y | Y | | | Y | strong |
| 55 | StEph | `iter` | | | Y | | Y | strong |
| 56 | StEph | `clone` | | Y | | | Y | none |
| 57 | StEph | `eq` | | Y | | | Y | assume |
| 58 | StEph | `next` | | Y | | | Y | strong |
| 59 | StEph | `into_iter` x2 | | Y | | | Y | none |
| 60 | StEph | `fmt` x2 | | Y | | | | none |
| | **ArraySeqStPer.rs** | | | | | | | |
| 61 | StPer | `new` | Y | Y | | | Y | strong |
| 62 | StPer | `length` | Y | Y | | | Y | strong |
| 63 | StPer | `nth` | Y | Y | | | Y | strong |
| 64 | StPer | `empty` | Y | Y | | | Y | strong |
| 65 | StPer | `singleton` | Y | Y | | | Y | strong |
| 66 | StPer | `subseq` | Y | Y | | | Y | strong |
| 67 | StPer | `subseq_copy` | Y | Y | | | Y | strong |
| 68 | StPer | `append` | Y | Y | | | Y | strong |
| 69 | StPer | `filter` | Y | Y | | | Y | strong |
| 70 | StPer | `update` | Y | Y | | | Y | strong |
| 71 | StPer | `is_empty` | Y | Y | | | Y | strong |
| 72 | StPer | `is_singleton` | Y | Y | | | Y | strong |
| 73 | StPer | `iterate` | Y | Y | | | Y | strong |
| 74 | StPer | `reduce` | Y | Y | | | Y | strong |
| 75 | StPer | `scan` | Y | Y | | | Y | strong |
| 76 | StPer | `map` | Y | Y | | | Y | strong |
| 77 | StPer | `tabulate` | Y | Y | | | Y | strong |
| 78 | StPer | `flatten` | Y | Y | | | Y | strong |
| 79 | StPer | `from_vec` | Y | Y | | | Y | strong |
| 80 | StPer | `iter` | | | Y | | Y | strong |
| 81 | StPer | `clone` | | Y | | | Y | none |
| 82 | StPer | `eq` | | Y | | | Y | assume |
| 83 | StPer | `next` | | Y | | | Y | strong |
| 84 | StPer | `into_iter` x2 | | Y | | | Y | none |
| 85 | StPer | `fmt` x2 | | Y | | | | none |
| | **ArraySeqMtEph.rs** | | | | | | | |
| 86 | MtEph | `new` | Y | Y | | | Y | strong |
| 87 | MtEph | `set` | Y | Y | | | Y | strong |
| 88 | MtEph | `length` | Y | Y | | | Y | strong |
| 89 | MtEph | `nth` | Y | Y | | | Y | strong |
| 90 | MtEph | `empty` | Y | Y | | | Y | strong |
| 91 | MtEph | `singleton` | Y | Y | | | Y | strong |
| 92 | MtEph | `subseq` | Y | Y | | | Y | strong |
| 93 | MtEph | `subseq_copy` | Y | Y | | | Y | strong |
| 94 | MtEph | `append` | Y | Y | | | Y | strong |
| 95 | MtEph | `filter` | Y | Y | | | Y | strong |
| 96 | MtEph | `update` | Y | Y | | | Y | strong |
| 97 | MtEph | `is_empty` | Y | Y | | | Y | strong |
| 98 | MtEph | `is_singleton` | Y | Y | | | Y | strong |
| 99 | MtEph | `iterate` | Y | Y | | | Y | strong |
| 100 | MtEph | `reduce` | Y | Y | | | Y | strong |
| 101 | MtEph | `scan` | Y | Y | | | Y | strong |
| 102 | MtEph | `map` | Y | Y | | | Y | strong |
| 103 | MtEph | `tabulate` | Y | Y | | | Y | strong |
| 104 | MtEph | `flatten` | Y | Y | | | Y | strong |
| 105 | MtEph | `from_vec` | Y | Y | | | Y | strong |
| 106 | MtEph | `iter` | | | Y | | Y | strong |
| 107 | MtEph | `map_par` | | | Y | | Y | strong |
| 108 | MtEph | `filter_par` | | | Y | | Y | strong |
| 109 | MtEph | `reduce_par` | | | Y | | Y | strong |
| 110 | MtEph | `clone` | | Y | | | Y | none |
| 111 | MtEph | `eq` | | Y | | | Y | assume |
| 112 | MtEph | `next` | | Y | | | Y | strong |
| 113 | MtEph | `into_iter` x2 | | Y | | | Y | none |
| 114 | MtEph | `fmt` x2 | | Y | | | | none |
| | **ArraySeqMtPer.rs** | | | | | | | |
| 115 | MtPer | `new` | Y | Y | | | Y | strong |
| 116 | MtPer | `length` | Y | Y | | | Y | strong |
| 117 | MtPer | `nth` | Y | Y | | | Y | strong |
| 118 | MtPer | `empty` | Y | Y | | | Y | strong |
| 119 | MtPer | `singleton` | Y | Y | | | Y | strong |
| 120 | MtPer | `subseq` | Y | Y | | | Y | strong |
| 121 | MtPer | `subseq_copy` | Y | Y | | | Y | strong |
| 122 | MtPer | `append` | Y | Y | | | Y | strong |
| 123 | MtPer | `filter` | Y | Y | | | Y | strong |
| 124 | MtPer | `update` | Y | Y | | | Y | strong |
| 125 | MtPer | `is_empty` | Y | Y | | | Y | strong |
| 126 | MtPer | `is_singleton` | Y | Y | | | Y | strong |
| 127 | MtPer | `iterate` | Y | Y | | | Y | strong |
| 128 | MtPer | `reduce` | Y | Y | | | Y | strong |
| 129 | MtPer | `scan` | Y | Y | | | Y | strong |
| 130 | MtPer | `map` | Y | Y | | | Y | strong |
| 131 | MtPer | `tabulate` | Y | Y | | | Y | strong |
| 132 | MtPer | `flatten` | Y | Y | | | Y | strong |
| 133 | MtPer | `from_vec` | Y | Y | | | Y | strong |
| 134 | MtPer | `iter` | | | Y | | Y | strong |
| 135 | MtPer | `map_par` | | | Y | | Y | strong |
| 136 | MtPer | `filter_par` | | | Y | | Y | strong |
| 137 | MtPer | `reduce_par` | | | Y | | Y | strong |
| 138 | MtPer | `clone` | | Y | | | Y | none |
| 139 | MtPer | `eq` | | Y | | | Y | assume |
| 140 | MtPer | `next` | | Y | | | Y | strong |
| 141 | MtPer | `into_iter` x2 | | Y | | | Y | none |
| 142 | MtPer | `fmt` x2 | | Y | | | | none |
| | **LinkedListStEph.rs** | | | | | | | |
| 143 | LLStEph | `new` | Y | | Y | | Y | strong |
| 144 | LLStEph | `set` | Y | | Y | | Y | strong |
| 145 | LLStEph | `length` | Y | | Y | | Y | strong |
| 146 | LLStEph | `nth` | Y | | Y | | Y | strong |
| 147 | LLStEph | `empty` | Y | | Y | | Y | strong |
| 148 | LLStEph | `singleton` | Y | | Y | | Y | strong |
| 149 | LLStEph | `subseq_copy` | Y | | Y | | Y | strong |
| 150 | LLStEph | `append` | Y | | Y | | Y | strong |
| 151 | LLStEph | `filter` | Y | | Y | | Y | strong |
| 152 | LLStEph | `update` | Y | | Y | | Y | strong |
| 153 | LLStEph | `is_empty` | Y | | Y | | Y | strong |
| 154 | LLStEph | `is_singleton` | Y | | Y | | Y | strong |
| 155 | LLStEph | `iterate` | Y | | Y | | Y | **weak** |
| 156 | LLStEph | `reduce` | Y | | Y | | Y | **weak** |
| 157 | LLStEph | `scan` | Y | | | | Y | — |
| 158 | LLStEph | `flatten` | Y | | | | Y | — |
| 159 | LLStEph | `map` | Y | | Y | | Y | strong |
| 160 | LLStEph | `tabulate` | Y | | Y | | Y | strong |
| 161 | LLStEph | `from_vec` | Y | | Y | | Y | strong |
| 162 | LLStEph | `iter` | | | Y | | Y | strong |
| 163 | LLStEph | `clone` | | Y | | | Y | none |
| 164 | LLStEph | `eq` | | Y | | | Y | assume |
| 165 | LLStEph | `next` | | Y | | | Y | strong |
| 166 | LLStEph | `into_iter` x2 | | Y | | | Y | none |
| 167 | LLStEph | `fmt` x2 | | Y | | | | none |
| | **LinkedListStPer.rs** | | | | | | | |
| 168 | LLStPer | `new` | Y | | Y | | Y | strong |
| 169 | LLStPer | `length` | Y | | Y | | Y | strong |
| 170 | LLStPer | `nth` | Y | | Y | | Y | strong |
| 171 | LLStPer | `empty` | Y | | Y | | Y | strong |
| 172 | LLStPer | `singleton` | Y | | Y | | Y | strong |
| 173 | LLStPer | `subseq_copy` | Y | | Y | | Y | strong |
| 174 | LLStPer | `append` | Y | | Y | | Y | strong |
| 175 | LLStPer | `filter` | Y | | Y | | Y | strong |
| 176 | LLStPer | `update` | Y | | Y | | Y | strong |
| 177 | LLStPer | `is_empty` | Y | | Y | | Y | strong |
| 178 | LLStPer | `is_singleton` | Y | | Y | | Y | strong |
| 179 | LLStPer | `iterate` | Y | | Y | | Y | **weak** |
| 180 | LLStPer | `reduce` | Y | | Y | | Y | **weak** |
| 181 | LLStPer | `scan` | Y | | | | Y | — |
| 182 | LLStPer | `flatten` | Y | | | | Y | — |
| 183 | LLStPer | `map` | Y | | Y | | Y | strong |
| 184 | LLStPer | `tabulate` | Y | | Y | | Y | strong |
| 185 | LLStPer | `from_vec` | Y | | Y | | Y | strong |
| 186 | LLStPer | `iter` | | | Y | | Y | strong |
| 187 | LLStPer | `clone` | | Y | | | Y | none |
| 188 | LLStPer | `eq` | | Y | | | Y | assume |
| 189 | LLStPer | `next` | | Y | | | Y | strong |
| 190 | LLStPer | `into_iter` x2 | | Y | | | Y | none |
| 191 | LLStPer | `fmt` x2 | | Y | | | | none |

### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **ITT** = implemented in `impl Trait for Type` (inherits trait spec).
- **BI** = implemented in bare `impl Type` (own spec, not connected to trait).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **Spec**: strong = full requires+ensures; weak = requires only, no ensures on return; assume = proved via `assume`; none = no spec; — = in trait only, no impl exists.

### Key difference: LinkedList files

The LinkedList files declare traits (`LinkedListStEphBaseTrait`, `LinkedListStEphRedefinableTrait`)
with full specs, but the implementations are in bare `impl<T> LinkedListStEphS<T>` blocks (BI) — **not**
`impl LinkedListStEphBaseTrait for LinkedListStEphS` (ITT). The trait specs exist but are disconnected
from the implementations. `scan` and `flatten` are declared in the trait but have no impl at all.

### Summary column notes

- **Trait** counts include fns declared in the trait even if no impl exists (scan, flatten in LinkedList).
- **ITT** includes both custom-trait impls and std-trait impls (Clone, PartialEq, Iterator, IntoIterator, Debug, Display).
- **None** includes trait-only fns with no impl ("—" in detail), external fns, and fns with no spec at all.

### Proof Holes

| # | Module | assume | external | Proof fns (clean/holed) |
|---|--------|:------:|:--------:|:-----------------------:|
| 1 | ArraySeq | 1 | 2 | 10 / 0 |
| 2 | ArraySeqStEph | 1 | 0 | 1 / 0 |
| 3 | ArraySeqStPer | 1 | 0 | 1 / 0 |
| 4 | ArraySeqMtEph | 1 | 0 | 1 / 0 |
| 5 | ArraySeqMtPer | 1 | 0 | 1 / 0 |
| 6 | LinkedListStEph | 1 | 0 | 0 / 0 |
| 7 | LinkedListStPer | 1 | 0 | 0 / 0 |

All 7 `assume` holes are the same PartialEq pattern. This is a Verus limitation:
the solver cannot resolve `self.eq_spec(other)` through trait dispatch in a
`TraitMethodImpl`, and Verus panics if you attempt to call trait spec methods
inside the proof block. The 2 `external` holes in ArraySeq are `#[verifier::external]`
on `IntoIterator`/iterator-related impls.

### Structural Notes

- **ArraySeq.rs** is the reference implementation with the fullest function set (23 trait + 4 ML + 2 IS).
- **St/Mt variants** mirror ArraySeq's trait pattern but split into `BaseTrait` + `RedefinableTrait`.
- **LinkedList files** declare traits with full specs, but implementations are in bare `impl Type` blocks (BI), **not** `impl Trait for Type` (ITT). The traits are disconnected from the impls.
- **Mt variants** add `map_par`, `filter_par`, `reduce_par` in bare impl blocks (BI).
- **Functions only in ArraySeq**: `collect`, `find_key`, `inject`, `scan_inclusive`, `remove`, `insert`, `iterate_prefixes`, `iter_mut`.
- **LinkedList weak specs**: `iterate` and `reduce` have `requires` only, no `ensures` on the return value.
- **LinkedList unimplemented**: `scan` and `flatten` are in the trait declarations but have no impl.
