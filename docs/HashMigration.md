# Hash wrapper migration (r209, agent 1)

Date: 2026-09-21. Verus 0.2026.09.13. Plan: `plans/r209-agent1-hash-migration.md`,
executing `docs/HashSpecsMigration.md` §3. Nothing committed.

Outcome in one paragraph. `src/vstdplus/hash_set_with_view_plus.rs`,
`hash_map_with_view_plus.rs` and `hash_set_specs.rs` are deleted, with their two
PTTs and `rust_verify_test/Cargo.toml` entries. Every call site (Chap05, 17, 49,
50, 51, 61–66, the three Chap57–59 broadcast lines, `tests/Chap62`,
`tests/Chap63`, `tests/Chap66`, `standards/using_hashmap_standard.rs`) uses
`std::collections::HashSet`/`HashMap` with vstd `std_specs::hash`. The three
trusted additions of §3.2 live in `src/vstdplus/hash_specs_plus.rs` with two
verified experiments. Chap05, Chap17, Chap50 and Chap66 verify at 0 errors,
0 warnings; Chap49, 51, 61–64 type-check and stop in their unmigrated Chap18/19
dependencies; Chap65 does not reach type-checking because rustc aborts on an
unresolved import in the unedited `src/Chap65/KruskalStEph.rs`. No `assume`,
`accept`, `admit`, `external_body` or `requires true` was added; no `ensures` was
weakened. 812 `Alg Analysis` annotations were read against their bodies: 604
confirmed, 136 are textbook quotes, 72 disputed (§5, D1–D15).

## 1. Results

"Before" is the count of lines naming `HashSetWithViewPlus`,
`HashMapWithViewPlus`, `hash_set_specs`, `hash_set_with_view_plus` or
`hash_map_with_view_plus` in `HEAD`; "After" is the same count on disk. The
three "1" residues are comments (§5, item 9). Logs are in `logs/`; footnoted
names are given below the table. Every log was read in full.

| # | Chap | File | Before | After | Result | Log |
|---|------|------|-------:|------:|--------|-----|
| 1 | vstdplus | hash_specs_plus.rs | new | 0 | in the Chap05 run below | validate.20260921-093012.log |
| 2 | exp | vstd_hash_set_clone_plus.rs | new | 0 | 183 verified, 0 errors | L1 |
| 3 | exp | vstd_hash_eq_plus.rs | new | 0 | 182 verified, 0 errors | L2 |
| 4 | 05 | SetStEph.rs | 8 | 1 | 760 verified, 0 errors | validate.20260921-093012.log |
| 5 | 05 | SetMtEph.rs | 10 | 1 | 760 verified, 0 errors | validate.20260921-093012.log |
| 6 | std | using_hashmap_standard.rs | 11 | 1 | 6 verified, 0 errors | L3 |
| 7 | 17 | MathSeq.rs | 3 | 0 | 635 verified, 0 errors | validate.20260921-083557.log |
| 8 | 66 | BoruvkaStEph.rs | 13 | 0 | 795 verified, 0 errors | validate.20260921-084327.log |
| 9 | 66 | BoruvkaMtEph.rs | 34 | 0 | 795 verified, 0 errors | validate.20260921-084327.log |
| 10 | 66 | tests/TestBoruvkaStEph.rs | 3 | 0 | edited, not run | — |
| 11 | 66 | tests/TestBoruvkaMtEph.rs | 3 | 0 | edited, not run | — |
| 12 | 49 | MinEditDistStEph.rs | 4 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 13 | 49 | MinEditDistStPer.rs | 5 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 14 | 49 | MinEditDistMtEph.rs | 10 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 15 | 49 | MinEditDistMtPer.rs | 10 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 16 | 49 | SubsetSumStEph.rs | 4 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 17 | 49 | SubsetSumStPer.rs | 5 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 18 | 49 | SubsetSumMtEph.rs | 10 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 19 | 49 | SubsetSumMtPer.rs | 10 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-084828.log |
| 20 | 50 | MatrixChainStEph.rs | 6 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 21 | 50 | MatrixChainStPer.rs | 6 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 22 | 50 | MatrixChainMtEph.rs | 12 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 23 | 50 | MatrixChainMtPer.rs | 8 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 24 | 50 | OptBinSearchTreeStEph.rs | 6 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 25 | 50 | OptBinSearchTreeStPer.rs | 6 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 26 | 50 | OptBinSearchTreeMtEph.rs | 12 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 27 | 50 | OptBinSearchTreeMtPer.rs | 10 | 0 | 756 verified, 0 errors | validate.20260921-085534.log |
| 28 | 51 | SeqSpecsAndLemmas.rs | 0 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-085917.log |
| 29 | 51 | TopDownDPStEph.rs | 4 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-085917.log |
| 30 | 51 | TopDownDPStPer.rs | 10 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-085917.log |
| 31 | 51 | TopDownDPMtEph.rs | 8 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-085917.log |
| 32 | 51 | TopDownDPMtPer.rs | 8 | 0 | type-checked, blocked on Chap18/19 | validate.20260921-085917.log |
| 33 | 57 | DijkstraStEphF64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 34 | 57 | DijkstraStEphU64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 35 | 58 | BellmanFordStEphF64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 36 | 58 | BellmanFordStEphI64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 37 | 59 | JohnsonMtEphF64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 38 | 59 | JohnsonStEphF64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 39 | 59 | JohnsonStEphI64.rs | 1 | 0 | not reached (full run aborts) | validate.20260921-092814.log |
| 40 | 61 | EdgeContractionStEph.rs | 2 | 0 | type-checked, blocked on Chap19 | validate.20260921-091115.log |
| 41 | 61 | EdgeContractionMtEph.rs | 4 | 0 | type-checked, blocked on Chap19 | validate.20260921-091115.log |
| 42 | 61 | VertexMatchingStEph.rs | 2 | 0 | type-checked, blocked on Chap19 | validate.20260921-091115.log |
| 43 | 61 | VertexMatchingMtEph.rs | 4 | 0 | type-checked, blocked on Chap19 | validate.20260921-091115.log |
| 44 | 62 | StarPartitionStEph.rs | 5 | 0 | type-checked, blocked on Chap19 | validate.20260921-091932.log |
| 45 | 62 | StarPartitionMtEph.rs | 36 | 0 | type-checked, blocked on Chap19 | validate.20260921-091932.log |
| 46 | 62 | StarContractionStEph.rs | 13 | 0 | type-checked, blocked on Chap19 | validate.20260921-091932.log |
| 47 | 62 | StarContractionMtEph.rs | 14 | 0 | type-checked, blocked on Chap19 | validate.20260921-091932.log |
| 48 | 62 | tests/TestStarPartitionStEph.rs | 1 | 0 | edited, not run | — |
| 49 | 62 | tests/TestStarPartitionMtEph.rs | 1 | 0 | edited, not run | — |
| 50 | 62 | tests/TestStarContractionStEph.rs | 2 | 0 | edited, not run | — |
| 51 | 62 | tests/TestStarContractionMtEph.rs | 1 | 0 | edited, not run | — |
| 52 | 63 | ConnectivityStEph.rs | 14 | 0 | type-checked, blocked on Chap19 | validate.20260921-092508.log |
| 53 | 63 | ConnectivityMtEph.rs | 17 | 0 | type-checked, blocked on Chap19 | validate.20260921-092508.log |
| 54 | 63 | tests/TestConnectivityStEph.rs | 1 | 0 | edited, not run | — |
| 55 | 63 | tests/TestConnectivityMtEph.rs | 1 | 0 | edited, not run | — |
| 56 | 64 | SpanTreeStEph.rs | 4 | 0 | type-checked, blocked on Chap19 | validate.20260921-092658.log |
| 57 | 64 | SpanTreeMtEph.rs | 4 | 0 | type-checked, blocked on Chap19 | validate.20260921-092658.log |
| 58 | 64 | TSPApproxStEph.rs | 2 | 0 | type-checked, blocked on Chap19 | validate.20260921-092658.log |
| 59 | 65 | PrimStEph.rs | 2 | 0 | not type-checked (Kruskal E0432) | validate.20260921-092701.log |
| 60 | 65 | UnionFindNoPCStEph.rs | 5 | 0 | not type-checked (Kruskal E0432) | validate.20260921-092701.log |
| 61 | 65 | UnionFindPCStEph.rs | 5 | 0 | not type-checked (Kruskal E0432) | validate.20260921-092701.log |
| 62 | — | full `scripts/validate.sh` | — | — | 2 E0432 errors, rustc aborts | validate.20260921-092814.log |

- L1: `logs/validate-standard-vstd_hash_set_clone_plus.20260921-083043.log`
- L2: `logs/validate-standard-vstd_hash_eq_plus.20260921-083044.log`
- L3: `logs/validate-standard-using_hashmap_standard.20260921-083707.log`

Error attribution in the blocked runs (every `-->` line, grouped by file):

| # | Chap | Log | Errors | Files named by `-->` |
|---|------|-----|-------:|----------------------|
| 1 | 49 | validate.20260921-084828.log | 35 | Chap18 ×10, Chap19 ×4 files; 0 in Chap49 |
| 2 | 51 | validate.20260921-085917.log | 35 | Chap18 ×10, Chap19 ×4 files; 0 in Chap51 |
| 3 | 61 | validate.20260921-091115.log | 13 | four Chap19 files (F1); 0 in Chap61 |
| 4 | 62 | validate.20260921-091932.log | 13 | four Chap19 files (F1); 0 in Chap62 |
| 5 | 63 | validate.20260921-092508.log | 13 | four Chap19 files (F1); 0 in Chap63 |
| 6 | 64 | validate.20260921-092658.log | 13 | four Chap19 files (F1); 0 in Chap64 |
| 7 | 65 | validate.20260921-092701.log | 1 | Chap65/KruskalStEph.rs:29 (F2) |
| 8 | all | validate.20260921-092814.log | 2 | F2 and Chap41/ArraySetStEph.rs:44 (F3) |

- F1: `src/Chap19/ArraySeqMtEph.rs` (5), `ArraySeqMtEphSlice.rs` (1),
  `ArraySeqStEph.rs` (3), `ArraySeqStPer.rs` (3): `E0599: no method named view
  found for std::slice::Iter` / `std::vec::IntoIter` (the pre-09.13 iterator
  model). The Chap49/51 runs add the ten Chap18 files with the same error.
- F2: `E0432: unresolved import crate::Chap05::SetStEph::SetStEph::iter_invariant`,
  a name r208 removed; `KruskalStEph.rs` is not in the r209 list and was not edited.
- F3: `E0432: unresolved import crate::vstdplus::seq_set::lemma_push_not_contains_to_set`,
  also removed by r208; `ArraySetStEph.rs` was not edited.

Chap50 needed one proof round: the first isolate run (`validate.20260921-085437.log`,
752 verified, 4 errors) failed `memo_size` in all four St files because vstd's
`HashMap::len` is an uninterpreted `spec_hash_map_len` fixed by
`axiom_spec_hash_map_len`, which is in `group_hash_axioms` and not in the default
broadcast. Adding the broadcast and the `Pair` key-model trigger fixed it.

## 2. Edits per file

The wrapper forwarded every call to the std type, so each method's cost is
unchanged in every file below; the only exec changes are the two rule-1
substitutions (field type; delegated iteration), except where §5 item 2 says
otherwise. Iterator conversions replace `let mut it = x.iter(); loop { match
it.next() ... }` with `iter_invariant(&it)` / `it@.0`, `it@.1` invariants by
`for e in it: x.iter()` with `it.seq()`, `it.index()` and a ghost snapshot
`into_iter_hash_keys(it)` (sets) or `vstd::std_specs::hash::into_iter(it)` (maps),
per `docs/PropheticIterators.md`. Each entry below gives Chap, file(s), the kind
of edit, and old → new text.

### 2.0 `src/vstdplus/hash_specs_plus.rs` (new) and `src/lib.rs`

New module, registered as `pub mod hash_specs_plus;` in the vstdplus block of
`src/lib.rs` (the three deleted `pub mod` lines removed) and added to the
dependency list in `scripts/validate-standard.sh`. Contents in §4. `lib.rs`
also lists the two new experiments commented out with `SUCCEEDS` annotations.

### 2.1 Chap05 `SetStEph.rs`, `SetMtEph.rs`

The diff against `HEAD` also contains r208's iterator work on these two files;
only the hash-related edits are listed.

1. Chap 05, both, import: `use crate::vstdplus::hash_set_with_view_plus::...::*;`
   → `use std::collections::HashSet;` and
   `use crate::vstdplus::hash_specs_plus::hash_specs_plus::lemma_hash_set_clone_eq;`.
2. Chap 05, both, broadcast: `group_hash_set_with_view_plus_axioms` →
   `vstd::std_specs::hash::group_hash_axioms`.
3. Chap 05, both, field: `pub elements: HashSetWithViewPlus<T>` →
   `pub elements: HashSet<T>`.
4. Chap 05, both, view: the wrapper's `Set<T::V>` → `self.elements@.map(|x: T| x@)`.
5. Chap 05, both, wf (`spec_setsteph_wf_generic` / `spec_setmteph_wf_generic`):
   `s@.finite() && valid_key_type::<V>()` → `valid_key_type::<V>()`.
6. Chap 05, both, `Clone` ensures: `clone@.finite(), clone@ == self@` →
   `clone@ == self@`; body `self.elements.clone()` now followed by
   `proof { lemma_hash_set_clone_eq(self.elements@, elements@); }`.
7. Chap 05, both, constructors: `HashSetWithViewPlus::new()` → `HashSet::new()`;
   `HashSetWithViewPlus::with_capacity(capacity)` → `HashSet::with_capacity(capacity)`.
8. Chap 05, both, `PartialEq::eq`: `accept(equal == (self@ == other@))` unchanged
   (§5 item 3); comment now "HashSet eq has no vstd specification".
9. Chap 05, both, `impl Hash` (outside `verus!`): `self.elements.hash(state)` →
   per-element `key.hash(state)` loop (§5 item 2).
10. Chap 05, both, `split` invariants: `first@.finite(), second@.finite()` → deleted.
11. Chap 05, SetMtEph, RwLock predicate: `v@.finite() && valid_key_type::<T>()` →
    `valid_key_type::<T>()`; ghost shadow `self.ghost_locked_set@.finite()` deleted.

The proof bridge from the raw `Set<T>` to the mapped view is the family
`lemma_viewed_{contains,mem,insert,empty,len}` and `lemma_iter_keys_view` in
each file (r208), unchanged here except that `lemma_viewed_len` no longer needs
`finite()`.

### 2.2 `src/standards/using_hashmap_standard.rs`

Rewritten from a comment-only module to a compiled example `CountTable<K>`
following §3.1 steps 1–5: field `HashMap<K, u64>`; view `self.counts@`
(`Map<K, u64>`); `spec_counttable_wf` is `obeys_key_model::<K>()`; `iter`
returns `hash_map::Iter` with vstd's postconditions restated; `size` is a
for-loop over it with `it.seq()`/`it.index()`; `broadcast use group_hash_axioms`.
The prose now says when a mapped view needs `deep_view` +
`lemma_hashmap_deepview_properties` (maps) or `@.map(|k| k@)` (sets), and that
`obeys_feq_view_injective` is not a raw-view precondition. Verified 6/0 (L3).

### 2.3 Chap17 `MathSeq.rs`

1. Chap 17, import: `hash_set_with_view_plus::*` → `use std::collections::HashSet;`.
2. Chap 17, broadcast: `vstd::std_specs::hash::axiom_random_state_builds_valid_hashers,
   vstd::std_specs::hash::axiom_contains_deref_key, ...group_hash_set_with_view_plus_axioms`
   → `vstd::std_specs::hash::group_hash_axioms`.
3. Chap 17, `dedup` local: `let mut seen: HashSetWithViewPlus<T> = HashSetWithViewPlus::new();`
   → `let mut seen: HashSet<T> = HashSet::new();`.
4. Chap 17, `dedup` invariant: `seen@.finite()` → deleted.
5. Chap 17, `dedup` proof: `lemma_cloned_view_eq(x, x_clone)` →
   `assert(cloned(x, x_clone))` (the feq broadcast turns it into equality).

### 2.4 Chap66 `BoruvkaStEph.rs`, `BoruvkaMtEph.rs`, `tests/Chap66`

1. Chap 66, both, imports: `hash_map_with_view_plus::*`, `SetStEph::iter_invariant`,
   `feq::obeys_feq_view_injective` → `use std::collections::HashMap;`,
   `vstd::std_specs::hash::{obeys_key_model, into_iter_hash_keys}`,
   `vstd::std_specs::iter::*`.
2. Chap 66, both, broadcast: added `vstd::std_specs::hash::group_hash_axioms`.
3. Chap 66, both, types: `HashMapWithViewPlus<V, (V, WrappedF64, usize)>`,
   `HashMapWithViewPlus<V, bool>`, `HashMapWithViewPlus<V, V>` →
   `HashMap<V, (V, WrappedF64, usize)>`, `HashMap<V, bool>`, `HashMap<V, V>`;
   `Arc<HashMapWithViewPlus<..>>` → `Arc<HashMap<..>>`.
4. Chap 66, both, ensures/invariants (bridge finiteness, 6 sites):
   `forall|k: V::V| #[trigger] bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite()`
   → `forall|k: V| #[trigger] bridges@.contains_key(k) ==> bridges@[k].1.spec_is_finite()`
   (raw keys; same for `r@`, `merged@`, `right_bridges@`).
5. Chap 66, both, requires: `obeys_feq_view_injective::<V>()` (13 sites, trait and
   impl) → deleted (§5 item 5); StEph bounds
   `obeys_feq_full::<V>(), obeys_feq_view_injective::<V>()` → `obeys_feq_full::<V>()`
   (three sites, also for `LabeledEdge<V>`).
6. Chap 66, both, loop invariants: `iter_invariant(&it), iter_seq == it@.1` and
   `decreases iter_seq.len() - it@.0` → `eit.seq().unref() == iter_seq` with
   `let ghost iter_seq = into_iter_hash_keys(it)`; map loops add
   `obeys_key_model::<V>()`.
7. Chap 66, MtEph, merge loop in `vertex_bridges_mt`: `let ghost iter_seq = it@.1`
   → `let rit = right_bridges.iter(); let ghost r_seq = vstd::std_specs::hash::into_iter(rit);`
   with invariant `it.seq().unref() == r_seq` and body assert
   `r_seq[it.index()].0 == *v && r_seq[it.index()].1 == *entry`.
8. Chap 66, StEph, `vertex_bridges` body assert: `edges@.contains(iter_seq[it@.0 - 1]@)`
   → `edges@.contains(iter_seq[eit.index()]@)`.
