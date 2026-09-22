# Run-time tests restored (r211)

Date: 2026-09-21. Plan: `plans/r211-cargo-build-and-rtt.md`. Subject: the live
APAS-VERUS working tree on `main` (main `4bd6b4f92` plus the uncommitted
r207-r210 state). Nothing is committed in APAS-VERUS. Verus 0.2026.09.13.671956e;
Rust 1.98.1 (`rust-toolchain.toml`).

Result, measured: the library and all 264 test binaries build under `cargo`
(`logs/cargo-build.20260921-152058.log`, 0 errors); 4199 run-time tests pass,
0 fail, 0 ignored (`logs/rtt.20260921-152215.log`); Chap05 verifies at 760 and
Chap06 at 1037 items with 0 errors and 0 warnings; the full `validate.sh`
reports the same 162 errors in the same 53 files as before this round.

## 1. Cause

vstd at rev `671956e` declares its specification modules only for Verus:

```
vstd/vstd.rs:96    #[cfg(verus_keep_ghost)]
vstd/vstd.rs:97    pub mod std_specs;
```

`cargo` sets no `verus_keep_ghost`, so under `cargo` there is no
`vstd::std_specs` at all and no proof fn in `vstd::iset`. The r210 fixture's
`logs/rtt.20260921-112745.log` recorded the consequence: `could not compile
apas-verus (lib) due to 168 previous errors`, of which 119 were `cannot find
type VerusForLoopWrapper` (the manual loops r208 wrote in Chap05 and Chap06 on
vstd's for-loop wrapper, which lives in `std_specs::iter`), 45 were `cannot
find std_specs in vstd` (ungated `use vstd::std_specs::…` lines; 8 of the 45
were the lines the r210 tool added to Chap18, which are not in this tree),
2 were `cannot find trait IteratorSpecImpl` (spec-only impls), and 2 were
unresolved `vstd::iset` lemma imports.

The `verus!` macro desugars a `for` loop through the wrapper in both build
modes, so `for` loops were never affected; only the hand-written manual loops
named the wrapper type.

## 2. Edits

### 2.1 Every gated import (39 `use` lines) and the two spec-only impls

Each `use` line below now carries `#[cfg(verus_keep_ghost)]` on the line
before it, the one cfg form CLAUDE.md allows. "Old" is the line before this
round, "New" the line after. Chap `-` marks a file under `src/standards/`
(row 38: `src/vstdplus/`). Import: I = `use vstd::std_specs::iter::*;`,
H = `use vstd::std_specs::hash::*;`, F = `use vstd::iset::fold::is_fun_commutative;`,
L = `use vstd::iset::lemma_iset_finite_if_subset_of_seq;`.

| # | Chap | File | Old | New | Import |
|---|------|------|----:|----:|--------|
| 1 | 05 | RelationStEph.rs | 35 | 36 | I |
| 2 | 05 | SetStEph.rs | 37 | 38 | I |
| 3 | 05 | SetMtEph.rs | 51 | 52 | I |
| 4 | 05 | MappingStEph.rs | 36 | 37 | I |
| 5 | 66 | BoruvkaStEph.rs | 33 | 34 | I |
| 6 | 66 | BoruvkaMtEph.rs | 32 | 33 | I |
| 7 | - | iterators_standard.rs | 95 | 96 | I |
| 8 | - | prophetic_iterators_standard.rs | 79 | 80 | I |
| 9 | - | table_of_contents_standard.rs | 59 | 60 | I |
| 10 | 17 | MathSeq.rs | 40 | 41 | I |
| 11 | 06 | DirGraphStEph.rs | 32 | 33 | I |
| 12 | - | mod_standard.rs | 32 | 33 | I |
| 13 | - | wrapping_iterators_standard.rs | 76 | 77 | I |
| 14 | - | deep_view_standard.rs | 24 | 25 | I |
| 15 | 06 | WeightedDirGraphStEphUsize.rs | 28 | 29 | I |
| 16 | 06 | WeightedDirGraphStEphI16.rs | 28 | 29 | I |
| 17 | 06 | WeightedDirGraphStEphI32.rs | 28 | 29 | I |
| 18 | 06 | LabUnDirGraphMtEph.rs | 44 | 45 | I |
| 19 | - | view_standard.rs | 20 | 21 | I |
| 20 | 06 | WeightedDirGraphStEphI64.rs | 28 | 29 | I |
| 21 | 06 | WeightedDirGraphStEphU128.rs | 28 | 29 | I |
| 22 | - | using_hashmap_standard.rs | 77 | 78 | H |
| 23 | - | using_hashmap_standard.rs | 78 | 80 | I |
| 24 | 06 | WeightedDirGraphStEphIsize.rs | 28 | 29 | I |
| 25 | 06 | WeightedDirGraphStEphU32.rs | 28 | 29 | I |
| 26 | 06 | WeightedDirGraphStEphU8.rs | 28 | 29 | I |
| 27 | 06 | LabUnDirGraphStEph.rs | 31 | 32 | I |
| 28 | 06 | UnDirGraphMtEph.rs | 44 | 45 | I |
| 29 | 06 | WeightedDirGraphStEphF64.rs | 28 | 29 | I |
| 30 | 06 | WeightedDirGraphStEphI8.rs | 28 | 29 | I |
| 31 | 06 | LabDirGraphStEph.rs | 31 | 32 | I |
| 32 | 06 | LabDirGraphMtEph.rs | 44 | 45 | I |
| 33 | 06 | DirGraphMtEph.rs | 44 | 45 | I |
| 34 | 06 | WeightedDirGraphStEphI128.rs | 28 | 29 | I |
| 35 | 06 | UnDirGraphStEph.rs | 32 | 33 | I |
| 36 | 06 | WeightedDirGraphStEphU16.rs | 28 | 29 | I |
| 37 | 06 | WeightedDirGraphStEphU64.rs | 28 | 29 | I |
| 38 | - | seq_set.rs | 28 | 29 | F |
| 39 | - | finite_sets_standard.rs | 60 | 61 | L |

The plan counted 164 `use vstd::std_specs…` lines crate-wide; 125 of them
already carried the gate (r207-r209 wrote them so) and the two `//!` lines in
`iterator_ptt_standard.rs` are documentation text. `src/vstdplus/feq.rs:25`
is ungated at line level but sits under the module-level
`#[cfg(verus_keep_ghost)]` on `pub mod feq` and did not error; it was left as
it is. The 47 ungated lines in `src/experiments/` belong to modules that
`lib.rs` comments out; they do not reach `cargo` and were not edited.
`broadcast use vstd::std_specs::…` lines (Chap21, Chap54, Chap61, Chap63,
Chap64) are proof text that the `verus!` macro erases and did not error.

| # | Chap | File | Line | Impl gated |
|---|------|------|-----:|------------|
| 1 | - | prophetic_iterators_standard.rs | 174 | `IteratorSpecImpl for CountIter` |
| 2 | - | wrapping_iterators_standard.rs | 282 | `IteratorSpecImpl for OuterIter<'a, T>` |

Both impls hold only spec fns; under `cargo` the trait does not exist. This
is the form CLAUDE.md prescribes for `PartialEqSpecImpl`.

### 2.2 Every manual loop rewritten (119 sites, 22 files)

The one exec change of the round. At every site the loop body's statements
and their order are unchanged; what changes is the loop's driver and its
ghost bookkeeping:

Old form (r208; `X` the iterator expression, `s` the ghost `Seq` bound to
`into_iter_hash_keys(X)`):

```rust
let mut it = VerusForLoopWrapper::new(X);
loop
    invariant
        it.wf(),
        IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
        IteratorSpec::decrease(&it.iter) is Some,
        it.seq().unref() == s,
        … it.index() …
    decreases IteratorSpec::decrease(&it.iter)->0,
{
    let ghost old_pos = it.index();
    match it.next() {
        Some(x) => { proof { assert(s[old_pos] == *x); } … },
        None => { … },
    }
}
```

New form (this round):

```rust
let mut it = X;
let ghost mut pos: int = 0;
loop
    invariant
        IteratorSpec::obeys_prophetic_iter_laws(&it),
        IteratorSpec::decrease(&it) is Some,
        0 <= pos <= s.len(),
        IteratorSpec::remaining(&it).len() == s.len() - pos,
        forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
            ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == s[pos + i],
        … pos …
    decreases IteratorSpec::decrease(&it)->0,
{
    let ghost old_pos = pos;
    match it.next() {
        Some(x) => { proof { pos = pos + 1; assert(s[old_pos] == *x); } … },
        None => { … },
    }
}
```

`let mut it = X;` is a move of the iterator into the loop variable, not a
call; the wrapper's `new`, `wf()`, `seq()`, `index()` and the field access
`it.iter` are gone. `pos` is a ghost counter of items consumed, advanced in
the `Some` arm's first proof block; the two `remaining` clauses are the
wrapper's own `wf_inner` (vstd/std_specs/iter.rs:848) written out. `it.next()`
is now `Iterator::next` on the std iterator, whose contract is the
`IteratorSpec` one (vstd/std_specs/iter.rs:35). Work and span are unchanged at
every site: the exec statements are the same, one `next()` per iteration; the
wrapper's `next` was one `next()` plus ghost updates.

The ghost variable is named `pos`, or `<prefix>_pos` where the iterator
variable is `<prefix>_it`. "Line" is the line of `let ghost mut … pos: int = 0;`
in the file after the edit. Old form and new form are as above at every row;
the loop bodies were not touched except for the `pos = pos + 1;` ghost
statement.

| # | Chap | File | Function | Line | Counter |
|---|------|------|----------|-----:|---------|
| 1 | 05 | SetStEph.rs | `all_nonempty` | 669 | `parts_pos` |
| 2 | 05 | SetStEph.rs | `partition_on_elt` | 712 | `parts_pos` |
| 3 | 05 | SetStEph.rs | `partition` | 788 | `s1_pos` |
| 4 | 05 | SetMtEph.rs | `cartesian_product` | 649 | `pos` |
| 5 | 05 | SetMtEph.rs | `all_nonempty` | 802 | `parts_pos` |
| 6 | 05 | SetMtEph.rs | `partition_on_elt` | 845 | `parts_pos` |
| 7 | 05 | SetMtEph.rs | `partition` | 921 | `s1_pos` |
| 8 | 05 | MappingStEph.rs | `is_functional_SetStEph_at` | 356 | `pos` |
| 9 | 05 | MappingStEph.rs | `is_functional_SetStEph` | 399 | `pos` |
| 10 | 06 | DirGraphStEph.rs | `ng_of_vertices` | 307 | `pos` |
| 11 | 06 | DirGraphStEph.rs | `n_plus` | 379 | `pos` |
| 12 | 06 | DirGraphStEph.rs | `n_minus` | 446 | `pos` |
| 13 | 06 | DirGraphStEph.rs | `n_plus_of_vertices` | 512 | `pos` |
| 14 | 06 | DirGraphStEph.rs | `n_minus_of_vertices` | 583 | `pos` |
| 15 | 06 | UnDirGraphStEph.rs | `ng` | 248 | `pos` |
| 16 | 06 | UnDirGraphStEph.rs | `ng_of_vertices` | 321 | `pos` |
| 17 | 06 | LabDirGraphStEph.rs | `arcs` | 221 | `pos` |
| 18 | 06 | LabDirGraphStEph.rs | `get_arc_label` | 290 | `pos` |
| 19 | 06 | LabDirGraphStEph.rs | `has_arc` | 342 | `pos` |
| 20 | 06 | LabDirGraphStEph.rs | `n_plus` | 398 | `pos` |
| 21 | 06 | LabDirGraphStEph.rs | `n_minus` | 463 | `pos` |
| 22 | 06 | LabUnDirGraphStEph.rs | `edges` | 215 | `pos` |
| 23 | 06 | LabUnDirGraphStEph.rs | `get_edge_label` | 291 | `pos` |
| 24 | 06 | LabUnDirGraphStEph.rs | `has_edge` | 351 | `pos` |
| 25 | 06 | LabUnDirGraphStEph.rs | `ng` | 412 | `pos` |
| 26 | 06 | LabDirGraphMtEph.rs | `arcs` | 307 | `pos` |
| 27 | 06 | LabDirGraphMtEph.rs | `get_arc_label` | 397 | `pos` |
| 28 | 06 | LabDirGraphMtEph.rs | `has_arc` | 449 | `pos` |
| 29 | 06 | LabUnDirGraphMtEph.rs | `edges` | 282 | `pos` |
| 30 | 06 | LabUnDirGraphMtEph.rs | `get_edge_label` | 375 | `pos` |
| 31 | 06 | LabUnDirGraphMtEph.rs | `has_edge` | 434 | `pos` |
| 32 | 06 | WeightedDirGraphStEphF64.rs | `from_weighed_edges` | 130 | `pos` |
| 33 | 06 | WeightedDirGraphStEphF64.rs | `weighed_edges` | 196 | `pos` |
| 34 | 06 | WeightedDirGraphStEphF64.rs | `out_neighbors_weighed` | 239 | `pos` |
| 35 | 06 | WeightedDirGraphStEphF64.rs | `in_neighbors_weighed` | 303 | `pos` |
| 36 | 06 | WeightedDirGraphStEphI128.rs | `from_weighed_edges` | 155 | `pos` |
| 37 | 06 | WeightedDirGraphStEphI128.rs | `weighed_edges` | 221 | `pos` |
| 38 | 06 | WeightedDirGraphStEphI128.rs | `out_neighbors_weighed` | 264 | `pos` |
| 39 | 06 | WeightedDirGraphStEphI128.rs | `in_neighbors_weighed` | 328 | `pos` |
| 40 | 06 | WeightedDirGraphStEphI128.rs | `total_weight` | 392 | `pos` |
| 41 | 06 | WeightedDirGraphStEphI128.rs | `edges_above_weight` | 441 | `pos` |
| 42 | 06 | WeightedDirGraphStEphI128.rs | `edges_below_weight` | 502 | `pos` |
| 43 | 06 | WeightedDirGraphStEphI16.rs | `from_weighed_edges` | 155 | `pos` |
| 44 | 06 | WeightedDirGraphStEphI16.rs | `weighed_edges` | 214 | `pos` |
| 45 | 06 | WeightedDirGraphStEphI16.rs | `out_neighbors_weighed` | 257 | `pos` |
| 46 | 06 | WeightedDirGraphStEphI16.rs | `in_neighbors_weighed` | 321 | `pos` |
| 47 | 06 | WeightedDirGraphStEphI16.rs | `total_weight` | 385 | `pos` |
| 48 | 06 | WeightedDirGraphStEphI16.rs | `edges_above_weight` | 434 | `pos` |
| 49 | 06 | WeightedDirGraphStEphI16.rs | `edges_below_weight` | 495 | `pos` |
| 50 | 06 | WeightedDirGraphStEphI32.rs | `from_weighed_edges` | 155 | `pos` |
| 51 | 06 | WeightedDirGraphStEphI32.rs | `weighed_edges` | 214 | `pos` |
| 52 | 06 | WeightedDirGraphStEphI32.rs | `out_neighbors_weighed` | 257 | `pos` |
| 53 | 06 | WeightedDirGraphStEphI32.rs | `in_neighbors_weighed` | 321 | `pos` |
| 54 | 06 | WeightedDirGraphStEphI32.rs | `total_weight` | 385 | `pos` |
| 55 | 06 | WeightedDirGraphStEphI32.rs | `edges_above_weight` | 434 | `pos` |
| 56 | 06 | WeightedDirGraphStEphI32.rs | `edges_below_weight` | 495 | `pos` |
| 57 | 06 | WeightedDirGraphStEphI64.rs | `from_weighed_edges` | 155 | `pos` |
| 58 | 06 | WeightedDirGraphStEphI64.rs | `weighed_edges` | 214 | `pos` |
| 59 | 06 | WeightedDirGraphStEphI64.rs | `out_neighbors_weighed` | 257 | `pos` |
| 60 | 06 | WeightedDirGraphStEphI64.rs | `in_neighbors_weighed` | 321 | `pos` |
| 61 | 06 | WeightedDirGraphStEphI64.rs | `total_weight` | 385 | `pos` |
| 62 | 06 | WeightedDirGraphStEphI64.rs | `edges_above_weight` | 434 | `pos` |
| 63 | 06 | WeightedDirGraphStEphI64.rs | `edges_below_weight` | 495 | `pos` |
| 64 | 06 | WeightedDirGraphStEphI8.rs | `from_weighed_edges` | 155 | `pos` |
| 65 | 06 | WeightedDirGraphStEphI8.rs | `weighed_edges` | 214 | `pos` |
| 66 | 06 | WeightedDirGraphStEphI8.rs | `out_neighbors_weighed` | 257 | `pos` |
| 67 | 06 | WeightedDirGraphStEphI8.rs | `in_neighbors_weighed` | 321 | `pos` |
| 68 | 06 | WeightedDirGraphStEphI8.rs | `total_weight` | 385 | `pos` |
| 69 | 06 | WeightedDirGraphStEphI8.rs | `edges_above_weight` | 434 | `pos` |
| 70 | 06 | WeightedDirGraphStEphI8.rs | `edges_below_weight` | 495 | `pos` |
| 71 | 06 | WeightedDirGraphStEphIsize.rs | `from_weighed_edges` | 155 | `pos` |
| 72 | 06 | WeightedDirGraphStEphIsize.rs | `weighed_edges` | 214 | `pos` |
| 73 | 06 | WeightedDirGraphStEphIsize.rs | `out_neighbors_weighed` | 257 | `pos` |
| 74 | 06 | WeightedDirGraphStEphIsize.rs | `in_neighbors_weighed` | 321 | `pos` |
| 75 | 06 | WeightedDirGraphStEphIsize.rs | `total_weight` | 385 | `pos` |
| 76 | 06 | WeightedDirGraphStEphIsize.rs | `edges_above_weight` | 434 | `pos` |
| 77 | 06 | WeightedDirGraphStEphIsize.rs | `edges_below_weight` | 495 | `pos` |
| 78 | 06 | WeightedDirGraphStEphU128.rs | `from_weighed_edges` | 155 | `pos` |
| 79 | 06 | WeightedDirGraphStEphU128.rs | `weighed_edges` | 214 | `pos` |
| 80 | 06 | WeightedDirGraphStEphU128.rs | `out_neighbors_weighed` | 257 | `pos` |
| 81 | 06 | WeightedDirGraphStEphU128.rs | `in_neighbors_weighed` | 321 | `pos` |
| 82 | 06 | WeightedDirGraphStEphU128.rs | `total_weight` | 385 | `pos` |
| 83 | 06 | WeightedDirGraphStEphU128.rs | `edges_above_weight` | 435 | `pos` |
| 84 | 06 | WeightedDirGraphStEphU128.rs | `edges_below_weight` | 496 | `pos` |
| 85 | 06 | WeightedDirGraphStEphU16.rs | `from_weighed_edges` | 155 | `pos` |
| 86 | 06 | WeightedDirGraphStEphU16.rs | `weighed_edges` | 214 | `pos` |
| 87 | 06 | WeightedDirGraphStEphU16.rs | `out_neighbors_weighed` | 257 | `pos` |
| 88 | 06 | WeightedDirGraphStEphU16.rs | `in_neighbors_weighed` | 321 | `pos` |
| 89 | 06 | WeightedDirGraphStEphU16.rs | `total_weight` | 385 | `pos` |
| 90 | 06 | WeightedDirGraphStEphU16.rs | `edges_above_weight` | 435 | `pos` |
| 91 | 06 | WeightedDirGraphStEphU16.rs | `edges_below_weight` | 496 | `pos` |
| 92 | 06 | WeightedDirGraphStEphU32.rs | `from_weighed_edges` | 155 | `pos` |
| 93 | 06 | WeightedDirGraphStEphU32.rs | `weighed_edges` | 214 | `pos` |
| 94 | 06 | WeightedDirGraphStEphU32.rs | `out_neighbors_weighed` | 257 | `pos` |
| 95 | 06 | WeightedDirGraphStEphU32.rs | `in_neighbors_weighed` | 321 | `pos` |
| 96 | 06 | WeightedDirGraphStEphU32.rs | `total_weight` | 385 | `pos` |
| 97 | 06 | WeightedDirGraphStEphU32.rs | `edges_above_weight` | 435 | `pos` |
| 98 | 06 | WeightedDirGraphStEphU32.rs | `edges_below_weight` | 496 | `pos` |
| 99 | 06 | WeightedDirGraphStEphU64.rs | `from_weighed_edges` | 155 | `pos` |
| 100 | 06 | WeightedDirGraphStEphU64.rs | `weighed_edges` | 214 | `pos` |
| 101 | 06 | WeightedDirGraphStEphU64.rs | `out_neighbors_weighed` | 257 | `pos` |
| 102 | 06 | WeightedDirGraphStEphU64.rs | `in_neighbors_weighed` | 321 | `pos` |
| 103 | 06 | WeightedDirGraphStEphU64.rs | `total_weight` | 385 | `pos` |
| 104 | 06 | WeightedDirGraphStEphU64.rs | `edges_above_weight` | 435 | `pos` |
| 105 | 06 | WeightedDirGraphStEphU64.rs | `edges_below_weight` | 496 | `pos` |
| 106 | 06 | WeightedDirGraphStEphU8.rs | `from_weighed_edges` | 155 | `pos` |
| 107 | 06 | WeightedDirGraphStEphU8.rs | `weighed_edges` | 214 | `pos` |
| 108 | 06 | WeightedDirGraphStEphU8.rs | `out_neighbors_weighed` | 257 | `pos` |
| 109 | 06 | WeightedDirGraphStEphU8.rs | `in_neighbors_weighed` | 321 | `pos` |
| 110 | 06 | WeightedDirGraphStEphU8.rs | `total_weight` | 385 | `pos` |
| 111 | 06 | WeightedDirGraphStEphU8.rs | `edges_above_weight` | 435 | `pos` |
| 112 | 06 | WeightedDirGraphStEphU8.rs | `edges_below_weight` | 496 | `pos` |
| 113 | 06 | WeightedDirGraphStEphUsize.rs | `from_weighed_edges` | 155 | `pos` |
| 114 | 06 | WeightedDirGraphStEphUsize.rs | `weighed_edges` | 214 | `pos` |
| 115 | 06 | WeightedDirGraphStEphUsize.rs | `total_weight` | 385 | `pos` |
| 116 | 06 | WeightedDirGraphStEphUsize.rs | `out_neighbors_weighed` | 257 | `pos` |
| 117 | 06 | WeightedDirGraphStEphUsize.rs | `in_neighbors_weighed` | 321 | `pos` |
| 118 | 06 | WeightedDirGraphStEphUsize.rs | `edges_above_weight` | 435 | `pos` |
| 119 | 06 | WeightedDirGraphStEphUsize.rs | `edges_below_weight` | 496 | `pos` |

Site-specific details, all ghost:

1. Chap05 `SetMtEph.rs` `cartesian_product` (row 4): `let ghost idx =
   it.index() - 1;` became `let ghost idx = pos - 1;`; the old form had no
   `old_pos` and none was added.
2. Chap05 `SetStEph.rs` and `SetMtEph.rs` `all_nonempty`, `partition_on_elt`,
   `partition` (rows 1-3, 5-7) and `MappingStEph.rs` (rows 8-9): the old
   `Some` arm had no proof block; the increment is a new one-line
   `proof { parts_pos = parts_pos + 1; }` (or `pos`).
3. Chap06 `DirGraphStEph.rs` and `UnDirGraphStEph.rs`, the `*_of_vertices`
   functions (rows 10, 13, 14, 16): the increment is the first line of the
   existing `proof { lemma_seq_index_in_map_to_set(u_seq, old_pos); }` block.
4. Chap06 `WeightedDirGraphStEph*.rs` `total_weight` (12 integer files):
   `wa_seq.take(it.index())` in the invariant and in the `Some`-arm `=~=`
   assert became `wa_seq.take(pos)`; the assert follows the increment, so
   `pos` there is the new index, as `it.index()` was.

Per-file validation after each file, all at 0 errors and 0 warnings:
Chap05 `logs/validate.20260921-150920.log` (SetStEph) and
`logs/validate.20260921-151051.log` (SetMtEph, MappingStEph): 760 verified;
Chap06 `logs/validate.20260921-151207.log` (DirGraphStEph) through
`logs/validate.20260921-152003.log` (WeightedDirGraphStEphF64), twenty runs,
each 1037 verified. No `VerusForLoopWrapper`, `it.wf()`, `it.seq()`,
`it.index()` or `&it.iter` remains in `src/Chap05` or `src/Chap06`; 119 `pos`
counters exist.

## 3. Standards and the experiment

### 3.1 Experiment: `src/experiments/prophetic_manual_loop_next.rs`

Question: which manual `loop { match it.next() { .. } }` form verifies on the
iterator's own `next()`? Result: SUCCEEDS, 10 verified, 0 errors
(`logs/validate-standard-prophetic_manual_loop_next.20260921-145935.log`).
Five forms are in the file: the index-wise form of section 2.2 over
`std::slice::Iter`; the same with an early `return` from the `Some` arm (the
`all_nonempty` shape), which needs `orig == a.seq@` as an invariant because
Verus isolates the loop and proves the postcondition at the `return` from the
invariants alone; the consuming `std::vec::IntoIter` (owned items, no deref);
and the `for x in it: coll.iter()` form.

A second experiment, `src/experiments/prophetic_manual_loop_next_skip.rs`,
holds the alternative single clause `IteratorSpec::remaining(&it).unref() ==
orig.skip(pos)` with an extensional `=~=` re-establishment: FAILS, 2 verified,
4 errors (`logs/validate-standard-prophetic_manual_loop_next_skip.20260921-145939.log`):
`0 <= pos <= orig.len()` not re-established, both `Some`-arm asserts and the
`None`-arm `pos == orig.len()` fail, because Z3 does not relate
`orig.skip(pos).len()` and `orig.skip(pos)[i]` to `orig` without
`lemma_seq_skip_len`/`lemma_seq_skip_index` (vstd/seq_lib.rs:3706, :3732),
which are not broadcast by default. The index-wise form needs no seq lemma
and no `=~=`, and is the one adopted. Both experiments are listed, commented
out, in `src/lib.rs` with their result.

### 3.2 Standards

1. `src/standards/iterators_standard.rs`: the manual-loop idiom in the
   module doc rewritten to section 2.2's form; the `decreases` paragraph now
   names `remaining()` and says why the wrapper is not used. Check:
   `scripts/validate-standard.sh iterators_standard deps ptt`, 207 verified,
   0 errors, 0 warnings
   (`logs/validate-standard-iterators_standard.20260921-150200.log`).
2. `src/standards/prophetic_iterators_standard.rs`: the two module-doc
   paragraphs that said a manual loop wraps the iterator in
   `VerusForLoopWrapper` now describe the `next()` form;
   `impl IteratorSpecImpl for CountIter` gated. Check:
   `scripts/validate-standard.sh prophetic_iterators_standard deps ptt`, 211
   verified, 0 errors, 0 warnings
   (`logs/validate-standard-prophetic_iterators_standard.20260921-150233.log`).
3. `src/standards/iterator_ptt_standard.rs`: pattern table rows 1, 2, 5; the
   loop-borrow-iter template; the consuming-variants paragraph; the Mt and
   tree notes that named `VerusForLoopWrapper<...>`. A documentation-only
   file (its last lines say it does not compile); nothing to run.
4. `src/standards/wrapping_iterators_standard.rs`: `impl IteratorSpecImpl for
   OuterIter` and the import gated. Not re-run: its PTT
   `Provewrapping_iterators_standard.rs` still uses the wrapper (Needs
   discussion 3).
5. `rust_verify_test/tests/standards/Proveiterators_standard.rs`: the three
   manual loops (loop-borrow-iter, loop-borrow-into, loop-consume) and the
   header. Its body module `scratch/ptt_bodies/ptt_iterators_standard.rs`,
   rewritten identically, is what item 1's `ptt` verified.
6. `rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs`:
   the three manual loops (loop-borrow, loop-consume, loop-custom over
   `CountIter`) and the header. Its body module
   `scratch/ptt_bodies/ptt_prophetic_iterators_standard.rs` is what item 2's
   `ptt` verified.
7. `docs/PropheticIterators.md`: the manual-loop paragraph replaced by the
   new idiom, its reason, the `skip` finding and the `return`-inside-loop
   rule; both experiments added to See also.
8. `docs/APAS-VERUSIterators.rs`: the manual-loop idiom and the six-pattern
   table.
9. `.cursor/rules/apas-verus/collection-iterators.mdc`: pattern table, loop
   template, consuming paragraph, loop-vs-for table.

The custom-iterator loop (item 6, `CountIter`) verifies with the same
invariants: `remaining()` is `closed` there, and the constructor's
`ensures IteratorSpec::remaining(&it) == Seq::new(..)` supplies the sequence.

## 4. Run-time tests

`scripts/rtt.sh` exits at once with `error: no such command: nextest`
(`logs/rtt.20260921-152145.log`): `cargo-nextest` is not installed for the
1.98.1 toolchain (no binary under `~/.cargo/bin` or any toolchain's `bin/`).
The run was therefore made with `cargo test --release --no-fail-fast`, ANSI
stripped, into the script's log location, after a `cargo build --release
--tests` (`logs/cargo-build.20260921-152058.log`, `Finished` in 25 s, 0
errors, 34 warnings of which 4 are in the library and all pre-existing:
`Chap43/OrderedTableStEph.rs:793` unexpected cfg `never`,
`Chap18/ArraySeqMtEph.rs:1338` unneeded `mut`, `vstdplus/threads_plus.rs:105`
and `:108` negative impls; the other 30 are the `all_chapters` cfg warning
per test target).

| # | Measure | Value | Log |
|---|---------|------:|-----|
| 1 | test binaries built and run | 264 | logs/rtt.20260921-152215.log |
| 2 | tests passed | 4199 | same |
| 3 | tests failed | 0 | same |
| 4 | tests ignored | 0 | same |
| 5 | doc-test target (`--doc`) | 1 passed, 12 failed | same |

The 12 doc-test failures are the ```` ``` ```` blocks in the module docs of
`src/standards/partial_eq_eq_clone_standard.rs` (lines 47, 70, 76, 91, 101,
116, 133, 144, 153, 171) and `src/standards/spec_wf_standard.rs` (lines 42,
65): Verus text (`requires`, `open spec fn`) that rustdoc compiles as Rust
(`expected one of `->`, `where`, or `{`, found `requires``). They predate this
round, are not in any file this round edited, and `cargo nextest`, which
`scripts/rtt.sh` calls, does not run doc-tests. Under the script's own
semantics every run-time test passes.

Re-enabled `[[test]]` entries in `Cargo.toml` (commented out in r208 because
their chapters imported the deleted `vstdplus::hash_map_with_view_plus`; r209
moved the chapters to `std::collections::HashMap`):

| # | Chap | Test target | Result |
|---|------|-------------|--------|
| 1 | 62 | TestStarContractionMtEph | ok |
| 2 | 62 | TestStarContractionStEph | ok |
| 3 | 62 | TestStarPartitionMtEph | ok |
| 4 | 62 | TestStarPartitionStEph | ok |
| 5 | 63 | TestConnectivityMtEph | ok |
| 6 | 63 | TestConnectivityStEph | ok |
| 7 | 66 | TestBoruvkaMtEph | ok |
| 8 | 66 | TestBoruvkaStEph | ok |

`test_partial_order` stays commented out (`vstdplus::partial_order` is
commented out in `src/lib.rs`). No test file needed an edit.

## 5. Regression

| # | Run | Log | Result |
|---|-----|-----|--------|
| 1 | isolate Chap02 | validate.20260921-152308.log | 631 verified, 0 errors, 0 warnings |
| 2 | isolate Chap03 | validate.20260921-152312.log | 622 verified, 0 errors, 0 warnings |
| 3 | isolate Chap05 (final) | validate.20260921-151051.log | 760 verified, 0 errors, 0 warnings |
| 4 | isolate Chap06 (final) | validate.20260921-152003.log | 1037 verified, 0 errors, 0 warnings |
| 5 | isolate Chap17 | validate.20260921-152316.log | 645 verified, 0 errors, 0 warnings |
| 6 | isolate Chap50 | validate.20260921-152320.log | 766 verified, 0 errors, 0 warnings |
| 7 | isolate Chap66 | validate.20260921-152325.log | 805 verified, 0 errors, 0 warnings |
| 8 | full, before this round | validate.20260921-102521.log | 162 errors, 53 files, 824 warnings |
| 9 | full, after | validate.20260921-152341.log | 162 errors, 53 files, 824 warnings |

Runs 8 and 9 stop at the same front-end errors (136 `no method named _ found
for struct`, 13 mismatched types, 8 `no field`, 4 `no method … for enum`, 1
unsatisfied trait bound: the unmigrated chapters' old-model iterator code),
and the set of files named by an error is identical (`diff` of the two
sorted file lists is empty). The plan's "51 files" counted differently from
the 53 that the `-->` line after each `error` line gives; the comparison is
like for like.

## 6. The tool and the new fixture

### 6.1 `iterator-upgrade` (CSTs round r0727)

`~/projects/CSTs/processes/iterator-upgrade-lib/src/definer.rs` `add_use_iter`
now emits `#[cfg(verus_keep_ghost)]` on the line before
`use vstd::std_specs::iter::*;` in a source module (`template.rs` gains the
`CFG_KEEP_GHOST` constant; `site.rs`'s class doc and `Testdefiner.rs`'s
expected text follow). Proof-time-test bodies keep the bare import: a
`verus_code!` body is compiled by `rust_verify_test` under Verus only, and the
standards' own PTT sources use the bare form. The tool emits no spec-only
impl (`grep IteratorSpecImpl` over its sources: 0), so there is nothing to
gate there. Tests: iterator-upgrade-lib 22 passed
(`processes/logs/test-iterator-upgrade-lib-20260921-152555.log`),
iterator-upgrade 2 passed (`processes/logs/test-iterator-upgrade-20260921-152555.log`),
build `processes/logs/build-iterator-upgrade-20260921-152553.log`. Committed
in CSTs as round r0727, commit `e7a371a67` (plan and report under its `plans/` and `reports/`).

### 6.2 Fixture baseline

`~/projects/CSTs/processes/iterator-upgrade/tests/fixtures/APAS-VERUS`
(gitignored in CSTs; its own git repository) was replaced by an `rsync
--delete` of this working tree at the end of step 7, excluding `.git/`,
`target/` and `logs/`; `diff -rq` between the two trees (same exclusions) is
empty. Committed there as `4df0fbc` ("r211 fixture baseline"), on top of the
r210 history (`c520913` baseline plus the sixteen r210 apply commits, head
`598938a`).

### 6.3 Chap18 re-run on the new fixture

Logs for rows 1-2 are `iterator-upgrade-<log>` under CSTs `processes/iterator-upgrade/analyses/`;
rows 4-5 under the fixture's `logs/`.

| # | Step | Log | Result |
|---|------|-----|--------|
| 1 | `--detect` | detect-20260921-152648.log | 188 sites, 8 residuals, as r210 |
| 2 | `--apply` | apply-20260921-152656.log | 16 files, 553(+) 1537(-); r210 545(+) |
| 3 | hand fix 1 | (Edit tool) | `into_iter` ensures as r210's text |
| 4 | isolate Chap18 | validate.20260921-152707.log | 998 verified, 2 errors, 153 s |
| 5 | RTT (`cargo test`) | rtt.20260921-152955.log | 4199 passed, 0 failed; Chap18 8/8 ok |

Row 2's eight extra insertions are the gate lines. Row 3 removed the
`requires` from `ArraySeqMtEphSlice.rs`'s `into_iter` and made its `ensures`
conditional on `spec_arrayseqmtephslice_wf()`, exactly as r210 did. Row 5 ran
`cargo test --release --no-fail-fast` since `cargo-nextest` is absent; its
`--doc` target has the same 12 pre-existing standards doc-test failures as
section 4. The two open errors of row 4 are as r210 recorded them: `ArraySeqMtEphSlice.rs:1565`
`arc_vec_as_slice` precondition inside `into_iter` (needs an exec-body
change, not made), and `ArraySeqStEph.rs:781` rlimit in the untouched `scan`
loop. The RTT check that r210 could not reach is now reached: the library
builds and the Chap18 tests pass.

The diff differs from r210's by the eight `#[cfg(verus_keep_ghost)]` lines,
so a fresh patch series was emitted: `plans/r211-chap18-patches/0001..0016`,
one commit per file from `4df0fbc` (fixture head `e4d948d`), and every patch
passes `git apply --check` on this tree. Patch 0011 carries hand fix 1; the
other fifteen are the tool's output unchanged.

## 7. Alg Analysis

Every `/// - Alg Analysis` line in every file this round edited was read and
none was edited (`git diff` of the working tree against `main` contains no
added or removed `Alg Analysis` line). Counts per edited source file:

"Lines" is the file's count of `Alg Analysis` lines; "Loops" the rows of
section 2.2's table whose functions live in the file ("-" = import gated
only); "Code review" the cost those functions' Code review lines state
(unchanged, since the loops execute the same statements).

| # | Chap | File | Lines | Loops | Code review |
|---|------|------|------:|-------|-------------|
| 1 | 05 | SetStEph.rs | 49 | 1-3 | O(\|parts\|) x2, O(\|a\| x \|parts\|) |
| 2 | 05 | SetMtEph.rs | 59 | 4-7 | O(\|a\| x \|b\|), then as row 1 |
| 3 | 05 | MappingStEph.rs | 39 | 8-9 | O(\|s\|), O(\|s\|^2) |
| 4 | 05 | RelationStEph.rs | 27 | - | - |
| 5 | 06 | DirGraphStEph.rs | 53 | 10-14 | O(\|S\| x \|A\|) x3, O(\|A\|) x2 |
| 6 | 06 | DirGraphMtEph.rs | 85 | - | - |
| 7 | 06 | UnDirGraphStEph.rs | 33 | 15-16 | O(\|E\|), O(\|S\| x \|E\|) |
| 8 | 06 | UnDirGraphMtEph.rs | 53 | - | - |
| 9 | 06 | LabDirGraphStEph.rs | 33 | 17-21 | O(\|A\|) each |
| 10 | 06 | LabDirGraphMtEph.rs | 47 | 26-28 | O(\|A\|) each |
| 11 | 06 | LabUnDirGraphStEph.rs | 30 | 22-25 | O(\|E\|) each |
| 12 | 06 | LabUnDirGraphMtEph.rs | 48 | 29-31 | O(\|E\|) each |
| 13 | 06 | WeightedDirGraphStEphF64.rs | 24 | 32-35 | O(\|V\| + \|E\|), O(\|A\|) x3 |
| 14 | 06 | WeightedDirGraphStEph*.rs (12 int) | 34 each | 36-119 | as row 13, then O(\|A\|) x3 |
| 15 | 17 | MathSeq.rs | 36 | - | - |
| 16 | 66 | BoruvkaStEph.rs | 18 | - | - |
| 17 | 66 | BoruvkaMtEph.rs | 25 | - | - |
| 18 | - | vstdplus/seq_set.rs | 0 | - | - |

1067 lines in total (row 14 is 408). The loop-form change keeps every exec
statement and executes one `next()` per iteration as before, so every Code
review line of the 119 functions stands as written; the span lines of the Mt
files' three functions (rows 10, 12) are sequential scans and stand as well.
No line is disputed on cost. One format
observation goes to `docs/AlgorithmicAnalysisIssues.md` section 6: the ten
`APAS (Ch05 Def 5.6): definitional predicate, no cost specified.` lines in
`MappingStEph.rs` (185, 191, 197, 203, 209) are the "no cost stated" APAS
line that `alg_analysis_notation_standard.rs` says not to write.

## 8. Needs discussion

1. `cargo-nextest` is not installed, so `scripts/rtt.sh` cannot run. The
   results above come from `cargo test --release --no-fail-fast` written to
   the script's log location. Either install `cargo-nextest` for the 1.98.1
   toolchain, or change the script; the script also carries a 120 s
   `timeout` that a cold `--release` build would exceed (the warm build here
   took 25 s).
2. The 12 doc-test failures in `partial_eq_eq_clone_standard.rs` and
   `spec_wf_standard.rs`: `cargo test` compiles their ```` ``` ```` doc blocks
   as Rust. `cargo nextest` skips doc-tests, so the script would not see them.
   Marking the blocks ```` ```text ```` or ```` ```ignore ```` closes them; not
   done, since the files were outside this round's edits.
3. `rust_verify_test/tests/standards/Provewrapping_iterators_standard.rs`,
   `Provedeep_view_standard.rs`, `Provemod_standard.rs`,
   `Provetable_of_contents_standard.rs` and `Proveview_standard.rs`, and the
   matching `scratch/ptt_bodies/` modules, still write their manual loops on
   `VerusForLoopWrapper`. They compile (proof-time tests run under Verus
   only) and the plan named only the three iterator standards, so they were
   left; they no longer match the template in `iterator_ptt_standard.rs`.
4. Under `cargo`, `#[cfg(verus_keep_ghost)]` on `impl IteratorSpecImpl` removes
   the impl, and the gated imports remove `IteratorSpec` from scope. Exec
   code never names either (the `verus!` macro erases `ensures`, `invariant`,
   `decreases` and `proof` blocks), so nothing else needed a gate; the cargo
   build is the evidence.
5. The `rsync --delete` that replaced the fixture removed two r210 scratch
   files that existed only in the fixture and were untracked in its git:
   `scratch/r210-extract-ptt-bodies.awk` and
   `scratch/r210-validate-ptt-chap18.sh`, with the
   `scratch/ptt_bodies/ptt_Chap18_*.rs` bodies they produced. The plan asked
   for a replacement of the fixture's contents; `docs/IteratorTransformerChap18.md`
   section 6 describes the harness they built, and the fixture's r210 logs
   (`logs/validate-ptt-Chap18.20260921-111426.log` and the others) were kept,
   since `logs/` was excluded from the rsync.
6. `src/standards/iterator_ptt_standard.rs` is a comment-only file; the plan's
   check `validate-standard.sh iterator_ptt_standard deps ptt` has nothing to
   verify (no PTT body module exists for it) and was not run.
7. The plan's site counts were taken from the r210 fixture, which had the
   r210 Chap18 patches applied: 45 `std_specs` errors there are 37 ungated
   lines here (the eight Chap18 lines are in the r211 patch series, gated).
8. Hole audit of the whole uncommitted working tree against `main`
   (`git diff -U0 -- src`, added lines only): no `assume(`, `accept(`,
   `admit(`, or `requires true` was added by any round; the two added
   `#[verifier::external_body]` lines are r208's `std::hash::Hash::hash`
   bodies in `src/Chap05/SetStEph.rs` and `SetMtEph.rs` (present in the r210
   fixture baseline `598938a` before this round began). r211 added none.
9. The index holds a staged rename `src/vstdplus/seq_set.rs` ->
   `src/vstdplus/seq_set_pre_0913.rs` from r208 (its `lib.rs` line 249 says
   the old file is kept as a record); the working tree's `seq_set.rs` is
   r208's replacement, which this round gated. It is r208's state, untouched.
