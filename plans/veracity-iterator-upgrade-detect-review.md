# Report — Review of `plans/veracity-iterator-upgrade-detect.md`

**Date:** 2026-05-22
**Reviewer:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject plan:** `plans/veracity-iterator-upgrade-detect.md` (SCOPING, 2026-05-22)
**Disposition:** GREEN with required fixes (§3) and a Phase-2 recommendation (§5).

## 1. TL;DR

The plan is solid — read-only scope, paranoid `--root` containment, AST-only
matching, three output formats. **Ship `--detect` first** so we can see real
numbers from the pilot and the full 71-file sweep.

But the design must take cumulative human review burden seriously, because
**many people are going to look at this output** — orchestrator, file owners,
reviewers, anyone re-running after a fixture refresh. The plan implies a
"2–3 hour single sweep." The real cost is the same work multiplied by N
people. We need to plan for `--dry-run-apply` (Phase 2) as the actual
review-and-migrate vehicle; `--detect` is the inventory pass that proves the
matchers are right.

## 2. Why review-burden matters at the design stage

Per-person stepping through `--detect` findings:

| # | Step | Estimate |
|--:|:-----|---------:|
| 1 | Findings emitted (71 files × ~14 per file: ~10 D + ~4 T) | ~1,000 |
| 2 | Per-finding time in `M-x compile` (jump + read + judge) | 30–60 s |
| 3 | Time for one pass, one person | 8–16 h |
| 4 | Likely passes (initial review + reviewers + post-fixture-refresh) | 3–5 |
| 5 | Likely audience (orchestrator + N file owners + reviewers) | 3–6 people |
| 6 | Median cumulative review time (rows 3 × 4 × 5, mid-point) | **70–150 person-hours** |

Even the optimistic end is a person-week. That is the cost function the
design has to optimize. `--detect` is the right first step *only* if it
buys us a Phase 2 that drives the per-finding number toward zero.

## 3. Required fixes to the plan (apply before coding)

