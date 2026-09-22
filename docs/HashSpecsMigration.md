# Hash set and hash table specifications: APAS (2025) versus vstd 0.2026.09.13, and the migration plan

Date: 2026-09-20. Round: r209 (opens). Author: orchestrator.

APAS wrote `src/vstdplus/hash_set_with_view_plus.rs` (300 lines),
`hash_map_with_view_plus.rs` (303 lines) and `hash_set_specs.rs` (37 lines)
when vstd had no usable specifications for `std::collections::HashSet` and
`HashMap`. vstd `std_specs/hash.rs` at `release/0.2026.09.13.671956e` now
specifies both types. This document tables each APAS specification against
vstd's, gives the plan to retire the APAS modules (deletions first, additions
second), and records the experiments that show which APAS postconditions are
now derived with no trusted code.

## 1. Summary

| # | Measurement | Value |
|---|-------------|------:|
| 1 | APAS trusted items in the three modules (`external_body` fns, `admit` axiom, empty `assume_specification`) | 24 |
| 2 | APAS postconditions vstd now proves, shown by experiment | 20 of 22 |
| 3 | APAS postconditions vstd lacks | 2: `HashSet::clone` (`t@ == s@`), `PartialEq::eq` on both types |
| 4 | Lines the three modules can lose once their users migrate | 640 |
| 5 | User files (`src/`, `tests/`, PTTs) that name the wrappers | 45 in 13 chapters plus one standard |

Experiments (section 4): `src/experiments/vstd_hash_set_derived.rs` 11 verified
0 errors; `vstd_hash_map_derived.rs` 12 verified 0 errors;
`vstd_hash_set_clone.rs` fails on `t@ == s@`, as expected.

## 2. What APAS specified versus what vstd now has

Status values: "provided" (same statement in vstd), "stronger" (vstd's
statement implies APAS's), "definable" (a one-line spec fn over vstd's view
gives APAS's), "missing" (vstd has nothing), "obsolete" (the concept no
longer exists at 09.13). vstd line numbers are in `source/vstd/std_specs/hash.rs`
unless another file is named. "Test" names the fn in the experiments.

### 2.1 `HashSetWithViewPlus` (Chap vstdplus, `hash_set_with_view_plus.rs`)

