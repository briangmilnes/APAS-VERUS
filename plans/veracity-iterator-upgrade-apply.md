# Plan — `veracity-iterator-upgrade --dry-run-apply` and `--apply`

Status: SCOPING. Author: 2026-05-23.

Phase 2 of the prophetic-iterator migration tooling. Adds two new
modes alongside the existing `--detect`:

- `--dry-run-apply` — emit unified diffs per file, no mutation.
- `--apply` — rewrite files in place inside the veracity fixture.

`--detect` stays unchanged and remains the inventory/clustering pass
(see `plans/veracity-iterator-upgrade-detect.md`). This plan is
about turning that inventory into actual rewrites.

Companion documents:
- Detect plan: `plans/veracity-iterator-upgrade-detect.md`
- Detect review: `plans/veracity-iterator-upgrade-detect-review.md`
- Standard: `src/standards/prophetic_iterators_standard.rs`
- User reference: `docs/PropheticIterators.md`

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace
on Rust source. All edits must be token-aware or AST-aware. Parse
`ensures`/`requires` blocks with brace/comma/semicolon awareness. The
existing `verus_syn` re-parse + `render_expr` round-trip the matchers
already use is the right substrate for both modes — extend it, don't
sidestep it.

## 0. Work-area constraint — READ FIRST

Both new modes obey the same containment as `--detect`:

- `--root PATH` is REQUIRED, no default.
- The tool refuses to run if `--root` (after `realpath`) is not under
  `*/veracity/tests/fixtures/`. The
  `--i-know-what-im-doing-not-a-fixture` override still exists,
  verbose on purpose. Same rule as detect.
- `--out-dir` MUST be under `--root` or `/tmp`. Same rule.

`--apply` is the dangerous mode. It additionally requires:

- The fixture is a clean git checkout (`git status --porcelain` empty).
  If not, the tool refuses with `--apply: fixture has uncommitted
  changes; commit or stash before running`. Override:
  `--apply-on-dirty` (verbose on purpose).
- The tool prints a one-line confirmation banner at startup naming
  the absolute fixture path and the file count it will touch, then
  proceeds.

Reset path if `--apply` produces something wrong:
`cd <fixture-root> && git checkout -- .` — the fixture is a snapshot,
so the recovery is one command. Do NOT use `git clean`; that destroys
analyses output.

## 1. Goal

Given a `--detect` run's findings, produce — for every file with
mechanical findings — a unified diff (`--dry-run-apply`) or an
in-place rewrite (`--apply`) that:

1. Deletes every D1–D10 item (500 findings).
2. Applies every textually-trivial transform (T2, T3, T4, T9, T10:
   480 findings — substitution and clause-deletion).
3. Applies every templated transform (T1, T6, T8: 218 findings).
4. Leaves U-CUSTOM, U-CHAIN, U-CLASS findings (38) alone. These
   require human judgment and are not mechanical.

Total mechanical reach: 1198 of 1236 findings (~97 %). The remaining
38 are human work.

The tool MUST NOT touch any file outside `--root`.

## 2. Invocation

```
veracity-iterator-upgrade --dry-run-apply --root PATH [--out-dir PATH]
                                          [--ignore GLOB ...]
                                          [--only-classes LIST]

veracity-iterator-upgrade --apply         --root PATH
                                          [--ignore GLOB ...]
                                          [--only-classes LIST]
                                          [--apply-on-dirty]
```

- `--out-dir` (dry-run only) — default `analyses/`. Writes per-file
  diffs to `<out-dir>/diffs/<rel-path>.diff` plus a manifest at
  `<out-dir>/iterator-upgrade-apply.md` summarising the run.
- `--only-classes LIST` — comma-separated, e.g. `T9,T10,T3,T2,T4`
  to restrict to the textually-trivial set, or `D1,D2,...,D10` for
  delete-only, or `T1,T6,T8` for template-only. Default is "all
  mechanical classes" — D1–D10 + T1–T10. The U-classes are never
  applied.
- `--apply` has no `--out-dir`; it writes in place. It still emits a
  summary to stdout (or `analyses/iterator-upgrade-apply.log`) but
  the source files are the deliverable.

The modes are mutually exclusive with each other and with `--detect`.

