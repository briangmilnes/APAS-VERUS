# Chap02 to Chap06 validation on Verus 0.2026.09.13

Round r208, agent 1. Plan: `plans/r208-agent1-chap02-06.md`. Every run is
`scripts/validate.sh isolate ChapNN`; every log named below is in `logs/`.
Nothing is committed. `rtt.sh` and `ptt.sh` were not run.

## 1. Results

Measured start (`logs/validate.20260920-184251.log`, `isolate Chap02`): 690
verified, 13 errors (all `src/vstdplus/seq_set.rs`, `Set::fold` postconditions),
8 warnings (`Types.rs` 4, `seq_set.rs` 4, deprecated `finite()`).

| # | Chap | File | Errors before | After | Log |
|---|------|------|---------------|-------|-----|
| 1 | 02 | HFSchedulerMtEph.rs, FibonacciHFScheduler.rs | 13 (seq_set), 8 warnings | 620 verified, 0 errors, 0 warnings | validate.20260920-185820.log |
| 2 | 03 | InsertionSortStEph.rs, KleeneStPer.rs | 0 (blocked by seq_set) | 611 verified, 0 errors, 0 warnings | validate.20260920-185838.log |
| 3 | 05 | SetStEph, SetMtEph, RelationStEph, MappingStEph, KleeneStPer | 13 name-resolution errors (log 185851) | 747 verified, 2 errors, 0 warnings | validate.20260920-192155.log |
| 4 | 06 | 8 graph files + 13 weighted files | 14 name-resolution errors (log 192404) | 1024 verified, 2 errors, 0 warnings | validate.20260920-194725.log |

The 2 errors in rows 3 and 4 are the same two: `SetStEph::clone` and
`SetMtEph::clone` (section 6.2). They are in Chap05; `isolate Chap06` reports
them because Chap06 depends on Chap05. Every Chap06 function verifies. No
trigger notes remain in any log.

## 2. Phase 0: vstdplus

### 2.1 Modules commented out in `src/lib.rs` (`// r208:` reasons in place)

| # | Module | Reason |
|---|--------|--------|
| 1 | `VecQueue` | 0 users; vstd `std_specs/vecdeque.rs` |
| 2 | `hash_set_specs` | 0 users; only consumer was the wrapper's clone |
| 3 | `partial_order` | 0 users in src |
| 4 | `seq_set_pre_0913` | the renamed old `seq_set.rs`, kept as record |

`sqrt` was left in (plan said 0 users): its `assume_specification` for
`usize::isqrt` is used without an import by `src/Chap21/Algorithm21_6.rs:85`
and `src/Chap21/Exercise21_8.rs:187`. Needs-discussion item 3.

