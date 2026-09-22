# r209 agent 3 — survey of hash table implementations verified in Verus

Date: 2026-09-21. One background subagent, research only. No source edits, no
Verus runs.

## Question

Which hash table implementations have been verified in Verus, with what
specifications and what trusted base? In particular: has anyone verified
Rust's default hash table, the SwissTable implementation in the `hashbrown`
crate that backs `std::collections::HashMap` and `HashSet`, in Verus or in any
other Rust verifier?

## Where to look

1. `~/projects/verus/source/vstd/std_specs/hash.rs` and `hash_map.rs`,
   `hash_set.rs`: what vstd itself does (these are `assume_specification`s,
   trusted, not proofs). State that plainly.
2. `~/projects/verus/examples/` (this checkout has `cuckoo_hash_table/`; read
   its README and record what is proven), and
   `~/projects/verus/source/rust_verify_test/tests/` for hash tests.
3. The Verus "verita" benchmark project list in the Verus repository
   (`gh api` on `verus-lang/verus`, look for the verita configuration and the
   projects it checks out) and the publications page of
   `https://verus-lang.github.io/verus/`: each project that contains a hash
   table.
4. GitHub code search (`gh api search/code` or `gh search code`) for Verus
   hash tables: queries such as `hashbrown verus`, `"verus!" HashMap impl`,
   `SwissTable verus`, `"IteratorSpecImpl" hash`, `open_hash verus`,
   `RawTable verus`, organisation `verus-lang`, and the known Verus
   codebases (Anvil, VeriSMo, Verus IronKV, verified-node-replication,
   page-table, memory-allocator, Atmosphere, Splinter, and whatever verita
   lists).
5. `~/projects/VerusCodebases/` if it exists (CLAUDE.md names it; it may not be
   on this machine).
6. Other Rust verifiers, for the hashbrown question only: Creusot, Prusti,
   Kani, Aeneas, Gillian-Rust, RustHornBelt, Flux; search for `hashbrown` with
   each. Record any proof of the SwissTable probing, group scanning (SSE2 /
   generic), resize, or `RawTable` invariants, with the paper or repository.
7. WebSearch for the papers: "verified hash table Rust", "SwissTable
   verification", "hashbrown formal verification".

## Rules

Read `plans/r207-round-plan.md`; its common rules bind you. Edit nothing under
`src/`, `tests/`, `rust_verify_test/`, `scripts/`, or `plans/`. Run no Verus,
cargo, or test script. No subagents. No commits. Every claim of "verified"
must name the artifact (repository, path, commit or paper) and what was
proven; a project that only trusts `assume_specification`s is "specified, not
verified". Write in the ComputAItionalThinking vocabulary.

## Deliverable: `docs/VerusHashTableSurvey.md`

1. Answer first: whether hashbrown has been verified anywhere, by whom, what
   part, and what remains trusted.
2. Table of Verus hash table implementations found: `#`, project, path or
   URL, table kind (chained, open addressing linear/quadratic, cuckoo, Robin
   Hood, SwissTable), what is proven (functional spec, memory safety only,
   termination, cost), trusted base (`external_body` count or `assume`
   count if readable), Verus version or date, and whether it is generic over
   the key type.
3. Table of hashbrown or SwissTable verification efforts in any verifier.
4. What vstd itself provides (specification only) and how APAS uses it
   (`docs/HashSpecsMigration.md`).
5. Assessment: whether any found implementation could replace APAS's use of
   `std::collections::HashMap` with a verified table without changing the
   O(1) expected cost specification, and what it would cost to adopt.
6. Searches run with zero results, so the negative is recorded.