## 3. What gets rewritten

The 1198 mechanical findings fall into three operational categories.

### 3.1 Range deletions (D1–D10, 500 findings)

For each D-finding, delete the contiguous line range `[line_start,
line_end]` recorded by the detect pass. Adjust subsequent line
numbers in the same file accordingly.

Ordering inside a file: apply deletions bottom-up (highest line
first). That keeps earlier-finding line numbers valid for the next
deletion without re-scanning.

Boundary cases:

- A D-item may be immediately preceded or followed by a blank line
  that was its visual separator. Delete trailing blank lines that
  become orphaned (two or more consecutive blank lines collapse to
  one) — do NOT touch blank lines that border other items still
  present.
- A D-item inside `verus! { ... }` keeps the macro braces intact.
  The matcher already locates the span correctly; just delete the
  bytes.

### 3.2 Token substitutions (T2, T3, T9, T10 = 427, plus T4 = 53)

These are the textually-trivial rewrites. For each finding, take the
clause's AST node (already recorded by the detect pass), walk it,
replace every:

- `it@.0` → `it.index()`
- `it@.1` → `it.seq()`

Re-render via `verus_syn::render_expr` and substitute the rendered
string into the source range `[col_start..col_end]` on `line`.

T4 (`iter_invariant(&it),`) is a clause-deletion: locate the call
expression inside its parent `ensures`/`invariant`/`requires` block,
remove the call and its trailing comma + leading whitespace, leave
the surrounding clauses intact. The brace-aware comma handling
already used by the matcher applies here.

### 3.3 Templated transforms (T1, T6, T8 = 218)

These need shape-matched template output. Each has a fixed template
in the plan; render the template with the per-finding bindings, then
substitute as in §3.2.

- **T1** — `it@.0 == K` becomes
  `IteratorSpec::remaining(&it).len() + K == it.seq().len(),` where
  `K` is the integer literal from the old (preserve its suffix:
  `0` stays `0`, `0int` stays `0int`).
- **T6** — `<expr>.len() - it@.0` becomes the constant
  `IteratorSpec::decrease(&it).unwrap(),`. The backing-seq
  expression on the LHS is discarded by the new form.
- **T8** — the constructor `ensures` triple replaces a three-clause
  block:

  ```
  it@.0 == 0,
  it@.1 == self.<F>@,
  iter_invariant(&it),
  ```

  becomes

  ```
  IteratorSpec::remaining(&it) == self.<F>@.as_ref(),
  IteratorSpec::decrease(&it) is Some,
  IteratorSpec::initial_value_relation(&it, &it),
  ```

  where `<F>` is the source field name (`seq`, `data`, `inner`, …)
  captured by the detect matcher. If the source uses `self.<F>@`
  (no `.as_ref()`) on the source's own iterator return, the new
  form keeps it that way for `into_iter`-style ensures and uses
  `.as_ref()` for borrowing-style. Use the source's structure as
  the template binding.

If a T8 finding doesn't match the canonical three-clause shape (e.g.
the source has extra clauses interleaved, or the order differs),
DOWNGRADE the finding to "skip with note" rather than guess. The
reviewer can hand-fix the outlier.

## 4. What gets skipped

The 38 U-class findings stay untouched in `--apply`:

| Class | Count | Why skipped |
|---|---:|---|
| U-CUSTOM | 18 | The 3 AVLTreeSeq files need hand-written `IteratorSpecImpl` |
| U-CHAIN  | 12 | Migration depends on backing-collection order |
| U-CLASS  |  8 | Pin-list vs matcher disagreement; human reconciles |

`--apply` records each U-finding's `file:line` in its run log so
reviewers know what's left. `--dry-run-apply` does the same; the
per-file diff for a U-only file is empty.

## 5. Wonky-parsing acceptance

`verus_syn::render_expr` is not byte-for-byte source-preserving. Round
trips through it normalize:

- whitespace (collapses runs of spaces; one space around binary ops)
- redundant parens (drops them where precedence allows)
- comments (lost — the renderer doesn't carry them)
- multi-line expression layout (collapses to one line, or breaks
  differently than the source)

**The user accepts these formatting changes for this migration.**
The mechanical correctness of the rewrite is what matters; cosmetics
can be normalized later if needed. The tool MUST NOT try to
heroically preserve formatting via string manipulation around the
renderer — that's exactly the string-hacking trap.

Consequences for the tool:

- Diffs in `--dry-run-apply` will show formatting changes even on
  unchanged sub-expressions. That's fine; flag this in the run
  manifest's preamble so reviewers don't mistake reformatting for
  semantic change.
- Inline comments inside a rewritten `ensures`/`invariant` clause
  are LOST. If a finding's source range contains a comment, the
  tool downgrades to "skip with note: comment present" rather than
  silently discarding it.

## 6. `--dry-run-apply` output

```
<out-dir>/
├── iterator-upgrade-apply.md      # run manifest (summary + skip list)
├── iterator-upgrade-apply.json    # same, machine-readable
└── diffs/
    ├── Chap05/MappingStEph.rs.diff
    ├── Chap05/RelationStEph.rs.diff
    ├── …
    └── vstdplus/hash_set_with_view_plus.rs.diff
```

Each `.diff` is a unified diff with the wide-MD `<style>` block
omitted (it's a patch, not a report). Apply with `git apply
<diff>` from `--root`.

The manifest carries:

- Same wide-MD `<style>` block as the detect report.
- Totals: `D=… T=… U-skipped=… files-changed=… findings-applied=…`.
- Per-file table: `Chap | File | Iter | D | T | T-skipped (notes)`.
- A "Skipped findings" table listing each finding the tool chose
  not to rewrite (comment present, T8 atypical shape, etc.) with
  file:line and reason.

No prose summary or count-delta narration — the manifest tables and
the diffs are the report.

## 7. `--apply` output

In-place rewrites, sequenced as:

1. For each file, sort findings within the file: D-class
   bottom-up by `line_start`; T-class bottom-up by `(line,
   col_start)`. This keeps earlier line numbers stable.
2. Apply findings in that order.
3. Write the file atomically (write to temp, fsync, rename).
4. Continue to the next file.

`--apply` produces a run log at `<root>/analyses/iterator-upgrade-apply.log`
with the same totals + per-file + skipped-findings tables as the
dry-run manifest. The log is committed alongside the rewrites so
the diff between before-rewrite and after-rewrite analyses files is
recoverable.

After `--apply`, the recommended next step is to run `--detect`
again on the rewritten fixture and confirm:

- D=0 (every D-finding was deleted)
- T=0 for all classes except whatever T8 outliers were skipped
- U-class counts unchanged

A `--detect` re-run with `T+D > 0` after `--apply` indicates the
mutator missed something. Investigate; do not paper over.

## 8. Verification

`--apply` is not "done" until Verus accepts the result. Out of scope
for the veracity tool itself, but the fixture-side validation is:

```
cd <fixture-root>
# vstd is already provided by verus; no extra setup
scripts/validate.sh  # if the fixture has one
# or, per-file:
verus --crate-type lib src/<chap>/<file>.rs
```

The fixture's `scripts/validate.sh` (if present) is the gate. A green
validate against a freshly-applied fixture is the migration's success
criterion. The veracity agent SHOULD run validate after each
`--apply` it produces, and report green/red. It should NOT attempt
to fix Verus errors — those go back to the APAS-VERUS side for
review.

## 9. Implementation outline

1. **Re-use the detect AST.** Both `--dry-run-apply` and `--apply`
   begin with a full detect scan (in-process, not a separate
   binary invocation). The findings drive the rewrites.
2. **Mutator visitor.** A new `verus_syn::visit_mut::VisitMut` impl
   walks each file's AST, mutates matched nodes per §3, and emits
   the rewritten file via the existing renderer.
3. **Per-finding rewrite plan.** Each detect finding gets a
   `Rewrite { kind: Delete(Range) | Substitute(Range, String) |
   Template(Range, String) | Skip(reason) }`. The plan is built
   first (no I/O); then `--dry-run-apply` renders diffs from it and
   `--apply` writes files from it. Same plan, different sinks.
4. **Comment detection.** Before computing any T-substitute or
   T-template rewrite, check if the source range contains a `//`
   or `/* */` token. If yes, downgrade to `Skip(reason: "comment
   present")`. Same check for blank-line-only intra-clause
   formatting that the renderer would collapse — if the source has
   visible structure that would be lost, skip.
5. **Atomic write.** Standard tempfile-then-rename for `--apply`.
6. **Unit tests.** Each T-class and D-class fixture from
   §13a.10 grows a `T<n>/golden.rewritten.rs` (the post-mutation
   source). The mutator integration test runs `--apply` on each
   fixture and `diff -u` against the rewritten golden.

## 10. Acceptance criteria

- `veracity-iterator-upgrade --dry-run-apply --root <fixture>`
  produces a `diffs/` tree and an apply-manifest. Zero source files
  modified (verified with `git status` inside fixture).
- `veracity-iterator-upgrade --apply --root <fixture>` rewrites
  files in place. Re-running `--detect` afterward reports D=0 and
  T=0 (modulo any explicit `Skip` findings from T8 outliers /
  comment-present cases).
- U-CUSTOM, U-CHAIN, U-CLASS counts unchanged across the apply.
- `scripts/validate.sh` from inside the fixture compiles cleanly
  (verification green or — on first run — verus errors that are
  about the proof, not about parse / type / iterator-API mismatch).
- The apply log lists every skipped finding with a one-line reason.
- `--apply` refuses to run on a dirty fixture without
  `--apply-on-dirty`.
- `--apply` refuses to run with `--root` outside `*/tests/fixtures/`
  without `--i-know-what-im-doing-not-a-fixture`.

## 11. Non-goals

- **Verification fixes.** If the rewritten code doesn't verify under
  Verus, that's a separate human-review pass — the tool's job ends
  at "produces a syntactically valid prophetic-iterator-shaped
  source."
- **Formatting preservation.** See §5. The renderer normalizes; we
  accept it.
- **Comment preservation.** Findings whose source range contains a
  comment are skipped with a note. The tool does not attempt to
  carry comments through the rewrite.
- **U-class handling.** The 38 U-class findings are human work.
  The tool reports their locations and stops there.
- **Apply against the live APAS-VERUS tree.** Veracity never touches
  the live tree. After `--apply` on the fixture, the APAS-VERUS
  side patches its own files by hand (or via a separate
  fixture-to-live sync tool, out of scope here).

## 12. Risks

1. **T8 shape heterogeneity.** The constructor `ensures` triple is
   sensitive to source order and field name. The migration plan
   counted 29 T8 findings; if a non-trivial fraction is atypical,
   the Skip count could grow. Acceptable — flag them, move on.
2. **Comment-bearing clauses.** Same: skip with note. The fixture's
   APAS-VERUS code has scattered `// Veracity: NEEDED assert` and
   similar markers in/around iterator clauses. Expect a double-digit
   skip count from this alone.
3. **Renderer regressions.** A future change to `verus_syn`'s
   renderer could subtly alter output. Pin the verus_syn version
   used by the mutator and add a smoke test (a multi-line clause
   that exercises the operators we care about) to the unit suite.
4. **Partial-application leaving a half-migrated fixture.** A crash
   or kill mid-`--apply` could leave some files rewritten and others
   not. The atomic per-file write keeps each file individually
   consistent; the fixture-wide reset is `git checkout -- .` as
   noted in §0.
5. **`--detect` after `--apply` not reaching 0.** If it doesn't, the
   gap is the actionable item. Don't ship `--apply` to others until
   the round-trip closes on the calibration fixture.

## 13. Out of scope

- Phase 3 ("apply to live APAS-VERUS"). That's a separate plan if it
  ever happens; the path is most likely "veracity emits a patch
  series, APAS-VERUS agent reviews and applies one-by-one with proof
  budget allocation," not "veracity mutates the live tree."

## 14. See also

- `plans/veracity-iterator-upgrade-detect.md` — the detect plan, §7,
  §13a (Phase 1.5 layout requirements), §14 (Phase 2 stub this plan
  replaces).
- `plans/veracity-iterator-upgrade-detect-do-deferred.md` — the four
  deferrals closed by the May 23 veracity work.
- `~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}`
  — current detect output.
- `~/projects/veracity/src/bin/iterator_upgrade.rs` — current detect
  source; the mutator extends this binary, not a new one.