| # | Chap | APAS item (line) | APAS statement | vstd 09.13 | Status | Test |
|---|------|------------------|----------------|------------|--------|------|
| 1 | vstdplus | struct + `View` (54–66) | `V = Set<Key::V>`, `view = inner@.map(\|k\| k@)` | `HashSet` view is `Set<Key>` (`ExHashSet`, 900); `HashSetWithView` (`hash_set.rs:27`) has `Set<Key::V>` but no `iter`/`Clone`/`PartialEq` | definable | `view_plus` |
| 2 | vstdplus | `axiom_…_finite` (74, `admit`) | `s@.finite()` | `Set` is finite by type (#2486) | obsolete | — |
| 3 | vstdplus | `new` (105, ext. body) | requires `obeys_key_model`, `obeys_feq_full`; ensures empty | `HashSet::new` (927): no requires, ensures `m@ == Set::empty()` | stronger | `derived_new` |
| 4 | vstdplus | `with_capacity` (116) | as `new` | 940 | stronger | `derived_with_capacity` |
| 5 | vstdplus | `len` (127) | `len == self@.len()` | 913 via `spec_hash_set_len`; needs `obeys_key_model` | provided | `derived_len` |
| 6 | vstdplus | `contains` (135) | requires `obeys_key_model`; `contains == self@.contains(k)` | 948 via `set_contains_borrowed_key` + `axiom_set_contains_deref_key` | provided | `derived_contains` |
| 7 | vstdplus | `insert` (145) | `self@ == old.insert(k)`, `inserted == !old.contains(k)` | 957, same two conjuncts | provided | `derived_insert` |
| 8 | vstdplus | trait `iter` ensures (89–97) | two containment directions, `no_duplicates` | `HashSet::iter` (1115): `remaining.unref().to_set() == m@`, `no_duplicates`, `len == m@.len()`, `decrease is Some` | stronger | `derived_iter`, `derived_for_loop` |
| 9 | vstdplus | `HashSetWithViewPlusIter` + `View` + `iter_invariant` + `next` ensures (167–207) | old `(int, Seq)` model | `IteratorSpecImpl for hash_set::Iter` (864) | obsolete | `derived_for_loop` |
| 10 | vstdplus | ghost iterator + `ForLoopGhostIteratorNew/ForLoopGhostIterator` (209–270) | old for-loop model | `VerusForLoopWrapper` | obsolete | `derived_for_loop` |
| 11 | vstdplus | `Clone` (273, ext. body) | `clone@ == self@` | none; `HashMap::clone` exists (578), `HashSet::clone` does not | missing | `vstd_hash_set_clone` FAILS |
| 12 | vstdplus | `Hash` (282, ext. body) | none | none; `std` does not implement `Hash` for `HashSet` | APAS-only | — |
| 13 | vstdplus | `PartialEq`/`Eq` (291, ext. body) | none | none | missing | — |

vstd items with no APAS counterpart: `is_empty` (920), `remove` (1005 area,
via `sets_differ_by_borrowed_key`), `get` (1038 area), `clear` (1108),
`reserve`, `Default::default`, `axiom_hashset_decreases`, the
`obeys_key_model` axioms for `bool`, every integer type and their `Box`es, and
`builds_valid_hashers::<RandomState>`. Tests: `derived_is_empty`,
`derived_remove`, `derived_clear`.

### 2.2 `hash_set_specs.rs` (Chap vstdplus)

| # | Chap | APAS item | APAS statement | vstd 09.13 | Status |
|---|------|-----------|----------------|------------|--------|
| 1 | vstdplus | `assume_specification [HashSet::clone]` | no postcondition | none | missing (and APAS's had no content either) |

### 2.3 `HashMapWithViewPlus` (Chap vstdplus, `hash_map_with_view_plus.rs`)

| # | Chap | APAS item (line) | APAS statement | vstd 09.13 | Status | Test |
|---|------|------------------|----------------|------------|--------|------|
| 1 | vstdplus | struct + uninterp `View` (50–60) | `V = Map<Key::V, Value>` | `HashMap` view is `Map<Key, Value>` (412); `deep_view` gives `Map<Key::V, Value::V>` with `lemma_hashmap_deepview_properties` (466) under `injective(\|k\| k.deep_view())`; `HashMapWithView` (`hash_map.rs:26`) has `Map<Key::V, Value>` but no `iter`/`Clone`/`PartialEq` | definable | `derived_mapped_view` |
| 2 | vstdplus | trait `new` (66) | requires `obeys_key_model`, `obeys_feq_view_injective`; ensures empty | `HashMap::new` (591): no requires | stronger | `derived_new` |
| 3 | vstdplus | `len` (73) | `count == self@.len()` | 564 via `spec_hash_map_len` | provided | `derived_len` |
| 4 | vstdplus | `is_empty` (76) | `is_empty == self@.is_empty()` | 571 | provided | `derived_is_empty` |
| 5 | vstdplus | `get` (79) | `Some(v) => contains_key && *v == self@[k]`; `None => !contains_key` | `get` (686 area) via `maps_borrowed_key_to_value` + axiom | provided | `derived_get` |
| 6 | vstdplus | `insert` (86) | `self@ == old.insert(k, v)` | 615; also returns the previous value | stronger | `derived_insert` |
| 7 | vstdplus | `clear` (89) | `self@ == Map::empty()` | 814 | provided | `derived_clear` |
| 8 | vstdplus | `contains_key` (92) | `contains == self@.contains_key(k)` | 661 via `contains_borrowed_key` + axiom | provided | `derived_contains_key` |
| 9 | vstdplus | `remove` (95) | `Some(v) => old contains && v == old[k] && self@ == old.remove(k)`; `None => …` | 776 via `borrowed_key_removed` + axiom | provided | `derived_remove` |
| 10 | vstdplus | trait `iter` ensures (102–116) | pairs in map, keys covered, `no_duplicates` | `HashMap::iter` (372): those three, plus `to_set == kv_pairs()`, `len`, `decrease` | stronger | `derived_iter` |
| 11 | vstdplus | iterator + ghost iterator machinery (173–280) | old model | `IteratorSpecImpl for hash_map::Iter` (351) | obsolete | — |
| 12 | vstdplus | `Clone` (283, ext. body) | `cloned@ == self@` | 578: `other@.dom() == this@.dom()` and per-key `cloned(this@[k], other@[k])` | provided when `cloned` is equality on `Value` (integers, `feq` types); needs a bridge for generic `Value` | `derived_clone` |
| 13 | vstdplus | `PartialEq`/`Eq` (292, ext. body) | none | none | missing | — |

vstd items with no APAS counterpart: `keys` (821), `values` (834), `reserve`,
`Default::default`, the entry API, `axiom_hashmap_decreases`,
`lemma_hashmap_deepview_dom/values`. Test: `derived_keys`.

### 2.4 Preconditions: APAS `feq` versus vstd

APAS required `obeys_feq_full::<Key>()` or `obeys_feq_view_injective::<Key>()`
(view injectivity: `x@ == y@ ==> x == y`). vstd's raw specs require only
`obeys_key_model::<Key>()` and `builds_valid_hashers::<S>()`, both discharged
by `group_hash_axioms` for the standard key types and `RandomState`. View
injectivity reappears only where APAS wants the mapped view: vstd states it as
`injective(|k| k.deep_view())` (`lemma_hashmap_deepview_properties`) and, in
its own wrappers, as `forall|k1, k2| k1@ == k2@ ==> k1 == k2`. APAS's
`obeys_feq_view_injective` is the same proposition under another name.

## 3. Migration plan

### 3.1 Phase A: deletions, what vstd provides

| # | Step | Detail | Acceptance |
|---|------|--------|------------|
| 1 | Field type | in every user, `HashSetWithViewPlus<K>` → `std::collections::HashSet<K>`, `HashMapWithViewPlus<K, V>` → `std::collections::HashMap<K, V>`; the exec bodies do not change, the wrapper only forwarded | isolate run of the chapter |
| 2 | View | where the collection's view was `Set<K::V>`, define it as `elements@.map(\|k\| k@)` (r208 did this for `SetStEph`); where it was `Map<K::V, V>`, use `deep_view()` with `lemma_hashmap_deepview_properties`, or `Map::new(dom.map(..), fv)` when `Value` has no `DeepView` | same |
| 3 | Preconditions | `obeys_feq_full` / `obeys_feq_view_injective` in `requires` → `obeys_key_model::<K>()` plus, only where the mapped view is used, `injective(\|k\| k.deep_view())` | same |
| 4 | Iteration | delegated iteration per `src/standards/prophetic_iterators_standard.rs`: return `hash_set::Iter` / `hash_map::Iter`; loop invariants over `it.index()`, `it.seq()` | same |
| 5 | Broadcasts | `broadcast use vstd::std_specs::hash::group_hash_axioms;` replaces `group_hash_set_with_view_plus_axioms` | same |
| 6 | Delete | `hash_set_with_view_plus.rs`, `hash_map_with_view_plus.rs`, `hash_set_specs.rs`, their two PTTs, and the `lib.rs` lines, when no user remains | full validate |

Users, by chapter (hit counts of the two type names):

| # | Chap | Files | Hits | Notes |
|---|------|------:|-----:|-------|
| 1 | 05 | 2 | 2 | `SetStEph.rs`, `SetMtEph.rs`: migrated in r208; only the `Clone` postcondition remains (Phase B item 1) |
| 2 | 17 | 1 | 1 | `MathSeq.rs` |
| 3 | 49 | 8 | 50 | MinEditDist and SubsetSum, St/Mt, Eph/Per: memo tables |
| 4 | 50 | 8 | 58 | MatrixChain and OptBinSearchTree: memo tables |
| 5 | 51 | 4 | 26 | TopDownDP |
| 6 | 61 | 4 | 8 | EdgeContraction, VertexMatching |
| 7 | 62 | 4 | 60 | StarContraction, StarPartition (`StarPartitionMtEph.rs` 34) |
| 8 | 63 | 2 | 29 | Connectivity |
| 9 | 64 | 3 | 8 | SpanTree, TSPApprox |
| 10 | 65 | 3 | 10 | Prim, UnionFind ×2 |
| 11 | 66 | 2 | 45 | Boruvka (`BoruvkaMtEph.rs` 33) |
| 12 | std | 1 | 9 | `using_hashmap_standard.rs`: rewrite to the raw `HashMap` recipe |
| 13 | tests | 3 | 6 | `tests/Chap62`, `tests/Chap66` |
| 14 | PTT | 2 | 10 | `rust_verify_test/tests/vstdplus/Hash{Set,Map}WithViewPlus.rs`: delete with the modules |

Order: Chap05 (finish), the standard, then chapters in numeric order, one
isolate run each. Chap49–51 are one recipe repeated (a memo `HashMap` keyed by
a tuple); Chap61–66 are graph algorithms over `SetStEph` and a `HashMap`.

### 3.2 Phase B: additions, what vstd lacks

| # | Addition | Where | Statement | Why |
|---|----------|-------|-----------|-----|
| 1 | `HashSet::clone` specification | upstream PR to vstd `std_specs/hash.rs`, mirrored locally in `src/vstdplus/hash_specs_plus.rs` until merged | `assume_specification<K: Clone + Eq + Hash, S: Clone, A: Clone>[<HashSet<K,S,A> as Clone>::clone](this) -> (other) ensures other@.len() == this@.len(), forall k: other@.contains(k) <==> exists k0: this@.contains(k0) && cloned(k0, k)`; for keys where `cloned` is equality this gives `other@ == this@` | the two remaining Chap05 errors; the `HashMap::clone` spec at 578 is the model |
| 2 | `PartialEq::eq` specification for `HashSet` and `HashMap` | same file; upstream proposal was not taken | `ensures r == (a@ == b@)` under `obeys_key_model::<K>()` and `obeys_eq_spec` for `K` (and `V`) | APAS's `SetStEph::eq` and `MappingStEph::eq` currently accept it (permitted pattern); with the spec the accepts close |
| 3 | `Hash` for a set-of-sets | stays APAS-only: the one `external_body` `Hash` impl in `SetStEph`/`SetMtEph`, or drop `Hash` if `partition` is restated | `std` has no `Hash for HashSet`; nothing to derive |
| 4 | `cloned` → equality bridge for generic `Value` | `src/vstdplus/feq.rs` already states it for `feq` types | lets item 12 of 2.3 give `cloned@ == self@` for every APAS value type | needed by Chap49–51 memo tables whose values are structs |

Items 1 and 2 come first: item 1 unblocks Chap05, and both are small
`assume_specification`s that replace `external_body` on APAS's side.

## 4. Experiments

| # | Chap | File | Question | Result | Log |
|---|------|------|----------|--------|-----|
| 1 | exp | `vstd_hash_set_derived.rs` | which `HashSetWithViewPlus` postconditions are derivable | 11 verified, 0 errors: all but `Clone` | `logs/validate-standard-vstd_hash_set_derived.20260920-222048.log` |
| 2 | exp | `vstd_hash_set_clone.rs` | is `HashSet::clone` specified | 0 verified, 1 error: postcondition `t@ == s@` not satisfied | `logs/validate-standard-vstd_hash_set_clone.20260920-222027.log` |
| 3 | exp | `vstd_hash_map_derived.rs` | which `HashMapWithViewPlus` postconditions are derivable | 12 verified, 0 errors: all, including `Clone` for integer values and the mapped view via `deep_view` | `logs/validate-standard-vstd_hash_map_derived.20260920-222027.log` |

Each experiment has one fn per APAS method whose `ensures` is APAS's
postcondition; the body is the single `std` call. They are listed commented
out in `src/lib.rs` with their status and run with
`scripts/validate-standard.sh --experiment <name>`.

## 5. Sources

- `src/vstdplus/hash_set_with_view_plus.rs`, `hash_map_with_view_plus.rs`, `hash_set_specs.rs` at `4bd6b4f92`
- `~/projects/verus/source/vstd/std_specs/hash.rs`, `hash_set.rs`, `hash_map.rs` at `release/0.2026.09.13.671956e`
- `docs/VstdplusReview.md` §3.10–3.12 (r207 agent 2)
- `docs/Chap02to06Validation.md` §5 (r208, the Chap05 migration already done)
