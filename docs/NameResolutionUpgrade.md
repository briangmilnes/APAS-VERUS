# Name resolution on Verus 0.2026.09.13

Round r207, agent 1. Date: 2026-09-20. Plan: `plans/r207-agent1-name-resolution.md`.

Goal: make every name in `src/` resolve on the 09.13 vstd so that rustc proceeds
from name resolution to type checking. Stop condition: the newest validate log
contains no `E0425`, `E0432`, or `E0407`. The type errors that appear next are
inventoried in section 4 and not fixed.

Result: the stop condition is met. `logs/validate.20260920-151314.log` contains
0 name-resolution errors, 273 type-check errors, and 940 deprecation warnings.
90 files under `src/` changed (138 insertions, 173 deletions). No exec body,
no `/// - Alg Analysis` line, and no `assume`, `accept`, `external_body`,
`admit`, or `requires` clause was added or changed.

## 1. Starting measurement

Log: `logs/validate.20260920-145749.log` (2,520 lines). rustc stopped in name
resolution with 167 errors across 90 files: `E0425` 165, `E0432` 1, `E0407` 1.

Paths in this table drop the `vstd::` prefix.

| # | Unresolved name | Sites | Files | Replacement in 09.13 vstd |
|---|-----------------|------:|------:|---------------------------|
| 1 | `set::group_set_axioms` | 73 | 62 | `set::group_set_lemmas` |
| 2 | `map::group_map_axioms` | 33 | 33 | `map::group_map_lemmas` |
| 3 | `seq_lib::seq_to_set_is_finite` | 22 | 4 | removed; `Set::finite()` is `true` |
| 4 | `set::fold::lemma_fold_{empty,insert}` | 26 | 1 | `iset::fold::…` over `ISet` |
| 5 | `std_specs::slice::group_slice_axioms` | 2 | 2 | `slice::group_slice_axioms` |
| 6 | `relations::injective_on` | 3 | 3 | method `Set::injective_on` |
| 7 | `set::axiom_set_insert_len` | 2 | 1 | `set::lemma_set_insert_len` |
| 8 | `set::axiom_set_new` | 1 | 1 | `set::lemma_set_new` |
| 9 | `set::axiom_set_insert_finite` | 1 | 1 | removed |
| 10 | `set_lib::lemma_set_union_finite_iff` | 1 | 1 | removed |
| 11 | `axiom_spec_slice_iter`, `…_into_iter` | 2 | 1 | removed with the old iterator model |
| 12 | `initial_value_relation` (impl method) | 1 | 1 | not a member of `IteratorSpecImpl` |

Definitions read in `~/projects/verus/source/vstd/` before each edit:
`group_set_lemmas` set.rs:575; `group_map_lemmas` map.rs:335; `Set::finite`
set.rs:228; `Set::fold` set.rs:291-307; `iset::fold::lemma_fold_insert`
iset.rs:613 and `lemma_fold_empty` iset.rs:627; `group_slice_axioms`
slice.rs:223; `Set::injective_on` set_lib.rs:188 (delegating to
iset_lib.rs:122); `lemma_set_insert_len` set.rs:504; `lemma_set_new`
set.rs:319; `IteratorSpecImpl` std_specs/iter.rs:21-66. Arity and role match
the old name in every group except 8, where `lemma_set_new(f, a)` adds
`requires Set::<A>::new(f) is Some` (section 5).

## 2. Edits

Legend: paths drop the `vstd::` prefix. `Line` is the line number in the file
before the edit (deletions shift later lines). `Chap` is `—` for files outside
a chapter directory. `s'` stands for `prefix.to_set().to_iset()`. Every edit was
an exact-token replacement or a whole-statement deletion made with the Edit tool.

### 2.1 Group 1: `group_set_axioms` to `group_set_lemmas` (73 sites, 62 files)

