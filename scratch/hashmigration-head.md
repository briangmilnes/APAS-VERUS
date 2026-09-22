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