9. Chap 66, tests: wrapper `use` lines deleted; closure parameter types
   `&HashMapWithViewPlus<..>` → `&HashMap<..>`.

Loops converted (all rule-1(b)): StEph `vertex_bridges`, `bridge_star_partition`
(three vertex loops, partition and remaining loops, edge loop), `mst_weight`;
MtEph merge loops in `hash_coin_flips_mt`, `build_partition_map_mt`,
`vertex_bridges_mt`, `filter_tail_to_head_mt`, the partition/remaining loops in
`bridge_star_partition_mt` and `boruvka_mst_mt`, the vertex and edge loops in
`boruvka_mst_mt_with_seed`, `mst_weight`.

### 2.5 Chap49 (8 files)

1. Chap 49, all 8, import: `hash_map_with_view_plus::*` (+ `arc_rwlock::*` in
   the four Mt files) → `use std::collections::HashMap;`.
2. Chap 49, all 8, field: `memo: HashMapWithViewPlus<Pair<usize, X>, Y>` →
   `memo: HashMap<Pair<usize, X>, Y>` (`X, Y` = `usize, usize` for MinEditDist,
   `i32, bool` for SubsetSum); `HashMapWithViewPlus::new()` → `HashMap::new()`.
3. Chap 49, MinEditDistStEph/StPer, spec:
   `spec_memo_bounded(memo_view: Map<(usize, usize), usize>)` with
   `forall|k: (usize, usize)|` → `Map<Pair<usize, usize>, usize>`,
   `forall|k: Pair<usize, usize>|`.
4. Chap 49, MinEditDistMtEph/MtPer, spec: `spec_memo_bounded` added with the
   same text (the Mt files previously had no memo predicate).
5. Chap 49, MinEditDistMtEph/MtPer, `new_arc_memo` requires:
   `val@.dom().finite()` → `spec_memo_bounded(val@)`; RwLock `inv`:
   `v@.dom().finite()` → `spec_memo_bounded(v@)`.
6. Chap 49, SubsetSumMtEph/MtPer, `new_arc_memo` requires: `val@.dom().finite()`
   → deleted; RwLock `inv`: `v@.dom().finite()` → `true` with a comment
   (§5 item 13).
7. Chap 49, four Mt files, Arc: `new_arc_rwlock(val, Ghost(Inv))` →
   `Arc::new(RwLock::new(val, Ghost(Inv)))`; `clone_arc_rwlock(&s.memo)` /
   `clone_arc_rwlock(memo)` → `s.memo.clone()` / `memo.clone()`.
8. Chap 49, MinEditDist ×4, proof: `proof { let _ = Pair_feq_trigger::<usize, usize>(); }`
   at the top of `med_rec` bodies (§5 item 12).
9. Chap 49, StPer ×2, `eq`/`Debug` outside `verus!`: `self.memo.inner` → `self.memo`.

### 2.6 Chap50 (8 files)

1. Chap 50, all 8, import: wrapper (+ `arc_rwlock` in Mt) → `use std::collections::HashMap;`.
2. Chap 50, all 8, field: `memo: HashMapWithViewPlus<Pair<usize, usize>, X>` →
   `memo: HashMap<Pair<usize, usize>, X>`; ghost view struct field
   `memo: Map<(usize, usize), X>` → `memo: Map<Pair<usize, usize>, X>`.
3. Chap 50, MatrixChain ×4, spec: `spec_memo_correct(dims, memo: Map<(usize, usize), usize>)`
   with `memo.contains_key((a, b))`, `memo[(a, b)]` → `Map<Pair<usize, usize>, usize>`,
   `memo.contains_key(Pair(a, b))`, `memo[Pair(a, b)]`.
4. Chap 50, all 8, ensures: `self@.memo == Map::<(usize, usize), X>::empty()` →
   `Map::<Pair<usize, usize>, X>::empty()`; `self@.memo.contains_key((i, j))` →
   `self@.memo.contains_key(Pair(i, j))`.
5. Chap 50, OptBinSearchTreeStEph/StPer, `obst_rec` requires/ensures/invariants:
   `s@.memo.dom().finite()` (and `old(s)`) → deleted.
6. Chap 50, St ×4, broadcast: added `vstd::std_specs::hash::group_hash_axioms`.
7. Chap 50, MatrixChainMtEph/MtPer, RwLock `inv`:
   `v@.dom().finite() && spec_memo_correct(self.dims, v@)` →
   `spec_memo_correct(self.dims, v@)`.
8. Chap 50, OptBinSearchTreeMtEph/MtPer, RwLock `inv`: `v@.dom().finite()` →
   `true` with a comment (§5 item 13).
9. Chap 50, Mt ×4, Arc: `new_arc_rwlock(..)` → `Arc::new(RwLock::new(..))`;
   `clone_arc*(..)` → `.clone()`.
10. Chap 50, MatrixChainMtEph/MtPer, assert-foralls in `matrix_chain_rec`:
    `(a, b)` keys → `Pair(a, b)`.
11. Chap 50, all 8, proof: `Pair_feq_trigger::<usize, usize>()` at the top of
    `matrix_chain_rec` / `obst_rec` and in `memo_size` (§5 item 12).
12. Chap 50, St ×4, header comment → `//! Uses HashMap for the memo table.`

### 2.7 Chap51 (4 files + `SeqSpecsAndLemmas.rs`)

1. Chap 51, SeqSpecsAndLemmas.rs, spec:
   `spec_memo_correct(memo: Map<(usize, usize), usize>, s, t)` with `(a, b)` →
   `Map<Pair<usize, usize>, usize>`, `Pair(a, b)`; adds `use crate::Types::Types::Pair;`.
2. Chap 51, TopDownDP ×4, import: wrapper (+ `arc_rwlock` in Mt) → `HashMap`.
3. Chap 51, TopDownDP ×4, spec: `spec_memo(&self) -> Map<(usize, usize), usize>` →
   `Map<Pair<usize, usize>, usize>`.
4. Chap 51, TopDownDP ×4, trait ensures: `self.spec_memo().contains_key((i, j))`,
   `old(self).spec_memo().insert((i, j), value)`, `Map::<(usize, usize), usize>::empty()`
   → `Pair(i, j)` forms.
5. Chap 51, TopDownDP ×4, requires/ensures: `old(memo)@.dom().finite()`,
   `memo@.dom().finite()` → deleted.
6. Chap 51, St ×2, broadcast: added `group_hash_axioms`.
7. Chap 51, Mt ×2, RwLock predicate: `impl RwLockPredicate<HashMapWithViewPlus<..>>`
   with `v@.dom().finite() && spec_memo_correct(v@, ..)` →
   `impl RwLockPredicate<HashMap<Pair<usize, usize>, usize>>` with
   `spec_memo_correct(v@, self.seq_s, self.seq_t)`.
8. Chap 51, Mt ×2, Arc: `new_arc_rwlock`, `clone_arc_rwlock` →
   `Arc::new(RwLock::new(..))`, `memo.clone()`.
9. Chap 51, all 4, proof: `Pair_feq_trigger` in `memo_size`, `is_memoized`,
   `get_memoized`, `insert_memo`, `med_recursive`, `med_recursive_sequential`,
   `med_recursive_parallel`.
10. Chap 51, all 4, `Debug`: `self.memo.inner` → `self.memo`.

### 2.8 Chap61 (4 files)

1. Chap 61, all 4, import: `hash_map_with_view_plus::*` → `use std::collections::HashMap;`;
   `#[cfg(verus_keep_ghost)] use vstd::std_specs::hash::obeys_key_model;`.
2. Chap 61, EdgeContractionStEph, broadcast: new TOC section 3 with
   `broadcast use vstd::std_specs::hash::group_hash_axioms;`.
3. Chap 61, all 4, types: `HashMapWithViewPlus<V, V>` → `HashMap<V, V>`;
   `HashMapWithViewPlus::new()` → `HashMap::<V, V>::new()`.
4. Chap 61, EdgeContractionStEph, loop over `vertex_to_block`:
   `loop { match it.next() ... }` →
   `for kv in vertex_to_block.iter() invariant valid_key_type_Edge::<V>(),
   obeys_key_model::<V>(), new_vertices.spec_setsteph_wf(), { let (_, representative) = kv; ... }`.

### 2.9 Chap62 (4 files + `tests/Chap62`, `tests/Chap63`)

The partition map's spec keys stay `V::V`: every `X@` of `HashMap` type in a
spec became `key_view(X@)` (`hash_specs_plus::key_view`, §4), so
`spec_valid_partition_map::<V>(graph.V@, centers@, key_view(partition_map@))`
reads as before.

1. Chap 62, all 4, import: wrapper → `use std::collections::HashMap;`;
   `#[cfg(verus_keep_ghost)] use crate::vstdplus::hash_specs_plus::hash_specs_plus::key_view;`.
2. Chap 62, StarPartition ×2, broadcast: wrapper axioms →
   `vstd::std_specs::hash::group_hash_axioms`,
   `crate::vstdplus::hash_specs_plus::hash_specs_plus::group_key_view_lemmas`.
3. Chap 62, StarPartitionStEph, return type: `(SetStEph<V>, HashMapWithViewPlus<V, V>)`
   → `(SetStEph<V>, HashMap<V, V>)`.
4. Chap 62, StarPartitionStEph, ensures:
   `spec_valid_partition_map::<V>(graph.V@, partition.0@, partition.1@)` →
   `spec_valid_partition_map::<V>(graph.V@, partition.0@, key_view(partition.1@))`;
   invariants and ghost snapshots `partition_map@` → `key_view(partition_map@)`.
5. Chap 62, StarPartitionMtEph, every spec/invariant/ghost `let`: `X@` for a
   `HashMap`-typed `X` → `key_view(X@)` (file rewritten with `Write`, §5 item 14).
6. Chap 62, StarPartitionMtEph, merge loop in `hash_coin_flips_mt`:
   `loop { match it.next() }` with `it@.0`/`it@.1` → `for kv in it: rit` with
   `it.seq().unref() == it_seq`, `forall|idx| 0 <= idx < it.index() ==>
   key_view(merged@).contains_key(it_seq[idx].0@)`, a pair-witness invariant
   for `right@`, and a post-loop `choose` argument; the same shape in
   `build_vertex_to_index_mt`, `build_satellite_map_mt`, `build_partition_map_mt`.
7. Chap 62, StarPartitionMtEph, Arc: `clone_arc(&x)` → `x.clone()` (vstd
   `Arc::clone` spec).
8. Chap 62, StarContraction ×2, closure bounds and requires:
   `G: Fn(&SetStEph<V>, &HashMapWithViewPlus<V, V>) -> ..` → `&HashMap<V, V>`;
   `spec_valid_partition_map::<V>(.., partition_map@)` →
   `key_view(partition_map@)` / `key_view((*partition_map)@)`.
9. Chap 62, StarContractionMtEph, `Arc::new(partition_map.clone())`: wrapper
   clone spec → `proof { lemma_hash_map_clone_eq(partition_map@, (*part_map_arc)@); }`.
10. Chap 62, tests (and `tests/Chap63`): wrapper `use` lines deleted; closure
    parameter `_part: &HashMapWithViewPlus<usize, usize>` → `&HashMap<usize, usize>`.

### 2.10 Chap63 (2 files)

1. Chap 63, both, import: wrapper → `HashMap`;
   `#[cfg(verus_keep_ghost)] use vstd::std_specs::hash::{into_iter_hash_keys, obeys_key_model};`;
   MtEph also `key_view`.
2. Chap 63, both, broadcast: new section 3 `group_hash_axioms` (MtEph also
   `group_key_view_lemmas`).
3. Chap 63, both, types: `HashMapWithViewPlus<V, V>` → `HashMap<V, V>`.
4. Chap 63, both, `build_quotient_edges` loop: `it@`-based invariants →
   `let ghost edge_seq = into_iter_hash_keys(it); for edge in iter: it invariant
   iter.seq().unref() == edge_seq, edge_seq.map(|i: int, e: Edge<V>| e@).to_set() == graph_edges@, ...`.
5. Chap 63, both, base closure loop: `elem_seq`, `iter.elements` invariants →
   for-loop with invariants over `it.seq()`.
6. Chap 63, MtEph, `compose_maps_parallel` ensures/invariants: `composed@`,
   `result@`, `partition_map@` → `key_view(..)` of each.

### 2.11 Chap64 (3 files)

1. Chap 64, SpanTree ×2, import: wrapper → `HashMap`;
   `#[cfg(verus_keep_ghost)] use vstd::std_specs::hash::{into_iter_hash_keys, obeys_key_model};`.
2. Chap 64, SpanTree ×2, broadcast: new section 3 `group_hash_axioms`.
3. Chap 64, SpanTree ×2, invariants: Part 1 loop adds `obeys_key_model::<V>()`;
   Part 2 `for qe in iter: it_qt invariant ..., obeys_key_model::<Edge<V>>()`.
4. Chap 64, SpanTree ×2, inner search: `loop` over `original_edges` with `it@` →
   `let it_oe = original_edges.elements.iter(); for oe in oit: it_oe invariant .. { ...; break; }`
   (comment: "Uses elements.iter() (std HashSet::iter, no wf required)").
5. Chap 64, SpanTree ×2, `verify_spanning_tree` loop: `it@.1` snapshot →
   `into_iter_hash_keys` + `iter.seq().unref() == edge_seq`.
6. Chap 64, TSPApproxStEph, types: `HashSetWithViewPlus<V>`, `::new()` →
   `HashSet<V>`, `HashSet::<V>::new()`.
7. Chap 64, TSPApproxStEph, loops: three `it@` loops → `for n in neighbors.iter() { ng_vec.push(n.clone()); }`,
   `for te in tree_edges.iter() invariant tree_edges.spec_setsteph_wf(), { ...; break; }` (×2).

### 2.12 Chap65 (3 files; not type-checked, see §1)

1. Chap 65, PrimStEph, import: `HashSetWithViewPlus` → `use std::collections::HashSet;`;
   cfg imports `set_key_view`, `into_iter_hash_keys`.
2. Chap 65, PrimStEph, broadcast: added `group_key_view_lemmas`, `group_hash_axioms`.
3. Chap 65, PrimStEph, `HashSetWithViewPlus::new()` → `HashSet::<V>::new()`.
4. Chap 65, PrimStEph, invariants: `visited@.finite()` → deleted;
   `visited@.contains(e.0)`, `visited@.contains(u@)` →
   `set_key_view(visited@).contains(e.0)`, `set_key_view(visited@).contains(u@)`.
5. Chap 65, PrimStEph, neighbour loop: `it@`-based →
   `let ghost ng_seq = into_iter_hash_keys(it); for v in vit: it invariant
   vit.seq().unref() == ng_seq, ng_seq.no_duplicates(), ..., forall|e|
   used_pairs.contains(e) ==> (e.0 != u@ || (exists|j: int| 0 <= j < vit.index()
   && #[trigger] ng_seq[j]@ == e.1))`, with `assert(ng_seq[pos] == *v)` in the body.
6. Chap 65, PrimStEph, `mst_weight`: `it@` loop →
   `for edge in mst_edges.iter() invariant mst_edges@.len() > 0, { .. }`.
7. Chap 65, UnionFind ×2, import/broadcast: wrapper → `HashMap`; cfg `key_view`;
   new section 3 `broadcast use { group_hash_axioms, group_key_view_lemmas }`.
8. Chap 65, UnionFind ×2, fields: `HashMapWithViewPlus<V, V>`, `HashMapWithViewPlus<V, usize>`
   → `HashMap<V, V>`, `HashMap<V, usize>`; `::new()` → `HashMap::new()`.
9. Chap 65, UnionFind ×2, every spec: `self.parent@`, `self.rank@`, `uf.parent@`,
   `uf.rank@`, `old(self).parent@`, `old(self).rank@` → `key_view(..)` of each.
10. Chap 65, UnionFindNoPC, `spec_uf_wf`: `uf.parent@.dom().finite() && ...` →
    conjunct deleted.

### 2.13 Chap57–59 (7 files) and deletions

Each of `DijkstraStEphF64/U64`, `BellmanFordStEphF64/I64`,
`JohnsonMtEphF64/StEphF64/StEphI64` lost one broadcast line,
`crate::vstdplus::hash_set_with_view_plus::...::group_hash_set_with_view_plus_axioms,`;
nothing else in them referenced the wrappers. Deleted:
`src/vstdplus/hash_set_with_view_plus.rs`, `hash_map_with_view_plus.rs`,
`hash_set_specs.rs`, `rust_verify_test/tests/vstdplus/HashSetWithViewPlus.rs`,
`HashMapWithViewPlus.rs`; the two `[[test]]` blocks in
`rust_verify_test/Cargo.toml`; the three `pub mod` lines in `src/lib.rs`.

## 3. Alg Analysis annotations

Every `/// - Alg Analysis` line in every edited source file (812 lines in 39
files; `SeqSpecsAndLemmas.rs`, the two `UnionFind*` files and the standard carry
none). `Line` is the annotation's line on disk; the annotation cell is the
Work/Span head of the text, `CR:` for "Code review (Claude Opus 4.6):",
`APAS:` for a textbook citation. Verdicts: `confirmed` (the body has that
work and span), `quote` (an APAS citation, not a claim about the code),
`disputed Dn` (reason Dn in §5 item 1). Totals: 604 confirmed, 136 quotes,
72 disputed.