| # | Chap | File | Line | Old | New |
|---|------|------|-----:|-----|-----|
| 1 | — | Types.rs | 63 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 2 | — | ParaPairs.rs | 31 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 3 | — | hash_set_with_view_plus.rs | 44 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 4 | — | seq_set.rs | 21 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 5 | — | seq_set.rs | 154 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 6 | — | seq_set.rs | 195 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 7 | — | seq_set.rs | 208 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 8 | — | seq_set.rs | 255 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 9 | — | seq_set.rs | 264 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 10 | — | seq_set.rs | 278 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 11 | — | seq_set.rs | 326 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 12 | — | seq_set.rs | 340 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 13 | — | seq_set.rs | 371 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 14 | — | seq_set.rs | 410 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 15 | — | seq_set.rs | 616 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 16 | 05 | RelationStEph.rs | 53 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 17 | 05 | MappingStEph.rs | 58 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 18 | 05 | SetMtEph.rs | 73 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 19 | 05 | KleeneStPer.rs | 40 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 20 | 05 | SetStEph.rs | 57 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 21 | 06 | DirGraphStEph.rs | 57 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 22 | 06 | UnDirGraphStEph.rs | 55 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 23 | 06 | LabDirGraphStEph.rs | 46 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 24 | 06 | LabUnDirGraphStEph.rs | 52 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 25 | 06 | WeightedDirGraphStEphU8.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 26 | 06 | WeightedDirGraphStEphU16.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 27 | 06 | WeightedDirGraphStEphU32.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 28 | 06 | WeightedDirGraphStEphU64.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 29 | 06 | WeightedDirGraphStEphU128.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 30 | 06 | WeightedDirGraphStEphUsize.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 31 | 06 | WeightedDirGraphStEphI8.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 32 | 06 | WeightedDirGraphStEphI16.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 33 | 06 | WeightedDirGraphStEphI32.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 34 | 06 | WeightedDirGraphStEphI64.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 35 | 06 | WeightedDirGraphStEphI128.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 36 | 06 | WeightedDirGraphStEphIsize.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 37 | 06 | WeightedDirGraphStEphF64.rs | 45 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 38 | 06 | DirGraphMtEph.rs | 70 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 39 | 06 | UnDirGraphMtEph.rs | 69 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 40 | 06 | LabDirGraphMtEph.rs | 67 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 41 | 06 | LabUnDirGraphMtEph.rs | 67 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 42 | 17 | MathSeq.rs | 63 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 43 | 38 | BSTParaStEph.rs | 71 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 44 | 38 | BSTParaMtEph.rs | 69 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 45 | 39 | BSTTreapSpecsAndLemmas.rs | 33 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 46 | 39 | BSTTreapStEph.rs | 91 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 47 | 39 | BSTTreapMtEph.rs | 61 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 48 | 39 | BSTParaTreapMtEph.rs | 62 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 49 | 40 | BSTKeyValueStEph.rs | 61 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 50 | 41 | OrdKeyMap.rs | 58 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 51 | 41 | ArraySetStEph.rs | 57 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 52 | 41 | ArraySetEnumMtEph.rs | 71 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 53 | 41 | AVLTreeSetStEph.rs | 61 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 54 | 41 | AVLTreeSetStPer.rs | 61 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 55 | 41 | AVLTreeSetMtEph.rs | 59 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 56 | 41 | AVLTreeSetMtPer.rs | 65 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 57 | 43 | OrderedSpecsAndLemmas.rs | 28 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 58 | 43 | OrderedTableStEph.rs | 59 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 59 | 43 | OrderedTableStPer.rs | 61 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 60 | 43 | OrderedSetStEph.rs | 56 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 61 | 43 | OrderedSetMtEph.rs | 56 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 62 | 43 | OrderedSetStPer.rs | 58 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 63 | 52 | AdjTableGraphSpecsAndLemmas.rs | 38 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 64 | 52 | AdjTableGraphStEph.rs | 55 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 65 | 52 | AdjTableGraphStPer.rs | 59 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 66 | 52 | AdjTableGraphMtPer.rs | 52 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 67 | 52 | EdgeSetGraphStEph.rs | 47 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 68 | 52 | EdgeSetGraphStPer.rs | 52 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 69 | 52 | EdgeSetGraphMtEph.rs | 50 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 70 | 52 | EdgeSetGraphMtPer.rs | 47 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 71 | 65 | PrimStEph.rs | 65 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 72 | 66 | BoruvkaStEph.rs | 55 | `set::group_set_axioms` | `set::group_set_lemmas` |
| 73 | 66 | BoruvkaMtEph.rs | 58 | `set::group_set_axioms` | `set::group_set_lemmas` |

### 2.2 Group 2: `group_map_axioms` to `group_map_lemmas` (33 sites, 33 files)

