
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
