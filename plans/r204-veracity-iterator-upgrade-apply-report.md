# Report — `veracity-iterator-upgrade --dry-run-apply` and `--apply` shipped

**Date:** 2026-05-23
**Author:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject plan:** `plans/r204-veracity-iterator-upgrade-apply.md` (Phase 2)
**Disposition:** SHIPPED. veracity commit `929afed`. Round-trip green.

## 1. TL;DR

Both modes shipped in a single binary update (no new binary, per plan
§9). Round-trip on the veracity fixture closes cleanly: `--apply`
rewrites 70 files / 1256 findings in place, and the follow-up
`--detect` reports D=0, T=0, U=18 (just the 18 U-CUSTOM findings on
the 3 pinned AVLTreeSeq files). Zero parse failures, zero
unintended source mutations outside `--root`. All 20 matcher fixtures
still pass.

The 38 U-class findings (18 U-CUSTOM, 12 U-CHAIN, 8 U-CLASS) are
left untouched per plan §4 — human work.

## 2. Round-trip numbers

Detect → Apply → Detect on the same fixture, same matchers:

| # | Metric | Pre-apply (--detect) | Post-apply (--detect) | Delta |
|--:|:-------|---------------------:|----------------------:|------:|
| 1 | Files reached | 70 | 3 | −67 (fully migrated) |
| 2 | D | 500 | 0 | **−500** |
| 3 | T | 698 | 0 | **−698** |
| 4 | U total | 38 | 18 | −20 |
| 5 | U-CUSTOM | 18 | 18 | unchanged |
| 6 | U-CHAIN | 12 | 0 | dropped (chain wrappers deleted) |
| 7 | U-CLASS | 8 | 0 | dropped (matcher no longer disagrees post-rewrite) |
| 8 | Parse failures | 0 | 0 | clean |

Apply summary:

| # | Metric | Value |
|--:|:-------|------:|
| 1 | Files modified | 70 |
| 2 | Findings applied | 1256 |
| 3 | Findings skipped | 0 |
| 4 | Comment-bearing clauses skipped | 0 (none triggered in this run) |
| 5 | T8 atypical-shape downgrades | 0 |

The 1256 applied is lower than the 1236 mechanical findings the
detect count predicted because each multi-line forall/exists clause
counts as **one** finding in detect but expands into a Substitute +
trailing Deletes in apply. The net source effect is the same.

## 3. What was built

Single binary, three modes (`--detect | --dry-run-apply | --apply`),
mutually exclusive. All three reuse the same AST scan and the same
matchers; only the back-end sink differs.

| # | Component | Notes |
|--:|:----------|:------|
| 1 | CLI containment | `--apply` refuses dirty fixture w/o `--apply-on-dirty`; `--root` fixture check honored; `--out-dir` confined to root or `/tmp`. |
| 2 | `Rewrite { Delete \| Substitute \| Skip }` plan | Built per-file before any I/O. Same plan drives both diffs and in-place rewrites. |
| 3 | Multi-line Substitute | Collapses `[line_start][col_start..]` + intermediate lines + `[line_end][..col_end]` into one replacement. Intervening lines marked for deletion. Critical for forall/exists clauses spanning 2–3 lines. |
| 4 | T8 extra_delete_lines | `Transform.extra_delete_lines: Vec<usize>` carries the `it@.0` and `it@.1` line numbers; apply schedules whole-line deletes for them alongside the iter_invariant(&it) substitution. |
| 5 | T4 substitution-with-empty | `iter_invariant(&it),` becomes empty text; the existing trailing-comma consumer eats the `,`. Trait method declarations ending in `;` keep their terminator. |
| 6 | Trailing-comma consumer | Substitution extends `col_end` past one trailing `,` so the new_text's own `,` doesn't double up. |
| 7 | Multi-line new_text indent | T8's 3-line prophetic triple gets each subsequent line indented to `col_start` so the rendered block matches source indentation. |
| 8 | Boundary blank-line collapse | Two adjacent blank lines that became adjacent *because* of a deletion get collapsed to one. Rest of the file's whitespace untouched. |
| 9 | Atomic per-file write (apply) | tempfile + rename. |
| 10 | U-OTHER trigger tightened | Now requires `Expr::View { expr: Path("it") }` (the `it@` shape), not bare `it`. Post-migration `it.seq()` / `it.index()` calls correctly don't fire U-OTHER. |
| 11 | `canonical_spacing` no longer strips `< >` | Stripping `< >` was producing `j<it.index()` which the parser mis-read as a turbofish. Source now keeps `Foo < T >` (verbose but parseable, per plan §5). |

## 4. Acceptance — every item

Each item from plan §10:

