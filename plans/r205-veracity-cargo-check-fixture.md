# Plan — r205 veracity-side: `cargo check` gate on the rewritten fixture

Status: SCOPING. Author: 2026-05-23.

**This plan is for veracity, working in `~/projects/veracity/`.**
APAS-VERUS-side trial of the rewritten files in the live tree is
separate and not covered here.

Goal: confirm the Phase 2 `--apply` output is syntactically and
type-correct Rust. Run `cargo check` inside the veracity fixture, fix
any errors that surface, report back. The Verus verification gate is
APAS-VERUS-side and out of scope.

Companion documents:
- `plans/r204-veracity-iterator-upgrade-apply.md` — the apply plan
- `plans/r204-veracity-iterator-upgrade-apply-report.md` — Phase 2 result
- `docs/PropheticIterators.md` — target shape
- `src/standards/prophetic_iterators_standard.rs` — canonical shape

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive
find-and-replace on Rust source. All edits (including manual fixes
to the rewritten files) use AST-aware tooling or careful manual
editing — never sed/awk/perl one-liners on the source.

## 0. Work-area and explicit permission

The work happens **entirely inside**:

```
~/projects/veracity/tests/fixtures/APAS-VERUS/
```

Veracity MUST NOT read from, write to, or otherwise touch the live
APAS-VERUS tree at `~/projects/APAS-VERUS/`. The live tree is
where the human and the APAS-VERUS agents work; veracity edits there
would race with in-flight proof work.

### `cargo check` permission

The project-wide CLAUDE.md rule "**never run `cargo build` /
`cargo check` for verification**" does NOT apply to this work.

- This is **not a verification attempt.** It's a syntactic + type
  gate. Verus is not invoked. No proofs are checked.
- The work happens **inside the veracity fixture**, not the live
  APAS-VERUS tree. The fixture exists for exactly this kind of
  trial.
- Veracity has full permission to run `cargo check` (and `cargo
  build` if it helps surface link/codegen issues), iterate, and
  ship fixes until both check variants below are green.

## 1. What to run

```
cd ~/projects/veracity/tests/fixtures/APAS-VERUS
cargo check
```

`cargo check` must be green. The `verus!` macro strips spec/proof
items by default; rustc typechecks only the executable Rust. This
catches mutator damage to iterator return types, missing methods,
malformed `IteratorSpec::*` calls, broken trait impls, unclosed
braces, lost trailing-`;` on trait method declarations.

If `cargo check` has cross-chapter scope issues (a file in ChapNN
depending on a chapter veracity's fixture didn't pull in), use the
isolate-feature pattern that APAS-VERUS already supports:

```
cargo check --features Chap05
```

Start with Chap05 isolation since that's the next migration target;
expand outward (`Chap06`, then full crate) once that's green.

**Note: there is no plain-rustc gate for `ensures`/`requires`/
`invariant` clause shapes.** Setting `--cfg verus_keep_ghost` would
enable Verus-only syntax in `vstd` that plain rustc cannot parse
(verified r205: ~8400 errors in vstd alone, none in user code). The
right gate for spec-clause Rust correctness is `scripts/validate.sh`
(invokes the Verus compiler), which is APAS-VERUS-side per §4. Do
not attempt a `verus_keep_ghost` variant of `cargo check`.

## 2. What to fix and how

Two paths, in this order of preference:

### 2.1 Mutator-source fix (preferred)

The bug is in `~/projects/veracity/src/bin/iterator_upgrade.rs`.
Fix it there:

1. Identify the error pattern from `cargo check` output.
2. Reproduce on a minimal `.rs` fixture under
   `tests/fixtures/iterator-upgrade-detect/` (add a new T-class or
   D-class fixture if the bug doesn't fit existing classes).
3. Add the case to the failing-on-purpose state, fix the mutator,
   confirm the new fixture's golden matches.
4. Re-apply on the APAS-VERUS fixture:

   ```
   cd ~/projects/veracity/tests/fixtures/APAS-VERUS
   git checkout -- src
   cd ~/projects/veracity
   ./target/release/veracity-iterator-upgrade --apply \
     --root tests/fixtures/APAS-VERUS \
     --i-know-what-im-doing-not-a-fixture --apply-on-dirty
   ```

5. Re-run `cargo check`. Loop until green.

This is the right fix for any error pattern affecting more than one
file.

### 2.2 In-fixture hand-edit (workaround)

If a single-file workaround is materially faster than a mutator
fix (e.g. a one-off T8 atypical shape on a single file), hand-edit
the rewritten file in the fixture and tag it. Add this comment
right under the SPDX license header at the top of the file:

```rust
// VERACITY-FIXTURE-MANUAL-FIX r205: <one-line reason>
```

The tag tells future readers (and the next `--apply` run) that the
file carries a manual fix and should not be assumed re-derivable
from the mutator.

Do NOT use 2.2 if the same pattern affects multiple files; that is
2.1's job.

## 3. Report back

When `cargo check` is green, write a short report at:

```
/home/milnes/projects/APAS-VERUS/plans/r205-veracity-cargo-check-fixture-report.md
```

(This is on the APAS-VERUS side — that's the one exception to §0's
"no live-tree writes" rule, and only for this one report file. The
report touches `plans/`, not `src/`; it's a plan-side communication
artifact, not a code change.)

Content — keep it short, tabular, no prose narration:

1. Final `cargo check` status (green).
2. Table of mutator-source fixes shipped:
   `# | veracity commit | one-line summary | error class`.
3. Table of in-fixture hand-edits:
   `# | file | reason | tag-line text`.
4. Table of errors veracity could not fix and is escalating to
   APAS-VERUS (very rare — usually empty):
   `# | file | line | error excerpt | why not fixed`.

The `cargo check` log itself is the evidence; no need to transcribe
it into the report.

## 4. Non-goals

- **Verus verification.** Out of scope. Verifying the rewritten
  code is APAS-VERUS-side, gated by `scripts/validate.sh`.
- **Hand-porting U-CUSTOM (AVLTreeSeq) files.** Out of scope. Those
  3 files need `IteratorSpecImpl` written from scratch and that's
  APAS-VERUS-side work per the iterator standard.
- **Touching `src/` in the live tree at `~/projects/APAS-VERUS/`** —
  except the single report file in `plans/`, per §3.
- **Phase 3 (apply to live).** Not happening. The live changes are
  manual `cp` per file, gated by APAS-VERUS, after this report
  comes back green.

## 5. References

- `plans/r204-veracity-iterator-upgrade-apply.md` — apply plan
- `plans/r204-veracity-iterator-upgrade-apply-report.md` — Phase 2 result
- `~/projects/veracity/src/bin/iterator_upgrade.rs` — mutator source
- `~/projects/veracity/tests/fixtures/APAS-VERUS/` — fixture root
- `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/` — matcher unit-test fixtures