| # | Chap | File | Line | Old | New |
|---|------|------|-----:|-----|-----|
| 1 | — | hash_map_with_view_plus.rs | 43 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 2 | 05 | MappingStEph.rs | 74 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 3 | 40 | BSTKeyValueStEph.rs | 61 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 4 | 41 | OrdKeyMap.rs | 57 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 5 | 42 | TableSpecsAndLemmas.rs | 21 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 6 | 42 | TableStEph.rs | 62 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 7 | 42 | TableStPer.rs | 61 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 8 | 42 | TableMtEph.rs | 61 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 9 | 43 | OrderedSpecsAndLemmas.rs | 27 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 10 | 43 | OrderedTableStEph.rs | 58 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 11 | 43 | OrderedTableMtEph.rs | 59 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 12 | 43 | AugOrderedTableStEph.rs | 54 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 13 | 43 | AugOrderedTableMtEph.rs | 65 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 14 | 43 | OrderedTableStPer.rs | 60 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 15 | 43 | AugOrderedTableStPer.rs | 55 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 16 | 43 | OrderedTableMtPer.rs | 53 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 17 | 50 | MatrixChainStEph.rs | 58 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 18 | 50 | MatrixChainStPer.rs | 57 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 19 | 50 | MatrixChainMtEph.rs | 67 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 20 | 50 | MatrixChainMtPer.rs | 66 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 21 | 50 | OptBinSearchTreeStEph.rs | 56 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 22 | 50 | OptBinSearchTreeStPer.rs | 56 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 23 | 50 | OptBinSearchTreeMtEph.rs | 68 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 24 | 50 | OptBinSearchTreeMtPer.rs | 66 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 25 | 51 | SeqSpecsAndLemmas.rs | 25 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 26 | 51 | TopDownDPStEph.rs | 51 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 27 | 51 | TopDownDPStPer.rs | 51 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 28 | 51 | TopDownDPMtEph.rs | 56 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 29 | 51 | TopDownDPMtPer.rs | 56 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 30 | 52 | AdjTableGraphSpecsAndLemmas.rs | 37 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 31 | 52 | AdjTableGraphStEph.rs | 54 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 32 | 52 | AdjTableGraphStPer.rs | 58 | `map::group_map_axioms` | `map::group_map_lemmas` |
| 33 | 52 | AdjTableGraphMtPer.rs | 51 | `map::group_map_axioms` | `map::group_map_lemmas` |

### 2.3 Group 3: `seq_to_set_is_finite` call statements deleted (22 sites, 4 files)

`Set::finite()` is now `open spec fn finite(self) -> bool { true }` (set.rs:228),
so each deleted statement asserted a tautology. No enclosing `proof` block
became empty; every block keeps at least one other statement.

| # | Chap | File | Line | Old | New |
|---|------|------|-----:|-----|-----|
| 1 | — | seq_set.rs | 485 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 2 | — | seq_set.rs | 779 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 3 | — | seq_set.rs | 905 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 4 | — | seq_set.rs | 944 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 5 | — | seq_set.rs | 984 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 6 | — | seq_set.rs | 1023 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 7 | — | seq_set.rs | 1062 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 8 | — | seq_set.rs | 1094 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 9 | — | seq_set.rs | 1127 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 10 | — | seq_set.rs | 1159 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 11 | — | seq_set.rs | 1191 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 12 | — | seq_set.rs | 1223 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 13 | — | seq_set.rs | 1255 | `seq_lib::seq_to_set_is_finite(prefix)` | (statement deleted) |
| 14 | 41 | ArraySetStEph.rs | 612 | `…(filtered.elements@)` | (statement deleted) |
| 15 | 41 | ArraySetStEph.rs | 698 | `…(common.elements@)` | (statement deleted) |
| 16 | 41 | ArraySetStEph.rs | 781 | `…(remaining.elements@)` | (statement deleted) |
| 17 | 41 | ArraySetStEph.rs | 906 | `…(combined.elements@)` | (statement deleted) |
| 18 | 41 | ArraySetStEph.rs | 984 | `…(self.elements@)` | (statement deleted) |
| 19 | 41 | ArraySetStEph.rs | 1045 | `…(self.elements@)` | (statement deleted) |
| 20 | 41 | AVLTreeSetMtEph.rs | 277 | `…(views)` | (statement deleted) |
| 21 | 41 | AVLTreeSetMtEph.rs | 406 | `…(seq@)` | (statement deleted) |
| 22 | 41 | AVLTreeSetMtPer.rs | 290 | `…(views)` | (statement deleted) |

### 2.4 Group 4: fold lemmas in `seq_set.rs` (26 sites, 1 file)