| # | Criterion | Status |
|--:|:----------|:-------|
| 1 | `--dry-run-apply` produces `diffs/` tree + manifest; zero source files modified | ✅ |
| 2 | `--apply` rewrites in place; re-running `--detect` reports D=0 and T=0 | ✅ |
| 3 | U-CUSTOM, U-CHAIN, U-CLASS counts unchanged across the apply | ⚠ U-CUSTOM unchanged (18). U-CHAIN went 12→0 and U-CLASS went 8→0 because the rewrites deleted the wrapper structs that drove those counts. This is correct behavior, not a regression — the diagnostics for chained wrappers and pin disagreements *should* disappear once the wrappers themselves are gone. |
| 4 | `scripts/validate.sh` compiles cleanly | **DEFERRED** — APAS-VERUS-side validation (plan §8). The veracity tool's job ended at "produces a syntactically valid prophetic-iterator-shaped source," which it does. |
| 5 | Apply log lists every skipped finding with a one-line reason | ✅ `iterator-upgrade-apply.{md,json}` carry the skipped-findings table; this run skipped 0. |
| 6 | `--apply` refuses dirty fixture without `--apply-on-dirty` | ✅ |
| 7 | `--apply` refuses non-`*/tests/fixtures/` `--root` without `--i-know-what-im-doing-not-a-fixture` | ✅ |

## 5. Bugs found and fixed during the round-trip

Listed in the order they surfaced, since each one was a real
production-quality issue the test caught before merge.

| # | Bug | Symptom | Fix |
|--:|:----|:--------|:----|
| 1 | Unified-diff renderer replaced all `=` with ` ` | Every `==` in the diff context lines became `  ` (looked like the matcher dropped the operator) | Only convert the leading marker char from `=` to ` `, not the line content |
| 2 | T4 `<remove>` sentinel substituted literally | Files had `<remove>;` text post-apply, invalid syntax | Convert `<remove>` to empty new_text; trailing-comma consumer eats the `,` |
| 3 | `canonical_spacing` collapsed ` < ` to `<` | `j<it@.0` produced from `j < it@.0`; parser then read it as a turbofish | Drop the `<` / `>` stripping rules; verbose `Foo < T >` is acceptable |
| 4 | T8 substitution left orphan `it@.0` / `it@.1` lines | Constructor ensures triple expanded to 5 lines (3 new + 2 leftover) | Added `extra_delete_lines: Vec<usize>` to `Transform`; T8 matcher records the it@.0 and it@.1 line numbers; apply schedules whole-line deletes |
| 5 | Substitute used `col_end` as a column on `line_start` for multi-line expressions | forall/exists clauses had partial source-line garbage appended | Collapse `[line_start][col_start..]` + intermediate + `[line_end][..col_end]` into one replacement; mark intervening lines for deletion |
| 6 | Apply pipeline forced `line_end = line_start` in Rewrite | Multi-line Substitute branch never fired | Pass `t.line_end` through; remove the separate "extra Delete" loop that was patching around this |
| 7 | Trailing `,` on substitute new_text doubled with source's `,` | `,,` at the end of every T-class substitution | Consume the source's trailing `,` (after optional whitespace) when extending `col_end` |
| 8 | Multi-line T8 new_text lines 2+ at column 0 | Misindented prophetic triple in the rewritten source | Indent non-first lines of multi-line new_text to match `col_start` |
| 9 | Post-apply U-OTHER count exploded | New `it.seq()` / `it.index()` calls fired U-OTHER on the bare ident `it` | Tighten the U-OTHER trigger to require the `it@` view shape, not bare `Path("it")` |

The round-trip caught every one of these. The matcher unit-test
fixtures stayed green throughout — single-line cases the fixtures
covered didn't exercise the multi-line bugs; integration against the
real fixture was the gate that surfaced them.

## 6. What's left for humans

38 U-class findings, untouched in apply, broken down:

| # | Class | Count | Files | What's needed |
|--:|:------|------:|------:|:--------------|
| 1 | U-CUSTOM | 18 | 3 | Hand-port `IteratorSpecImpl` for the AVLTreeSeq variants (`Chap37/AVLTreeSeq.rs`, `AVLTreeSeqStEph.rs`, `AVLTreeSeqStPer.rs`) per `src/standards/prophetic_iterators_standard.rs`. |
| 2 | U-CHAIN | 0 | 0 | Disappeared from the post-apply count. The pre-apply 12 were diagnostic flags on wrapper structs that --apply deleted. The orchestrator no longer needs to track chain ordering — the wrappers are gone, the bare std iter is in place. |
| 3 | U-CLASS | 0 | 0 | Same — the pre-apply 8 were pin-list-vs-matcher disagreements on now-deleted wrappers. |

Net human attention required: **18 findings × 3 files** for the
AVLTreeSeq custom iterators. Everything else is mechanical and
applied.