| # | Chap | File | Line | fn | Annotation | Verdict |
|---|------|------|-----:|----|------------|---------|
| 1 | 05 | SetStEph.rs | 219 | from_vec | APAS: Work O(\|v\|), Span O(1) | quote |
| 2 | 05 | SetStEph.rs | 220 | from_vec | CR: Work O(\|v\|), Span O(\|v\|) | confirmed |
| 3 | 05 | SetStEph.rs | 225 | iter | APAS: Work O(1), Span O(1) | quote |
| 4 | 05 | SetStEph.rs | 226 | iter | CR: Work O(1), Span O(1) | confirmed |
| 5 | 05 | SetStEph.rs | 237 | to_seq | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 6 | 05 | SetStEph.rs | 244 | empty | APAS: Work O(1), Span O(1) | quote |
| 7 | 05 | SetStEph.rs | 245 | empty | CR: Work O(1), Span O(1) | confirmed |
| 8 | 05 | SetStEph.rs | 250 | singleton | APAS: Work O(1), Span O(1) | quote |
| 9 | 05 | SetStEph.rs | 251 | singleton | CR: Work O(1), Span O(1) | confirmed |
| 10 | 05 | SetStEph.rs | 256 | size | APAS: Work O(1), Span O(1) | quote |
| 11 | 05 | SetStEph.rs | 257 | size | CR: Work O(1), Span O(1) | confirmed |
| 12 | 05 | SetStEph.rs | 262 | mem | APAS: Work O(1), Span O(1) | quote |
| 13 | 05 | SetStEph.rs | 263 | mem | CR: Work O(1), Span O(1) | confirmed |
| 14 | 05 | SetStEph.rs | 268 | insert | APAS: Work O(1), Span O(1) | quote |
| 15 | 05 | SetStEph.rs | 269 | insert | CR: Work O(1), Span O(1) | confirmed |
| 16 | 05 | SetStEph.rs | 277 | union | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 17 | 05 | SetStEph.rs | 278 | union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 18 | 05 | SetStEph.rs | 286 | disjoint_union | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 19 | 05 | SetStEph.rs | 287 | disjoint_union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 20 | 05 | SetStEph.rs | 298 | intersection | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 21 | 05 | SetStEph.rs | 299 | intersection | CR: Work O(\|a\|), Span O(\|a\|) | confirmed |
| 22 | 05 | SetStEph.rs | 306 | elt_cross_set | CR: Work O(\|s2\|), Span O(\|s2\|) | confirmed |
| 23 | 05 | SetStEph.rs | 316 | cartesian_product | APAS: Work O(\|a\| × \|b\|), Span O(1) | quote |
| 24 | 05 | SetStEph.rs | 317 | cartesian_product | CR: Work O(\|a\| × \|b\|), Span O(\|a\| × \|b\|) | confirmed |
| 25 | 05 | SetStEph.rs | 327 | all_nonempty | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 26 | 05 | SetStEph.rs | 335 | partition_on_elt | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 27 | 05 | SetStEph.rs | 349 | partition | APAS: Work O(\|a\| × \|parts\|), Span O(1) | quote |
| 28 | 05 | SetStEph.rs | 350 | partition | CR: Work O(\|a\| × \|parts\|), Span O(\|a\| ×… | confirmed |
| 29 | 05 | SetStEph.rs | 368 | split | APAS: Work O(\|self\|), Span O(1) | quote |
| 30 | 05 | SetStEph.rs | 369 | split | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 31 | 05 | SetStEph.rs | 386 | choose | APAS: Work O(1), Span O(1) | quote |
| 32 | 05 | SetStEph.rs | 387 | choose | CR: Work O(1), Span O(1) | confirmed |
| 33 | 05 | SetStEph.rs | 410 | from_vec | CR: Work O(\|v\|), Span O(\|v\|) | confirmed |
| 34 | 05 | SetStEph.rs | 436 | to_seq | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 35 | 05 | SetStEph.rs | 455 | empty | CR: Work O(1), Span O(1) | confirmed |
| 36 | 05 | SetStEph.rs | 460 | singleton | CR: Work O(1), Span O(1) | confirmed |
| 37 | 05 | SetStEph.rs | 471 | size | CR: Work O(1), Span O(1) | confirmed |
| 38 | 05 | SetStEph.rs | 479 | mem | CR: Work O(1), Span O(1) | confirmed |
| 39 | 05 | SetStEph.rs | 485 | insert | CR: Work O(1), Span O(1) | confirmed |
| 40 | 05 | SetStEph.rs | 495 | union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 41 | 05 | SetStEph.rs | 518 | disjoint_union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 42 | 05 | SetStEph.rs | 571 | intersection | CR: Work O(\|a\|), Span O(\|a\|) | confirmed |
| 43 | 05 | SetStEph.rs | 600 | cartesian_product | CR: Work O(\|a\| x \|b\|), Span O(\|a\| x \|b\|) | confirmed |
| 44 | 05 | SetStEph.rs | 629 | elt_cross_set | CR: Work O(\|s2\|), Span O(\|s2\|) | confirmed |
| 45 | 05 | SetStEph.rs | 662 | all_nonempty | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 46 | 05 | SetStEph.rs | 700 | partition_on_elt | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 47 | 05 | SetStEph.rs | 767 | partition | CR: Work O(\|a\| x \|parts\|), Span O(\|a\| x… | confirmed |
| 48 | 05 | SetStEph.rs | 818 | split | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 49 | 05 | SetStEph.rs | 861 | choose | CR: Work O(1), Span O(1) | confirmed |
| 50 | 05 | SetMtEph.rs | 231 | from_vec | APAS: Work O(\|v\|), Span O(1) | quote |
| 51 | 05 | SetMtEph.rs | 232 | from_vec | CR: Work O(\|v\|), Span O(\|v\|) | confirmed |
| 52 | 05 | SetMtEph.rs | 237 | iter | APAS: Work O(1), Span O(1) | quote |
| 53 | 05 | SetMtEph.rs | 238 | iter | CR: Work O(1), Span O(1) | confirmed |
| 54 | 05 | SetMtEph.rs | 249 | to_seq | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 55 | 05 | SetMtEph.rs | 256 | empty | APAS: Work O(1), Span O(1) | quote |
| 56 | 05 | SetMtEph.rs | 257 | empty | CR: Work O(1), Span O(1) | confirmed |
| 57 | 05 | SetMtEph.rs | 262 | singleton | APAS: Work O(1), Span O(1) | quote |
| 58 | 05 | SetMtEph.rs | 263 | singleton | CR: Work O(1), Span O(1) | confirmed |
| 59 | 05 | SetMtEph.rs | 268 | size | APAS: Work O(1), Span O(1) | quote |
| 60 | 05 | SetMtEph.rs | 269 | size | CR: Work O(1), Span O(1) | confirmed |
| 61 | 05 | SetMtEph.rs | 274 | mem | APAS: Work O(1), Span O(1) | quote |
| 62 | 05 | SetMtEph.rs | 275 | mem | CR: Work O(1), Span O(1) | confirmed |
| 63 | 05 | SetMtEph.rs | 280 | insert | APAS: Work O(1), Span O(1) | quote |
| 64 | 05 | SetMtEph.rs | 281 | insert | CR: Work O(1), Span O(1) | confirmed |
| 65 | 05 | SetMtEph.rs | 289 | union | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 66 | 05 | SetMtEph.rs | 290 | union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 67 | 05 | SetMtEph.rs | 298 | disjoint_union | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 68 | 05 | SetMtEph.rs | 299 | disjoint_union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 69 | 05 | SetMtEph.rs | 310 | intersection | APAS: Work O(\|a\| + \|b\|), Span O(1) | quote |
| 70 | 05 | SetMtEph.rs | 311 | intersection | CR: Work O(\|a\|), Span O(\|a\|) | confirmed |
| 71 | 05 | SetMtEph.rs | 318 | elt_cross_set | CR: Work O(\|s2\|), Span O(\|s2\|) | confirmed |
| 72 | 05 | SetMtEph.rs | 328 | cartesian_product | APAS: Work O(\|a\| × \|b\|), Span O(\|b\|) | quote |
| 73 | 05 | SetMtEph.rs | 329 | cartesian_product | CR: Work O(\|a\| × \|b\|), Span O(\|a\| × \|b\|) | confirmed |
| 74 | 05 | SetMtEph.rs | 340 | all_nonempty | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 75 | 05 | SetMtEph.rs | 348 | partition_on_elt | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 76 | 05 | SetMtEph.rs | 362 | partition | APAS: Work O(\|a\| × \|parts\|), Span O(1) | quote |
| 77 | 05 | SetMtEph.rs | 363 | partition | CR: Work O(\|a\| × \|parts\|), Span O(\|a\| ×… | confirmed |
| 78 | 05 | SetMtEph.rs | 381 | split | APAS: Work O(\|self\|), Span O(1) | quote |
| 79 | 05 | SetMtEph.rs | 382 | split | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 80 | 05 | SetMtEph.rs | 398 | choose | APAS: Work O(1), Span O(1) | quote |
| 81 | 05 | SetMtEph.rs | 399 | choose | CR: Work O(1), Span O(1) | confirmed |
| 82 | 05 | SetMtEph.rs | 422 | from_vec | CR: Work O(\|v\|), Span O(\|v\|) | confirmed |
| 83 | 05 | SetMtEph.rs | 448 | to_seq | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 84 | 05 | SetMtEph.rs | 467 | empty | CR: Work O(1), Span O(1) | confirmed |
| 85 | 05 | SetMtEph.rs | 472 | singleton | CR: Work O(1), Span O(1) | confirmed |
| 86 | 05 | SetMtEph.rs | 483 | size | CR: Work O(1), Span O(1) | confirmed |
| 87 | 05 | SetMtEph.rs | 491 | mem | CR: Work O(1), Span O(1) | confirmed |
| 88 | 05 | SetMtEph.rs | 497 | insert | CR: Work O(1), Span O(1) | confirmed |
| 89 | 05 | SetMtEph.rs | 507 | union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 90 | 05 | SetMtEph.rs | 529 | disjoint_union | CR: Work O(\|a\| + \|b\|), Span O(\|a\| + \|b\|) | confirmed |
| 91 | 05 | SetMtEph.rs | 579 | intersection | CR: Work O(\|a\|), Span O(\|a\|) | confirmed |
| 92 | 05 | SetMtEph.rs | 607 | elt_cross_set | CR: Work O(\|s2\|), Span O(\|s2\|) | confirmed |
| 93 | 05 | SetMtEph.rs | 637 | cartesian_product | CR: Work O(\|a\| x \|b\|), Span O(\|a\| x \|b\|) | confirmed |
| 94 | 05 | SetMtEph.rs | 791 | all_nonempty | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 95 | 05 | SetMtEph.rs | 829 | partition_on_elt | CR: Work O(\|parts\|), Span O(\|parts\|) | confirmed |
| 96 | 05 | SetMtEph.rs | 896 | partition | CR: Work O(\|a\| x \|parts\|), Span O(\|a\| x… | confirmed |
| 97 | 05 | SetMtEph.rs | 947 | split | CR: Work O(\|self\|), Span O(\|self\|) | confirmed |
| 98 | 05 | SetMtEph.rs | 990 | choose | CR: Work O(1), Span O(1) | confirmed |
| 99 | 05 | SetMtEph.rs | 1087 | empty | CR: Work O(1), Span O(1) | confirmed |
| 100 | 05 | SetMtEph.rs | 1092 | size | CR: Work O(1), Span O(1) | confirmed |
| 101 | 05 | SetMtEph.rs | 1096 | mem | CR: Work O(1), Span O(1) | confirmed |
| 102 | 05 | SetMtEph.rs | 1100 | insert | CR: Work O(1), Span O(1) | confirmed |
| 103 | 05 | SetMtEph.rs | 1106 | choose | CR: Work O(1), Span O(1) | confirmed |
| 104 | 05 | SetMtEph.rs | 1129 | empty | CR: Work O(1), Span O(1) | confirmed |
| 105 | 05 | SetMtEph.rs | 1139 | size | CR: Work O(1), Span O(1) | confirmed |
| 106 | 05 | SetMtEph.rs | 1150 | mem | CR: Work O(1), Span O(1) | confirmed |
| 107 | 05 | SetMtEph.rs | 1161 | insert | CR: Work O(1), Span O(1) | confirmed |
| 108 | 05 | SetMtEph.rs | 1173 | choose | CR: Work O(1), Span O(1) | confirmed |
| 109 | 17 | MathSeq.rs | 122 | new | CR: O(n) | confirmed |
| 110 | 17 | MathSeq.rs | 128 | set | CR: O(1) | confirmed |
| 111 | 17 | MathSeq.rs | 137 | length | CR: O(1). | confirmed |
| 112 | 17 | MathSeq.rs | 141 | nth | CR: O(1) | confirmed |
| 113 | 17 | MathSeq.rs | 146 | empty | CR: O(1). | confirmed |
| 114 | 17 | MathSeq.rs | 150 | singleton | CR: O(1). | confirmed |
| 115 | 17 | MathSeq.rs | 156 | add_last | CR: amortized O(1) | confirmed |
| 116 | 17 | MathSeq.rs | 163 | delete_last | CR: O(1) | confirmed |
| 117 | 17 | MathSeq.rs | 172 | is_empty | CR: O(1). | confirmed |
| 118 | 17 | MathSeq.rs | 176 | is_singleton | CR: O(1). | confirmed |
| 119 | 17 | MathSeq.rs | 180 | from_vec | CR: O(1) | confirmed |
| 120 | 17 | MathSeq.rs | 184 | with_len | CR: O(n) | confirmed |
| 121 | 17 | MathSeq.rs | 190 | subseq | CR: O(1) | confirmed |
| 122 | 17 | MathSeq.rs | 200 | subseq_copy | CR: O(length) | confirmed |
| 123 | 17 | MathSeq.rs | 208 | domain | CR: O(n) | confirmed |
| 124 | 17 | MathSeq.rs | 214 | range | CR: O(n) expected | confirmed |
| 125 | 17 | MathSeq.rs | 221 | multiset_range | CR: O(n) expected | confirmed |
| 126 | 17 | MathSeq.rs | 230 | iter | CR: O(1) | disputed D1 |
| 127 | 17 | MathSeq.rs | 263 | new | CR: Work O(n), Span O(n) | confirmed |
| 128 | 17 | MathSeq.rs | 270 | set | CR: Work O(1), Span O(1) | confirmed |
| 129 | 17 | MathSeq.rs | 281 | length | CR: Work O(1), Span O(1) | confirmed |
| 130 | 17 | MathSeq.rs | 287 | nth | CR: Work O(1), Span O(1) | confirmed |
| 131 | 17 | MathSeq.rs | 293 | empty | CR: Work O(1), Span O(1) | confirmed |
| 132 | 17 | MathSeq.rs | 299 | singleton | CR: Work O(1), Span O(1) | confirmed |
| 133 | 17 | MathSeq.rs | 305 | add_last | CR: Work O(1) amortized, Span O(1) amor… | confirmed |
| 134 | 17 | MathSeq.rs | 311 | delete_last | CR: Work O(1), Span O(1) | confirmed |
| 135 | 17 | MathSeq.rs | 317 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 136 | 17 | MathSeq.rs | 323 | is_singleton | CR: Work O(1), Span O(1) | confirmed |
| 137 | 17 | MathSeq.rs | 329 | from_vec | CR: Work O(1), Span O(1) | confirmed |
| 138 | 17 | MathSeq.rs | 335 | with_len | CR: Work O(n), Span O(n) | confirmed |
| 139 | 17 | MathSeq.rs | 341 | subseq | CR: Work O(1), Span O(1) | confirmed |
| 140 | 17 | MathSeq.rs | 351 | subseq_copy | CR: Work O(k), Span O(k) | confirmed |
| 141 | 17 | MathSeq.rs | 361 | domain | CR: Work O(n), Span O(n) | confirmed |
| 142 | 17 | MathSeq.rs | 380 | range | CR: Work O(n), Span O(n) | confirmed |
| 143 | 17 | MathSeq.rs | 434 | multiset_range | CR: Work O(n), Span O(n) | confirmed |
| 144 | 17 | MathSeq.rs | 617 | iter_mut | CR: O(1) | disputed D1 |
| 145 | 49 | MinEditDistStEph.rs | 97 | new | CR: Work O(1), Span O(1) | confirmed |
| 146 | 49 | MinEditDistStEph.rs | 105 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 147 | 49 | MinEditDistStEph.rs | 112 | min_edit_distance | APAS: Work O(\|S\| * \|T\|), Span O(\|S\| + \|… | quote |
| 148 | 49 | MinEditDistStEph.rs | 113 | min_edit_distance | CR: Work O(\|S\|·\|T\|), Span O(\|S\|·\|T\|) | confirmed |
| 149 | 49 | MinEditDistStEph.rs | 121 | source | CR: Work O(1), Span O(1) | confirmed |
| 150 | 49 | MinEditDistStEph.rs | 126 | target | CR: Work O(1), Span O(1) | confirmed |
| 151 | 49 | MinEditDistStEph.rs | 131 | set_source | CR: Work O(n), Span O(n) | confirmed |
| 152 | 49 | MinEditDistStEph.rs | 139 | set_target | CR: Work O(n), Span O(n) | confirmed |
| 153 | 49 | MinEditDistStEph.rs | 147 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 154 | 49 | MinEditDistStEph.rs | 154 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 155 | 49 | MinEditDistStEph.rs | 162 | min_edit_distance_rec | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) | quote |
| 156 | 49 | MinEditDistStEph.rs | 163 | min_edit_distance_rec | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 157 | 49 | MinEditDistStEph.rs | 218 | new | CR: Work O(1), Span O(1) | confirmed |
| 158 | 49 | MinEditDistStEph.rs | 232 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 159 | 49 | MinEditDistStEph.rs | 243 | min_edit_distance | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 160 | 49 | MinEditDistStEph.rs | 253 | source | CR: Work O(1), Span O(1) | confirmed |
| 161 | 49 | MinEditDistStEph.rs | 256 | target | CR: Work O(1), Span O(1) | confirmed |
| 162 | 49 | MinEditDistStEph.rs | 259 | set_source | CR: Work O(n), Span O(n) | confirmed |
| 163 | 49 | MinEditDistStEph.rs | 265 | set_target | CR: Work O(n), Span O(n) | confirmed |
| 164 | 49 | MinEditDistStEph.rs | 271 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 165 | 49 | MinEditDistStEph.rs | 274 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 166 | 49 | MinEditDistStEph.rs | 301 | source_mut | CR: Work O(1), Span O(1) | confirmed |
| 167 | 49 | MinEditDistStEph.rs | 305 | target_mut | CR: Work O(1), Span O(1) | confirmed |
| 168 | 49 | MinEditDistStPer.rs | 97 | new | CR: Work O(1), Span O(1) | confirmed |
| 169 | 49 | MinEditDistStPer.rs | 105 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 170 | 49 | MinEditDistStPer.rs | 112 | min_edit_distance | APAS: Work O(\|S\| * \|T\|), Span O(\|S\| + \|… | quote |
| 171 | 49 | MinEditDistStPer.rs | 113 | min_edit_distance | CR: Work O(\|S\|·\|T\|), Span O(\|S\|·\|T\|) | confirmed |
| 172 | 49 | MinEditDistStPer.rs | 118 | source | CR: Work O(1), Span O(1) | confirmed |
| 173 | 49 | MinEditDistStPer.rs | 123 | target | CR: Work O(1), Span O(1) | confirmed |
| 174 | 49 | MinEditDistStPer.rs | 128 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 175 | 49 | MinEditDistStPer.rs | 136 | min_edit_distance_rec | APAS: Work O(\|S\|×\|T\|), Span O(\|S\|+\|T\|) | quote |
| 176 | 49 | MinEditDistStPer.rs | 137 | min_edit_distance_rec | CR: Work O(\|S\|×\|T\|), Span O(\|S\|×\|T\|) | confirmed |
| 177 | 49 | MinEditDistStPer.rs | 192 | new | CR: Work O(1), Span O(1) | confirmed |
| 178 | 49 | MinEditDistStPer.rs | 206 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 179 | 49 | MinEditDistStPer.rs | 218 | min_edit_distance | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 180 | 49 | MinEditDistStPer.rs | 233 | source | CR: Work O(1), Span O(1) | confirmed |
| 181 | 49 | MinEditDistStPer.rs | 236 | target | CR: Work O(1), Span O(1) | confirmed |
| 182 | 49 | MinEditDistStPer.rs | 239 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 183 | 49 | MinEditDistMtEph.rs | 102 | new | CR: Work O(1), Span O(1) | confirmed |
| 184 | 49 | MinEditDistMtEph.rs | 113 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 185 | 49 | MinEditDistMtEph.rs | 121 | min_edit_distance | APAS: Work O(\|S\| * \|T\|), Span O(\|S\| + \|… | quote |
| 186 | 49 | MinEditDistMtEph.rs | 122 | min_edit_distance | CR: Work O(\|S\|·\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 187 | 49 | MinEditDistMtEph.rs | 132 | source | CR: Work O(1), Span O(1) | confirmed |
| 188 | 49 | MinEditDistMtEph.rs | 137 | target | CR: Work O(1), Span O(1) | confirmed |
| 189 | 49 | MinEditDistMtEph.rs | 142 | set_source | CR: Work O(n), Span O(n) | confirmed |
| 190 | 49 | MinEditDistMtEph.rs | 152 | set_target | CR: Work O(n), Span O(n) | confirmed |
| 191 | 49 | MinEditDistMtEph.rs | 162 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 192 | 49 | MinEditDistMtEph.rs | 170 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 193 | 49 | MinEditDistMtEph.rs | 178 | new_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 194 | 49 | MinEditDistMtEph.rs | 189 | clone_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 195 | 49 | MinEditDistMtEph.rs | 200 | min_edit_distance_rec | APAS: Work O(\|S\|×\|T\|), Span O(\|S\|+\|T\|) | quote |
| 196 | 49 | MinEditDistMtEph.rs | 201 | min_edit_distance_rec | CR: Work O(\|S\|×\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 197 | 49 | MinEditDistMtEph.rs | 320 | new | CR: Work O(1), Span O(1) | confirmed |
| 198 | 49 | MinEditDistMtEph.rs | 334 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 199 | 49 | MinEditDistMtEph.rs | 345 | min_edit_distance | CR: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 200 | 49 | MinEditDistMtEph.rs | 360 | source | CR: Work O(1), Span O(1) | confirmed |
| 201 | 49 | MinEditDistMtEph.rs | 363 | target | CR: Work O(1), Span O(1) | confirmed |
| 202 | 49 | MinEditDistMtEph.rs | 366 | set_source | CR: Work O(n), Span O(n) | confirmed |
| 203 | 49 | MinEditDistMtEph.rs | 374 | set_target | CR: Work O(n), Span O(n) | confirmed |
| 204 | 49 | MinEditDistMtEph.rs | 382 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 205 | 49 | MinEditDistMtEph.rs | 389 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 206 | 49 | MinEditDistMtEph.rs | 430 | source_mut | CR: Work O(1), Span O(1) | confirmed |
| 207 | 49 | MinEditDistMtEph.rs | 434 | target_mut | CR: Work O(1), Span O(1) | confirmed |
| 208 | 49 | MinEditDistMtPer.rs | 100 | new | CR: Work O(1), Span O(1) | confirmed |
| 209 | 49 | MinEditDistMtPer.rs | 111 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 210 | 49 | MinEditDistMtPer.rs | 119 | min_edit_distance | APAS: Work O(\|S\| * \|T\|), Span O(\|S\| + \|… | quote |
| 211 | 49 | MinEditDistMtPer.rs | 120 | min_edit_distance | CR: Work O(\|S\|·\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 212 | 49 | MinEditDistMtPer.rs | 127 | source | CR: Work O(1), Span O(1) | confirmed |
| 213 | 49 | MinEditDistMtPer.rs | 132 | target | CR: Work O(1), Span O(1) | confirmed |
| 214 | 49 | MinEditDistMtPer.rs | 137 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 215 | 49 | MinEditDistMtPer.rs | 145 | new_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 216 | 49 | MinEditDistMtPer.rs | 156 | clone_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 217 | 49 | MinEditDistMtPer.rs | 167 | min_edit_distance_rec | APAS: Work O(\|S\|×\|T\|), Span O(\|S\|+\|T\|) | quote |
| 218 | 49 | MinEditDistMtPer.rs | 168 | min_edit_distance_rec | CR: Work O(\|S\|×\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 219 | 49 | MinEditDistMtPer.rs | 284 | new | CR: Work O(1), Span O(1) | confirmed |
| 220 | 49 | MinEditDistMtPer.rs | 298 | from_sequences | CR: Work O(1), Span O(1) | confirmed |
| 221 | 49 | MinEditDistMtPer.rs | 309 | min_edit_distance | CR: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 222 | 49 | MinEditDistMtPer.rs | 325 | source | CR: Work O(1), Span O(1) | confirmed |
| 223 | 49 | MinEditDistMtPer.rs | 328 | target | CR: Work O(1), Span O(1) | confirmed |
| 224 | 49 | MinEditDistMtPer.rs | 331 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 225 | 49 | SubsetSumStEph.rs | 87 | new | CR: Work O(1), Span O(1) | confirmed |
| 226 | 49 | SubsetSumStEph.rs | 95 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 227 | 49 | SubsetSumStEph.rs | 100 | subset_sum | APAS: Work O(k * \|S\|), Span O(\|S\|) | quote |
| 228 | 49 | SubsetSumStEph.rs | 101 | subset_sum | CR: Work O(k·\|S\|), Span O(k·\|S\|) | confirmed |
| 229 | 49 | SubsetSumStEph.rs | 108 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 230 | 49 | SubsetSumStEph.rs | 113 | set | CR: Work O(n), Span O(n) | confirmed |
| 231 | 49 | SubsetSumStEph.rs | 119 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 232 | 49 | SubsetSumStEph.rs | 124 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 233 | 49 | SubsetSumStEph.rs | 132 | subset_sum_rec | APAS: Work O(k*\|S\|), Span O(\|S\|) | quote |
| 234 | 49 | SubsetSumStEph.rs | 133 | subset_sum_rec | CR: Work O(k*\|S\|), Span O(k*\|S\|) | confirmed |
| 235 | 49 | SubsetSumStEph.rs | 171 | new | CR: Work O(1), Span O(1) | confirmed |
| 236 | 49 | SubsetSumStEph.rs | 184 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 237 | 49 | SubsetSumStEph.rs | 194 | subset_sum | CR: Work O(k*\|S\|), Span O(k*\|S\|) | confirmed |
| 238 | 49 | SubsetSumStEph.rs | 209 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 239 | 49 | SubsetSumStEph.rs | 212 | set | CR: Work O(n), Span O(n) | confirmed |
| 240 | 49 | SubsetSumStEph.rs | 218 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 241 | 49 | SubsetSumStEph.rs | 221 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 242 | 49 | SubsetSumStEph.rs | 247 | multiset_mut | CR: Work O(1), Span O(1) | confirmed |
| 243 | 49 | SubsetSumStPer.rs | 87 | new | CR: Work O(1), Span O(1) | confirmed |
| 244 | 49 | SubsetSumStPer.rs | 95 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 245 | 49 | SubsetSumStPer.rs | 100 | subset_sum | APAS: Work O(k * \|S\|), Span O(\|S\|) | quote |
| 246 | 49 | SubsetSumStPer.rs | 101 | subset_sum | CR: Work O(k·\|S\|), Span O(k·\|S\|) | confirmed |
| 247 | 49 | SubsetSumStPer.rs | 107 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 248 | 49 | SubsetSumStPer.rs | 112 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 249 | 49 | SubsetSumStPer.rs | 120 | subset_sum_rec | APAS: Work O(k×\|S\|), Span O(\|S\|) | quote |
| 250 | 49 | SubsetSumStPer.rs | 121 | subset_sum_rec | CR: Work O(k×\|S\|), Span O(k×\|S\|) | confirmed |
| 251 | 49 | SubsetSumStPer.rs | 159 | new | CR: Work O(1), Span O(1) | confirmed |
| 252 | 49 | SubsetSumStPer.rs | 172 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 253 | 49 | SubsetSumStPer.rs | 182 | subset_sum | CR: Work O(k*\|S\|), Span O(k*\|S\|) | confirmed |
| 254 | 49 | SubsetSumStPer.rs | 202 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 255 | 49 | SubsetSumStPer.rs | 205 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 256 | 49 | SubsetSumMtEph.rs | 95 | new | CR: Work O(1), Span O(1) | confirmed |
| 257 | 49 | SubsetSumMtEph.rs | 105 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 258 | 49 | SubsetSumMtEph.rs | 112 | subset_sum | APAS: Work O(k * \|S\|), Span O(\|S\|) | quote |
| 259 | 49 | SubsetSumMtEph.rs | 113 | subset_sum | CR: Work O(k·\|S\|), Span O(\|S\|) | confirmed |
| 260 | 49 | SubsetSumMtEph.rs | 120 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 261 | 49 | SubsetSumMtEph.rs | 125 | set | CR: Work O(n), Span O(n) | confirmed |
| 262 | 49 | SubsetSumMtEph.rs | 133 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 263 | 49 | SubsetSumMtEph.rs | 139 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 264 | 49 | SubsetSumMtEph.rs | 147 | new_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 265 | 49 | SubsetSumMtEph.rs | 157 | clone_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 266 | 49 | SubsetSumMtEph.rs | 168 | subset_sum_rec | APAS: Work O(k×\|S\|), Span O(\|S\|) | quote |
| 267 | 49 | SubsetSumMtEph.rs | 169 | subset_sum_rec | CR: Work O(k×\|S\|), Span O(\|S\|) | confirmed |
| 268 | 49 | SubsetSumMtEph.rs | 260 | new | CR: Work O(1), Span O(1) | confirmed |
| 269 | 49 | SubsetSumMtEph.rs | 273 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 270 | 49 | SubsetSumMtEph.rs | 283 | subset_sum | CR: Work O(k*\|S\|), Span O(\|S\|) | confirmed |
| 271 | 49 | SubsetSumMtEph.rs | 302 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 272 | 49 | SubsetSumMtEph.rs | 305 | set | CR: Work O(n), Span O(n) | confirmed |
| 273 | 49 | SubsetSumMtEph.rs | 313 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 274 | 49 | SubsetSumMtEph.rs | 320 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 275 | 49 | SubsetSumMtEph.rs | 363 | multiset_mut | CR: Work O(1), Span O(1) | confirmed |
| 276 | 49 | SubsetSumMtPer.rs | 93 | new | CR: Work O(1), Span O(1) | confirmed |
| 277 | 49 | SubsetSumMtPer.rs | 103 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 278 | 49 | SubsetSumMtPer.rs | 110 | subset_sum | APAS: Work O(k * \|S\|), Span O(\|S\|) | quote |
| 279 | 49 | SubsetSumMtPer.rs | 111 | subset_sum | CR: Work O(k·\|S\|), Span O(\|S\|) | confirmed |
| 280 | 49 | SubsetSumMtPer.rs | 117 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 281 | 49 | SubsetSumMtPer.rs | 122 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 282 | 49 | SubsetSumMtPer.rs | 130 | new_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 283 | 49 | SubsetSumMtPer.rs | 140 | clone_arc_memo | CR: Work O(1), Span O(1) | confirmed |
| 284 | 49 | SubsetSumMtPer.rs | 151 | subset_sum_rec | APAS: Work O(k×\|S\|), Span O(\|S\|) | quote |
| 285 | 49 | SubsetSumMtPer.rs | 152 | subset_sum_rec | CR: Work O(k×\|S\|), Span O(\|S\|) | confirmed |
| 286 | 49 | SubsetSumMtPer.rs | 243 | new | CR: Work O(1), Span O(1) | confirmed |
| 287 | 49 | SubsetSumMtPer.rs | 256 | from_multiset | CR: Work O(1), Span O(1) | confirmed |
| 288 | 49 | SubsetSumMtPer.rs | 266 | subset_sum | CR: Work O(k*\|S\|), Span O(\|S\|) | confirmed |
| 289 | 49 | SubsetSumMtPer.rs | 285 | multiset | CR: Work O(1), Span O(1) | confirmed |
| 290 | 49 | SubsetSumMtPer.rs | 288 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 291 | 50 | MatrixChainStEph.rs | 175 | new | CR: Work O(1), Span O(1) | confirmed |
| 292 | 50 | MatrixChainStEph.rs | 182 | from_dimensions | CR: Work O(1), Span O(1) | confirmed |
| 293 | 50 | MatrixChainStEph.rs | 189 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 294 | 50 | MatrixChainStEph.rs | 196 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 295 | 50 | MatrixChainStEph.rs | 207 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 296 | 50 | MatrixChainStEph.rs | 211 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 297 | 50 | MatrixChainStEph.rs | 215 | set_dimension | CR: Work O(n), Span O(n) | disputed D3 |
| 298 | 50 | MatrixChainStEph.rs | 223 | update_dimension | CR: Work O(n), Span O(n) | disputed D3 |
| 299 | 50 | MatrixChainStEph.rs | 232 | clear_memo | CR: Work O(n), Span O(n) | disputed D3 |
| 300 | 50 | MatrixChainStEph.rs | 239 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 301 | 50 | MatrixChainStEph.rs | 243 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 302 | 50 | MatrixChainStEph.rs | 254 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 303 | 50 | MatrixChainStEph.rs | 279 | new | CR: Work O(1), Span O(1) | confirmed |
| 304 | 50 | MatrixChainStEph.rs | 294 | from_dimensions | CR: Work O(1), Span O(1) | confirmed |
| 305 | 50 | MatrixChainStEph.rs | 309 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 306 | 50 | MatrixChainStEph.rs | 337 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 307 | 50 | MatrixChainStEph.rs | 347 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 308 | 50 | MatrixChainStEph.rs | 399 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 309 | 50 | MatrixChainStEph.rs | 411 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 310 | 50 | MatrixChainStEph.rs | 416 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 311 | 50 | MatrixChainStEph.rs | 421 | set_dimension | CR: Work O(n), Span O(n) | disputed D3 |
| 312 | 50 | MatrixChainStEph.rs | 427 | update_dimension | CR: Work O(n), Span O(n) | disputed D3 |
| 313 | 50 | MatrixChainStEph.rs | 433 | clear_memo | CR: Work O(n), Span O(n) | disputed D3 |
| 314 | 50 | MatrixChainStEph.rs | 436 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 315 | 50 | MatrixChainStEph.rs | 505 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 316 | 50 | MatrixChainStEph.rs | 506 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 317 | 50 | MatrixChainStEph.rs | 514 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 318 | 50 | MatrixChainStEph.rs | 515 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 319 | 50 | MatrixChainStEph.rs | 522 | fmt | APAS: Work O(1), Span O(1) | quote |
| 320 | 50 | MatrixChainStEph.rs | 523 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 321 | 50 | MatrixChainStEph.rs | 540 | fmt | APAS: Work O(1), Span O(1) | quote |
| 322 | 50 | MatrixChainStEph.rs | 541 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 323 | 50 | MatrixChainStEph.rs | 556 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 324 | 50 | MatrixChainStEph.rs | 557 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 325 | 50 | MatrixChainStPer.rs | 174 | new | CR: Work O(1), Span O(1) | confirmed |
| 326 | 50 | MatrixChainStPer.rs | 181 | from_dimensions | CR: Work O(1), Span O(1) | confirmed |
| 327 | 50 | MatrixChainStPer.rs | 188 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 328 | 50 | MatrixChainStPer.rs | 195 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 329 | 50 | MatrixChainStPer.rs | 205 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 330 | 50 | MatrixChainStPer.rs | 209 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 331 | 50 | MatrixChainStPer.rs | 213 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 332 | 50 | MatrixChainStPer.rs | 217 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 333 | 50 | MatrixChainStPer.rs | 228 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 334 | 50 | MatrixChainStPer.rs | 253 | new | CR: Work O(1), Span O(1) | confirmed |
| 335 | 50 | MatrixChainStPer.rs | 268 | from_dimensions | CR: Work O(1), Span O(1) | confirmed |
| 336 | 50 | MatrixChainStPer.rs | 283 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 337 | 50 | MatrixChainStPer.rs | 311 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 338 | 50 | MatrixChainStPer.rs | 321 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 339 | 50 | MatrixChainStPer.rs | 374 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 340 | 50 | MatrixChainStPer.rs | 386 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 341 | 50 | MatrixChainStPer.rs | 391 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 342 | 50 | MatrixChainStPer.rs | 396 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 343 | 50 | MatrixChainStPer.rs | 450 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 344 | 50 | MatrixChainStPer.rs | 451 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 345 | 50 | MatrixChainStPer.rs | 458 | fmt | APAS: Work O(1), Span O(1) | quote |
| 346 | 50 | MatrixChainStPer.rs | 459 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 347 | 50 | MatrixChainStPer.rs | 476 | fmt | APAS: Work O(1), Span O(1) | quote |
| 348 | 50 | MatrixChainStPer.rs | 477 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 349 | 50 | MatrixChainStPer.rs | 492 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 350 | 50 | MatrixChainStPer.rs | 493 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 351 | 50 | MatrixChainMtEph.rs | 162 | new | CR: Work O(1), Span O(1) | confirmed |
| 352 | 50 | MatrixChainMtEph.rs | 166 | from_dimensions | CR: Work O(n), Span O(n) | confirmed |
| 353 | 50 | MatrixChainMtEph.rs | 170 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 354 | 50 | MatrixChainMtEph.rs | 174 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 355 | 50 | MatrixChainMtEph.rs | 187 | dimensions | CR: Work O(n), Span O(n) | confirmed |
| 356 | 50 | MatrixChainMtEph.rs | 192 | set_dimension | CR: Work O(n), Span O(n) | confirmed |
| 357 | 50 | MatrixChainMtEph.rs | 199 | update_dimension | CR: Work O(n), Span O(n) | confirmed |
| 358 | 50 | MatrixChainMtEph.rs | 207 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 359 | 50 | MatrixChainMtEph.rs | 212 | clear_memo | CR: Work O(1), Span O(1) | confirmed |
| 360 | 50 | MatrixChainMtEph.rs | 217 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 361 | 50 | MatrixChainMtEph.rs | 220 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 362 | 50 | MatrixChainMtEph.rs | 232 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 363 | 50 | MatrixChainMtEph.rs | 244 | parallel_min_reduction | CR: Work O(n), Span O(n) | confirmed |
| 364 | 50 | MatrixChainMtEph.rs | 280 | new | CR: Work O(1), Span O(1) | confirmed |
| 365 | 50 | MatrixChainMtEph.rs | 291 | from_dimensions | CR: Work O(n), Span O(n) | confirmed |
| 366 | 50 | MatrixChainMtEph.rs | 304 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 367 | 50 | MatrixChainMtEph.rs | 330 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 368 | 50 | MatrixChainMtEph.rs | 350 | parallel_min_reduction | CR: Work O(n), Span O(n) | confirmed |
| 369 | 50 | MatrixChainMtEph.rs | 370 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 370 | 50 | MatrixChainMtEph.rs | 473 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 371 | 50 | MatrixChainMtEph.rs | 491 | dimensions | CR: Work O(n), Span O(n) | confirmed |
| 372 | 50 | MatrixChainMtEph.rs | 503 | set_dimension | CR: Work O(n), Span O(n) | confirmed |
| 373 | 50 | MatrixChainMtEph.rs | 521 | update_dimension | CR: Work O(n), Span O(n) | confirmed |
| 374 | 50 | MatrixChainMtEph.rs | 540 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 375 | 50 | MatrixChainMtEph.rs | 553 | clear_memo | CR: Work O(1), Span O(1) | confirmed |
| 376 | 50 | MatrixChainMtEph.rs | 562 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 377 | 50 | MatrixChainMtEph.rs | 663 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 378 | 50 | MatrixChainMtEph.rs | 664 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 379 | 50 | MatrixChainMtEph.rs | 677 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 380 | 50 | MatrixChainMtEph.rs | 678 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 381 | 50 | MatrixChainMtEph.rs | 690 | fmt | APAS: Work O(1), Span O(1) | quote |
| 382 | 50 | MatrixChainMtEph.rs | 691 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 383 | 50 | MatrixChainMtEph.rs | 722 | fmt | APAS: Work O(1), Span O(1) | quote |
| 384 | 50 | MatrixChainMtEph.rs | 723 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 385 | 50 | MatrixChainMtEph.rs | 742 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 386 | 50 | MatrixChainMtEph.rs | 743 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 387 | 50 | MatrixChainMtPer.rs | 161 | new | CR: Work O(1), Span O(1) | confirmed |
| 388 | 50 | MatrixChainMtPer.rs | 165 | from_dimensions | CR: Work O(n), Span O(n) | confirmed |
| 389 | 50 | MatrixChainMtPer.rs | 169 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 390 | 50 | MatrixChainMtPer.rs | 173 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 391 | 50 | MatrixChainMtPer.rs | 184 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 392 | 50 | MatrixChainMtPer.rs | 188 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 393 | 50 | MatrixChainMtPer.rs | 192 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 394 | 50 | MatrixChainMtPer.rs | 195 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 395 | 50 | MatrixChainMtPer.rs | 206 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 396 | 50 | MatrixChainMtPer.rs | 218 | parallel_min_reduction | CR: Work O(n), Span O(n) | confirmed |
| 397 | 50 | MatrixChainMtPer.rs | 252 | new | CR: Work O(1), Span O(1) | confirmed |
| 398 | 50 | MatrixChainMtPer.rs | 262 | from_dimensions | CR: Work O(n), Span O(n) | confirmed |
| 399 | 50 | MatrixChainMtPer.rs | 273 | from_dim_pairs | CR: Work O(n), Span O(n) | confirmed |
| 400 | 50 | MatrixChainMtPer.rs | 298 | multiply_cost | CR: Work O(1), Span O(1) | confirmed |
| 401 | 50 | MatrixChainMtPer.rs | 308 | parallel_min_reduction | CR: Work O(n), Span O(n) | confirmed |
| 402 | 50 | MatrixChainMtPer.rs | 328 | matrix_chain_rec | CR: Work O(n^3), Span O(n^3) | confirmed |
| 403 | 50 | MatrixChainMtPer.rs | 431 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 404 | 50 | MatrixChainMtPer.rs | 449 | dimensions | CR: Work O(1), Span O(1) | confirmed |
| 405 | 50 | MatrixChainMtPer.rs | 452 | num_matrices | CR: Work O(1), Span O(1) | confirmed |
| 406 | 50 | MatrixChainMtPer.rs | 458 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 407 | 50 | MatrixChainMtPer.rs | 538 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 408 | 50 | MatrixChainMtPer.rs | 539 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 409 | 50 | MatrixChainMtPer.rs | 546 | fmt | APAS: Work O(1), Span O(1) | quote |
| 410 | 50 | MatrixChainMtPer.rs | 547 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 411 | 50 | MatrixChainMtPer.rs | 578 | fmt | APAS: Work O(1), Span O(1) | quote |
| 412 | 50 | MatrixChainMtPer.rs | 579 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 413 | 50 | MatrixChainMtPer.rs | 597 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 414 | 50 | MatrixChainMtPer.rs | 598 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 415 | 50 | OptBinSearchTreeStEph.rs | 98 | new | CR: Work O(1), Span O(1) | confirmed |
| 416 | 50 | OptBinSearchTreeStEph.rs | 104 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 417 | 50 | OptBinSearchTreeStEph.rs | 111 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 418 | 50 | OptBinSearchTreeStEph.rs | 117 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 419 | 50 | OptBinSearchTreeStEph.rs | 120 | keys | CR: Work O(1), Span O(1) | confirmed |
| 420 | 50 | OptBinSearchTreeStEph.rs | 124 | set_key_prob | CR: Work O(1), Span O(1) | disputed D2 |
| 421 | 50 | OptBinSearchTreeStEph.rs | 131 | update_prob | CR: Work O(1), Span O(1) | disputed D2 |
| 422 | 50 | OptBinSearchTreeStEph.rs | 138 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 423 | 50 | OptBinSearchTreeStEph.rs | 142 | clear_memo | CR: Work O(1), Span O(1) | disputed D2 |
| 424 | 50 | OptBinSearchTreeStEph.rs | 148 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 425 | 50 | OptBinSearchTreeStEph.rs | 157 | new | CR: Work O(1), Span O(1) | confirmed |
| 426 | 50 | OptBinSearchTreeStEph.rs | 167 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 427 | 50 | OptBinSearchTreeStEph.rs | 191 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 428 | 50 | OptBinSearchTreeStEph.rs | 200 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 429 | 50 | OptBinSearchTreeStEph.rs | 212 | keys | CR: Work O(1), Span O(1) | confirmed |
| 430 | 50 | OptBinSearchTreeStEph.rs | 215 | set_key_prob | CR: Work O(1), Span O(1) | disputed D2 |
| 431 | 50 | OptBinSearchTreeStEph.rs | 221 | update_prob | CR: Work O(1), Span O(1) | disputed D2 |
| 432 | 50 | OptBinSearchTreeStEph.rs | 228 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 433 | 50 | OptBinSearchTreeStEph.rs | 231 | clear_memo | CR: Work O(1), Span O(1) | disputed D2 |
| 434 | 50 | OptBinSearchTreeStEph.rs | 234 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 435 | 50 | OptBinSearchTreeStEph.rs | 243 | obst_rec_st_eph | APAS: Work O(n^3), Span O(n lg n) | quote |
| 436 | 50 | OptBinSearchTreeStEph.rs | 244 | obst_rec_st_eph | CR: Work O(n^3), Span O(n^3) | confirmed |
| 437 | 50 | OptBinSearchTreeStEph.rs | 358 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 438 | 50 | OptBinSearchTreeStEph.rs | 359 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 439 | 50 | OptBinSearchTreeStEph.rs | 367 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 440 | 50 | OptBinSearchTreeStEph.rs | 368 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 441 | 50 | OptBinSearchTreeStEph.rs | 375 | fmt | APAS: Work O(1), Span O(1) | quote |
| 442 | 50 | OptBinSearchTreeStEph.rs | 376 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 443 | 50 | OptBinSearchTreeStEph.rs | 393 | fmt | APAS: Work O(1), Span O(1) | quote |
| 444 | 50 | OptBinSearchTreeStEph.rs | 394 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 445 | 50 | OptBinSearchTreeStEph.rs | 409 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 446 | 50 | OptBinSearchTreeStEph.rs | 410 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 447 | 50 | OptBinSearchTreeStPer.rs | 98 | new | CR: Work O(1), Span O(1) | confirmed |
| 448 | 50 | OptBinSearchTreeStPer.rs | 104 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 449 | 50 | OptBinSearchTreeStPer.rs | 111 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 450 | 50 | OptBinSearchTreeStPer.rs | 117 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 451 | 50 | OptBinSearchTreeStPer.rs | 120 | keys | CR: Work O(1), Span O(1) | confirmed |
| 452 | 50 | OptBinSearchTreeStPer.rs | 124 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 453 | 50 | OptBinSearchTreeStPer.rs | 128 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 454 | 50 | OptBinSearchTreeStPer.rs | 137 | new | CR: Work O(1), Span O(1) | confirmed |
| 455 | 50 | OptBinSearchTreeStPer.rs | 147 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 456 | 50 | OptBinSearchTreeStPer.rs | 172 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 457 | 50 | OptBinSearchTreeStPer.rs | 181 | optimal_cost | CR: Work O(n^3), Span O(n^3) | confirmed |
| 458 | 50 | OptBinSearchTreeStPer.rs | 194 | keys | CR: Work O(1), Span O(1) | confirmed |
| 459 | 50 | OptBinSearchTreeStPer.rs | 197 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 460 | 50 | OptBinSearchTreeStPer.rs | 200 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 461 | 50 | OptBinSearchTreeStPer.rs | 209 | obst_rec_st_per | APAS: Work O(n^3), Span O(n lg n) | quote |
| 462 | 50 | OptBinSearchTreeStPer.rs | 210 | obst_rec_st_per | CR: Work O(n^3), Span O(n^3) | confirmed |
| 463 | 50 | OptBinSearchTreeStPer.rs | 324 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 464 | 50 | OptBinSearchTreeStPer.rs | 325 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 465 | 50 | OptBinSearchTreeStPer.rs | 332 | fmt | APAS: Work O(1), Span O(1) | quote |
| 466 | 50 | OptBinSearchTreeStPer.rs | 333 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 467 | 50 | OptBinSearchTreeStPer.rs | 350 | fmt | APAS: Work O(1), Span O(1) | quote |
| 468 | 50 | OptBinSearchTreeStPer.rs | 351 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 469 | 50 | OptBinSearchTreeStPer.rs | 366 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 470 | 50 | OptBinSearchTreeStPer.rs | 367 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 471 | 50 | OptBinSearchTreeMtEph.rs | 100 | new | CR: Work O(1), Span O(1) | confirmed |
| 472 | 50 | OptBinSearchTreeMtEph.rs | 104 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 473 | 50 | OptBinSearchTreeMtEph.rs | 109 | from_key_probs | CR: Work O(n), Span O(n) | confirmed |
| 474 | 50 | OptBinSearchTreeMtEph.rs | 113 | optimal_cost | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 475 | 50 | OptBinSearchTreeMtEph.rs | 117 | keys | CR: Work O(n), Span O(n) | confirmed |
| 476 | 50 | OptBinSearchTreeMtEph.rs | 122 | set_key_prob | CR: Work O(n), Span O(n) | confirmed |
| 477 | 50 | OptBinSearchTreeMtEph.rs | 129 | update_prob | CR: Work O(n), Span O(n) | confirmed |
| 478 | 50 | OptBinSearchTreeMtEph.rs | 136 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 479 | 50 | OptBinSearchTreeMtEph.rs | 141 | clear_memo | CR: Work O(1), Span O(1) | confirmed |
| 480 | 50 | OptBinSearchTreeMtEph.rs | 146 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 481 | 50 | OptBinSearchTreeMtEph.rs | 153 | obst_rec | APAS: Work O(n^3), Span O(n lg n) | quote |
| 482 | 50 | OptBinSearchTreeMtEph.rs | 154 | obst_rec | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 483 | 50 | OptBinSearchTreeMtEph.rs | 209 | parallel_min_split_cost | CR: Work O(l), Span O(lg l) | confirmed |
| 484 | 50 | OptBinSearchTreeMtEph.rs | 299 | new | CR: Work O(1), Span O(1) | confirmed |
| 485 | 50 | OptBinSearchTreeMtEph.rs | 310 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 486 | 50 | OptBinSearchTreeMtEph.rs | 334 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 487 | 50 | OptBinSearchTreeMtEph.rs | 347 | optimal_cost | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 488 | 50 | OptBinSearchTreeMtEph.rs | 391 | keys | CR: Work O(n), Span O(n) | confirmed |
| 489 | 50 | OptBinSearchTreeMtEph.rs | 403 | set_key_prob | CR: Work O(n), Span O(n) | confirmed |
| 490 | 50 | OptBinSearchTreeMtEph.rs | 424 | update_prob | CR: Work O(n), Span O(n) | confirmed |
| 491 | 50 | OptBinSearchTreeMtEph.rs | 445 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 492 | 50 | OptBinSearchTreeMtEph.rs | 456 | clear_memo | CR: Work O(m), Span O(m) | confirmed |
| 493 | 50 | OptBinSearchTreeMtEph.rs | 465 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 494 | 50 | OptBinSearchTreeMtEph.rs | 586 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 495 | 50 | OptBinSearchTreeMtEph.rs | 587 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 496 | 50 | OptBinSearchTreeMtEph.rs | 600 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 497 | 50 | OptBinSearchTreeMtEph.rs | 601 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 498 | 50 | OptBinSearchTreeMtEph.rs | 619 | fmt | APAS: Work O(1), Span O(1) | quote |
| 499 | 50 | OptBinSearchTreeMtEph.rs | 620 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 500 | 50 | OptBinSearchTreeMtEph.rs | 655 | fmt | APAS: Work O(1), Span O(1) | quote |
| 501 | 50 | OptBinSearchTreeMtEph.rs | 656 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 502 | 50 | OptBinSearchTreeMtEph.rs | 672 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 503 | 50 | OptBinSearchTreeMtEph.rs | 673 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 504 | 50 | OptBinSearchTreeMtPer.rs | 90 | new | CR: Work O(1), Span O(1) | confirmed |
| 505 | 50 | OptBinSearchTreeMtPer.rs | 94 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 506 | 50 | OptBinSearchTreeMtPer.rs | 99 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 507 | 50 | OptBinSearchTreeMtPer.rs | 103 | optimal_cost | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 508 | 50 | OptBinSearchTreeMtPer.rs | 107 | keys | CR: Work O(1), Span O(1) | confirmed |
| 509 | 50 | OptBinSearchTreeMtPer.rs | 111 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 510 | 50 | OptBinSearchTreeMtPer.rs | 115 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 511 | 50 | OptBinSearchTreeMtPer.rs | 122 | obst_rec | APAS: Work O(n^3), Span O(n lg n) | quote |
| 512 | 50 | OptBinSearchTreeMtPer.rs | 123 | obst_rec | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 513 | 50 | OptBinSearchTreeMtPer.rs | 178 | parallel_min_split_cost | CR: Work O(l), Span O(lg l) | confirmed |
| 514 | 50 | OptBinSearchTreeMtPer.rs | 266 | new | CR: Work O(1), Span O(1) | confirmed |
| 515 | 50 | OptBinSearchTreeMtPer.rs | 276 | from_keys_probs | CR: Work O(n), Span O(n) | confirmed |
| 516 | 50 | OptBinSearchTreeMtPer.rs | 299 | from_key_probs | CR: Work O(1), Span O(1) | confirmed |
| 517 | 50 | OptBinSearchTreeMtPer.rs | 308 | optimal_cost | CR: Work O(n^3), Span O(n lg n) | confirmed |
| 518 | 50 | OptBinSearchTreeMtPer.rs | 344 | keys | CR: Work O(1), Span O(1) | confirmed |
| 519 | 50 | OptBinSearchTreeMtPer.rs | 347 | num_keys | CR: Work O(1), Span O(1) | confirmed |
| 520 | 50 | OptBinSearchTreeMtPer.rs | 353 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 521 | 50 | OptBinSearchTreeMtPer.rs | 444 | into_iter | APAS: Work O(1), Span O(1) | quote |
| 522 | 50 | OptBinSearchTreeMtPer.rs | 445 | into_iter | CR: Work O(1), Span O(1) | confirmed |
| 523 | 50 | OptBinSearchTreeMtPer.rs | 458 | fmt | APAS: Work O(1), Span O(1) | quote |
| 524 | 50 | OptBinSearchTreeMtPer.rs | 459 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 525 | 50 | OptBinSearchTreeMtPer.rs | 484 | fmt | APAS: Work O(1), Span O(1) | quote |
| 526 | 50 | OptBinSearchTreeMtPer.rs | 485 | fmt | CR: Work O(1), Span O(1) | confirmed |
| 527 | 50 | OptBinSearchTreeMtPer.rs | 498 | into_iter | APAS: Work O(n), Span O(n) | quote |
| 528 | 50 | OptBinSearchTreeMtPer.rs | 499 | into_iter | CR: Work O(n), Span O(n) | confirmed |
| 529 | 51 | TopDownDPStEph.rs | 77 | new | CR: Work O(1), Span O(1) | confirmed |
| 530 | 51 | TopDownDPStEph.rs | 86 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 531 | 51 | TopDownDPStEph.rs | 91 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 532 | 51 | TopDownDPStEph.rs | 96 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 533 | 51 | TopDownDPStEph.rs | 101 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 534 | 51 | TopDownDPStEph.rs | 106 | is_memoized | CR: Work O(1), Span O(1) | confirmed |
| 535 | 51 | TopDownDPStEph.rs | 111 | get_memoized | CR: Work O(1), Span O(1) | confirmed |
| 536 | 51 | TopDownDPStEph.rs | 121 | insert_memo | CR: Work O(1), Span O(1) | confirmed |
| 537 | 51 | TopDownDPStEph.rs | 129 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 538 | 51 | TopDownDPStEph.rs | 137 | set_s | CR: Work O(1), Span O(1) | confirmed |
| 539 | 51 | TopDownDPStEph.rs | 145 | set_t | CR: Work O(1), Span O(1) | confirmed |
| 540 | 51 | TopDownDPStEph.rs | 153 | med_memoized | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) … | quote |
| 541 | 51 | TopDownDPStEph.rs | 154 | med_memoized | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 542 | 51 | TopDownDPStEph.rs | 168 | med_recursive | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) … | quote |
| 543 | 51 | TopDownDPStEph.rs | 169 | med_recursive | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 544 | 51 | TopDownDPStEph.rs | 214 | new | CR: Work O(1), Span O(1) | confirmed |
| 545 | 51 | TopDownDPStEph.rs | 225 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 546 | 51 | TopDownDPStEph.rs | 227 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 547 | 51 | TopDownDPStEph.rs | 230 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 548 | 51 | TopDownDPStEph.rs | 237 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 549 | 51 | TopDownDPStEph.rs | 244 | is_memoized | CR: Work O(1), Span O(1) | confirmed |
| 550 | 51 | TopDownDPStEph.rs | 250 | get_memoized | CR: Work O(1), Span O(1) | confirmed |
| 551 | 51 | TopDownDPStEph.rs | 259 | insert_memo | CR: Work O(1), Span O(1) | confirmed |
| 552 | 51 | TopDownDPStEph.rs | 265 | clear_memo | CR: Work O(n), Span O(n) | confirmed |
| 553 | 51 | TopDownDPStEph.rs | 270 | set_s | CR: Work O(n), Span O(n) | confirmed |
| 554 | 51 | TopDownDPStEph.rs | 276 | set_t | CR: Work O(n), Span O(n) | confirmed |
| 555 | 51 | TopDownDPStEph.rs | 283 | med_memoized | CR: Work O(n*m), Span O(n*m) | confirmed |
| 556 | 51 | TopDownDPStEph.rs | 292 | med_recursive | CR: Work O(n*m), Span O(n*m) | confirmed |
| 557 | 51 | TopDownDPStPer.rs | 77 | new | CR: Work O(1), Span O(1) | confirmed |
| 558 | 51 | TopDownDPStPer.rs | 86 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 559 | 51 | TopDownDPStPer.rs | 91 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 560 | 51 | TopDownDPStPer.rs | 96 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 561 | 51 | TopDownDPStPer.rs | 101 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 562 | 51 | TopDownDPStPer.rs | 106 | is_memoized | CR: Work O(1), Span O(1) | confirmed |
| 563 | 51 | TopDownDPStPer.rs | 111 | get_memoized | CR: Work O(1), Span O(1) | confirmed |
| 564 | 51 | TopDownDPStPer.rs | 121 | with_memo_table | CR: Work O(1), Span O(1) | confirmed |
| 565 | 51 | TopDownDPStPer.rs | 127 | clear_memo | CR: Work O(1), Span O(1) | confirmed |
| 566 | 51 | TopDownDPStPer.rs | 134 | med_memoized | CR: Work O(n*m), Span O(n*m) | confirmed |
| 567 | 51 | TopDownDPStPer.rs | 141 | med_recursive | CR: Work O(n*m), Span O(n*m) | confirmed |
| 568 | 51 | TopDownDPStPer.rs | 186 | new | CR: Work O(1), Span O(1) | confirmed |
| 569 | 51 | TopDownDPStPer.rs | 198 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 570 | 51 | TopDownDPStPer.rs | 200 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 571 | 51 | TopDownDPStPer.rs | 203 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 572 | 51 | TopDownDPStPer.rs | 210 | memo_size | CR: Work O(1), Span O(1) | confirmed |
| 573 | 51 | TopDownDPStPer.rs | 217 | is_memoized | CR: Work O(1), Span O(1) | confirmed |
| 574 | 51 | TopDownDPStPer.rs | 223 | get_memoized | CR: Work O(1), Span O(1) | confirmed |
| 575 | 51 | TopDownDPStPer.rs | 232 | with_memo_table | CR: Work O(1), Span O(1) | confirmed |
| 576 | 51 | TopDownDPStPer.rs | 237 | clear_memo | CR: Work O(1), Span O(1) | confirmed |
| 577 | 51 | TopDownDPStPer.rs | 250 | med_memoized | CR: Work O(n*m), Span O(n*m) | confirmed |
| 578 | 51 | TopDownDPStPer.rs | 262 | med_recursive | CR: Work O(n*m), Span O(n*m) | confirmed |
| 579 | 51 | TopDownDPMtEph.rs | 87 | new | CR: Work O(1), Span O(1) | confirmed |
| 580 | 51 | TopDownDPMtEph.rs | 92 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 581 | 51 | TopDownDPMtEph.rs | 94 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 582 | 51 | TopDownDPMtEph.rs | 97 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 583 | 51 | TopDownDPMtEph.rs | 104 | set_s | CR: Work O(1), Span O(1) | confirmed |
| 584 | 51 | TopDownDPMtEph.rs | 106 | set_t | CR: Work O(1), Span O(1) | confirmed |
| 585 | 51 | TopDownDPMtEph.rs | 110 | med_memoized_concurrent | CR: Work O(n*m), Span O(n*m) | confirmed |
| 586 | 51 | TopDownDPMtEph.rs | 122 | med_memoized_parallel | CR: Work O(n*m), Span O(n+m) | confirmed |
| 587 | 51 | TopDownDPMtEph.rs | 161 | new | CR: Work O(1), Span O(1) | confirmed |
| 588 | 51 | TopDownDPMtEph.rs | 170 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 589 | 51 | TopDownDPMtEph.rs | 175 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 590 | 51 | TopDownDPMtEph.rs | 180 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 591 | 51 | TopDownDPMtEph.rs | 185 | set_s | CR: Work O(1), Span O(1) | confirmed |
| 592 | 51 | TopDownDPMtEph.rs | 193 | set_t | CR: Work O(1), Span O(1) | confirmed |
| 593 | 51 | TopDownDPMtEph.rs | 201 | med_memoized_concurrent | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) … | quote |
| 594 | 51 | TopDownDPMtEph.rs | 202 | med_memoized_concurrent | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 595 | 51 | TopDownDPMtEph.rs | 216 | med_memoized_parallel | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) … | quote |
| 596 | 51 | TopDownDPMtEph.rs | 217 | med_memoized_parallel | CR: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 597 | 51 | TopDownDPMtEph.rs | 236 | med_recursive_sequential | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) … | quote |
| 598 | 51 | TopDownDPMtEph.rs | 237 | med_recursive_sequential | CR: Work O(\|S\|*\|T\|), Span O(\|S\|*\|T\|) | confirmed |
| 599 | 51 | TopDownDPMtEph.rs | 317 | med_recursive_parallel | APAS: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) … | quote |
| 600 | 51 | TopDownDPMtEph.rs | 318 | med_recursive_parallel | CR: Work O(\|S\|*\|T\|), Span O(\|S\|+\|T\|) | confirmed |
| 601 | 51 | TopDownDPMtPer.rs | 87 | new | CR: Work O(1), Span O(1) | confirmed |
| 602 | 51 | TopDownDPMtPer.rs | 92 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 603 | 51 | TopDownDPMtPer.rs | 94 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 604 | 51 | TopDownDPMtPer.rs | 97 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 605 | 51 | TopDownDPMtPer.rs | 105 | med_memoized_concurrent | CR: Work O(n*m), Span O(n*m) | confirmed |
| 606 | 51 | TopDownDPMtPer.rs | 117 | med_memoized_parallel | CR: Work O(n*m), Span O(n+m) | confirmed |
| 607 | 51 | TopDownDPMtPer.rs | 155 | new | CR: Work O(1), Span O(1) | confirmed |
| 608 | 51 | TopDownDPMtPer.rs | 164 | s_length | CR: Work O(1), Span O(1) | confirmed |
| 609 | 51 | TopDownDPMtPer.rs | 169 | t_length | CR: Work O(1), Span O(1) | confirmed |
| 610 | 51 | TopDownDPMtPer.rs | 174 | is_empty | CR: Work O(1), Span O(1) | confirmed |
| 611 | 51 | TopDownDPMtPer.rs | 179 | med_memoized_concurrent | CR: Work O(n*m), Span O(n*m) | confirmed |
| 612 | 51 | TopDownDPMtPer.rs | 186 | med_memoized_parallel | CR: Work O(n*m), Span O(n+m) | confirmed |
| 613 | 51 | TopDownDPMtPer.rs | 198 | med_recursive_sequential | CR: Work O(n*m), Span O(n*m) | confirmed |
| 614 | 51 | TopDownDPMtPer.rs | 278 | med_recursive_parallel | CR: Work O(n*m), Span O(n+m) | confirmed |
| 615 | 61 | EdgeContractionStEph.rs | 68 | edge_contract | APAS: Work O(n), Span O(lg^2 n) | quote |
| 616 | 61 | EdgeContractionStEph.rs | 69 | edge_contract | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 617 | 61 | EdgeContractionStEph.rs | 78 | contract_round | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 618 | 61 | EdgeContractionStEph.rs | 92 | edge_contract | APAS: Work O(n), Span O(lg^2 n) | quote |
| 619 | 61 | EdgeContractionStEph.rs | 93 | edge_contract | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 620 | 61 | EdgeContractionStEph.rs | 182 | contract_round | APAS: Work O(\|V\| + \|E\|), Span O(\|V\| + \|… | quote |
| 621 | 61 | EdgeContractionStEph.rs | 183 | contract_round | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 622 | 61 | EdgeContractionStEph.rs | 184 | contract_round | CR: Work Θ(\|V\| + \|E\|), Span Θ(\|V\| + \|E\|) | confirmed |
| 623 | 61 | EdgeContractionMtEph.rs | 59 | edge_contract_mt | CR: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | disputed D4 |
| 624 | 61 | EdgeContractionMtEph.rs | 68 | contract_round_mt | CR: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | disputed D4 |
| 625 | 61 | EdgeContractionMtEph.rs | 85 | edge_contract_mt | APAS: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | quote |
| 626 | 61 | EdgeContractionMtEph.rs | 86 | edge_contract_mt | CR: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | disputed D4 |
| 627 | 61 | EdgeContractionMtEph.rs | 87 | edge_contract_mt | CR: Work Θ(\|V\| + \|E\|), Span Θ(\|V\| + \|E\|) | confirmed |
| 628 | 61 | EdgeContractionMtEph.rs | 148 | build_edges_parallel | CR: Work Θ(\|E\|), Span Θ(lg \|E\|) | disputed D5 |
| 629 | 61 | EdgeContractionMtEph.rs | 233 | contract_round_mt | APAS: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | quote |
| 630 | 61 | EdgeContractionMtEph.rs | 234 | contract_round_mt | CR: Work O(\|V\| + \|E\|), Span O(lg \|V\|) | disputed D4 |
| 631 | 61 | EdgeContractionMtEph.rs | 235 | contract_round_mt | CR: Work Θ(\|E\|^2), Span Θ(\|E\|) | confirmed |
| 632 | 61 | VertexMatchingStEph.rs | 57 | greedy_matching | CR: Work O(\|E\|), Span O(\|E\|) | confirmed |
| 633 | 61 | VertexMatchingStEph.rs | 63 | parallel_matching_st | CR: Work O(\|E\|^2), Span O(\|E\|^2) | confirmed |
| 634 | 61 | VertexMatchingStEph.rs | 73 | greedy_matching | APAS: Work O(\|E\|), Span O(\|E\|) | quote |
| 635 | 61 | VertexMatchingStEph.rs | 74 | greedy_matching | CR: Work O(\|E\|), Span O(\|E\|) | confirmed |
| 636 | 61 | VertexMatchingStEph.rs | 75 | greedy_matching | CR: Work Θ(\|E\|), Span Θ(\|E\|) | confirmed |
| 637 | 61 | VertexMatchingStEph.rs | 111 | parallel_matching_st | CR: matches APAS | confirmed |
| 638 | 61 | VertexMatchingStEph.rs | 112 | parallel_matching_st | CR: Work Θ(\|E\|²), Span Θ(\|E\|²) | confirmed |
| 639 | 61 | VertexMatchingMtEph.rs | 59 | parallel_matching_mt | CR: Work O(\|E\|), Span O(lg \|V\|) | disputed D4 |
| 640 | 61 | VertexMatchingMtEph.rs | 74 | parallel_matching_mt | APAS: Work O(\|E\|), Span O(lg \|V\|) | quote |
| 641 | 61 | VertexMatchingMtEph.rs | 75 | parallel_matching_mt | CR: Work O(\|E\|), Span O(lg \|V\|) | disputed D4 |
| 642 | 61 | VertexMatchingMtEph.rs | 76 | parallel_matching_mt | CR: Work Θ(\|E\|^2), Span Θ(\|E\|) | confirmed |
| 643 | 61 | VertexMatchingMtEph.rs | 112 | flip_coins_parallel | APAS: Work O(\|E\|), Span O(1) | quote |
| 644 | 61 | VertexMatchingMtEph.rs | 113 | flip_coins_parallel | CR: Work O(\|E\|), Span O(1) | disputed D4 |
| 645 | 61 | VertexMatchingMtEph.rs | 114 | flip_coins_parallel | CR: Work Θ(\|E\|), Span Θ(\|E\|) | confirmed |
| 646 | 61 | VertexMatchingMtEph.rs | 145 | select_edges_parallel | APAS: Work O(\|E\|), Span O(lg \|V\|) | quote |
| 647 | 61 | VertexMatchingMtEph.rs | 146 | select_edges_parallel | CR: Work O(\|E\|), Span O(lg \|V\|) | disputed D4 |
| 648 | 61 | VertexMatchingMtEph.rs | 147 | select_edges_parallel | CR: Work Θ(\|E\|^2), Span Θ(lg \|E\| + \|E\|) | confirmed |
| 649 | 61 | VertexMatchingMtEph.rs | 180 | select_edges_recursive | CR: Work Θ(k * \|E\|), Span Θ(lg k + \|E\|) | confirmed |
| 650 | 61 | VertexMatchingMtEph.rs | 228 | should_select_edge | APAS: Work O(degree(u) + degree(v)), Sp… | quote |
| 651 | 61 | VertexMatchingMtEph.rs | 229 | should_select_edge | CR: Work O(degree(u) + degree(v)), Span… | disputed D4 |
| 652 | 61 | VertexMatchingMtEph.rs | 230 | should_select_edge | CR: Work Θ(\|E\|), Span Θ(\|E\|) | confirmed |
| 653 | 62 | StarPartitionStEph.rs | 89 | sequential_star_partition | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 654 | 62 | StarPartitionStEph.rs | 99 | sequential_star_partition | APAS: Work O(n + m), Span O(n + m) | quote |
| 655 | 62 | StarPartitionStEph.rs | 100 | sequential_star_partition | CR: Work O(n + m), Span O(n + m) | confirmed |
| 656 | 62 | StarPartitionStEph.rs | 101 | sequential_star_partition | CR: Work Θ(n + m), Span Θ(n + m) | confirmed |
| 657 | 62 | StarPartitionMtEph.rs | 114 | parallel_star_partition | APAS: Work O(n + m), Span O(lg n) | quote |
| 658 | 62 | StarPartitionMtEph.rs | 115 | parallel_star_partition | CR: Work O((n + m) lg(n + m)), Span O(l… | disputed D6 |
| 659 | 62 | StarPartitionMtEph.rs | 130 | hash_coin | CR: Work O(1), Span O(1) | confirmed |
| 660 | 62 | StarPartitionMtEph.rs | 144 | hash_coin_flips_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 661 | 62 | StarPartitionMtEph.rs | 268 | build_th_edges_mt | CR: Work O(m), Span O(lg m) | disputed D7 |
| 662 | 62 | StarPartitionMtEph.rs | 441 | build_p_vec_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 663 | 62 | StarPartitionMtEph.rs | 548 | build_vertex_to_index_mt | CR: Work O(n lg n), Span O(lg n) | disputed D8 |
| 664 | 62 | StarPartitionMtEph.rs | 729 | build_satellite_map_mt | CR: Work O(m lg m), Span O(lg m) | disputed D8 |
| 665 | 62 | StarPartitionMtEph.rs | 948 | build_p_vec_with_inject_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 666 | 62 | StarPartitionMtEph.rs | 1211 | build_partition_map_mt | CR: Work O(n lg n), Span O(lg n) | disputed D8 |
| 667 | 62 | StarPartitionMtEph.rs | 1419 | build_centers_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 668 | 62 | StarPartitionMtEph.rs | 1517 | parallel_star_partition | APAS: Work O(n + m), Span O(lg n) | quote |
| 669 | 62 | StarPartitionMtEph.rs | 1518 | parallel_star_partition | CR: Work O((n + m) lg(n + m)), Span O(l… | disputed D6 |
| 670 | 62 | StarContractionStEph.rs | 72 | star_contract | APAS: Work O((n + m) lg n), Span O(lg^2… | quote |
| 671 | 62 | StarContractionStEph.rs | 73 | star_contract | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 672 | 62 | StarContractionStEph.rs | 96 | contract_to_vertices | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 673 | 62 | StarContractionStEph.rs | 107 | star_contract_fuel | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 674 | 62 | StarContractionStEph.rs | 188 | star_contract | APAS: Work O((n + m) lg n), Span O(lg^2… | quote |
| 675 | 62 | StarContractionStEph.rs | 189 | star_contract | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 676 | 62 | StarContractionStEph.rs | 238 | build_quotient_graph | CR: matches APAS | confirmed |
| 677 | 62 | StarContractionStEph.rs | 239 | build_quotient_graph | CR: Work O(m), Span O(m) | confirmed |
| 678 | 62 | StarContractionStEph.rs | 344 | contract_to_vertices | APAS: Work O((n + m) lg n), Span O((n +… | quote |
| 679 | 62 | StarContractionStEph.rs | 345 | contract_to_vertices | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 680 | 62 | StarContractionStEph.rs | 346 | contract_to_vertices | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 681 | 62 | StarContractionMtEph.rs | 93 | star_contract_mt | CR: Work O((n + m) lg n), Span O(lg^2 n) | disputed D9 |
| 682 | 62 | StarContractionMtEph.rs | 116 | contract_to_vertices_mt | CR: Work O((n + m) lg n), Span O(lg^2 n) | disputed D9 |
| 683 | 62 | StarContractionMtEph.rs | 127 | star_contract_mt_fuel | CR: Work O((n + m) lg n), Span O(lg^2 n) | disputed D9 |
| 684 | 62 | StarContractionMtEph.rs | 207 | star_contract_mt | APAS: Work O((n + m) lg n), Span O(lg^2… | quote |
| 685 | 62 | StarContractionMtEph.rs | 208 | star_contract_mt | CR: Work O((n + m) lg n), Span O(lg^2 n) | disputed D9 |
| 686 | 62 | StarContractionMtEph.rs | 209 | star_contract_mt | CR: Work O((n + m) lg n), Span O(n lg n) | disputed D10 |
| 687 | 62 | StarContractionMtEph.rs | 259 | build_quotient_graph_parallel | CR: matches APAS | confirmed |
| 688 | 62 | StarContractionMtEph.rs | 260 | build_quotient_graph_parallel | CR: Work O(m), Span O(lg m) | disputed D5 |
| 689 | 62 | StarContractionMtEph.rs | 327 | route_edges_parallel | CR: matches APAS | confirmed |
| 690 | 62 | StarContractionMtEph.rs | 328 | route_edges_parallel | CR: Work O(k), Span O(lg k) | disputed D5 |
| 691 | 62 | StarContractionMtEph.rs | 437 | contract_to_vertices_mt | APAS: Work O((n + m) lg n), Span O(lg^2… | quote |
| 692 | 62 | StarContractionMtEph.rs | 438 | contract_to_vertices_mt | CR: Work O((n + m) lg n), Span O(lg^2 n) | disputed D9 |
| 693 | 62 | StarContractionMtEph.rs | 439 | contract_to_vertices_mt | CR: Work O((n + m) lg n), Span O((n + m… | confirmed |
| 694 | 63 | ConnectivityStEph.rs | 72 | count_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 695 | 63 | ConnectivityStEph.rs | 78 | connected_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 696 | 63 | ConnectivityStEph.rs | 84 | count_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 697 | 63 | ConnectivityStEph.rs | 90 | connected_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 698 | 63 | ConnectivityStEph.rs | 103 | count_components | APAS: Work O((n+m) lg n), Span O((n+m) … | quote |
| 699 | 63 | ConnectivityStEph.rs | 104 | count_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 700 | 63 | ConnectivityStEph.rs | 105 | count_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 701 | 63 | ConnectivityStEph.rs | 127 | connected_components | APAS: Work O((n+m) lg n), Span O((n+m) … | quote |
| 702 | 63 | ConnectivityStEph.rs | 128 | connected_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 703 | 63 | ConnectivityStEph.rs | 129 | connected_components | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 704 | 63 | ConnectivityStEph.rs | 149 | build_quotient_edges | CR: Work O(m), Span O(m) | confirmed |
| 705 | 63 | ConnectivityStEph.rs | 200 | count_components_hof | APAS: Work O((n+m) lg n), Span O((n+m) … | quote |
| 706 | 63 | ConnectivityStEph.rs | 201 | count_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 707 | 63 | ConnectivityStEph.rs | 202 | count_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 708 | 63 | ConnectivityStEph.rs | 223 | connected_components_hof | APAS: Work O((n+m) lg n), Span O((n+m) … | quote |
| 709 | 63 | ConnectivityStEph.rs | 224 | connected_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 710 | 63 | ConnectivityStEph.rs | 225 | connected_components_hof | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 711 | 63 | ConnectivityMtEph.rs | 77 | count_components_mt | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 712 | 63 | ConnectivityMtEph.rs | 83 | connected_components_mt | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 713 | 63 | ConnectivityMtEph.rs | 92 | count_components_hof | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 714 | 63 | ConnectivityMtEph.rs | 98 | connected_components_hof | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 715 | 63 | ConnectivityMtEph.rs | 114 | count_components_mt | APAS: Work O((n+m) lg n), Span O(lg² n)… | quote |
| 716 | 63 | ConnectivityMtEph.rs | 115 | count_components_mt | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 717 | 63 | ConnectivityMtEph.rs | 116 | count_components_mt | CR: Work O((n+m) lg n), Span O(m) | confirmed |
| 718 | 63 | ConnectivityMtEph.rs | 138 | connected_components_mt | APAS: Work O((n+m) lg n), Span O(lg² n)… | quote |
| 719 | 63 | ConnectivityMtEph.rs | 139 | connected_components_mt | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 720 | 63 | ConnectivityMtEph.rs | 140 | connected_components_mt | CR: Work O((n+m) lg n), Span O(n lg n) | confirmed |
| 721 | 63 | ConnectivityMtEph.rs | 163 | compose_maps_parallel | CR: Work O(\|P\|), Span O(\|P\|) | confirmed |
| 722 | 63 | ConnectivityMtEph.rs | 202 | count_components_hof | APAS: Work O((n+m) lg n), Span O(lg^2 n… | quote |
| 723 | 63 | ConnectivityMtEph.rs | 203 | count_components_hof | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 724 | 63 | ConnectivityMtEph.rs | 204 | count_components_hof | CR: Work O((n+m) lg n), Span O(m) | confirmed |
| 725 | 63 | ConnectivityMtEph.rs | 223 | connected_components_hof | APAS: Work O((n+m) lg n), Span O(lg^2 n… | quote |
| 726 | 63 | ConnectivityMtEph.rs | 224 | connected_components_hof | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 727 | 63 | ConnectivityMtEph.rs | 225 | connected_components_hof | CR: Work O((n+m) lg n), Span O(n lg n) | confirmed |
| 728 | 64 | SpanTreeStEph.rs | 65 | spanning_tree_star_contraction | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 729 | 64 | SpanTreeStEph.rs | 71 | verify_spanning_tree | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 730 | 64 | SpanTreeStEph.rs | 81 | spanning_tree_star_contraction | APAS: Work O((n+m) lg n), Span O((n+m) … | quote |
| 731 | 64 | SpanTreeStEph.rs | 82 | spanning_tree_star_contraction | CR: Work O((n+m) | disputed D11 |
| 732 | 64 | SpanTreeStEph.rs | 83 | spanning_tree_star_contraction | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 733 | 64 | SpanTreeStEph.rs | 175 | verify_spanning_tree | CR: Work O(\|V\| + \|E_tree\|), Span O(\|V\| … | confirmed |
| 734 | 64 | SpanTreeMtEph.rs | 66 | spanning_tree_star_contraction_mt | CR: Work O((n+m) lg n), Span O(lg^2 n) | disputed D9 |
| 735 | 64 | SpanTreeMtEph.rs | 74 | verify_spanning_tree | CR: Work O(\|V\| + \|E\|), Span O(\|V\| + \|E\|) | confirmed |
| 736 | 64 | SpanTreeMtEph.rs | 84 | spanning_tree_star_contraction_mt | APAS: Work O((n+m) lg n), Span O(lg² n) | quote |
| 737 | 64 | SpanTreeMtEph.rs | 85 | spanning_tree_star_contraction_mt | CR: Work O((n+m) | disputed D11 |
| 738 | 64 | SpanTreeMtEph.rs | 86 | spanning_tree_star_contraction_mt | CR: Work O((n+m) lg n), Span O((n+m) lg… | confirmed |
| 739 | 64 | SpanTreeMtEph.rs | 181 | verify_spanning_tree | CR: Work O(\|V\| + \|E_tree\|), Span O(\|E_t… | confirmed |
| 740 | 64 | TSPApproxStEph.rs | 64 | euler_tour | CR: Work O(\|V\|), Span O(\|V\|) | confirmed |
| 741 | 64 | TSPApproxStEph.rs | 77 | shortcut_tour | CR: Work O(\|V\|), Span O(\|V\|) | confirmed |
| 742 | 64 | TSPApproxStEph.rs | 82 | tour_weight | CR: Work O(\|V\|), Span O(\|V\|) | confirmed |
| 743 | 64 | TSPApproxStEph.rs | 93 | approx_metric_tsp | CR: Work O(\|V\|), Span O(\|V\|) | confirmed |
| 744 | 64 | TSPApproxStEph.rs | 110 | vec_contains_pair | CR: Work O(n), Span O(n) | confirmed |
| 745 | 64 | TSPApproxStEph.rs | 133 | euler_tour | APAS: Work O(n), Span O(n) | quote |
| 746 | 64 | TSPApproxStEph.rs | 134 | euler_tour | CR: Work O(n), Span O(n) | confirmed |
| 747 | 64 | TSPApproxStEph.rs | 135 | euler_tour | CR: Work O(n), Span O(n) | confirmed |
| 748 | 64 | TSPApproxStEph.rs | 165 | euler_tour_dfs | CR: Work O(n * m_tree), Span O(n * m_tr… | confirmed |
| 749 | 64 | TSPApproxStEph.rs | 256 | shortcut_tour | APAS: Work O(n), Span O(n) | quote |
| 750 | 64 | TSPApproxStEph.rs | 257 | shortcut_tour | CR: Work O(n), Span O(n) | confirmed |
| 751 | 64 | TSPApproxStEph.rs | 258 | shortcut_tour | CR: Work O(n), Span O(n) | confirmed |
| 752 | 64 | TSPApproxStEph.rs | 305 | tour_weight | APAS: Work O(n), Span O(n) | quote |
| 753 | 64 | TSPApproxStEph.rs | 306 | tour_weight | CR: Work O(n), Span O(n) | confirmed |
| 754 | 64 | TSPApproxStEph.rs | 307 | tour_weight | CR: Work O(n), Span O(n) | confirmed |
| 755 | 64 | TSPApproxStEph.rs | 347 | get_neighbors | CR: Work O(m), Span O(m) | confirmed |
| 756 | 64 | TSPApproxStEph.rs | 357 | get_edge_weight | CR: Work O(m), Span O(m) | confirmed |
| 757 | 64 | TSPApproxStEph.rs | 386 | approx_metric_tsp | APAS: Work O(n+m), Span O(n+m) | quote |
| 758 | 64 | TSPApproxStEph.rs | 387 | approx_metric_tsp | CR: Work O(n+m), Span O(n+m) | confirmed |
| 759 | 64 | TSPApproxStEph.rs | 388 | approx_metric_tsp | CR: Work O(n+m), Span O(n+m) | confirmed |
| 760 | 65 | PrimStEph.rs | 91 | prim_mst | CR: Work O(m log n), Span O(m log n) | disputed D12 |
| 761 | 65 | PrimStEph.rs | 101 | mst_weight | CR: Work O(m), Span O(m) | confirmed |
| 762 | 65 | PrimStEph.rs | 249 | pq_entry_new | CR: Work Θ(1), Span Θ(1) | confirmed |
| 763 | 65 | PrimStEph.rs | 268 | prim_mst | APAS: Work O(m lg n), Span O(m lg n) | quote |
| 764 | 65 | PrimStEph.rs | 269 | prim_mst | CR: Work O(m lg n), Span O(m lg n) | disputed D12 |
| 765 | 65 | PrimStEph.rs | 270 | prim_mst | CR: Work O(m^2 lg n), Span O(m^2 lg n) | confirmed |
| 766 | 65 | PrimStEph.rs | 467 | mst_weight | CR: matches APAS | confirmed |
| 767 | 65 | PrimStEph.rs | 468 | mst_weight | CR: Work O(\|MST\|), Span O(\|MST\|) | confirmed |
| 768 | 65 | PrimStEph.rs | 523 | cmp | CR: Work Θ(1), Span Θ(1) | confirmed |
| 769 | 65 | PrimStEph.rs | 532 | partial_cmp | CR: Work Θ(1), Span Θ(1) | confirmed |
| 770 | 66 | BoruvkaStEph.rs | 98 | coin_flip | CR: Work O(1), Span O(1) | confirmed |
| 771 | 66 | BoruvkaStEph.rs | 125 | vertex_bridges | CR: Work O(\|E\|), Span O(\|E\|) | confirmed |
| 772 | 66 | BoruvkaStEph.rs | 136 | bridge_star_partition | CR: Work O(\|V\|), Span O(\|V\|) | confirmed |
| 773 | 66 | BoruvkaStEph.rs | 150 | boruvka_mst | APAS: Work O(m lg n), Span O(lg^3 n) | quote |
| 774 | 66 | BoruvkaStEph.rs | 151 | boruvka_mst | CR: Work O(m lg n), Span O(m lg n) | confirmed |
| 775 | 66 | BoruvkaStEph.rs | 171 | boruvka_mst_with_seed | APAS: Work O(m lg n), Span O(lg^3 n) | quote |
| 776 | 66 | BoruvkaStEph.rs | 172 | boruvka_mst_with_seed | CR: Work O(m lg n), Span O(lg^3 n) | disputed D13 |
| 777 | 66 | BoruvkaStEph.rs | 173 | boruvka_mst_with_seed | APAS: Work O(m lg n), Span O(lg^2 n) | quote |
| 778 | 66 | BoruvkaStEph.rs | 174 | boruvka_mst_with_seed | CR: Work O(m lg n), Span O(m lg n) | confirmed |
| 779 | 66 | BoruvkaStEph.rs | 192 | mst_weight | CR: Work O(m), Span O(m) | confirmed |
| 780 | 66 | BoruvkaStEph.rs | 211 | vertex_bridges | APAS: Work O(m), Span O(log m) | quote |
| 781 | 66 | BoruvkaStEph.rs | 212 | vertex_bridges | CR: Work O(m) | confirmed |
| 782 | 66 | BoruvkaStEph.rs | 272 | bridge_star_partition | APAS: Work O(n), Span O(log n) | quote |
| 783 | 66 | BoruvkaStEph.rs | 273 | bridge_star_partition | CR: Work O(n) | confirmed |
| 784 | 66 | BoruvkaStEph.rs | 344 | boruvka_mst | APAS: Work O(m log n), Span O(log^2 n) | quote |
| 785 | 66 | BoruvkaStEph.rs | 345 | boruvka_mst | CR: Work O(m log n) | confirmed |
| 786 | 66 | BoruvkaStEph.rs | 440 | boruvka_mst_with_seed | APAS: Work O(m log n), Span O(log^2 n) | quote |
| 787 | 66 | BoruvkaStEph.rs | 441 | boruvka_mst_with_seed | CR: Work O(m log n) | confirmed |
| 788 | 66 | BoruvkaMtEph.rs | 79 | vertex_bridges_mt | CR: Work O(\|E\|), Span O(lg \|E\|) | disputed D7 |
| 789 | 66 | BoruvkaMtEph.rs | 94 | bridge_star_partition_mt | CR: Work O(\|V\|), Span O(lg \|V\|) | disputed D14 |
| 790 | 66 | BoruvkaMtEph.rs | 108 | boruvka_mst_mt | APAS: Work O(m lg n), Span O(lg^2 n) | quote |
| 791 | 66 | BoruvkaMtEph.rs | 109 | boruvka_mst_mt | CR: Work O(m lg n), Span O(lg^2 n) | disputed D15 |
| 792 | 66 | BoruvkaMtEph.rs | 126 | boruvka_mst_mt_with_seed | APAS: Work O(m lg n), Span O(lg^2 n) | quote |
| 793 | 66 | BoruvkaMtEph.rs | 127 | boruvka_mst_mt_with_seed | CR: Work O(m lg n), Span O(lg^2 n) | disputed D15 |
| 794 | 66 | BoruvkaMtEph.rs | 144 | mst_weight | CR: Work O(m), Span O(m) | confirmed |
| 795 | 66 | BoruvkaMtEph.rs | 189 | hash_coin | CR: Work O(1), Span O(1) | confirmed |
| 796 | 66 | BoruvkaMtEph.rs | 204 | hash_coin_flips_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 797 | 66 | BoruvkaMtEph.rs | 266 | compute_remaining_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 798 | 66 | BoruvkaMtEph.rs | 334 | collect_mst_labels_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 799 | 66 | BoruvkaMtEph.rs | 402 | build_partition_map_mt | CR: Work O(n), Span O(lg n) | disputed D7 |
| 800 | 66 | BoruvkaMtEph.rs | 472 | vertex_bridges_mt | APAS: Work O(m), Span O(log m) | quote |
| 801 | 66 | BoruvkaMtEph.rs | 473 | vertex_bridges_mt | CR: Work O(m), Span O(log m) | disputed D7 |
| 802 | 66 | BoruvkaMtEph.rs | 474 | vertex_bridges_mt | CR: Work O(m), Span O(log m) | disputed D7 |
| 803 | 66 | BoruvkaMtEph.rs | 579 | bridge_star_partition_mt | APAS: Work O(n), Span O(log n) | quote |
| 804 | 66 | BoruvkaMtEph.rs | 580 | bridge_star_partition_mt | CR: Work O(n), Span O(log n) | disputed D14 |
| 805 | 66 | BoruvkaMtEph.rs | 581 | bridge_star_partition_mt | CR: Work O(n), Span O(log n) | disputed D14 |
| 806 | 66 | BoruvkaMtEph.rs | 639 | filter_tail_to_head_mt | CR: Work O(n), Span O(log n) | disputed D7 |
| 807 | 66 | BoruvkaMtEph.rs | 725 | boruvka_mst_mt | APAS: Work O(m lg n), Span O(lg^2 n) | quote |
| 808 | 66 | BoruvkaMtEph.rs | 726 | boruvka_mst_mt | CR: Work O(m lg n), Span O(lg^2 n) | disputed D15 |
| 809 | 66 | BoruvkaMtEph.rs | 833 | reroute_edges_mt | CR: Work O(m), Span O(log m) | disputed D7 |
| 810 | 66 | BoruvkaMtEph.rs | 922 | boruvka_mst_mt_with_seed | APAS: Work O(m lg n), Span O(lg^2 n) | quote |
| 811 | 66 | BoruvkaMtEph.rs | 923 | boruvka_mst_mt_with_seed | CR: Work O(m lg n), Span O(lg^2 n) | disputed D15 |
| 812 | 66 | BoruvkaMtEph.rs | 969 | mst_weight | CR: Work O(m), Span O(m) | confirmed |

## 4. Additions

All in `src/vstdplus/hash_specs_plus.rs`. Three trusted `assume_specification`s
(vstd's form of a trusted specification; no `external_body`), each with a doc
comment naming the upstream proposal it mirrors, plus untrusted proven lemmas.

### 4.1 `HashSet::clone`

```rust
/// Proposal: `std_specs/hash.rs`, beside `HashMap::clone` (line 578), with
/// the same shape: element-wise `cloned` and equal cardinality.
pub assume_specification<K: Clone, S: Clone, A: std::alloc::Allocator + Clone>[
    <HashSet<K, S, A> as Clone>::clone
](this: &HashSet<K, S, A>) -> (other: HashSet<K, S, A>)
    ensures
        spec_hash_set_cloned(this@, other@),
;

#[verifier::opaque]
pub open spec fn spec_hash_set_cloned<K: Clone>(this: Set<K>, other: Set<K>) -> bool {
    &&& other.len() == this.len()
    &&& forall|k: K| #[trigger] other.contains(k)
            <==> exists|k0: K| #[trigger] this.contains(k0) && cloned(k0, k)
}
```

Upstream wording (vstd 0.2026.09.13 `std_specs/hash.rs:578`, `HashMap::clone`):
`ensures other@.dom() == this@.dom(), forall|key| #[trigger] other@.dom().contains(key)
==> cloned(this@[key], other@[key])`. The plan's wording for the set:
"ensuring `other@.len() == this@.len()` and `forall|k| other@.contains(k) <==>
exists|k0| this@.contains(k0) && cloned(k0, k)`". The predicate is exactly that,
made opaque because the inline quantifier's Skolemized `cloned(k0, k)` terms
matched `group_feq_axioms` and `SetStEph::clone` hit its rlimit (65,541
instantiations of the quantifier, 118,530 of each feq axiom;
`logs/validate.20260921-082833.log`, isolate Chap05 with `--profile`).
`lemma_hash_set_clone_eq(this, other)` proves `other == this` from it with no
hypothesis on `K`, because vstd defines `cloned(a, b)` as
`strictly_cloned(a, b) || a == b`.

Experiment `src/experiments/vstd_hash_set_clone_plus.rs`: SUCCEEDS, 183 verified,
0 errors (L1), three derivations: `u64` keys, a generic `Key: Eq + Hash + Clone`
with no hypothesis, and the mapped view `t@.map(|k| k@) == s@.map(|k| k@)` that
`SetStEph::clone` ensures.

### 4.2 `HashSet::eq`, `HashMap::eq`

```rust
pub assume_specification<K: Eq + Hash, S: BuildHasher, A: std::alloc::Allocator>[
    <HashSet<K, S, A> as PartialEq<HashSet<K, S, A>>>::eq
](a: &HashSet<K, S, A>, b: &HashSet<K, S, A>) -> (r: bool)
    ensures
        obeys_key_model::<K>() && builds_valid_hashers::<S>() && obeys_eq::<K>()
            ==> r == (a@ == b@),
;

pub assume_specification<K: Eq + Hash, V: PartialEq, S: BuildHasher, A: std::alloc::Allocator>[
    <HashMap<K, V, S, A> as PartialEq<HashMap<K, V, S, A>>>::eq
](a: &HashMap<K, V, S, A>, b: &HashMap<K, V, S, A>) -> (r: bool)
    ensures
        obeys_key_model::<K>() && builds_valid_hashers::<S>() && obeys_eq::<K>()
            && obeys_concrete_eq::<V>()
            ==> r == (a@ == b@),
;
```

Upstream wording (`docs/HashSpecsMigration.md` §3.2 item 2, not taken upstream):
"`r == (a@ == b@)` under `obeys_key_model::<K>()` and the `laws_eq::obeys_eq` of
`K` (and `V`)". Deviation: for the map's values this uses
`laws_eq::obeys_concrete_eq::<V>()` rather than `obeys_eq::<V>()`; Rust compares
values with `V::eq`, and `Map` equality in the view is structural, so the value
type's `eq_spec` must be `==`, which `obeys_eq` alone does not say (§5 item 4).

Experiment `src/experiments/vstd_hash_eq_plus.rs`: SUCCEEDS, 182 verified, 0
errors (L2): `*a == *b` on `HashSet<u64>` and `HashMap<u64, u64>` gives
`r == (a@ == b@)` with `group_hash_axioms` and `laws_eq::group_laws_eq`
discharging every hypothesis.

### 4.3 Proven (untrusted) projections and lemmas

`key_view<K: View, V>(m: Map<K, V>) -> Map<K::V, V>` and
`set_key_view<K: View>(s: Set<K>) -> Set<K::V>` (`s.map(|k| k@)`) reproduce the
wrappers' mapped views on top of vstd's raw views, so Chap62–65 keep their
`V::V`-keyed partition, spanning-tree and union-find specs. Broadcast group
`group_key_view_lemmas`: `lemma_key_view_{contains,insert,empty,key,len}` and
`lemma_set_key_view_{contains,insert,empty,len}`, the `contains`/`insert`/`len`
ones under `obeys_feq_view_injective::<K>()`, with multi-pattern triggers
(`key_view(m), m.contains_key(k)`; `key_view(m).contains_key(k@)`;
`key_view(m)[k@]`; `key_view(m.insert(k, v))`) chosen to avoid matching loops.
`lemma_hash_map_clone_eq` turns vstd's `HashMap::clone` postcondition into
`other == this` under `obeys_feq_clone::<V>()`. All verified in the Chap05 runs
(`validate.20260921-091103.log`, 759; `validate.20260921-093012.log`, 760 after
the second trigger on `lemma_key_view_len`).

## 5. Needs discussion

### Item 1. Disputed annotations (72; codes used in §3)

Work and span below are read from the code. The systematic finding is D7/D8:
every divide-and-conquer helper in `StarPartitionMtEph.rs`,
`StarContractionMtEph.rs`, `EdgeContractionMtEph.rs` and `BoruvkaMtEph.rs` forks
with `ParaPair!` and then merges sequentially (a `for kv in right.iter()` insert
loop, a `while i < right.len()` push loop, or `left.union(&right)`), so the span
recurrence is S(n) = S(n/2) + Θ(n) = Θ(n) and the work is W(n) = 2W(n/2) + Θ(n)
= Θ(n lg n). Fixing that is an exec change (a parallel merge) and is out of
scope for r209.

- D1, Chap 17, `MathSeq.rs:230, 617` (`iter`, `iter_mut`). Says "returns
  iterator wrapper". `iter` returns std `slice::Iter` directly (r208); the O(1)
  cost stands, the wording is stale.
- D2, Chap 50, `OptBinSearchTreeStEph.rs:124, 131, 142, 215, 221, 231`
  (`set_key_prob`, `update_prob`, `clear_memo`). Says Work O(1), Span O(1).
  Each calls `self.memo.clear()`: Work Θ(|memo|) ≤ O(n²) (keys are (i, l)
  pairs), Span the same.
- D3, Chap 50, `MatrixChainStEph.rs:215, 223, 232, 421, 427, 433`
  (`set_dimension`, `update_dimension`, `clear_memo`). Says Work O(n) "clears
  memo HashMap". `memo.clear()` is Θ(|memo|) ≤ O(n²) for n matrices; O(n) only
  if n means entries.
- D4, Chap 61, `EdgeContractionMtEph.rs:59, 68, 86, 234` and
  `VertexMatchingMtEph.rs:59, 75, 113, 146, 229`. The trait lines and the first
  review line under each fn claim Span O(lg |V|), O(1) or O(degree). The second
  review line under each fn is the code: sequential loops (EdgeContraction 112,
  118, 131; VertexMatching 130, 163, 254), Span Θ(|V| + |E|) / Θ(|E|), and Work
  Θ(|E|²) because `should_select_edge` scans all edges.
- D5, Chap 61 `EdgeContractionMtEph.rs:148` (`build_edges_parallel`) and Chap 62
  `StarContractionMtEph.rs:260, 328` (`build_quotient_graph_parallel`,
  `route_edges_parallel`). Say Work Θ(k), Span Θ(lg k). `ParaPair!` then
  `left.union(&right)` (sequential, Θ(k)): Work Θ(k lg k), Span Θ(k).
- D6, Chap 62, `StarPartitionMtEph.rs:115, 1518` (`parallel_star_partition`).
  Says Span O(lg(n+m)). Six D&C loops each with linear-span merges (D7/D8):
  Span Θ(n + m); Work O((n+m) lg(n+m)) stands.
- D7, Chap 62 `StarPartitionMtEph.rs:144, 268, 441, 948, 1419`
  (`hash_coin_flips_mt`, `build_th_edges_mt`, `build_p_vec_mt`,
  `build_p_vec_with_inject_mt`, `build_centers_mt`) and Chap 66
  `BoruvkaMtEph.rs:79, 204, 266, 334, 402, 473, 474, 639, 833`
  (`vertex_bridges_mt`, `hash_coin_flips_mt`, `compute_remaining_mt`,
  `collect_mst_labels_mt`, `build_partition_map_mt`, `filter_tail_to_head_mt`,
  `reroute_edges_mt`). Say Work O(n), Span O(lg n) "D&C fork-join". Each has a
  sequential merge after the fork (`for kv in right.iter()` insert at 207/253/
  457/537/709, `while i < right.len()` push at 413/499/1118/317/384/906, or
  `union` at 1492): Work Θ(n lg n), Span Θ(n).
- D8, Chap 62, `StarPartitionMtEph.rs:548, 729, 1211` (`build_vertex_to_index_mt`,
  `build_satellite_map_mt`, `build_partition_map_mt`). Say Work O(n lg n),
  Span O(lg n) "sequential merge". Work right; the root merge alone is Θ(n):
  Span Θ(n).
- D9, Chap 62 `StarContractionMtEph.rs:93, 116, 127, 208, 438`, Chap 63
  `ConnectivityMtEph.rs:77, 83, 92, 98, 115, 139, 203, 224`, Chap 64
  `SpanTreeMtEph.rs:66`. Say Span O(lg² n). `star_contract_mt` has linear span
  per level (D6, D5, and the sequential `compose_maps_parallel`,
  ConnectivityMtEph line 163): Span Θ((n+m) lg n). The sibling review lines
  (StarContraction 209, 439; Connectivity 116, 140, 204, 225; SpanTree 86)
  already say so and are confirmed.
- D10, Chap 62, `StarContractionMtEph.rs:209` (`star_contract_mt`). Says
  "quotient build O(lg m) via ParaPair". The quotient build ends in `union`
  (D5): Θ(m) span; the headline Span O(n lg n) should read O((n+m) lg n).
- D11, Chap 64, `SpanTreeStEph.rs:82`, `SpanTreeMtEph.rs:85`. The line reads
  `Work O((n+m)` and stops: truncated mid-formula. The next line carries the
  full analysis.
- D12, Chap 65, `PrimStEph.rs:91, 269` (`prim_mst`). Say Work O(m lg n). Line
  270 (confirmed) reads the code: `LabUnDirGraphStEph` stores edges in a flat
  set, so `ng()` and `get_edge_label()` cost O(m) per call; Work O(m² lg n),
  Span the same.
- D13, Chap 66, `BoruvkaStEph.rs:172` (`boruvka_mst_with_seed`). Says Span
  O(lg³ n). St sequential; line 174 under the same fn says Span O(m lg n) = Work.
- D14, Chap 66, `BoruvkaMtEph.rs:94, 580, 581` (`bridge_star_partition_mt`).
  Say Span O(lg |V|). Calls D7 helpers (Θ(n) span) and runs sequential loops at
  614 and 626: Span Θ(n).
- D15, Chap 66, `BoruvkaMtEph.rs:109, 127, 726, 923` (`boruvka_mst_mt`,
  `boruvka_mst_mt_with_seed`). Say Work O(m lg n), Span O(lg² n). O(lg n)
  rounds, each Θ(m) span (D7, D14) and Θ(m lg m) work: Span Θ(m lg n), Work
  Θ(m lg m lg n).

### Item 2. Exec changes beyond rule 1

1. `src/Chap05/SetStEph.rs` and `SetMtEph.rs`, `impl Hash` (section 14, outside
   `verus!`): `self.elements.hash(state)` became a per-element `key.hash(state)`
   loop. `std::collections::HashSet` implements no `Hash`; the wrapper supplied
   one with exactly this body. This is in the diff against `HEAD`; if it
   predates r209 (r208's uncommitted Chap05 work is in the same diff) it is not
   this round's. Either way it is the wrapper's body moved, not new logic.
2. `SubsetSumMtEph/MtPer.rs` (Chap49) and `OptBinSearchTreeMtEph/MtPer.rs`
   (Chap50): the `RwLockPredicate::inv` bodies are now `true` with a comment.
   The previous body `v@.dom().finite()` is identically true at 09.13, so no
   invariant was lost, but the predicate still says nothing (item 13).
3. Chap51: `SeqSpecsAndLemmas.rs` was edited (the shared `spec_memo_correct`
   takes `Pair` keys); the plan listed four Chap51 files.
4. Everything else is field-type or delegated-iteration substitution; the
   `for`-loop conversions in Chap62–66 keep the loop bodies' statements and
   order (the `break` in the Chap64 inner searches is preserved).

### Item 3. The Chap05 `PartialEq` accepts did not close

Plan step 1 asked to try closing `accept(equal == (self@ == other@))` in
`SetStEph::eq` and `SetMtEph::eq` via the new `HashSet::eq` specification. It
cannot, for two reasons that hold regardless of the spec's shape:
`PartialEq::eq` is a std trait method and carries no `requires`, so neither
`obeys_key_model::<T>()`, `obeys_eq::<T>()` nor `obeys_feq_view_injective::<T>()`
is available in the body (the module wf `valid_key_type::<T>() &&
obeys_feq_full::<T>()` would supply all three, but there is no way to demand it
of `eq`'s caller); and the ensures is over the mapped view `Set<T::V>`, where
view-equal ⇒ raw-equal needs injectivity. The accepts stay; no hole was added.
A closing route is a non-trait `fn eq_spec(&self, other) requires
self.spec_setsteph_wf()` that `PartialEq::eq` cannot call without the hole
moving, or the `PartialEqSpecImpl` route (`obeys_eq_spec`) that
`partial_eq_eq_clone_standard.rs` describes.

### Item 4. `obeys_concrete_eq::<V>()` in `HashMap::eq`

The plan says `obeys_eq` of `V`; the spec uses `obeys_concrete_eq::<V>()`
(§4.2). `obeys_eq::<V>()` says `V::eq` agrees with `eq_spec`, which may differ
from `==`; the view's `Map<K, V>` equality is `==` on `V`. If the user prefers
the plan's wording with an `eq_spec`-based conclusion, the ensures would need
`Map` equality up to `eq_spec` and no APAS caller wants that. Every current
value type (`bool`, `usize`, `V`, `(V, WrappedF64, usize)`) is concrete.

### Item 5. Preconditions dropped in Chap66

Thirteen `obeys_feq_view_injective::<V>()` requires (and the same conjunct
inside `obeys_feq_full::<V>()`-style bounds) were deleted from
`BoruvkaStEph.rs` and `BoruvkaMtEph.rs`: with raw `Map<V, _>` views nothing
needs injectivity, and the standard now says not to write it. This weakens
preconditions (callers have less to prove), never an ensures. The files verify
at 795/0.

### Item 6. `finite()` left on spec-level sets

`.finite()` remains in Chap65 on ghost sets and maps that are not hash views:
`DA` and `used_pairs` in `PrimStEph.rs` (lines 328, 334, 407, 413), and in
`UnionFindNoPCStEph.rs` / `UnionFindPCStEph.rs` the `spec_subtree(..).finite()`
conjuncts of `spec_size_rank_inv`-style predicates, `parent.dom().finite()` /
`po.dom().finite()` / `pn.dom().finite()` in the ghost-map lemmas, and the
`assert(st.finite())` steps in their proofs (about 40 sites). Only the
hash-view conjunct of `spec_uf_wf` was deleted. They are identically true at
09.13 and harmless, and Chap65 has not been through rustc (item 7), so they
were left rather than touched blind.

### Item 7. Chap65 not type-checked; two pre-existing E0432s

`scripts/validate.sh isolate Chap65` and the full run abort in rustc on
`src/Chap65/KruskalStEph.rs:29` (`use crate::Chap05::SetStEph::SetStEph::iter_invariant`)
and, in the full run, `src/Chap41/ArraySetStEph.rs:44`
(`crate::vstdplus::seq_set::lemma_push_not_contains_to_set`). Both names were
removed by r208; neither file is in the r209 list, so per rule 4 they were not
edited. Until Kruskal is fixed the three Chap65 edits (PrimStEph, UnionFind ×2)
have not been through rustc.

### Item 8. Chap62–65 proofs are unverified

The `key_view` rewrites and the prophetic loop proofs in `StarPartitionMtEph.rs`
(the largest: the merge loop in `hash_coin_flips_mt` and the post-loop `choose`
witnesses), `StarContractionMtEph.rs`, `ConnectivityMtEph.rs`, `SpanTreeMtEph.rs`,
`PrimStEph.rs` and the `UnionFind*` files type-check (except Chap65) but have
not reached Z3, because Chap19 is unmigrated. Expect a proof round on each when
Chap19 lands; `group_key_view_lemmas` is designed for it but its triggers have
only been exercised in Chap05 and the experiments.

### Item 9. Residual name mentions

Comments only; `grep` of `src/`, `tests/`, `rust_verify_test/`, `Cargo.toml`.

| # | Chap | File | Lines | Kind |
|---|------|------|-------|------|
| 1 | — | src/lib.rs | 75, 204, 206 | experiment annotations |
| 2 | vstdplus | hash_specs_plus.rs | 31, 70, 81 | doc comments (history) |
| 3 | 05 | SetStEph.rs | 951 | comment |
| 4 | 05 | SetMtEph.rs | 1215 | comment |
| 5 | std | using_hashmap_standard.rs | 12 | prose ("deleted in r209") |
| 6 | — | Cargo.toml | 2144, 2201 | comments |
| 7 | exp | six experiment files (E1) | — | commented out in lib.rs |

- E1: `vstd_hash_set_derived.rs`, `vstd_hash_map_derived.rs`,
  `simple_hash_set_iter.rs`, `mut_struct_quantifier_limit.rs`,
  `hash_set_with_view_plus_loops.rs`, `vstd_hash_set_clone.rs`; the last three
  would not compile if enabled.

### Item 10. PTTs not updated

`rust_verify_test/tests/Chap17/ProveMathSeq.rs` and `prove_MathSeq_iters.rs`
still use the pre-09.13 iterator model (r208 territory); `ptt.sh` was not run
this round. No PTT referenced the deleted wrappers other than the two deleted
files.

### Item 11. `for` loops with `break`

The inner searches in `SpanTreeStEph.rs`, `SpanTreeMtEph.rs` and
`TSPApproxStEph.rs` (Chap64) are `for x in it: coll.iter() { ...; break; }`.
`docs/PropheticIterators.md` (line 232) says a loop must draw its conclusion
before `break`, because the prophetic equality does not survive it. These
conversions set their result (a found flag or value) inside the body before
breaking and the post-loop code uses only the invariant, not `it.seq()`; whether
the invariants are strong enough is not yet Z3-checked (item 8).

### Item 12. `Pair` key-model trigger

Chap49–51 bodies that call `get`/`insert`/`contains_key`/`len` on
`HashMap<Pair<usize, X>, _>` now start with
`proof { let _ = Pair_feq_trigger::<usize, X>(); }` so `group_Pair_axioms` yields
`obeys_key_model::<Pair<usize, X>>()`, on which vstd's postconditions are
conditional. It is proof-only. A `broadcast` form would remove the per-body
line if `Pair_feq_trigger` were made a broadcast lemma in `Types.rs`.

### Item 13. Vacuous lock invariants

`SubsetSumMtEphMemoInv`, `SubsetSumMtPerMemoInv` (Chap49) and the two OBST
`Inv`s (Chap50) have `inv == true` (item 2.2). The RwLockPredicate rule wants a
real invariant: for SubsetSum, that every key's index is below the multiset
length (needs a `pub ghost len: nat` field); for OBST, `spec_memo_correct` over
the key/prob table as `MatrixChainMtEph` already does. Both need the constructor
to pass the ghost context; deferred as it is beyond the migration.

### Item 14. Chap62 `StarPartitionMtEph.rs` was rewritten with `Write`

The file's specs are unchanged in content (`X@` → `key_view(X@)` throughout),
but because every function mentions the partition map the edit was done as a
whole-file `Write`. Review the diff (`git diff src/Chap62/StarPartitionMtEph.rs`)
rather than trusting the list in §2.9.
