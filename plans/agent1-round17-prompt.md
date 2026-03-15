# Agent 1 — Round 17: Spec Audit Chap41 + Chap42

## Project State

103 holes, 4150 verified, 38 clean chapters, 8 holed.

## Mission: Fix Weak/Missing requires/ensures Against APAS Prose

This round is about **spec correctness**, not hole closure. You are auditing every
trait function's `requires`/`ensures` against the textbook definitions in `prompts/`.
Where specs are weak or missing, write the correct spec from the prose. If the
corrected spec breaks an existing proof body, add `#[verifier::external_body]` to
preserve the strong spec. A strong spec + `external_body` is better than a weak
spec + proved body.

**Read `prompts/Chap41.txt` and `prompts/Chap42.txt` FIRST.** These contain the ADT
definitions that are the source of truth.

## Chap41: Sets (ADT 41.1)

Files: `src/Chap41/AVLTreeSetStEph.rs`, `src/Chap41/ArraySetEnumStEph.rs`,
`src/Chap41/AVLTreeSetStPer.rs`

### Weak/Missing Specs Found

**1. `from_seq` — MISSING in all 3 files**

Current ensures: only `constructed.spec_*_wf()` and maybe `constructed@.finite()`.
Missing: `constructed@ == seq@.to_set()` (prose: `fromSeq(S) = range(S)`).

Correct ensures:
```
ensures
    constructed.spec_*_wf(),
    constructed@.finite(),
    constructed@ =~= seq@.to_set(),
```

If the existing body can't prove `constructed@ =~= seq@.to_set()`, add `external_body`.

**2. `filter` — WEAK in all 3 files**

Current ensures: `filtered@.subset_of(self@)` — says result is a subset but not
WHICH subset.

Prose: `filter(f, S) = {x ∈ S | f(x)}`. The correct spec requires that the result
contains exactly the elements satisfying the predicate.

This is hard to express in Verus because the predicate is a closure. The strongest
expressible spec is:
```
ensures
    filtered.spec_*_wf(),
    filtered@.finite(),
    filtered@.subset_of(self@),
    forall|x| filtered@.contains(x) ==> self@.contains(x),
    // Completeness requires f.ensures, which is closure-dependent.
    // Add if the closure's ensures makes it expressible.
```

Check `src/standards/using_closures_standard.rs` for how to reference closure ensures.
If the closure trait bound includes `ensures(result == true ==> ...)`, use it.
Otherwise, `subset_of` may be the strongest provable spec — note it as "partial" in
the audit, not "correct".

### Functions That Are Already Correct

- `empty`, `singleton`, `size`, `find`, `insert`, `delete`, `intersection`,
  `difference`, `union`, `to_seq` — all have correct ensures matching the prose.

## Chap42: Tables (ADT 42.1)

Files: `src/Chap42/TableStEph.rs`, `src/Chap42/TableStPer.rs`

### Weak/Missing Specs Found

**1. `tabulate` — WEAK in both files**

Current ensures: `tabulated@.dom() =~= keys@` + wf. Missing: value semantics.
Prose: `tabulate(f, S) = {k ↦ f(k) : k ∈ S}`.

Correct ensures (if closure ensures is available):
```
ensures
    tabulated.spec_*_wf(),
    tabulated@.dom() =~= keys@,
    forall|k| keys@.contains(k) ==> tabulated@[k] == f_result_for_k,
```

The challenge: expressing `f(k)` in ensures when `f` is a closure. Check if the
closure trait bound has `ensures(result == f.call(k))` or similar. If not, the
current spec may be the strongest expressible — note as "partial".

**2. `map` — WEAK in both files**

Current: `self@.dom() == old(self)@.dom()` (StEph) / `mapped@.dom() == self@.dom()` (StPer).
Missing: value transformation semantics.
Prose: `map(f, T) = {k ↦ f(v) : (k ↦ v) ∈ T}`.

Need: `forall|k| self@.dom().contains(k) ==> self@[k] == f(old(self)@[k])` (StEph).

**3. `filter` — WEAK in both files**

Current: subset + value preservation. Missing: predicate semantics.
Prose: `filter(p, T) = {(k ↦ v) ∈ T | p(k,v)}`.

Need: `forall|k| self@.dom().contains(k) ==> p(k, self@[k])` (completeness of filter).

**4. `intersection` — WEAK in both files**

Current: domain correct. Missing: value formula.
Prose: `intersection(f, T1, T2) = {k ↦ f(v1, v2) : k ∈ dom(T1) ∩ dom(T2)}`.

Need: `forall|k| common@.dom().contains(k) ==> common@[k] == f(self@[k], other@[k])`.

**5. `union` — WEAK in both files**

Current: domain correct. Missing: value formula for keys in both tables.
Prose: Uses combine function for overlapping keys.

Note: `TableStPer::union` has partial value specs (self-only and other-only cases).
Missing: the both-keys case with combine.

**6. `insert` — WEAK in StEph only**

Current (StEph): `self@.contains_key(key@)` + domain correct. Missing: what value
is stored. `TableStPer::insert` is CORRECT (has 3-case value spec using closure ensures).

Copy the StPer pattern to StEph: specify `self@[key@]` in both cases (new key, existing key).

### Functions That Are Already Correct

- `empty`, `singleton`, `size`, `domain`, `find`, `delete`, `restrict`, `subtract`,
  `entries`/`collect` — all correct.

## Deliverables

1. **`src/Chap41/analyses/spec-audit.md`** — per-function table with classification
2. **`src/Chap42/analyses/spec-audit.md`** — per-function table with classification
3. Corrected trait ensures in the StEph and StPer files
4. `external_body` on any impl fn that can't prove the strengthened spec
5. Clean validation (0 errors)

## DO NOT TOUCH

- Chap43 (Agent 2)
- Chap37, Chap38, Chap39 (Agent 3)
- Chap45, Chap47 (Agent 4)
- Any Example files
- Mt/MtPer files (their specs delegate to StEph/StPer)

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **The prose is the source of truth.** Not what's easy to prove.
- **NO accept().** NO assume→accept.
- **Add `external_body` if you can't prove the correct spec.** Never weaken ensures.
- Read `src/standards/using_closures_standard.rs` before writing closure specs.
- Read `src/standards/partial_eq_eq_clone_standard.rs` for eq/clone patterns.
- Push to `agent1/ready`. Write `plans/agent1-round17-report.md`.

## Target

Audit all trait fns in Chap41 (3 files) + Chap42 (2 files). Fix ~8 weak specs.
Produce spec-audit.md for both chapters. Hole count may increase (that's correct —
strong spec + external_body > weak spec + proved body).