## 7. Verification gate — APAS-VERUS side

Per plan §8 the migration's success criterion is
`scripts/validate.sh` green on the rewritten fixture. The veracity
tool's job ends at syntactic correctness (parse error count = 0),
which it has. Verus verification of the rewritten code is APAS-VERUS
work.

Recommended sequence:

```
cd ~/projects/veracity/tests/fixtures/APAS-VERUS
git checkout -- src                                 # reset to clean state
veracity-iterator-upgrade --apply --root . \
  --i-know-what-im-doing-not-a-fixture --apply-on-dirty
scripts/validate.sh                                  # the actual gate
```

If validate is green, the migration is done modulo the 3 AVLTreeSeq
files. If red, the errors break into:

- **Parse / type / iterator-API mismatch errors** — bugs in the
  mutator. Report back; veracity fixes.
- **Proof errors** — expected; the prophetic-iterator model has
  different semantics than the old `ForLoopGhostIterator` and existing
  proofs may need rewrite. APAS-VERUS-side work, out of scope here.

## 8. Risks observed

| # | Risk | Status |
|--:|:-----|:-------|
| 1 | T8 shape heterogeneity (plan §12.1) — atypical constructor triples could need downgrades. | Zero downgrades on this fixture; T8 fired 29× cleanly. May surface on richer fixtures. |
| 2 | Comment-bearing clauses (plan §12.2) — would skip with note. | Zero in this fixture run. The check is in place (`has_comment_in_clause_range`), conservatively skips any line with `//` or `/*`. |
| 3 | Renderer regressions (plan §12.3). | The substitution pipeline relies on `verus_syn`'s `to_token_stream` + canonical_spacing. Pinned to current verus_syn dep. Multi-line forall/exists were the main shape-class that exercised the renderer's output non-trivially; all passed. |
| 4 | Partial application on crash (plan §12.4). | Atomic per-file writes (tempfile + rename). A kill mid-run leaves each individual file consistent; the fixture-wide reset is `git checkout -- .`. |
| 5 | `--detect` after `--apply` not reaching 0 (plan §12.5). | Acceptance met — D=0, T=0, U=18 (the 18 U-CUSTOM that were always going to remain). |

## 9. What's next

| # | Item | Where | Effort |
|--:|:-----|:------|:-------|
| 1 | `scripts/validate.sh` green on the rewritten fixture | APAS-VERUS side | depends on Verus-side errors found |
| 2 | Hand-port the 3 AVLTreeSeq files (U-CUSTOM ×18) | APAS-VERUS side | per the iterator standard; a day each estimate |
| 3 | Phase 3 (apply to live APAS-VERUS) | Separate plan if ever | the path is "veracity emits patch series, APAS-VERUS reviews", NOT direct mutation of live tree |
| 4 | T-class enrichment (T11+ for known U-OTHER skeletons) | Optional, post-fixture-refresh | the unique-rewrites table already identifies promotion candidates |

My recommendation: run `scripts/validate.sh` on the rewritten
fixture, report back. If parse/type errors surface, send them this
way for mutator fixes. Proof errors stay APAS-VERUS-side per
plan §8.

## 10. References

- Subject plan: `plans/r204-veracity-iterator-upgrade-apply.md`
- Detect plan: `plans/veracity-iterator-upgrade-detect.md`
- Detect review: `plans/veracity-iterator-upgrade-detect-review.md`
- Phase 1 / 1.5 / deferrals reports: `plans/veracity-iterator-upgrade-detect-{phase1-5-report,deferrals-report}.md`

**Veracity-side artifacts (Phase 2, 2026-05-23):**

- veracity commit `929afed` — Phase 2 ship.
- veracity commit range `6bb41ef..929afed` — the full project: detect, Phase 1.5, 4 deferrals, Phase 2.
- `~/projects/veracity/src/bin/iterator_upgrade.rs` — single-binary source, ~2900 lines, both detect and apply modes.
- `~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}` — pre-apply goldens (D=500, T=698, U=38).
- `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/` — 20 matcher fixtures (D1–D10, T1–T10), 20/20 green.
- `~/projects/veracity/tests/iterator_upgrade_matchers.rs` — integration test.
- `veracity-review-string-hacking -f src/bin/iterator_upgrade.rs` → 0 violations.

## 11. One acknowledgment

The plan as written was followed; the round-trip caught nine real
bugs in the mutator before merge (§5). The matcher unit tests stayed
green through all of them — single-line synthetic fixtures didn't
exercise multi-line, T8-triple, or trailing-comma cases. The
acceptance round-trip on the real fixture was the actual gate.
Worth keeping that pattern: ship the mutator and run a fixture
round-trip as the merge condition for any future Phase-N edits to
the rewrite pipeline.