`Set::fold(z, f)` is now `#[verifier::inline] self.to_iset().fold(z, f)`
(set.rs:291-307), and the fold lemmas live in `vstd::iset::fold` over `ISet`:
`lemma_fold_empty<A, B>(z: B, f)` ensures `ISet::empty().fold(z, f) == z`;
`lemma_fold_insert<A, B>(s: ISet<A>, z, f, a)` requires `s.finite()`,
`!s.contains(a)`, `is_fun_commutative(f)` and ensures
`s.insert(a).fold(z, f) == f(s.fold(z, f), a)`. The smallest change that
resolves and keeps each lemma's statement true is the module rename for the
base case and, for the step case, passing `prefix.to_set().to_iset()` (the
value `Set::fold` unfolds to). Type arguments and the other arguments are
unchanged. Every step site follows the same two-line pattern, so one
exact-string `replace_all` covered all 13.

| # | Chap | File | Line | Old | New |
|---|------|------|-----:|-----|-----|
| 1 | — | seq_set.rs | 462 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 2 | — | seq_set.rs | 486 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 3 | — | seq_set.rs | 754 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 4 | — | seq_set.rs | 780 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 5 | — | seq_set.rs | 897 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 6 | — | seq_set.rs | 906 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 7 | — | seq_set.rs | 936 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 8 | — | seq_set.rs | 945 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 9 | — | seq_set.rs | 976 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 10 | — | seq_set.rs | 985 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 11 | — | seq_set.rs | 1015 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 12 | — | seq_set.rs | 1024 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 13 | — | seq_set.rs | 1054 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 14 | — | seq_set.rs | 1063 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 15 | — | seq_set.rs | 1086 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 16 | — | seq_set.rs | 1095 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 17 | — | seq_set.rs | 1119 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 18 | — | seq_set.rs | 1128 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 19 | — | seq_set.rs | 1151 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 20 | — | seq_set.rs | 1160 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 21 | — | seq_set.rs | 1183 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 22 | — | seq_set.rs | 1192 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 23 | — | seq_set.rs | 1215 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 24 | — | seq_set.rs | 1224 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |
| 25 | — | seq_set.rs | 1247 | `set::fold::lemma_fold_empty` | `iset::fold::lemma_fold_empty` |
| 26 | — | seq_set.rs | 1256 | `set::fold::lemma_fold_insert(s,…)` | `iset::fold::lemma_fold_insert(s',…)` |

### 2.5 Groups 5 to 12 (14 sites, 8 files)

| # | Grp | Chap | File | Line | Old | New |
|---|----:|------|------|-----:|-----|-----|
| 1 | 5 | 18 | ArraySeqMtEphSlice.rs | 61 | `std_specs::slice::group_slice_axioms` | `slice::group_slice_axioms` |
| 2 | 5 | 19 | ArraySeqMtEphSlice.rs | 61 | `std_specs::slice::group_slice_axioms` | `slice::group_slice_axioms` |
| 3 | 6 | 41 | OrdKeyMap.rs | 202 | `relations::injective_on(proj, s)` | `s.injective_on(proj)` |
| 4 | 6 | 43 | OrderedSpecsAndLemmas.rs | 144 | `relations::injective_on(proj, s)` | `s.injective_on(proj)` |
| 5 | 6 | 53 | PQMinStPer.rs | 40-41 | `use vstd::relations::injective_on;` | (use and its cfg attribute deleted) |
| 6 | 7 | 44 | DocumentIndex.rs | 223 | `set::axiom_set_insert_len` | `set::lemma_set_insert_len` |
| 7 | 7 | 44 | DocumentIndex.rs | 283 | `set::axiom_set_insert_len` | `set::lemma_set_insert_len` |
| 8 | 8 | 06 | LabDirGraphStEph.rs | 341 | `set::axiom_set_new(pred, e)` | `set::lemma_set_new(pred, e)` |
| 9 | 9 | 44 | DocumentIndex.rs | 222 | `set::axiom_set_insert_finite(…)` | (statement deleted) |
| 10 | 10 | 65 | PrimStEph.rs | 295 | `lemma_set_union_finite_iff(…)` | (statement deleted) |
| 11 | 11 | — | prophetic_iterators_standard.rs | 65 | `broadcast use …axiom_spec_slice_iter` | (line deleted) |
| 12 | 11 | — | prophetic_iterators_standard.rs | 291 | `broadcast use …axiom_spec_into_iter` | (line deleted) |
| 13 | 12 | — | prophetic_iterators_standard.rs | 172-177 | `fn initial_value_relation` | (method, comment, attr deleted) |