`hash_set_with_view_plus`, `hash_map_with_view_plus`, `arc_rwlock` were
already commented out in r207. The two wrapper files and
`seq_set_pre_0913.rs` show r207's uncommitted `group_set_axioms ->
group_set_lemmas` renames in `git diff`; this round did not edit them.

### 2.2 `seq_set.rs` replaced

`git mv src/vstdplus/seq_set.rs src/vstdplus/seq_set_pre_0913.rs`; new
`src/vstdplus/seq_set.rs` (588 lines vs 1247), same module path, same public
names for everything Chap02 to Chap06 use. Broadcast: `vstd::seq::group_seq_lemmas`,
`vstd::seq_lib::group_seq_properties`, `vstd::set::group_set_lemmas`,
`vstd::iset::group_iset_lemmas`, `Seq::to_set_ensures`. No `finite()` anywhere.

| # | Item | Status |
|---|------|--------|
| 1 | 24 `spec_[signed_]weighted_{seq,set}_sum*` spec fns | copied |
| 2 | `lemma_seq_index_in_map_to_set`, `lemma_map_to_set_contains_index` | copied |
| 3 | `lemma_take_one_more_extends_the_seq_set_with_view` | reproved via `lemma_push_to_set_commute` |
| 4 | `lemma_seq_map_to_set_equality`, `lemma_take_one_more_intersect` | copied |
| 5 | 18 `lemma_[signed_]seq_fold_left_plus_is_weighted_seq_sum*`, `lemma_fold_left_int_equals_nat_as_int*` | copied |
| 6 | 12 `lemma_[signed_]weighted_seq_fold_equals_set_fold*` | reproved on `Set::fold` (inline `to_iset().fold`) |
| 7 | new private `lemma_no_dup_seq_fold_left_is_set_fold<A, B>` | generic bridge used by row 6 |

Row 7 states `seq.no_duplicates() && is_fun_commutative(f) ==>
seq.fold_left(z, f) == seq.to_set().fold(z, f)`, by induction on `drop_last`
with `vstd::iset::fold::{lemma_fold_empty, lemma_fold_insert}`.

Not carried over (no user in Chap02 to Chap06): `lemma_int_fold_equals_nat_fold_weighted`,
`lemma_map_not_contains_implies_all_ne` (used by `src/Chap17/MathSeq.rs`),
`lemma_nat_any_order_no_overflow`, `lemma_nat_fold_left_step`,
`lemma_nat_partial_sum_monotonic`, `lemma_no_dup_same_set_implies_same_multiset`,
`lemma_push_not_contains_to_set` (used by `src/Chap41/ArraySetStEph.rs`),
`lemma_push_not_contains_to_set_subset`, `lemma_push_not_contains_to_set_superset`,
`lemma_seq_map_to_set_eq_set_map`, `lemma_set_contains_iff_to_seq_map_contains`,
`lemma_set_contains_insert_idempotent`, `lemma_spec_nat_seq_fold_equals_spec_set_fold`,
`lemma_spec_nat_seq_sum_is_nat_set_sum`, `lemma_spec_nat_seq_sum_no_intermediate_overflow`,
`lemma_spec_nat_seq_sum_permutation_invariant`, `lemma_take_extends_set_subset`,
`lemma_take_extends_set_superset`, `lemma_take_full_to_set_with_view`,
`lemma_take_one_more_extends_the_seq_set`, `lemma_to_seq_gives_same_set`,
`lemma_to_seq_no_duplicates`, `lemma_u32_view_identity`, `lemma_weighted_fold_left_step`,
`lemma_weighted_seq_sum_is_set_sum`, `spec_nat_seq_sum`, `spec_nat_set_sum`.
Chap17 and Chap41 will need the two named lemmas restored when their rounds run
(Needs-discussion item 8).

### 2.3 `finite()` sites deleted

| # | Chap | File | Line | Old | New |
|---|------|------|------|-----|-----|
| 1 | - | Types.rs | 82-83 | `&&& gv.V.finite()` / `&&& gv.A.finite()` in `spec_graphview_wf` | deleted |
| 2 | - | Types.rs | 105-106 | same two conjuncts in `spec_labgraphview_wf` | deleted |
| 3 | - | Types.rs | 80, 103 | doc "finite sets and arc endpoints in V" | "arc endpoints in V" |
| 4 | - | seq_set.rs | 491, 524, 712, 785 | `requires s.finite()` | gone with the file |

### 2.4 RTT and PTT entries commented out

| # | File | Entry | Reason |
|---|------|-------|--------|
| 1 | Cargo.toml | `test_partial_order` | `partial_order` commented out |
| 2 | Cargo.toml | `TestStarContractionMtEph`, `TestStarContractionStEph` | import `hash_map_with_view_plus` |
| 3 | Cargo.toml | `TestStarPartitionMtEph`, `TestStarPartitionStEph` | same |
| 4 | Cargo.toml | `TestConnectivityMtEph`, `TestConnectivityStEph` | same |
| 5 | Cargo.toml | `TestBoruvkaMtEph`, `TestBoruvkaStEph` | same |
| 6 | rust_verify_test/Cargo.toml | `HashSetWithViewPlus`, `HashMapWithViewPlus` | modules commented out |

`tests/Chap11/TestFibonacciMt.rs` matched the grep on `f64.sqrt()` only; left alone.
No test file was deleted.

## 3. Chapter 2

No source edit. `HFSchedulerMtEph.rs` keeps its `Mutex`, `Condvar`, thread
spawn and `panic!`. 620 verified, 0 errors, 0 warnings.

## 4. Chapter 3

`InsertionSortStEph.rs`: no edit. `KleeneStPer.rs`:

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 03 | KleeneStPer.rs | 40 | `vstd::set::group_set_axioms` | `vstd::set::group_set_lemmas` | r207 rename (pre-existing) |
| 2 | 03 | KleeneStPer.rs | 184 | `requires ..., alphabet@.finite()` | `requires Self::spec_valid_key_type()` | finite by type |
| 3 | 03 | KleeneStPer.rs | 212 | wf `self@.finite() && valid_key_type::<T>()` | `valid_key_type::<T>()` | finite by type |
| 4 | 03 | KleeneStPer.rs | 229 | loop invariant `self.alphabet@.finite()` | deleted | finite by type |

611 verified, 0 errors, 0 warnings.

## 5. Chapter 5

Baseline `logs/validate.20260920-185851.log`: 13 errors, all
`HashSetWithViewPlus`/`HashSetWithViewPlusIter` name resolution. Intermediate:
`191838` (into_iter requires, `Hash::hash` unsupported), `192116` (747 verified,
2 errors, 6 trigger notes), final `192155` (747 verified, 2 errors, no notes).

### 5.1 `SetStEph.rs` (rewritten; line numbers are the new file)

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 05 | SetStEph.rs | 28-52 | imports of `hash_set_with_view_plus::*` | `std::collections::HashSet`, `vstd::std_specs::iter::*`, cfg-gated `vstd::std_specs::hash::{obeys_key_model, into_iter_hash_keys}` | wrapper gone |
| 2 | 05 | SetStEph.rs | 57-66 | broadcast `group_hash_set_with_view_plus_axioms` | `vstd::std_specs::hash::group_hash_axioms` | wrapper gone |
| 3 | 05 | SetStEph.rs | 81 | `pub elements: HashSetWithViewPlus<T>` | `pub elements: HashSet<T>` | rule 4(b) |
| 4 | 05 | SetStEph.rs | 89 | `view = self.elements@` | `self.elements@.map(\|x: T\| x@)` | raw HashSet views to `Set<T>` |
| 5 | 05 | SetStEph.rs | 102-104 | `spec_setsteph_wf_generic`: `s@.finite() && valid_key_type::<V>()` | `valid_key_type::<V>()` | finite by type |
| 6 | 05 | SetStEph.rs | 110-118 | `lemma_singleton_choose` `requires s.finite()` | requires dropped | finite by type |
| 7 | 05 | SetStEph.rs | 124-207 | none | new section-7 lemmas `lemma_viewed_contains`, `lemma_viewed_mem`, `lemma_viewed_insert`, `lemma_viewed_empty`, `lemma_viewed_len`, `lemma_iter_keys_view` | bridge `Set<T>.map(view)` to the trait specs |
| 8 | 05 | SetStEph.rs | 225-233 | `fn iter -> SetStEphIter`, ensures `it@.0 == 0`, `it@.1...` | `-> std::collections::hash_set::Iter<'a, T>`, ensures below | rule 3 |
| 9 | 05 | SetStEph.rs | 399-401 | wf `self@.finite() && valid_key_type::<T>()` | `valid_key_type::<T>() && obeys_feq_full::<T>()` | finite by type; feq needed for `lemma_viewed_mem` |
| 10 | 05 | SetStEph.rs | 427-431 | `iter` body wrapped inner iter | `let it = self.elements.iter(); proof { lemma_iter_keys_view(..) } it` | rule 4(a) |
| 11 | 05 | SetStEph.rs | 454-456, 461-466, 473, 479, 486-489, 523 | none | proof calls to the viewed lemmas in `empty`, `singleton`, `size`, `mem`, `insert`, `disjoint_union` | view is now a map |
| 12 | 05 | SetStEph.rs | for-loops in `from_vec`, `union`, `intersection`, `difference`, `filter`, `split`, `cartesian_product`, `is_subset`, `is_disjoint` | invariants on `iter.pos`, `iter.elements` | `let ghost s_seq = into_iter_hash_keys(it); for x in iter: it invariant iter.seq().unref() == s_seq, ... take(iter.index()) ...` | rule 3 |
| 13 | 05 | SetStEph.rs | 661-694, 699-763, 766-814 | manual loops on `SetStEphIter` (`all_nonempty`, `partition_on_elt`, `partition`) | `VerusForLoopWrapper::new(iter)` with invariants `it.wf()`, `obeys_prophetic_iter_laws(&it.iter)`, `decrease(&it.iter) is Some`, `it.seq().unref() == seq`; `decreases IteratorSpec::decrease(&it.iter)->0` | rule 3 |
| 14 | 05 | SetStEph.rs | 860-890 | `choose` read `it@.1[0]` | asserts `remaining(&it).len() > 0` then `s[0] == *element_ref` | rule 3 |
| 15 | 05 | SetStEph.rs | 905-925 | `IntoIterator` with `requires valid_key_type::<T>()`, `IntoIter = SetStEphIter` | `IntoIter = hash_set::Iter`; no requires; `ensures self.spec_setsteph_wf() ==> {...}` | Verus rejects `requires` on an external trait impl |
| 16 | 05 | SetStEph.rs | 939-941 | `Clone` ensures `clone@.finite(), clone@ == self@` | `clone@ == self@` | finite by type; FAILS (6.2) |
| 17 | 05 | SetStEph.rs | 944-958 | `Hash::hash` body `self.elements.hash(state)` | `#[verifier::external_body]` loop over `self.elements.iter()` calling `key.hash(state)` | `HashSet` has no `Hash`; see 6.1 |
| 18 | 05 | SetStEph.rs | 802-905 (old) | `SetStEphIter`, `View`, `iter_invariant`, `Iterator`, `SetStEphGhostIterator`, both `ForLoopGhostIterator*` impls, their Debug/Display | deleted | rule 3 |

