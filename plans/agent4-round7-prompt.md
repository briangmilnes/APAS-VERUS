# Agent 4 — Round 7: Chap41 + Chap45 + Chap26 + Chap66

## Mission

Close near-clean chapters (Chap66, Chap26) and advance Chap41 ArraySet proofs to help
unblock the Chap42/43 dependency chain. Also improve Chap45.

**Your success metric is holes eliminated and chapters closed.** Chap66 has 3 holes. Chap26
has 4 holes. Those are trivially closeable — if you don't close both, you've failed before
you started. The Chap41 ArraySet proofs are real proof work — set semantics, finiteness,
clone/view bridges. That's why you exist. Do not write "deferred" or "hard" or "would
require" — read the error, read vstd's `set_lib` and `seq_to_set` lemmas, write intermediate
assertions, decompose the obligation. Every assume is a proof obligation you can discharge.
ArraySetStEph is the keystone that unblocks Chap42 and Chap43. Prove it.

## Your Files (ONLY touch these)

**Chap66 (3 holes):**
1. `src/Chap66/BoruvkaStEph.rs` — 3 holes (ext_body)

**Chap26 (4 holes):**
2. `src/Chap26/ETSPStEph.rs` — 2 holes (ext_body)
3. `src/Chap26/ETSPMtEph.rs` — 2 holes (ext_body)

**Chap41 (14 holes, clean-deps files only):**
4. `src/Chap41/ArraySetStEph.rs` — 9 holes (assume: set semantics, clone/view bridge, finite)
5. `src/Chap41/ArraySetEnumMtEph.rs` — 5 holes (assume: finite set)

**Chap45 (3+ holes, clean-deps files only):**
6. `src/Chap45/BinaryHeapPQ.rs` — 2 holes (assume: multiset sorted)
7. `src/Chap45/LeftistHeapPQ.rs` — 1 hole (fn_missing_requires)
8. `src/Chap45/HeapsortExample.rs` — ~5 holes (fn_missing_spec + mixed)

**DO NOT touch Chap41/AVLTreeSet* files (blocked by Chap37). DO NOT touch Chap41/Example41_3.rs
(blocked by both AVLTreeSetStEph and ArraySetStEph).**

## Execution Order

1. Read `src/standards/*.rs` (all 15 files). Especially `partial_eq_eq_clone_standard.rs`
   and `finite_sets.rs` for set semantics proofs.
2. Run `scripts/holes.sh` on each chapter directory.
3. **BoruvkaStEph.rs** (3 holes): Remove ext_body. Read the algorithm, prove with real specs.
   Close Chap66 (+1 clean chapter).
4. **ETSPStEph.rs + ETSPMtEph.rs** (4 holes): Remove ext_body on ETSP functions.
   Close Chap26 (+1 clean chapter).
5. **ArraySetStEph.rs** (9 holes): These are real proof work.
   - Clone/view bridge assumes → use the eq/clone standard pattern (assume inside clone/eq body only).
   - `assume(finite())` → prove finiteness from array backing.
   - Set semantics assumes (insert/delete/contains) → intermediate assertions with seq↔set lemmas.
   Search vstd `seq_to_set` and `set_lib` lemmas.
6. **ArraySetEnumMtEph.rs** (5 holes): Finite set assumes. Similar pattern to ArraySetStEph.
7. **BinaryHeapPQ.rs** (2 holes): Multiset sorted property. May need intermediate lemmas.
8. **LeftistHeapPQ.rs** (1 hole): Add missing `requires` to `total_order_le`.
9. **HeapsortExample.rs** (~5 holes): Fix fn_missing_spec, prove remaining.
10. After each file: `scripts/validate.sh`. Fix all errors before moving on.
11. When done: `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.

## Rules

- Read CLAUDE.md fully.
- Never add assume(), accept(), admit(), or external_body.
- Never sequentialize Mt files.
- Search vstd before writing new lemmas.
- Commit to `agent4/ready` branch when done.

## Attitude

Prove big or go home. Chap66 and Chap26 are warmups — close them fast and move to the real
work. ArraySetStEph is the gate that the entire Chap42/43/52/53 chain is waiting on. Every
hole you close there has multiplicative impact downstream. The human is paying for compute
time and cleaning up after agents who write "too hard" instead of writing proofs. Don't be
that agent. Do the work.
