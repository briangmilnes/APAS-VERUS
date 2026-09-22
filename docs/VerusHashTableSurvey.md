# Hash tables verified in Verus, and the state of hashbrown verification

Date: 2026-09-21. Round: r209, agent 3. Research only; no source edits, no
Verus runs. Local checkout: `~/projects/verus` at
`release/rolling/0.2026.09.13.671956e` (commit `671956ec`, 2026-09-13).

Vocabulary. "Verified" means a prover discharged a specification against the
executable body. "Specified" means a contract was stated and taken on trust
(`assume_specification`, `external_body`, `admit()`, `#[trusted]`,
`extern_spec!`), with nothing proved about the body. "Client" means a project
that calls `std::collections::HashMap` under a trusted specification and
proves properties of its own code, not of the table.

## 1. Answer: has hashbrown (SwissTable) been verified?

No. No artifact found in this survey proves any part of the executable
hashbrown crate — `RawTable`, the control-byte group scan (SSE2 or generic),
the triangular probe sequence, resize and rehash, or the `HashMap`/`HashSet`
layers over it — in Verus or in any other Rust verifier (Creusot, Prusti,
Kani, Aeneas, Gillian-Rust, RustHornBelt, Flux, verify-rust-std). Verus's
vstd specifies `std::collections::HashMap` and `HashSet` from the outside
(`source/vstd/std_specs/hash.rs`, 1,450 lines: 45 `assume_specification`
items, 33 `broadcast proof fn` axioms whose bodies are `admit()`, 33
`uninterp spec fn`, 14 `external_body`, 11 `external_type_specification`,
3 proved lemmas); every Verus project that uses a standard hash map (IronKV,
CapybaraKV, Splinter, Amazon's semi-persistent containers, human-eval-verus)
is a client of that trusted specification. The closest thing to a hashbrown
proof is `nyuichi/rust-crate-proofs` (Creusot, created 2026-07-18, head
`3ecf33d` 2026-07-29), whose `hashbrown/0.17.1/src/verification.rs` (8.7 KB)
is a separate scalar model: a `ControlByte(u8)` with four proved functions
(`is_empty`, `is_empty_or_deleted`, `is_full`,
`convert_special_to_empty_and_full_to_deleted`) and a `Vec`-backed
`HashTable`/`HashMap`/`HashSet` with `new`, `len`, `is_empty`, `clear`,
`insert_unique`, `pop`; its `PROVENANCE.md` states that allocation, raw
pointers, SIMD groups, group matching, probing, rehashing, hash/equality
coherence, uniqueness, lookup, and insert/remove by key "remain outside
translation", and that the correspondence between the scalar model and the
real group bit expressions is exhaustively tested over `u8`, not proved. The
CSTs product `products/swisstable/README.md` (2026-07-21) records a further
negative: SwissTable has no peer-reviewed paper and no published formal
analysis of its probe cost; a from-scratch verification would be new work.
What remains trusted for APAS today is therefore the whole of hashbrown plus
`RandomState`/SipHash-1-3, taken on trust through vstd's 45
`assume_specification`s and 33 `admit()` axioms.

## 2. Hash table implementations in Verus

"Proven" names the property class the prover discharged; note N below the
table gives the detail. "Trusted" counts `external_body` (eb), `admit()`,
`assume`, or `accept()` in the table's own files. "Generic" says whether the
key type is a type parameter.

| # | Chap | Project / file | Kind | Proven | Trusted | Date | Generic |
|---|------|----------------|------|--------|---------|------|---------|
| 1 | ext | verus `examples/cuckoo_hash_table/` | cuckoo, 2 hashes, 4-way, fixed | functional, concurrent (note 1) | 1 eb + 2 eb in `main.rs`; 0 admit | 2026-07-15, PR #2326 | no: `u64`/`u64` |
| 2 | 47 | `ParaHashTableStEph.rs` | nested table over `Entry` | map refinement (note 2) | 0 | 2026-04-18 holes log | yes |
| 3 | 47 | `LinProbFlatHashTableStEph.rs` | open addr., linear probe, resize | map refinement (note 2) | 0 | same | yes |
| 4 | 47 | `QuadProbFlatHashTableStEph.rs` | open addr., triangular, `m = 2^k` | map refinement + permutation | 0 | same | yes |
| 5 | 47 | `DoubleHashFlatHashTableStEph.rs` | open addr., double hashing | map refinement (note 2) | 1 eb (`second_hash`) | same | yes |
| 6 | 47 | `FlatHashTable.rs` | shared flat-entry trait | entry specs | 1 `accept()` (l. 269) | same | yes |
| 7 | 47 | `VecChainedHashTableStEph.rs` | chaining, `Vec` buckets | map refinement (note 2) | 0 | same | yes |
| 8 | 47 | `LinkedListChainedHashTableStEph.rs` | chaining, `LinkedListStEphS` | map refinement (note 2) | 0 | same | yes |
| 9 | 47 | `StructChainedHashTable.rs` | chaining, recursive `Node` | map refinement (note 2) | 4 `accept()` | same | yes |
| 10 | ext | `zoratu/tlaplusplus verification/verus/` | lock-free linear probe, seqlock resize | protocol + probe model (note 3) | pointer-to-slice axiomatic | pushed 2026-08-30 | no: `u64` set |
| 11 | ext | `yaspar-org/semi-persistent .../map.rs` | log + std `HashMap` index | index invariant (note 4) | crate 27 eb, 1 axiom; table via vstd | pushed 2026-09-20 | yes |
| 12 | ext | `verified-ironkv .../hashmap_t.rs` | wrapper of std `HashMap` | nothing: `#![verus::trusted]` | 13 eb; `endpoint_` file 7 eb | before 2024-07 | no: `CKey` |
| 13 | ext | `microsoft/verified-storage` capybaraKV | client of std `HashMap` | KV layer only | vstd specification | verita `main` | client |
| 14 | ext | `verified-betrfs Implementation_v.rs` | client of `HashMapWithView` | — | vstd specification | — | client |
| 15 | ext | verus `vstd/hash_map.rs`, `hash_set.rs` | `*WithView` wrappers | nothing | 22 eb + 20 eb | 0.2026.09.13 | yes |

Notes.

1. Cuckoo (row 1): 1,456 lines plus `rwlock.rs` 683 and `main.rs` 70. Keys
   and values are `u64`; `HEIGHT = 1048576`, `WIDTH = 4`; `hash1`, `hash2`
   are fixed modular functions; no resize. `read`, `insert`, `delete` are
   specified with an `atomically` clause against a ghost
   `IMap<Key, Option<Value>>` (`tokenized_state_machine! Cuckoo`), rows are
   guarded by per-row `RwLock`. `insert` returns `success: bool`; its
   postcondition is `Some(new_val)` or unchanged. The one `external_body`
   in `cuckoo.rs` is `rand_u8`; `main.rs` has `print` and `runtime_assert`.
   No `assume` or `admit`. The directory is run by
   `rust_verify_test/tests/examples.rs` as a verifying example.
2. APAS Chap47 (rows 2–9): `View = Map<Key, Value>` via
   `spec_table_to_map`; `insert` ensures
   `table@ == old(table)@.insert(key, value)` plus well-formedness, with
   `requires obeys_feq_clone::<Key>()`, `::<Value>()`. Generic over
   `Key, Value, Entry, Metrics, H: Fn(&Key, usize) -> usize` with a ghost
   `spec_hash`. Trusted counts are from
   `src/Chap47/analyses/veracity-review-verus-proof-holes.log`
   (2026-04-18): `accept()` at `FlatHashTable.rs:269` and
   `StructChainedHashTable.rs:881, 899, 932, 947`; `external_body` on
   `DoubleHashFlatHashTableStEph.rs:105` (`second_hash`, SipHash). Nine
   files, 6,337 lines.
3. tlaplusplus (row 10): a fingerprint store for a TLA+ model checker.
   Tier B: 19 lemmas on a `Set<u64>` model (seqlock resize protocol,
   `theorem_no_fingerprint_lost`). Tier A (`seqlock_resize_tier_a.rs`): 31
   verified on a `Seq<u64>` model with `fp % capacity` linear probing
   (`lemma_probe_indices_distinct`, `lemma_insert_then_lookup`, CAS
   soundness). Shadow (`shard_methods.rs`): 17 verified on `PAtomicU64`
   cells with tracked permissions. The shipping code
   (`unsafe from_raw_parts` over mmap) is not verified; memory orderings are
   modeled as sequentially consistent. No values, no generic key.
4. semi-persistent (row 11, Amazon, Apache-2.0): `SpMap<K, V>` is an
   `AppendOnlyVec` log plus a `std::collections::HashMap<K, usize,
   IndexHasher>` index; the hasher is foldhash, the algorithm behind
   hashbrown 0.17's default. Proved: the index agrees with
   `is_last_occurrence`; `get_by_key`, `contains_key`, `restore`,
   `rebuild_index`. The crate's trust ledger: 27 `external_body`, 1
   `broadcast axiom fn` (`builds_valid_hashers::<IndexHasher>`), 0 `admit`,
   0 `assume`; verified against vstd 0.0.0-2026-08-02. The table itself is
   trusted through vstd.

Rows 1–10 are verified implementations; rows 11–14 are clients; row 15 is
specification only. Verus does not express asymptotic cost, so no row proves
a cost bound; APAS rows carry cost as `/// - Alg Analysis` annotations
(expected `O(1/(1−α))` for the flat tables, `O(1+α)` for chained).

Checked and excluded as not hash tables: `verified-node-replication`
(`nr_hashmap` is an unverified benchmark harness), `verified-memory-allocator`
(hits are in `mimalloc-bench` C sources), `verismo`, `verified-nrkernel`,
`svsm`, `anvil`, `vest`, `tla-rs` (152 hits, all transpiler and docs),
`asterinas/vostd`, `rlsf-verified`, `verified-graphs`, `human-eval-verus`
(`human_eval_054`, `_162` use `HashSet` as clients). The Verus
state-machine book page `source/docs/state_machines/src/examples/hash-table.md`
contains only the heading "Hash table (TODO)". The Verus publications page
(44 entries, fetched 2026-09-21) names no hash table.

## 3. hashbrown / SwissTable efforts in any verifier

| # | Chap | Verifier | Artifact | Covers | Status |
|---|------|----------|----------|--------|--------|
| 1 | ext | Verus (vstd) | `source/vstd/std_specs/hash.rs` | std API contracts (note 5) | specified, not verified |
| 2 | ext | Creusot | `rust-crate-proofs hashbrown/0.17.1` | scalar model (note 6) | partial model; `RawTable` excluded |
| 3 | ext | Creusot | `creusot-std .../hash_map.rs` | `extern_spec!`, `FMap` view | specified, not verified |
| 4 | ext | Creusot | `tests/should_succeed/hashmap_list.rs` | chained `MyHashMap<K, V>` (note 7) | verified, not hashbrown |
| 5 | ext | Aeneas (Lean) | `tests/src/hashmap.rs`, Lean output | chained map with resize, `Key = usize` | verified, not hashbrown |
| 6 | ext | Prusti | `viperproject/prusti-dev` | `hashbrown` in `Cargo.lock` only | none |
| 7 | ext | Kani, verify-rust-std | `model-checking/kani`, `verify-rust-std` | crate target lists only (note 8) | none |
| 8 | ext | Gillian-Rust, RustHornBelt, Flux | `Gillian`, `flux-rs/flux` | 0 hits; Flux depends on hashbrown | none |
| 9 | ext | Coq, Isabelle, Lean, Dafny | web search | no SwissTable formalization | none |
| 10 | ext | none (literature) | CSTs `products/swisstable/README.md` | no SwissTable paper (note 9) | citation only |

Notes.

5. vstd covers `HashMap`, `HashSet`, `DefaultHasher`, iterators, and the
   entry API (PR #2420). Issue #1835 (2025-07-22) traced a false-fact
   derivation through `HashMap::clone` to hashbrown issue #629 (a key whose
   `Clone` changes its hash); PR #2513 (2026-06-01) restated the `clone`
   specification. Section 4 counts the trusted items.
6. `hashbrown/0.17.1/src/verification.rs` (8.7 KB) is a separate proof
   target beside the unchanged crate: `ControlByte(u8)` with `valid_control`
   invariant and four proved functions, and `Vec`-backed `HashTable<T>`,
   `HashMap<K, V, S>`, `HashSet<T, S>` with `new`, `len`, `is_empty`,
   `clear`, `insert_unique`, `pop`. Reported result: "Proved (23 files)" in
   each of three feature configurations; 0 trusted bodies. `PROVENANCE.md`
   lists as excluded: allocation, pointer provenance, mirrored control
   bytes, SIMD group loads, group matching, quadratic probing, growth and
   rehashing, hash/equality coherence, uniqueness, lookup, insertion and
   removal by key, entry and raw-entry APIs, iterators. The scalar-to-group
   correspondence is an exhaustive `u8` test, not a proof.
7. `MyHashMap<K, V>` over `Vec<List<(K, V)>>`, `K: Hash + Copy + Eq +
   DeepModel`; `add` and `get` proved; `List::clone` is `#[trusted]`.
8. `hashbrown` appears in Kani's `tests/remote-target-lists/top-100-crates`
   files and `Cargo.lock`; 0 hits in `verify-rust-std`. The NFM 2026 paper
   "Verifying the Rust Standard Library" (arXiv 2606.17374, Cook et al.)
   abstract names no hash table.
9. The CSTs README (2026-07-21) finds no peer-reviewed SwissTable paper and
   no published formal analysis of its probe cost; the CSTs `std` wrapper
   crate (`collections/hash/map.rs` 149 lines, 102 `assume_specification`;
   `set.rs` 108 lines, 74) is a specification collection with no proofs, and
   its `51-52-swisstable-hash/CITATION.md` cites APAS Chap47 and vstd as the
   nearest existing proofs, match "partial".

## 4. What vstd provides, and how APAS uses it

vstd `std_specs/hash.rs` at 0.2026.09.13 is a specification, not a proof.
Its trusted base, counted by grep on 2026-09-21:

| # | Item | Count | Role |
|---|------|------:|------|
| 1 | all | `assume_specification` | 45 | hasher 3, map 19, set 12, entry 11 |
| 2 | `proof fn` with `admit()` body | 33 | key-model, hasher, bridge axioms |
| 3 | all | `uninterp spec fn` | 33 | views, `obeys_key_model`, hashers |
| 4 | all | `external_body` | 14 | opaque struct registrations |
| 5 | all | `external_type_specification` | 11 | map, set, iterators, entries |
| 6 | all | `external_trait_specification` | 2 | — |
| 7 | proved `broadcast proof fn` | 3 | `lemma_hashmap_deepview_*` |

The header of `hash.rs` states the model's three assumptions on `Key`
(deterministic `hash`, `==` agrees with identity, `clone` is identity) and that
a user key type needs `assume(obeys_key_model::<MyKey>())`. The wrappers
`hash_map.rs` (`HashMapWithView`, 22 `external_body`) and `hash_set.rs`
(`HashSetWithView`, 20 `external_body`) add nothing proved. The test file
`rust_verify_test/tests/hash.rs` holds 24 tests that exercise the
specification from client code.

APAS (`docs/HashSpecsMigration.md`, 2026-09-20) is migrating from its own
trusted wrappers (`hash_set_with_view_plus.rs`, `hash_map_with_view_plus.rs`,
`hash_set_specs.rs`: 24 trusted items, 640 lines) to the raw
`std::collections::HashMap`/`HashSet` under vstd's specification: 45 user
files in 13 chapters (05, 17, 49, 50, 51, 61–66, the standard, tests). The
experiments `vstd_hash_set_derived.rs` (11 verified, 0 errors) and
`vstd_hash_map_derived.rs` (12 verified, 0 errors) show 20 of 22 APAS
postconditions derive from vstd with no APAS trusted code; `HashSet::clone`
and `PartialEq::eq` remain missing (Phase B). Both before and after that
migration the SwissTable internals are trusted; the migration changes who
states the trust (vstd instead of APAS), not its extent.

APAS also owns the nine verified tables of Chap47 (section 2, rows 2–9),
which do not depend on `std::collections::HashMap` at all: the table storage
is `Vec<Entry>` and the hash function is a caller-supplied
`H: Fn(&Key, usize) -> usize` paired with a ghost `spec_hash`.

## 5. Assessment: can a verified table replace `std::collections::HashMap` in APAS?

| # | Chap | Candidate | Generic | Resize | Mt | Trusted base | Fit |
|---|------|-----------|---------|--------|----|--------------|-----|
| 1 | 47 | APAS flat tables (rows 3–5) | yes | yes | no | 1 eb, 1 `accept()` | closest (note 10) |
| 2 | 47 | APAS chained tables (rows 7–9) | yes | yes | no | 4 `accept()` | as row 1 |
| 3 | ext | verus cuckoo example | no | no | yes | 1 eb | no: fixed, `u64` |
| 4 | ext | tlaplusplus shard | no | protocol | yes | shipping unverified | no: set of `u64` |
| 5 | ext | semi-persistent `SpMap` | yes | via std | no | std under vstd | no gain: client |
| 6 | ext | vstd `HashMap` (current plan) | yes | yes | no | 45 assume + 33 admit | in progress |

Cost specification. Verus states no asymptotic cost, so the O(1) expected
cost of APAS's memo tables and graph sets is an `/// - Alg Analysis`
annotation in both designs. Row 1 keeps it: the flat tables carry
`Work O(1/(1−α)) expected` on `insert`, `lookup`, `delete` and
`O(n + m + m')` on resize, which is O(1) expected amortized at a constant
load factor, the same contract APAS's users state today. Rows 3 and 4 change
the ADT (no values, no growth) and are excluded on that ground.

Note 10, cost to adopt row 1, measured against `docs/HashSpecsMigration.md`
§3.1:

1. Field types in 45 files across 13 chapters change from `HashMap<K, V>` to
   `HashTable<K, V, FlatEntry<K, V>, Metrics, H>`; each user supplies an
   exec hash closure and a ghost `spec_hash`, proves `spec_hash_fn_valid`,
   and discharges `obeys_feq_clone::<Key>()` and `::<Value>()` at every
   `insert` call.
2. Iteration: Chap61–66 iterate their sets and maps. This survey did not
   confirm that the Chap47 tables implement
   `src/standards/iterators_standard.rs` (section 10 components); if they do
   not, that is new verified code per table before adoption.
3. `Clone` and `PartialEq` on the table: needed by the Chap49–51 memo tables
   and by `SetStEph`/`MappingStEph` equality; the Chap47 tables would need
   the bridge pattern of `partial_eq_eq_clone_standard.rs`.
4. Mt users (`SetMtEph`, `StarPartitionMtEph` 34 hits, `BoruvkaMtEph` 33
   hits) need the coarse `RwLock` wrapper of standard 16 around the table.
5. The trusted base moves from vstd's 78 trusted items on `std::HashMap`
   (shared with every verita project) to the hash function alone plus the 5
   `accept()`s and 1 `external_body` in Chap47, which are APAS's to close.

Measured against the in-progress plan (row 6: field-type substitution only,
exec bodies unchanged, 20 of 22 postconditions already derived), row 1 is a
second migration of the same 45 files at higher per-file cost, with the gain
that the table body is proved rather than assumed. No external artifact
provides a verified SwissTable to substitute; the choice is between vstd's
trusted `std::HashMap` and APAS's own verified Chap47 tables.

## 6. Searches run with zero (or only irrelevant) results

GitHub code search, `gh api search/code`, 2026-09-21; `total_count` first,
then what the hits were.

| # | Scope | Query | Result |
|---|-------|-------|--------|
| 1 | all | `hashbrown verus` | 2,136: logs, `.d` files, nix; no proof |
| 2 | all | `SwissTable verus` | 48: CSTs/trustd docs, lists; no proof |
| 3 | all | `RawTable verus` | 149: unrelated (F#, Java, newsletters) |
| 4 | all | `hashbrown assume_specification` | 238: logs, CSTs, semi-persistent note |
| 5 | all | `hashbrown external_type_specification` | 108: same set |
| 6 | all | `open_hash verus` | 0 |
| 7 | all | `IteratorSpecImpl hash` | 0 |
| 8 | all, `language:Rust` | `robin hood verus` | 38: all EFF word lists |
| 9 | `model-checking/verify-rust-std` | `hashbrown` | 0 |
| 10 | `model-checking/verify-rust-std` | `HashMap` | 0 |
| 11 | `AeneasVerif/aeneas` | `hashbrown` | 0 |
| 12 | `GillianPlatform/Gillian` | `hashbrown` | 0 |
| 13 | `GillianPlatform/Gillian` | `HashMap` | 0 |
| 14 | `creusot-rs/creusot` | `hashbrown` | 1: `Cargo.lock` |
| 15 | `viperproject/prusti-dev` | `hashbrown` | 1: `Cargo.lock` |
| 16 | `model-checking/kani` | `hashbrown` | 4: target lists, `Cargo.lock` |
| 17 | `flux-rs/flux` | `hashbrown` | 6: dependency use only |
| 18 | all | four-word query (note 11) | 2: a thesis draft, an agent note |
| 19 | `anvil-verifier/anvil` | `HashMap` | 0 |
| 20 | `parno/verified-nrkernel` | `HashMap` | 0 |
| 21 | `coconut-svsm/svsm` | `HashMap verus` | 0 |
| 22 | `asterinas/vostd` | `HashMap` | 0 |
| 23 | `unsoundsystem/rlsf-verified` | `HashMap` | 0 |
| 24 | all, `language:Rust` | `verified-graphs verus HashMap` | 0 |
| 25 | `verus-lang/verus` | `hash_table` | 2: `std_specs/hash.rs`, `examples.rs` |
| 26 | issues, `verus-lang/verus` | `hashbrown` | 2, unrelated (#2455, #1611) |

Note 11: row 18 is the exact query `hashbrown formal verification RawTable
proof`.

Web search, same day: `hashbrown SwissTable formal verification proof Rust`;
`"verified hash table" Rust Verus OR Creusot OR Prusti OR Aeneas OR Kani`;
`Verus verification "hash table" paper`; `verify-rust-std HashMap hashbrown
challenge Kani AWS`; `Gillian-Rust OR RustHornBelt OR Flux hashbrown
verification`; `"SwissTable" OR "Swiss table" verification Coq OR Isabelle OR
Lean OR Dafny`. None returned a SwissTable or hashbrown proof; the only
verified-hash-table hit was the Aeneas ICFP 2023 case study (section 3, row 5).

Local: `~/projects/VerusCodebases/` does not exist on this machine;
`~/projects/trustd` does not exist (its reports appear only on GitHub).
`~/projects/verus/examples/cuckoo_hash_table/` has no README; what it proves
was read from `cuckoo.rs` directly.

## 7. Sources

- `~/projects/verus/source/vstd/std_specs/hash.rs`, `hash_map.rs`, `hash_set.rs`, `rust_verify_test/tests/hash.rs`, `examples/cuckoo_hash_table/`, `tools/verita/run_configuration_all.toml`, `source/docs/state_machines/src/examples/hash-table.md` at `671956ec`
- `/home/milnes/projects/APAS-VERUS/src/Chap47/*.rs` and `src/Chap47/analyses/veracity-review-verus-proof-holes.log` (2026-04-18)
- `/home/milnes/projects/APAS-VERUS/docs/HashSpecsMigration.md` (2026-09-20)
- `~/projects/CSTs/products/swisstable/README.md`, `products/verified-algorithms/std/proofs/51-52-swisstable-hash/CITATION.md`, `products/verified-algorithms/std/src/collections/hash/{map,set}.rs`
- https://github.com/nyuichi/rust-crate-proofs (`hashbrown/0.17.1/PROVENANCE.md`, `src/verification.rs`, head `3ecf33ddab79f30b840c4e3fe108b75101f92a36`)
- https://github.com/yaspar-org/semi-persistent (`containers-verus/src/map.rs`, `hasher_spec.rs`, `doc/design/02-trust-boundary.md`)
- https://github.com/zoratu/tlaplusplus (`verification/verus/README.md`, `shard_methods.rs`, `src/storage/verified_fp_shard.rs`)
- https://github.com/verus-lang/verified-ironkv (`ironsht/src/hashmap_t.rs`, `endpoint_hashmap_t.rs`)
- https://github.com/microsoft/verified-storage (`capybaraKV/capybarakv/src/kv2/keys/impl_v.rs`)
- https://github.com/vmware-labs/verified-betrfs (`Splinter/src/implementation/Implementation_v.rs`)
- https://github.com/creusot-rs/creusot (`creusot-std/src/std/collections/hash_map.rs`, `tests/should_succeed/hashmap_list.rs`)
- https://github.com/AeneasVerif/aeneas (`tests/src/hashmap.rs`)
- Verus issues and PRs #1165, #1835, #2420, #2513; hashbrown issue #629
- https://verus-lang.github.io/verus/publications-and-projects/ (fetched 2026-09-21)
- arXiv 2606.17374 (Cook et al., "Verifying the Rust Standard Library", NFM 2026); arXiv 2510.25015 (VeriStruct)