New `iter` ensures (row 8), also the shape used by every Chap06 `iter_*`:

```rust
IteratorSpec::remaining(&it).unref().map(|i: int, k: T| k@).to_set() == self@,
IteratorSpec::remaining(&it).unref().no_duplicates(),
IteratorSpec::remaining(&it).len() == self@.len(),
forall |j: int| 0 <= j < IteratorSpec::remaining(&it).len() ==> self@.contains(#[trigger] IteratorSpec::remaining(&it).unref()[j]@),
into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref(),
IteratorSpec::decrease(&it) is Some,
```

### 5.2 `SetMtEph.rs` (rewritten; same rows as 5.1 at its own lines, plus)

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 05 | SetMtEph.rs | 62-64 | three `hash_set_with_view_plus` imports | removed | wrapper gone |
| 2 | 05 | SetMtEph.rs | 147-149 (old) | `open spec fn spec_finite(&self)` | deleted | finite by type |
| 3 | 05 | SetMtEph.rs | 582, 608, 640, 641, 649 (old) | `ret@.finite()`, `r@.finite()`, `product@.finite()`, `joined_views.finite()` in `cartesian_product` | deleted | finite by type |
| 4 | 05 | SetMtEph.rs | 630-680 | `cartesian_product` phase-1 loop on `SetMtEphIter` | `VerusForLoopWrapper` over `s1_iter`, `it_seq = into_iter_hash_keys(s1_iter)` | rule 3 |
| 5 | 05 | SetMtEph.rs | 1092, 1105 (old) | `LockedSetMtEphTrait::empty` `ensures s@.finite(), ...`; `insert` `ensures self@.finite(), ...` | conjuncts dropped | finite by type |
| 6 | 05 | SetMtEph.rs | 1116-1120 | `type_invariant wf: self.ghost_locked_set@.finite()` | `true` | finite by type; item 6 |
| 7 | 05 | SetMtEph.rs | 1180-1184 | `SetMtEphInv::inv: v@.finite() && valid_key_type::<T>()` | `valid_key_type::<T>()` | finite by type |
| 8 | 05 | SetMtEph.rs | 1203-1205 | `Clone` ensures with `finite()` | `clone@ == self@` | FAILS (6.2) |
| 9 | 05 | SetMtEph.rs | 1210-1222 | `Hash::hash` | as SetStEph row 17 | see 6.1 |

The lock-boundary `accept`s stay as they were.