Notes on rows 3 to 5: `Set::injective_on(self, r)` unfolds to
`self.to_iset().injective_on(r)`, whose body is the same universally
quantified statement the old free function had (iset_lib.rs:122). The
`PQMinStPer.rs` import was the only occurrence of the identifier in that file,
so the `use` line and the `#[cfg(verus_keep_ghost)]` attribute that belonged
to it were deleted together; no call site changed. Row 12 is inside an exec
body (`into_iter`); the plan names this deletion explicitly.

Files under `src/experiments/` (7 with `group_set_axioms`, 3 with the removed
iterator names) are commented out in `lib.rs`, absent from the log, and left
untouched as documentation of past experiments.

## 3. Validate error count after each group

Every run is `scripts/validate.sh` in full mode; each log was read after the
run. The rustc name-resolution pass takes 14 to 20 s and aborts before type
checking, so the count is exact for that pass.

| # | After group | Log | Errors | E0425 | E0432 | E0407 |
|---|-------------|-----|-------:|------:|------:|------:|
| 0 | start | `validate.20260920-145749.log` | 167 | 165 | 1 | 1 |
| 1 | 1 `group_set_lemmas` | `validate.20260920-150801.log` | 94 | 92 | 1 | 1 |
| 2 | 2 `group_map_lemmas` | `validate.20260920-150918.log` | 61 | 59 | 1 | 1 |
| 3 | 3 `seq_to_set_is_finite` | `validate.20260920-151005.log` | 39 | 37 | 1 | 1 |
| 4 | 4 fold lemmas | `validate.20260920-151027.log` | 13 | 11 | 1 | 1 |
| 5 | 5 slice group | `validate.20260920-151101.log` | 11 | 9 | 1 | 1 |
| 6 | 6 `injective_on` | `validate.20260920-151123.log` | 8 | 7 | 0 | 1 |
| 7 | 7 `lemma_set_insert_len` | `validate.20260920-151140.log` | 6 | 5 | 0 | 1 |
| 8 | 8 `lemma_set_new` | `validate.20260920-151158.log` | 5 | 4 | 0 | 1 |
| 9 | 9 `axiom_set_insert_finite` | `validate.20260920-151216.log` | 4 | 3 | 0 | 1 |
| 10 | 10 `lemma_set_union_finite_iff` | `validate.20260920-151233.log` | 3 | 2 | 0 | 1 |
| 11 | 11 iterator broadcast uses | `validate.20260920-151255.log` | 1 | 0 | 0 | 1 |
| 12 | 12 `initial_value_relation` | `validate.20260920-151314.log` | 0 | 0 | 0 | 0 |

Each group removed exactly its site count; no edit introduced a new
name-resolution error.

## 4. Errors after name resolution passes

Log: `logs/validate.20260920-151314.log` (8,673 lines). rustc type checking
reports 273 errors in 74 files and 940 warnings, then aborts before Verus
runs. Elapsed 14 s; peak `rust_verify` RSS 1,274 MB; Z3 not started.

### 4.1 By rustc code

Messages have backticked names replaced by `_`; the E0599 messages end with
"in the current scope", dropped here for width.

| # | Code | Count | rustc message |
|---|------|------:|---------------|
| 1 | E0609 | 93 | no field `_` on type `_` |
| 2 | E0599 | 89 | no method named `_` found for struct `_` |
| 3 | E0599 | 4 | no method named `_` found for enum `_` |
| 4 | E0308 | 61 | mismatched types |
| 5 | E0277 | 22 | the trait bound `_` is not satisfied |
| 6 | E0782 | 4 | expected a type, found a trait |

### 4.2 By root cause

The 273 errors reduce to seven vstd or Verus changes. Counts were taken from
the error blocks; every error block was read.

| # | Root cause (09.13 definition) | Codes | Count |
|---|-------------------------------|-------|------:|
| 1 | `Set::new(f)` returns `Option<Set<A>>` | E0308 56, E0277 21, E0599 4 | 81 |
| 2 | `Map::new` takes a domain `Set<K>` | E0308 5 | 5 |
| 3 | std iterators lack a `View` impl | E0599 84 | 84 |
| 4 | `for` ghost is `VerusForLoopWrapper` | E0609 93 | 93 |
| 5 | `Set::lemma_map_finite` removed | E0599 5 | 5 |
| 6 | `initial_value_relation` removed | E0782 4 | 4 |
| 7 | spec `==` requires `SpecEq<Rhs>` | E0277 1 | 1 |