| # | Issue | Where | Fix |
|--:|:------|:------|:----|
| 1 | `--root` is "REQUIRED, no default" (§0) but "default `.`" (§2). | §2 line 98 | Required, no default. Drop the §2 default. |
| 2 | D5 row count: §4 emits Debug + Display as two D5 rows; §10 calibration row 5 collapses to one row spanning 1133–1141. | §10 row 5 | One row per item. Two D5 entries in calibration. |
| 3 | §9 step 2 still waffles between "splice into synth `mod`" vs "walk token tree directly." | §9.2 | **Full-generic-feq style — confirmed.** Follow the exact pattern in `~/projects/veracity/src/bin/full_generic_feq.rs`:<br/>1. `ra_ap_syntax::SourceFile::parse(content, Edition::Edition2021)` (line 192) to get the token-level tree.<br/>2. Walk it to locate the `verus!` macro invocation; extract its body as a `&str` (skip with reason `"no verus! block"` if absent — see line 264).<br/>3. `verus_syn::parse_file(inner)` (line 270) on that body to get the Verus AST.<br/>4. Walk the AST with a custom visitor implementing `verus_syn::visit::Visit` (line 24 import) — one `visit_*` method per D-class / T-class trigger node.<br/>Outside-`verus!` items (D5, D9) still need scanning — use `ra_ap_syntax` tree walks on the outer file (matches what `full_generic_feq.rs` does for outside-`verus!` `use` lines around line 437). **Crates: `ra_ap_syntax` + `verus_syn` + `syn`** — all already in `Cargo.toml`; no new deps. |
| 4 | T3 metavariable `<expr>`: "balanced up to next top-level comma" — doesn't say anything about angle brackets. `Foo<A, B>` is a top-level-comma trap. | §5 detection rules | Spec: track `(`, `[`, `{`, **and** `<…>` nesting when the `<` is unambiguously a generic arg list (after a path). |
| 5 | U-OTHER trigger: "identifier classified as the loop iterator" — by literal name `it`, or by the binder from `for X in …`? | §6 + §9.6 | Decide. Recommendation: literal `it` only (matches APAS convention everywhere — see Chap18 pilot). Anything else gets U-OTHER. |
| 6 | Golden file path/format not specified. | §0, §13 | `tests/fixtures/APAS-VERUS/analyses/iterator-upgrade-detect.golden.compile` (the compile-format output is the most diffable). Calibration step compares produced output against this with `diff -u`. |
| 7 | D8 says "report impl head + `next` body's `ensures` block separately." Why? In `--detect` of a deletion, the whole block goes; splitting buys nothing. | §4 D8 | Drop the split for `--detect`. Single D8 finding spanning the whole impl. If `--apply` later needs the contract span, recompute then. |
| 8 | Cross-file `iter_invariant` (§12.3 risk #3). `verus_syn` doesn't do path resolution. Partial resolution will misclassify in both directions. | §6 + §12.3 | Punt: match `iter_invariant` only when defined in the same file. Any cross-file caller → U-OTHER with reason `cross-file iter_invariant — verify referent`. Calibrate against Chap05/06 chained wrappers. |
| 9 | T8 multi-line `new` text awkward in markdown tables (§12.4). | §7 + §12.4 | Option (a) wins: emit per-file "Constructor `ensures` rewrites" as a subsection below the per-file deletion/transform table. Keep the table single-line clean. In the compile-format log, T8 lines stay single-line with `\n` escapes — Emacs `next-error` still jumps correctly. |
| 10 | No matcher unit tests called out. T1–T8 are exactly the patterns that need golden tests. | §9 | Add §9.5b: one minimal `.rs` fixture per T-class and per D-class under `tests/fixtures/iterator-upgrade-detect/`. Expected output checked in. Adds maybe a day; saves debugging cycles after the matchers drift. |

These are all small fixes; none changes the shape of the deliverable.

## 4. Things the plan got right (worth preserving)

| # | Thing | Why it matters |
|--:|:------|:---------------|
| 1 | §0 work-area constraint — required `--root`, fixture-path refusal, verbose override, output containment. | Defends against accidental live-tree corruption while concurrent APAS-VERUS proof work is in flight. Exactly the right paranoia. |
| 2 | Pinned 3-file custom list with U-CLASS escape hatch. | Fails closed. A misclassified file surfaces; nothing silently mis-handled. |
| 3 | Three output formats (md / json / compile). | `compile`-format with `M-x compile` + `C-x \`` stepping is the right interface for the U-classes (where human judgment is actually needed). |
| 4 | Read-only single pass; `--apply` deferred to a separate plan. | Smallest reviewable unit. The work is gated on the matchers being right, which `--detect` is exactly designed to prove. |
| 5 | `--detect` exits 0 even with findings. | Right default for a discovery tool; doesn't fail CI. |
| 6 | Authoritative AST scan, not the `docs/PropheticIterators.md` table. | Drift-safe. |

## 5. Phase 2 recommendation — `--dry-run-apply`

`--detect` is the inventory; the migration itself is N people staring at
templated rewrites. The lowest-cost interface for confirming a templated
rewrite is a unified diff in tooling people already use (`git diff`,
GitHub / GitLab review, `vimdiff`).

Proposal for Phase 2 (separate plan, after `--detect` is calibrated):

- `veracity-iterator-upgrade --dry-run-apply --root <fixture> --out-dir <path>`
- Same AST scan, same matchers, same pinned classification.
- Output: per-file unified diff at `<out-dir>/<relative-path>.diff`,
  reflecting D-deletions + T-rewrites; **no source mutation**.
- Plus the existing `--detect` summary table (per-chapter D / T / U counts).
- Plus the existing compile-format log, but **restricted to U-classes only**
  — the human-attention list. T1–T8 don't appear because they live in the
  diff.
- Acceptance: reviewer skims one diff per file (~2 min), then `git apply`s
  the diff to the live tree, or rejects the file.

Per-person Phase 2 review time: **71 files × ~2 min = ~2.5 hours**, in a
tool people already know. Times N people, the cumulative number stays
inside the same order of magnitude (15–30 person-hours), versus 70–150
for the `--detect`-only path.

The implementer's work is largely unchanged — the `new_text` template
generator the plan already specifies in §5 (line 234) emits exactly what
the diff needs.

**Open question for the orchestrator:** does Phase 2 land as

- **(a)** veracity emits diffs, humans `git apply` per file, or
- **(b)** veracity also writes the modified files to `<out-dir>`, humans
  `cp <out-dir>/<path> <root>/<path>` per file?

Either works. (b) is one line of code more and friendlier to non-Emacs
reviewers. Decide before Phase 2 starts.

## 6. Smaller comments

| # | Comment | Severity |
|--:|:--------|:---------|
| 1 | `.compile` file extension is unusual. `.errors` or `.log` is more conventional; `compile-mode` doesn't care about the extension. | cosmetic |
| 2 | JSON header pins `verus_version` — good. Suggest also pinning the veracity binary's git SHA in the header so reruns are diffable on tool drift too. | nice-to-have |
| 3 | §11 non-goals could explicitly call out "verifying that the proposed `new` lines actually verify under Verus" — that's `--apply` + `scripts/validate.sh` territory. | clarity |
| 4 | U-CHAIN topological-ordering appendix — derivable from the chain graph the matcher already builds. Cheap to add. Saves an orchestrator from sorting 15 files by hand. | nice-to-have |
| 5 | The `--ignore` default list (§2) drops `analyses/**`. Good. Also drop `*.golden.compile` files so the calibration goldens don't get scanned. | small |

## 7. Suggested next moves

1. Apply §3 fixes to the plan (~20 minutes).
2. Build `--detect` per the corrected plan; calibrate against the
   `Chap18/ArraySeqStEph.rs` pilot.
3. Run against the fixture's 71 files; commit the report set to the
   fixture as the first golden.
4. Stop. Get the orchestrator's eyes on a real D/T/U inventory before
   designing `--dry-run-apply`.
5. After §4 looks right, write the Phase-2 plan and run that for the
   actual migration.

## 8. References

- Subject plan: `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect.md`
- Veracity parsing exemplar: `~/projects/veracity/src/bin/full_generic_feq.rs`
- Veracity CLAUDE.md (string-hacking ban, AST mandate, fixture path)
- Migration schedule: `~/projects/APAS-VERUS/plans/verus-0.2026.05.21-iterator-migration.md`
- Target shape: `~/projects/APAS-VERUS/src/standards/prophetic_iterators_standard.rs`