### 5.3 `RelationStEph.rs` (rewritten)

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 05 | RelationStEph.rs | 99-101 (old) | `spec_finite` | deleted | finite by type |
| 2 | 05 | RelationStEph.rs | 127 | `ensures domain@.finite(), domain@ == Set::<X::V>::new(\|x\| exists \|y\| self@.contains((x, y)))` | `ensures domain@ == self@.map(\|p: (X::V, Y::V)\| p.0)` | `Set::new` returns `Option`; item 5 |
| 3 | 05 | RelationStEph.rs | 134 | `ensures range@.finite(), range@ == Set::<Y::V>::new(\|y\| exists \|x\| self@.contains((x, y)))` | `ensures range@ == self@.map(\|p: (X::V, Y::V)\| p.1)` | same |
| 4 | 05 | RelationStEph.rs | 150 | `fn iter -> RelationStEphIter` | `-> hash_set::Iter<'a, Pair<X, Y>>`, body `self.pairs.iter()` | rule 3 |
| 5 | 05 | RelationStEph.rs | 166 | wf `self.pairs@.finite()` conjunct | `valid_key_type_Pair::<X, Y>()` | finite by type |
| 6 | 05 | RelationStEph.rs | 185-240 | `domain`/`range` loops, invariants on `iter.pos` | for-loop invariant `out@ == pairs_seq.take(iter.index()).map(..).to_set().map(\|p\| p.0)`; proof `lemma_take_one_more_extends_the_seq_set_with_view` + `lemma_set_map_insert_commute` | rule 3 |
| 7 | 05 | RelationStEph.rs | 264-285 | `IntoIterator` with requires, custom iter | delegating impl, conditional ensures | as 5.1 row 15 |
| 8 | 05 | RelationStEph.rs | 292 | `Clone` ensures `clone@.finite(), ...` | `clone@ == self@` (verifies through `SetStEph::clone`'s ensures; the field is `SetStEph<Pair<X, Y>>`) | finite by type |
| 9 | 05 | RelationStEph.rs | 300-425 (old) | iterator structs and ghost machinery | deleted | rule 3 |

### 5.4 `MappingStEph.rs` (rewritten)

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 05 | MappingStEph.rs | 96-101 | `Map::new(\|x\| exists \|y\| self.mapping@.contains((x, y)), \|x\| choose ...)` | `Map::new(self.mapping@.map(\|p: (A::V, B::V)\| p.0), \|x: A::V\| choose \|y: B::V\| self.mapping@.contains((x, y)))` | `Map::new(dom: Set<K>, fv)` at 09.13 |
| 2 | 05 | MappingStEph.rs | 134-171 | none | new `lemma_view_contains_pair(m, x, y)`: `m.mapping@.contains((x, y)) <==> (m@.dom().contains(x) && m@[x] == y)` under `is_functional_set`; `lemma_view_kv_pairs(m)`: `m@.kv_pairs() == m.mapping@` | bridge the new view |
| 3 | 05 | MappingStEph.rs | 137-139 (old) | `spec_finite` | deleted | finite by type |
| 4 | 05 | MappingStEph.rs | 250 | `ensures domain@.finite(), domain@ == self@.dom()` | `domain@ == self@.dom()` | finite by type |
| 5 | 05 | MappingStEph.rs | 256-259 | `ensures range@.finite(), range@ =~= Set::new(\|y\| exists \|x\| ... self@[x] == y), range@ == self@.values()` | `ensures range@ == self@.values()` | `Set::new` returns `Option`; the dropped conjunct is `values()` by definition (`Map::values() = dom().map(\|k\| self[k])`); item 5 |
| 6 | 05 | MappingStEph.rs | 273-280 | `iter` ensures `it@.1.map(..).to_set() == Set::new(\|p\| self@.dom().contains(p.0) && self@[p.0] == p.1)` | `remaining(&it).unref().map(..).to_set() == self@.kv_pairs()` (vstd `kv_pairs() = dom().map(\|k\| (k, self[k]))`), plus the standard triple | same |
| 7 | 05 | MappingStEph.rs | 288 | wf `self.mapping@.finite()` conjunct | dropped | finite by type |
| 8 | 05 | MappingStEph.rs | 340-420 | `is_functional_SetStEph_at`, `is_functional_SetStEph` manual loops | `VerusForLoopWrapper` | rule 3 |
| 9 | 05 | MappingStEph.rs | 460-551 | proofs in `mem`, `from_vec`, `from_relation`, `range`, `iter`, `empty` | use `lemma_view_contains_pair` / `lemma_view_kv_pairs`; `empty` asserts `result@.dom() =~= Set::empty()` and `result@ =~= Map::empty()` | new view |
| 10 | 05 | MappingStEph.rs | 562-580 | `IntoIterator` with requires | delegating impl, conditional ensures | as 5.1 row 15 |
| 11 | 05 | MappingStEph.rs | 679 (old) | `MappingLit!` used `HashSetWithViewPlus::new()` | `std::collections::HashSet::new()` | wrapper gone |
| 12 | 05 | MappingStEph.rs | 508-609 (old) | iterator structs and ghost machinery | deleted | rule 3 |

### 5.5 `KleeneStPer.rs`

See section 4; verified unchanged otherwise.

### 5.6 Errors fixed on the way

| # | Log | Error | Fix |
|---|-----|-------|-----|
| 1 | 191838 | "trait method implementation cannot declare requires clauses" on four `IntoIterator::into_iter` | requires removed, ensures made conditional on wf, bodies call the field iterator |
| 2 | 191838 | "`core::hash::Hash::hash` is not supported" in `SetStEph::hash` | relocated the wrapper's `external_body` body (6.1) |
| 3 | 192116 | 6 "automatically chose triggers" notes on `choose` | explicit `#[trigger]` on `s.contains(a)`, `r.contains(q)`, `m.contains(p)`, `self@.dom().contains(x)`, `m@.dom().contains(k)` |

`tests/Chap05/*.rs` reference no removed item (`grep` for the iterator types,
`.elements`, and the wrapper found nothing); they use the trait API. Not run.

## 6. Chapter 6

Baseline `logs/validate.20260920-192404.log`: 14 errors (4 broadcast lines
naming `hash_set_with_view_plus`, 10 `SetStEphIter` name resolutions).
Intermediate logs: `192925` (5 errors: 3 arc-loop invariants), `192954`
(Dir/UnDir St clean), `193334` (four St files clean), `193610` (Dir/UnDir Mt
clean), `193949` (6 errors: 4 `add_*` postconditions in the Lab Mt files),
`194151` (Lab Mt + U32 clean), final `194725` (1024 verified, 2 Chap05 errors).

During the conversion the not-yet-converted Chap06 modules were temporarily
commented out in `src/lib.rs` (marked `r208-temp`) so each step could reach
Z3; `src/lib.rs` was restored and `diff` against the saved copy
`scratch/lib.rs.r208-chap06-backup` is empty.

### 6.1 Edits common to the eight graph files

| # | Edit | Old | New |
|---|------|-----|-----|
| 1 | imports | none | `use vstd::std_specs::iter::*;` and cfg-gated `use vstd::std_specs::hash::into_iter_hash_keys;` |
| 2 | broadcast (4 St files) | `crate::vstdplus::hash_set_with_view_plus::...::group_hash_set_with_view_plus_axioms,` | line deleted |
| 3 | manual loop header | `let mut it = X.iter(); let ghost s = it@.1;` | `let s_iter = X.iter(); let ghost s = into_iter_hash_keys(s_iter); let mut it = VerusForLoopWrapper::new(s_iter);` |
| 4 | loop invariants | `it@.0 <= s.len(), it@.1 == s,` | `it.wf(), IteratorSpec::obeys_prophetic_iter_laws(&it.iter), IteratorSpec::decrease(&it.iter) is Some, it.seq().unref() == s,` |
| 5 | loop bound | `decreases s.len() - it@.0,` | `decreases IteratorSpec::decrease(&it.iter)->0,` |
| 6 | index | `it@.0` in invariants, `it@.0 - 1` after `next()` | `it.index()`; `let ghost old_pos = it.index();` before `match it.next()` and `old_pos` after |
| 7 | `Some(x)` arm | none | `proof { assert(s[old_pos] == *x); }` (links the returned reference to the ghost sequence through `unref`) |
| 8 | accumulator invariants | `acc@ == Set::new(\|w\| exists \|i\| 0 <= i < it@.0 && ...)` | `forall \|w\| #[trigger] acc@.contains(w) <==> exists \|i\| #![trigger s[i]] 0 <= i < it.index() && ...` (same content, no `Set::new`) |
| 9 | `None` arm | membership proofs both ways | same proofs plus `assert(acc@ =~= spec)` to close the set equality |
| 10 | section 10 | `XxxIter`, `View`, `iter_invariant`, `Iterator`, `XxxGhostIterator`, both `ForLoopGhostIterator*` impls, `IntoIterator` with `requires` | one delegating `IntoIterator` (`IntoIter = hash_set::Iter<'a, V>`, body `(&self.V).into_iter()`, `ensures self.V.spec_setsteph_wf() ==> {standard triple}`) |
| 11 | section 14 | Debug/Display for the deleted iterator structs | deleted |

Row 8 is the one place the plan's "filter or map" wording was not followed:
loop-accumulator invariants index a sequence, not a set, so a membership
`forall` states the same predicate without `Set::new`.

### 6.2 `Set::new` rewrites in spec fns (old and new text)

Every `Set::new(|w| ...)` whose `w` ranges over vertices became
`self@.V.filter(|w| ...)`; under `spec_graphview_wf`/`spec_labgraphview_wf`
(arc endpoints in V) the two are equal. Projections of `A` became `A.map`.

| # | Chap | File | Old line | New line | Old | New |
|---|------|------|----------|----------|-----|-----|
| 1 | 06 | DirGraphStEph.rs | 86 | 88 | `Set::new(\|w: V::V\| self@.A.contains((v, w)))` | `self@.V.filter(\|w: V::V\| self@.A.contains((v, w)))` |
| 2 | 06 | DirGraphStEph.rs | 90 | 92 | `Set::new(\|u: V::V\| self@.A.contains((u, v)))` | `self@.V.filter(\|u: V::V\| self@.A.contains((u, v)))` |
| 3 | 06 | DirGraphStEph.rs | 103 | 105 | `Set::new(\|w\| exists \|u\| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))` | `self@.V.filter(` same body `)` |
| 4 | 06 | DirGraphStEph.rs | 109 | 111 | same with `spec_n_minus` | `self@.V.filter(...)` |
| 5 | 06 | DirGraphStEph.rs | 115 | 117 | same with `spec_ng` | `self@.V.filter(...)` |
| 6 | 06 | UnDirGraphStEph.rs | 89 | 91 | `Set::new(\|w\| self@.A.contains((v, w)) \|\| self@.A.contains((w, v)))` | `self@.V.filter(...)` |
| 7 | 06 | UnDirGraphStEph.rs | 95 | 97 | `Set::new(\|w\| exists \|u\| ... spec_ng(u).contains(w))` | `self@.V.filter(...)` |
| 8 | 06 | LabDirGraphStEph.rs | 91 | 89 | `Set::new(\|w\| exists \|l\| #![trigger self@.A.contains((v, w, l))] self@.A.contains((v, w, l)))` | `self@.V.filter(...)` |
| 9 | 06 | LabDirGraphStEph.rs | 97 | 95 | same with `(u, v, l)` | `self@.V.filter(...)` |
| 10 | 06 | LabDirGraphStEph.rs | 101 | 99 | `spec_arcs = Set::new(\|e: (V::V, V::V)\| exists \|l\| self@.A.contains((e.0, e.1, l)))` | `self@.A.map(\|e: (V::V, V::V, L::V)\| (e.0, e.1))` |
| 11 | 06 | LabUnDirGraphStEph.rs | 91 | 89 | `Set::new(\|w\| exists \|l\| A.contains((v, w, l)) \|\| A.contains((w, v, l)))` (no trigger) | `self@.V.filter(\|w\| exists \|l\| #![trigger self@.A.contains((v, w, l))] #![trigger self@.A.contains((w, v, l))] ...)` |
| 12 | 06 | LabUnDirGraphStEph.rs | 96 | 96 | `spec_edges = Set::new(...)` as row 10 | `self@.A.map(\|e\| (e.0, e.1))` |
| 13 | 06 | DirGraphMtEph.rs | 179, 214, 898, 902 | 180, 215, 801, 805 | `spec_n_plus`/`spec_n_minus` in both traits, as rows 1-2 | `self@.V.filter(...)` |
| 14 | 06 | DirGraphMtEph.rs | 187, 222 | 188, 223 | `spec_n_plus_from_set = Set::new(\|w\| subarcs.contains((v, w)))`, `spec_n_minus_from_set` | `self@.V.filter(\|w\| subarcs.contains((v, w)))`, likewise |
| 15 | 06 | DirGraphMtEph.rs | 282, 288, 306, 312, 330, 336, 910, 914, 918 | 283, 289, 307, 313, 331, 337, 813, 817, 821 | the six `*_of_vertices[_from_set]` and the three Locked-trait copies, as rows 3-5 | `self@.V.filter(...)` |
| 16 | 06 | DirGraphMtEph.rs | 968, 978 | 869, 879 | Locked `n_plus` ensures `out_neighbors@ == Set::new(\|w\| self@.A.contains((v@, w)))`; `n_minus` likewise | `out_neighbors@ == self.spec_n_plus(v@)`; `in_neighbors@ == self.spec_n_minus(v@)` (the trait's own spec fns, same predicate) |
| 17 | 06 | UnDirGraphMtEph.rs | 123, 129, 191, 209, 606, 610 | 126, 132, 192, 210, 509, 513 | `spec_ng_from_set`, `spec_ng_of_vertices_from_set`, `spec_ng`, `spec_ng_of_vertices`, Locked copies | `self@.V.filter(...)` |
| 18 | 06 | LabDirGraphMtEph.rs | 117 | 116 | `spec_arcs = Set::new(...)` | `self@.A.map(\|e\| (e.0, e.1))` |
| 19 | 06 | LabDirGraphMtEph.rs | 193, 201, 207, 215 | 190, 198, 204, 212 | `spec_n_plus`, `_from_set`, `spec_n_minus`, `_from_set`: `Set::new(\|w\| exists \|l\| X.contains((v, w, l)))` (no trigger) | `self@.V.filter(\|w\| exists \|l\| #![trigger X.contains((v, w, l))] X.contains((v, w, l)))` |
| 20 | 06 | LabDirGraphMtEph.rs | 797, 801 | 732, 736 | Locked-trait `spec_n_plus`, `spec_n_minus` | `self@.V.filter(...)` |
| 21 | 06 | LabUnDirGraphMtEph.rs | 117 | 116 | `spec_edges = Set::new(...)` | `self@.A.map(\|e\| (e.0, e.1))` |
| 22 | 06 | LabUnDirGraphMtEph.rs | 125, 208, 741 | 124, 208, 692 | `spec_ng_from_set`, `spec_ng`, Locked `spec_ng`: `Set::new(\|w\| exists \|l\| X.contains((v, w, l)) \|\| X.contains((w, v, l)))` | `self@.V.filter(\|w\| exists \|l\| #![trigger X.contains((v, w, l))] #![trigger X.contains((w, v, l))] ...)` |

Total: 59 `Set::new` sites (44 spec fns and ensures above, 15
loop-accumulator invariants under 6.1 row 8). Zero remain in Chap06.

### 6.3 `finite()` and `spec_finite` deletions

| # | Chap | File | Old line | Old | New |
|---|------|------|----------|-----|-----|
| 1 | 06 | LabDirGraphStEph.rs | 84-86 | `open spec fn spec_finite(&self) -> bool { self@.V.finite() && self@.A.finite() }` | deleted |
| 2 | 06 | LabDirGraphStEph.rs | 141, 176, 183 | `ensures arcs@.finite(), ...`; `n_plus@.finite(), ...`; `n_minus@.finite(), ...` | conjunct dropped |
| 3 | 06 | LabUnDirGraphStEph.rs | 84-86 | `spec_finite` | deleted |
| 4 | 06 | DirGraphMtEph.rs | 128-129, 924-925 | `V@.finite(), A@.finite(),` in `from_sets` and Locked `new` requires | deleted |
| 5 | 06 | UnDirGraphMtEph.rs | 146-147, 616-617 | `V@.finite(), E@.finite(),` | deleted |
| 6 | 06 | LabDirGraphMtEph.rs | 109-111, 135-136, 159, 228, 242, 807-808 | `spec_finite`; `vertices@.finite(), labeled_arcs@.finite()` ×2; `arcs@.finite()`; `n_plus@.finite()`; `n_minus@.finite()` | deleted |
| 7 | 06 | LabUnDirGraphMtEph.rs | 109-111, 143-144 | `spec_finite`; `vertices@.finite(), labeled_edges@.finite()` | deleted |
| 8 | 06 | 13 × WeightedDirGraphStEph*.rs | `from_weighed_edges` requires | `edges@.finite(),` | deleted |

### 6.4 Per-file specifics beyond 6.1 to 6.3

| # | Chap | File | Line | Old | New | Reason |
|---|------|------|------|-----|-----|--------|
| 1 | 06 | DirGraphStEph.rs | 233-259 | inherent `iter_vertices`/`iter_arcs -> SetStEphIter`, `ensures true` | `-> hash_set::Iter<'_, V>` / `<'_, Edge<V>>` with the standard iter ensures | rule 3 |
| 2 | 06 | DirGraphStEph.rs | 5 loops | `it@`-model | 6.1 | rule 3 |
| 3 | 06 | UnDirGraphStEph.rs | 314 | none | `ng_of_vertices` invariant `neighbors@ <= self@.V` | `spec_ng_of_vertices` now filters V |
| 4 | 06 | LabDirGraphStEph.rs | 339-344 (old) | `has_arc` None arm called `vstd::set::lemma_set_new(pred, ...)` | `if exists \|l\| la_view.contains((from, to, l)) { choose l; lemma_map_to_set_contains_index(la_seq, (from, to, l)); }` | `lemma_set_new` is about `Option<Set>` now |
| 5 | 06 | LabDirGraphStEph.rs | `get_arc_label` None arm | empty proof block | same proof as row 4 | keeps the exists ensures provable |
| 6 | 06 | LabDirGraphStEph.rs | `arcs` None arm | proved `exists l` form | proves `arcs@.contains(e) <==> self.spec_arcs().contains(e)` through `lemma_map_contains` (choose `a` with `la_view.contains(a) && e == (a.0, a.1)`) | `spec_arcs` is now `A.map` |
| 7 | 06 | LabDirGraphStEph.rs | `n_plus`/`n_minus` None arms | as before | plus `assert(la_view.contains((v, w, la_seq[i]@.2)))` to supply the label witness | `V.filter` needs `V.contains(w)`, from wf |
| 8 | 06 | LabUnDirGraphStEph.rs | `get_edge_label`/`has_edge` None arms | empty proof blocks | case split on which orientation the chosen label sits in, then `lemma_map_to_set_contains_index` | as row 4 |
| 9 | 06 | LabUnDirGraphStEph.rs | `ng`, `edges` None arms | as before | label-witness asserts as row 7 | as row 7 |
| 10 | 06 | LabDirGraphMtEph.rs | 360-382 | `add_vertex`, `add_labeled_arc` bodies unchanged | proof block after the inserts: `assert forall \|u, w, l\| #[trigger] self@.A.contains((u, w, l)) implies self@.V.contains(u) && self@.V.contains(w) by { ... old(self)@ ... }` | the `spec_labgraphview_wf(self@)` postcondition no longer closed automatically (log 193949) |
| 11 | 06 | LabUnDirGraphMtEph.rs | 334-360 | `add_vertex`, `add_labeled_edge` | same proof blocks | same |
| 12 | 06 | LabDirGraphMtEph.rs, LabUnDirGraphMtEph.rs | `arcs`/`edges`, `get_*_label`, `has_*` | `it@`-model loops, `lemma_set_new`-free | 6.1 rows 3-9, and rows 4-9 of this table | rule 3 |
| 13 | 06 | 13 × WeightedDirGraphStEph*.rs | 7 loops each (4 in F64) | `it@`-model | 6.1 rows 1, 3-7; `take(it@.0 as int)` -> `take(it.index())`, `take((it@.0 - 1) as int)` -> `take(old_pos)` in `total_weight` | rule 3 |

The lock-boundary `accept`s and the two `assume`s in the Locked `add_*` bodies
of the Lab Mt files stay as they were. No `/// - Alg Analysis` line changed.

## 7. Exec changes beyond rule 4

| # | Chap | File | Function | Before | After | Work/span before | Work/span after |
|---|------|------|----------|--------|-------|------------------|-----------------|
| 1 | 05 | SetStEph.rs | `Hash::hash` | `self.elements.hash(state)` (called the wrapper's `external_body` loop) | the same loop, inlined, `external_body` | O(n), O(n) | O(n), O(n) |
| 2 | 05 | SetMtEph.rs | `Hash::hash` | same | same | O(n), O(n) | O(n), O(n) |

Everything else is a rule-4 substitution, a spec, or a proof. Needs-discussion
item 1 covers rows 1-2.

## 8. What is left

| # | Item | Detail |
|---|------|--------|
| 1 | 2 errors | `SetStEph::clone`, `SetMtEph::clone`; text in 6.2 below |
| 2 | Holes untouched | every `accept`/`assume` in Chap05/Chap06 lock boundaries and `PartialEq::eq` bodies, the `external_body` `Hash::hash` (relocated, not new trust) |
| 3 | RTTs not run | `tests/Chap05`, `tests/Chap06` read: no reference to removed items; 8 Chap62/63/66 entries and `test_partial_order` commented out in Cargo.toml |
| 4 | PTTs not run | two vstdplus entries commented out |
| 5 | Later chapters | Chap17 and Chap41 use two seq_set lemmas not carried over (2.2) |
| 6 | Full validate | not run (isolate only, per plan) |

Remaining error text (`logs/validate.20260920-194725.log`):

```
error: postcondition not satisfied
   --> src/Chap05/SetStEph.rs:940:21
    |
940 |             ensures clone@ == self@
    |                     ^^^^^^^^^^^^^^^ failed this postcondition
941 |         { SetStEph { elements: self.elements.clone() } }
    |           -------------------------------------------- at the end of the function body

error: postcondition not satisfied
    --> src/Chap05/SetMtEph.rs:1204:21
     |
1204 |             ensures clone@ == self@
     |                     ^^^^^^^^^^^^^^^ failed this postcondition
1205 |         { SetMtEph { elements: self.elements.clone() } }
     |           -------------------------------------------- at the end of the function body
```

## 9. Needs discussion

1. `Hash::hash` in `src/Chap05/SetStEph.rs:944` and `SetMtEph.rs:1210`.
   `std::collections::HashSet` implements no `Hash`, and Verus 0.2026.09.13
   rejects `key.hash(state)` on a generic `T` ("`core::hash::Hash::hash` is
   not supported"). The old body delegated to `HashSetWithViewPlus::hash`,
   which was `#[verifier::external_body]` with exactly this loop. I moved that
   body and its attribute into the two impls: the trust boundary is unchanged
   in content but is now in Chap05 rather than vstdplus, and the plan says
   never add `external_body`. Alternatives: `#[verifier::external]` on the
   impl, moving the impl outside `verus!` (section 14), or dropping `Hash`
   from `SetStEph` (only `SetStEph<SetStEph<T>>` in `partition` needs it).
   An earlier draft used `#[verifier::exec_allows_no_decreases_clause]`; that
   was replaced because the generic `hash` call is rejected regardless.
2. `Clone` for `SetStEph`/`SetMtEph` cannot be proved: vstd has an
   `assume_specification` for `HashMap::clone` (`std_specs/hash.rs:578`) but
   none for `HashSet::clone`, so `self.elements.clone()` has no spec. The
   permitted clone-body `assume` is forbidden this round. Options: the
   clone-body `assume` pattern, an `assume_specification` for
   `HashSet::clone` in vstdplus (trusted), or a cloning loop over `iter()`
   with `insert` (O(n) either way, but an exec-body change under rule 9).
3. `sqrt` left in `src/lib.rs`: `usize::isqrt` is used by
   `src/Chap21/Algorithm21_6.rs:85` and `src/Chap21/Exercise21_8.rs:187`
   without importing the module; commenting it out would break Chap21.
4. Four `IntoIterator::into_iter` impls (`SetStEph`, `SetMtEph`,
   `RelationStEph`, `MappingStEph`) and eight in Chap06 lost their
   `requires` (Verus: "trait method implementation cannot declare requires
   clauses") and now carry `ensures wf ==> {...}`. Callers that iterate a
   non-wf set get no facts instead of a precondition failure.
5. Restated ensures (same predicate, different vstd primitive):
   `RelationStEph::domain`/`range` (`Set::new` -> `self@.map`),
   `MappingStEph::range` (dropped the `Set::new` conjunct that duplicated
   `self@.values()`), `MappingStEph::iter`/`into_iter` (`Set::new` ->
   `self@.kv_pairs()`), `LockedDirGraphMtEphTrait::n_plus`/`n_minus`
   (`Set::new` -> `self.spec_n_plus(v@)`/`spec_n_minus`), and the 44
   `filter`/`map` rewrites in 6.2. The `V.filter` forms are equal to the old
   comprehensions only under the graph wf, which every caller requires.
6. `LockedSetMtEph::wf` type invariant (`SetMtEph.rs:1116`) is now `true`:
   its only conjunct was `ghost_locked_set@.finite()`. The plan's RwLock
   rule wants a real invariant; nothing about the ghost shadow remains to
   state.
7. `spec_finite` trait fns deleted in `SetMtEph`, `RelationStEph`,
   `MappingStEph`, `LabDirGraphStEph`, `LabUnDirGraphStEph`,
   `LabDirGraphMtEph`, `LabUnDirGraphMtEph`: they were `finite()` conjunctions,
   now trivially true. No caller in Chap02 to Chap06 used them.
8. Two old `seq_set` lemmas have users outside this round:
   `lemma_map_not_contains_implies_all_ne` (`src/Chap17/MathSeq.rs`) and
   `lemma_push_not_contains_to_set` (`src/Chap41/ArraySetStEph.rs`). They are
   in `seq_set_pre_0913.rs`; restoring them into the new file is a Chap17/41
   round task.
9. Eight Chap62/63/66 RTT entries commented out in `Cargo.toml` because
   `hash_map_with_view_plus` is gone; those chapters need the same treatment
   as Chap05 before the tests return.
10. `LabDirGraphMtEph`/`LabUnDirGraphMtEph` `add_*` needed explicit proof
    blocks for `spec_labgraphview_wf(self@)` (6.4 rows 10-11) that the old
    wrapper did not; the four Locked `add_*` bodies still carry their original
    `assume(self.ghost_locked_graph@ == locked_val@)`.
