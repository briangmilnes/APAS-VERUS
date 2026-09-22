# r209 agent 1 — retire the APAS hash wrappers in favour of vstd 0.2026.09.13

Date: 2026-09-21. One subagent, sequential. Inputs: `docs/HashSpecsMigration.md`
(the plan being executed; read all of it), `plans/r207-round-plan.md` (common
rules), `plans/r208-agent1-chap02-06.md` (rules 1–9 apply here too),
`docs/Chap02to06Validation.md` §5 (how Chap05 was migrated),
`src/experiments/vstd_hash_set_derived.rs` and `vstd_hash_map_derived.rs` (the
derivations, one fn per method: copy their shapes).

## Goal

No file under `src/`, `tests/`, or `rust_verify_test/` names
`HashSetWithViewPlus`, `HashMapWithViewPlus`, or `hash_set_specs`; the three
modules are deleted; every call site uses `std::collections::HashSet` or
`HashMap` with vstd's `std_specs::hash` specifications; the two additions of
`docs/HashSpecsMigration.md` §3.2 exist in `src/vstdplus/hash_specs_plus.rs`;
every `/// - Alg Analysis` annotation in every edited file has been read
against its function body and confirmed or disputed in the doc. Deliverable:
`docs/HashMigration.md` plus the source edits. Nothing committed.

## Rules specific to this round

1. Rules 1–9 of `plans/r208-agent1-chap02-06.md` apply. Rule 4's permitted
   exec changes are: (a) the field type substitution `HashSetWithViewPlus<K>` →
   `HashSet<K>` and `HashMapWithViewPlus<K, V>` → `HashMap<K, V>`; (b) the
   iterator substitution (delegated iteration). The wrapper forwarded every
   call, so each method's cost is unchanged; say so per file in the doc.
2. Algorithmic analyses. For every file you edit, read every
   `/// - Alg Analysis` line together with the body it annotates and record
   in the doc, per function: Chap, file, fn, the annotation text, and
   "confirmed" or "disputed: <reason>". Never edit an annotation. A disputed
   one goes under "Needs discussion" with the work and span you read from the
   code.
3. Additions. Write `src/vstdplus/hash_specs_plus.rs` (register it in
   `lib.rs`, section 9 of the module layout) holding: (a) an
   `assume_specification` for `<HashSet<K, S, A> as Clone>::clone` modelled on
   vstd's `HashMap::clone` at `std_specs/hash.rs:578`, ensuring
   `other@.len() == this@.len()` and
   `forall|k| other@.contains(k) <==> exists|k0| this@.contains(k0) && cloned(k0, k)`;
   (b) `assume_specification`s for `<HashSet<K, S, A> as PartialEq>::eq` and
   `<HashMap<K, V, S, A> as PartialEq>::eq` ensuring `r == (a@ == b@)` under
   `obeys_key_model::<K>()` and the `laws_eq::obeys_eq` of `K` (and `V`).
   These are the only new trusted items allowed this round. They are
   `assume_specification`s, vstd's form of a trusted specification, not
   `external_body` fns, and each carries a doc comment naming the upstream
   proposal it mirrors. Write one experiment per addition in
   `src/experiments/` (`vstd_hash_set_clone_plus.rs`, `vstd_hash_eq_plus.rs`)
   showing the postcondition APAS needs follows, verified with
   `scripts/validate-standard.sh --experiment <name>`, with `RESULT`, `DATE`,
   `VERUS`, `LOG` headers, listed commented-out in `lib.rs`.
4. Verification reach. `Cargo.toml` says Chap49 and Chap51 depend on Chap18
   and Chap19, Chap50 on Chap30, Chap61 and Chap62 on Chap19, Chap63 and
   Chap64 on Chap62, Chap65 on Chap45. Those dependencies are not yet migrated
   to 09.13, so `validate.sh isolate` for those chapters stops in rustc.
   rustc still type-checks every file in the run, so: edit the chapter, run
   `isolate ChapNN`, read the log, and fix every error whose `-->` names a file
   you edited; errors in the unmigrated dependencies are recorded, not fixed.
   Chap05, Chap17 (no deps), and Chap66 (dep Chap05) must reach Z3 and finish
   at 0 errors, 0 warnings.
5. RTTs (`tests/Chap62`, `tests/Chap66`): edit them to the raw types; `rtt.sh`
   is not run this round. PTTs `rust_verify_test/tests/vstdplus/Hash{Set,Map}WithViewPlus.rs`:
   delete the files and their `[[test]]` entries in `rust_verify_test/Cargo.toml`
   when the modules are deleted (the user asked for deletion in
   `docs/HashSpecsMigration.md` §3.1 step 6; this is the one place deletion is
   authorised).
6. `src/standards/using_hashmap_standard.rs`: rewrite its example to the raw
   `HashMap` recipe (§3.1 steps 1–5) and verify it with
   `scripts/validate-standard.sh using_hashmap_standard`.

## Order

| # | Step | Files | Check |
|---|------|------:|-------|
| 0 | additions (rule 3) + their experiments | 1 + 2 | experiments verify |
| 1 | Chap05 `SetStEph.rs`, `SetMtEph.rs`: `Clone` via the new spec; the `PartialEq::eq` accepts may now close via (b), try it, do not add holes | 2 | `isolate Chap05`: 0 errors |
| 2 | `standards/using_hashmap_standard.rs` | 1 | standard verifies |
| 3 | Chap17 `MathSeq.rs` | 1 | `isolate Chap17`: 0 errors |
| 4 | Chap66 `BoruvkaStEph.rs`, `BoruvkaMtEph.rs` + `tests/Chap66` | 2 + 2 | `isolate Chap66`: 0 errors |
| 5 | Chap49 (8), Chap50 (8), Chap51 (4) | 20 | own-file errors 0 (rule 4) |
| 6 | Chap61 (4), Chap62 (4) + `tests/Chap62`, Chap63 (2), Chap64 (3), Chap65 (3) | 16 + 1 | own-file errors 0 |
| 7 | delete the three modules, their `lib.rs` lines, the two PTTs and entries; `grep` shows no remaining name | — | full `scripts/validate.sh` log read; record its error count by file (the crate is not expected to verify yet) |

## Deliverable: `docs/HashMigration.md`

1. Results table: Chap, file, before (wrapper hits), after (`N verified, M errors` or "type-checked, blocked on ChapNN"), log.
2. Per file: the edits (field, view, requires, iterator, broadcast) with old and new text for every spec clause that changed.
3. The Alg Analysis table of rule 2, every annotation in every edited file.
4. The additions: their text, the upstream proposal wording, the experiment results.
5. Needs discussion: disputed annotations, any exec change beyond rule 1, anything that would not verify with its error text.