Definitions: `Set::new` set.rs:133; `Map::new` map.rs:67;
`VerusForLoopWrapper` std_specs/iter.rs:823 (fields `index`, `snapshot`,
`iter`, `history`); `ISet::lemma_map_finite` iset_lib.rs:718 (no `Set`
counterpart in set_lib.rs); `IteratorSpec` std_specs/iter.rs:21-66;
`spec_eq` builtin lib.rs:1249. Root cause 3 is the prophetic iterator model:
`std::slice::Iter`, `std::vec::IntoIter`, `hash_set::Iter`, `hash_map::Iter`,
and `str::Chars` no longer implement `View`.

Detail per root cause:

1. `Set::new`. 53 sites return `Set::new(...)` from a spec fn typed `Set<_>`
   ("expected `Set`, found `Option`"); 21 sites compare a `Set` view to
   `Set::new(...)` with `==` ("`Set<_>: SpecEq<Option<Set<_>>>` is not
   satisfied"); 4 sites call `.finite()`, `.subset_of()`, or `.contains()` on
   the `Option` (E0599 on enum). `Chap41/ArraySetEnumMtEph.rs:155` and `:161`
   hold the three remaining E0308 shapes ("expected `Option`, found `Set`",
   "expected `Option`, found `&mut Set`", "expected `Set`, found `usize`"),
   all consequences of the same `Option` return.
2. `Map::new`. The first argument is now the domain `Set<K>`; five sites pass a
   predicate closure ("expected `Set<_>`, found `FnSpec<(_,), bool>`"):
   `Chap05/MappingStEph.rs:95`, `Chap41/OrdKeyMap.rs:86`,
   `Chap43/OrderedSpecsAndLemmas.rs:38`, `Chap62/StarPartitionMtEph.rs:622`
   and `:1305`.
3. Iterator `View`. "no method named `view`" by receiver type:
   `std::vec::IntoIter` 60, `std::slice::Iter` 20, `std::str::Chars` 2,
   `hash_set::Iter` 1, `hash_map::Iter` 1. These are the old-model iterator
   wrappers (`self.inner@`, `it@.0`, `it@.1`) in sections 5 and 10 of the
   collection files and in five old-model standards files.
4. `VerusForLoopWrapper`. "no field `pos`" 61 and "no field `elements`" 32;
   the wrapper's fields are `index`, `snapshot`, `iter`, `history`. All sites
   are `for` loop invariants in `Chap05/SetStEph.rs` (37),
   `Chap05/SetMtEph.rs` (35), `Chap05/RelationStEph.rs` (6),
   `Chap59/JohnsonMtEphI64.rs` (4), `Chap59/JohnsonMtEphF64.rs` (4),
   `Chap64/SpanTreeStEph.rs` (2), `Chap64/SpanTreeMtEph.rs` (2),
   `Chap63/ConnectivityStEph.rs` (2), `Chap63/ConnectivityMtEph.rs` (1).
5. `lemma_map_finite`. `Chap41/OrdKeyMap.rs:169`,
   `Chap43/OrderedSpecsAndLemmas.rs:109`, `Chap53/PQMinStPer.rs:361`,
   `Chap65/PrimStEph.rs:293` and `:294`.
6. `initial_value_relation`. `src/standards/prophetic_iterators_standard.rs`
   lines 127, 213, 264, 281 (the `ensures` clauses that name the deleted
   method); these follow from the group-12 deletion and were expected.
7. `SpecEq`. `src/vstdplus/arc_rwlock.rs:65`, `ensures *cloned == *arc`,
   compares `T` with `Arc<T>`; the new `spec_eq` bound rejects it.

### 4.3 By file

| # | Chap | File | E0277 | E0308 | E0599 | E0609 | E0782 | Total |
|---|------|------|------:|------:|------:|------:|------:|------:|
| 1 | 05 | SetStEph.rs | 0 | 0 | 0 | 37 | 0 | 37 |
| 2 | 05 | SetMtEph.rs | 0 | 0 | 0 | 35 | 0 | 35 |
| 3 | 06 | DirGraphMtEph.rs | 2 | 15 | 0 | 0 | 0 | 17 |
| 4 | 06 | DirGraphStEph.rs | 5 | 5 | 0 | 0 | 0 | 10 |
| 5 | 05 | RelationStEph.rs | 4 | 0 | 0 | 6 | 0 | 10 |
| 6 | 41 | ArraySetEnumMtEph.rs | 0 | 6 | 3 | 0 | 0 | 9 |
| 7 | 06 | LabDirGraphMtEph.rs | 1 | 7 | 0 | 0 | 0 | 8 |
| 8 | 06 | UnDirGraphMtEph.rs | 0 | 6 | 0 | 0 | 0 | 6 |
| 9 | 06 | LabDirGraphStEph.rs | 3 | 3 | 0 | 0 | 0 | 6 |
| 10 | 19 | ArraySeqMtEph.rs | 0 | 0 | 5 | 0 | 0 | 5 |
| 11 | — | prophetic_iterators_standard.rs | 0 | 0 | 0 | 0 | 4 | 4 |
| 12 | 59 | JohnsonMtEphI64.rs | 0 | 0 | 0 | 4 | 0 | 4 |
| 13 | 59 | JohnsonMtEphF64.rs | 0 | 0 | 0 | 4 | 0 | 4 |
| 14 | 52 | EdgeSetGraphMtPer.rs | 1 | 2 | 1 | 0 | 0 | 4 |
| 15 | 06 | UnDirGraphStEph.rs | 2 | 2 | 0 | 0 | 0 | 4 |
| 16 | 06 | LabUnDirGraphMtEph.rs | 0 | 4 | 0 | 0 | 0 | 4 |
| 17 | 05 | MappingStEph.rs | 2 | 2 | 0 | 0 | 0 | 4 |
| 18 | — | view_standard.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 19 | — | table_of_contents_standard.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 20 | — | mod_standard.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 21 | — | iterators_standard.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 22 | — | deep_view_standard.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 23 | 41 | OrdKeyMap.rs | 0 | 1 | 2 | 0 | 0 | 3 |
| 24 | 23 | PrimTreeSeqStPer.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 25 | 23 | BalBinTreeStEph.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 26 | 19 | ArraySeqStPer.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 27 | 19 | ArraySeqStEph.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 28 | 18 | LinkedListStPer.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 29 | 18 | LinkedListStEph.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 30 | 18 | ArraySeqStPer.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 31 | 18 | ArraySeqStEph.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 32 | 18 | ArraySeqMtPer.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 33 | 18 | ArraySeqMtEph.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 34 | 18 | ArraySeq.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 35 | 17 | MathSeq.rs | 0 | 0 | 3 | 0 | 0 | 3 |
| 36 | 06 | LabUnDirGraphStEph.rs | 1 | 2 | 0 | 0 | 0 | 3 |
| 37 | 65 | PrimStEph.rs | 0 | 0 | 2 | 0 | 0 | 2 |
| 38 | 64 | SpanTreeStEph.rs | 0 | 0 | 0 | 2 | 0 | 2 |
| 39 | 64 | SpanTreeMtEph.rs | 0 | 0 | 0 | 2 | 0 | 2 |
| 40 | 63 | ConnectivityStEph.rs | 0 | 0 | 0 | 2 | 0 | 2 |
| 41 | 62 | StarPartitionMtEph.rs | 0 | 2 | 0 | 0 | 0 | 2 |
| 42 | 44 | DocumentIndex.rs | 0 | 0 | 2 | 0 | 0 | 2 |
| 43 | 43 | OrderedSpecsAndLemmas.rs | 0 | 1 | 1 | 0 | 0 | 2 |
| 44 | — | hash_set_with_view_plus.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 45 | — | hash_map_with_view_plus.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 46 | — | arc_rwlock.rs | 1 | 0 | 0 | 0 | 0 | 1 |
| 47 | — | wrapping_iterators_standard.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 48 | 63 | ConnectivityMtEph.rs | 0 | 0 | 0 | 1 | 0 | 1 |
| 49 | 53 | PQMinStPer.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 50 | 52 | EdgeSetGraphStPer.rs | 0 | 1 | 0 | 0 | 0 | 1 |
| 51 | 52 | EdgeSetGraphStEph.rs | 0 | 1 | 0 | 0 | 0 | 1 |
| 52 | 52 | EdgeSetGraphMtEph.rs | 0 | 1 | 0 | 0 | 0 | 1 |
| 53 | 43 | OrderedTableStPer.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 54 | 43 | OrderedTableStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 55 | 43 | OrderedSetStPer.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 56 | 43 | OrderedSetStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 57 | 41 | AVLTreeSetStPer.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 58 | 41 | AVLTreeSetStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 59 | 41 | AVLTreeSetMtPer.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 60 | 40 | BSTSizeStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 61 | 40 | BSTReducedStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 62 | 40 | BSTKeyValueStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 63 | 39 | BSTTreapStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 64 | 39 | BSTTreapMtEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 65 | 39 | BSTSetTreapMtEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 66 | 39 | BSTParaTreapMtEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 67 | 38 | BSTParaStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 68 | 37 | BSTSplayStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 69 | 37 | BSTRBStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 70 | 37 | BSTPlainStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 71 | 37 | BSTBBAlphaStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 72 | 37 | BSTAVLStEph.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 73 | 19 | ArraySeqMtEphSlice.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 74 | 18 | ArraySeqMtEphSlice.rs | 0 | 0 | 1 | 0 | 0 | 1 |
| 75 | — | (column totals) | 22 | 61 | 93 | 93 | 4 | 273 |

### 4.4 Warnings

All 940 warnings are one rustc message: use of deprecated method
`vstd::set::Set::<A>::finite`: "Every Set is always finite, so this is always
true." They come from `.finite()` calls in `spec_*_wf` predicates, `requires`,
`ensures`, and `invariant` clauses. The ten files with the most sites:
`Chap38/BSTParaMtEph.rs` 71, `Chap39/BSTParaTreapMtEph.rs` 60,
`Chap43/AugOrderedTableStEph.rs` 57, `Chap43/OrderedTableStEph.rs` 56,
`Chap43/AugOrderedTableMtEph.rs` 55, `Chap39/BSTTreapStEph.rs` 48,
`Chap41/OrdKeyMap.rs` 41, `Chap38/BSTParaStEph.rs` 40,
`Chap43/OrderedSetStEph.rs` 37, `Chap43/OrderedSetStPer.rs` 34. Removing them
is a separate pass; the finite-sets standard
(`src/standards/finite_sets_standard.rs`) also assumes `finite()` carries
information and will need agent 3's revision.

## 5. Judgement calls and items left alone

1. `lemma_set_new` (group 8, `Chap06/LabDirGraphStEph.rs:341`). The new lemma
   requires `Set::<A>::new(pred) is Some`. That is a new proof obligation at
   this site. The same file's `spec_arcs` (line 101) is a root-cause-1 site
   (`Set::new` now returns `Option`), so the enclosing proof will be rewritten
   when root cause 1 is addressed; the rename alone does not discharge the
   obligation.
2. Group 4 statements. The lemma statements in `seq_set.rs` are unchanged and
   still type-check (no error in that file in the final log). Discharging them
   at verification time now depends on two facts that were previously direct:
   `Set::empty().to_iset() == ISet::empty()` for the base case, and
   `s.insert(a).to_iset() == s.to_iset().insert(a)` for the step case. Both
   should follow from `Set::axiom_make_set` and `lemma_to_iset_finite` in
   `group_set_lemmas`, which the file already broadcasts. `is_fun_commutative`
   is not named in `seq_set.rs`, so its module move needs no edit.
3. Group 6 proofs. The two `assert(s.injective_on(proj)) by { … }` bodies
   still prove the quantifier over `s.contains`; the method unfolds to the
   same quantifier over `s.to_iset().contains`. Whether Z3 bridges
   `Set::contains` and `ISet::contains` without a hint is a verification-stage
   question not reached by this round.
4. Group 7. `lemma_set_insert_len` is a member of `group_set_lemmas`, so the
   two explicit calls in `Chap44/DocumentIndex.rs` are now redundant with the
   broadcast; they were kept as calls (rename only) to keep the edit minimal.
5. Deleted statements (groups 3, 9, 10). Each deleted call established a
   `finite()` fact that is now `true` by definition, so no proof fact was
   lost. The enclosing proof blocks all retain other statements.
6. `src/experiments/`. Ten experiment files reference removed names
   (`group_set_axioms` in 7, `axiom_spec_slice_iter`/`axiom_spec_into_iter`/
   `initial_value_relation` in 3). They are commented out in `lib.rs` and did
   not appear in any log. Not edited: experiments record what a toolchain
   version did and are not rewritten to pass.
7. Comment text. `src/standards/finite_sets_standard.rs` lines 69, 82, and
   132 mention `axiom_set_insert_finite`, `lemma_set_union_finite_iff`, and
   `seq_to_set_is_finite` in doc comments. They are prose, not code, and that
   file is agent 3's; not edited.
8. Standards files. Groups 11 and 12 edited
   `src/standards/prophetic_iterators_standard.rs` because the plan names
   those exact deletions. Its four remaining E0782 errors (section 4.2, root
   cause 6) and the old-model iterator errors in five other standards files
   are agent 3's scope.
9. Nothing was added: no `assume`, `accept`, `admit`, `external_body`,
   `requires true`, or `// veracity: no_requires`; no `ensures` weakened; no
   exec body changed except the one `broadcast use` deletion the plan names;
   no `/// - Alg Analysis` line touched.
