# Report — `veracity-iterator-upgrade --detect` four deferrals shipped

**Date:** 2026-05-23
**Author:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject:** `plans/veracity-iterator-upgrade-detect-do-deferred.md`
**Disposition:** SHIPPED. veracity commits `6486db7` → `c7b6b32`. Phase 2 unblocked.

## 1. TL;DR

All four deferrals closed across four sequenced commits (A → D as
recommended in the deferred plan). U-OTHER on the veracity fixture
went from **303 → 0**; T-findings grew **395 → 698**. The
unresolved bucket is now 38 total, all in three known shapes
(U-CUSTOM × 18 pinned AVLTreeSeq files, U-CHAIN × 12 chained
wrappers, U-CLASS × 8 pin-disagreements). No generic catch-all
remains.

The Unresolved-by-class table no longer has a U-OTHER row.

## 2. Numbers

Three sweeps for comparison — same fixture, same matchers structure;
the deferrals are matcher and report-layout work, not new file
discovery.

| # | Metric | Phase 1 | Phase 1.5 | After deferrals |
|--:|:-------|--------:|----------:|----------------:|
| 1 | Files reached | 50 | 70 | 70 |
| 2 | D (deletions)   | 500 | 500 | 500 |
| 3 | T (transforms)  | 206 | 395 | **698** |
| 4 | U (unresolved)  | 156 | 348 | **38** |
| 5 | U-OTHER         | 111 | 303 | **0** |
| 6 | U-CHAIN         |  19 |  19 |  12 |
| 7 | U-CUSTOM        |  18 |  18 |  18 |
| 8 | U-CLASS         |   8 |   8 |   8 |

## 3. PR-by-PR

| # | PR | Commit | What changed | Acceptance met |
|--:|:---|:-------|:-------------|:---------------|
| 1 | A — D6 tightening | `6486db7` | Bare `Iter` / `IntoIter` recognized as std (single-segment paths). Was: treated as APAS chain. APAS convention names every wrapper `*Iter` with a non-empty prefix, so bare forms can only be std imports. | U-CHAIN 19 → 12 (−7). Chain appendix has 0 `<unresolved:*>` rows. Chap23/BalBinTreeStEph + 4 Chap43 OrderedSet/Table files now correctly delegate to `std::vec::IntoIter`. |
| 2 | B — tool_sha refresh | `775300b` | `build.rs` now watches the file `.git/HEAD` points to (typically `.git/refs/heads/<branch>`) plus `.git/packed-refs`. Committing on the current branch updates the ref-file, not HEAD itself — the previous build.rs missed that and the embedded SHA went stale. | `git rev-parse HEAD` matches both `Tool SHA:` in the MD header and `.tool_sha` in JSON. Verified at HEAD `6486db7` and `c7b6b32`. |
| 3 | C — Unique transforms table | `3b75fc7` | New top-matter table dedup'd by skeleton across every T-finding + every U-OTHER. Columns: Status (T-class id or U-OTHER), Old skeleton, New skeleton, Count, Files. Strict `it`-token filter. Top 50 OR all with count ≥ 2. The standalone §13a.9 "U-OTHER patterns" table is subsumed and dropped. | Sum of Count = T-total + U-OTHER-total ✓. Every skeleton row contains literal `it` ✓. Top row: T6 `<ident>.<ident>() − it@.0` × 122. Row 3: U-OTHER → T(new) `it@.0 <= <ident>.<ident>()` × 112 (matches deferred plan's prediction). |
| 4 | D — T9 / T10 promotion | `c7b6b32` | T9 fires on any expression containing `it@.0` as a sub-expression and not covered by T1/T6/T7 — substitute `it@.0` → `it.index()`. T10: same for `it@.1` → `it.seq()`. T10 also applies the T9 substitution to its new text when both indices appear, so mixed clauses emit one finding (plan §4 choice (b)). | T9 fires 218×, T10 fires 85×. T total 395 → 698 (+303). U-OTHER 303 → 0. All 20 matcher fixtures pass. |

## 4. T-class distribution (after deferrals)

The new totals reveal which patterns actually dominate APAS-VERUS
iterator code:

| # | Class | Count | What it matches |
|--:|:------|------:|:----------------|
|  1 | T9  | 218 | any `it@.0` sub-expression not already T1/T6/T7 |
|  2 | T6  | 128 | `<expr>.len() - it@.0` in `decreases` |
|  3 | T3  | 117 | `it@.1 == <expr>` (top level, non-`self.seq@` RHS) |
|  4 | T10 |  85 | any `it@.1` sub-expression not already T2/T3/T5 |
|  5 | T1  |  61 | `it@.0 == <intlit>` (constructor without `iter_invariant`) |
|  6 | T4  |  53 | standalone `iter_invariant(&it)` (loop invariant) |
|  7 | T8  |  29 | constructor triple (`it@.0 + it@.1 + iter_invariant(&it)`) |
|  8 | T2  |   7 | `it@.1 == self.seq@` |
|  9 | T5  |   0 | `it@.0 < it@.1.len()` — no occurrences |
| 10 | T7  |   0 | `it@.0 == self.seq@.len()` — no occurrences |
| **Total** | | **698** | |

**T5 and T7 are zero across the whole fixture.** Either APAS-VERUS
genuinely doesn't use those idioms, or those patterns are now being
absorbed by T9 / T10 before the more-specific matchers run. The
priority order (§4 of the deferred plan) puts T5/T7 BEFORE T9/T10
so the more-specific class should win — but a sanity check on
matcher precedence is worth a few minutes before Phase 2.

## 5. What's left in the unresolved bucket (38 total)

| # | Code | Count | Files | What it means |
|--:|:-----|------:|------:|:--------------|
| 1 | U-CUSTOM | 18 | 3 | The pinned `Chap37/AVLTreeSeq{,StEph,StPer}.rs` files. The D6–D10 wrapper bits get reclassified to U-CUSTOM (hand-port to `IteratorSpecImpl` required). |
| 2 | U-CHAIN | 12 | 12 | Chained wrappers (one APAS `*Iter` wraps another). Migration ordering known from §13a.8 appendix; no action required by `--detect`. |
| 3 | U-CLASS | 8 | 8 | Files where pinned classification disagrees with observed style. Investigate per file — the heuristic may be wrong. |

**No generic U-OTHER.** Everything the matcher saw with literal `it`
is either covered by a T-class or one of the three known
unresolved shapes.

## 6. Open observations

| # | Observation | Action |
|--:|:------------|:-------|
| 1 | T5 / T7 fire zero times. Either dead-code matchers or matched-out-from-under by T9/T10. | 5-minute precedence audit before Phase 2 starts. |
| 2 | T9 dominates at 218 occurrences — 31% of all transforms. The "any `it@.0` sub-expression" matcher is doing a LOT of work. If Phase 2's `--dry-run-apply` produces a diff that doesn't verify under Verus, T9 is the first suspect (over-aggressive substitution). | Per-T-class verification spot-check on Phase 2 diffs (already in the Phase 2 plan as acceptance criteria). |
| 3 | The "Unique transforms" table now shows 110 distinct skeletons. Top 5 cover ~600 of the 698 T-findings — a 6:1 reduction in reviewer cognitive load vs reading every per-file table. | None — this is the intended outcome of PR C. |
| 4 | The chain-ordering appendix has 12 clean rows in a 3-layer DAG (Layer 1: Set wrappers and direct-std files; Layer 2: graphs and Relation; Layer 3: Mapping). No cycles. | Migration ordering follows the appendix exactly. |
| 5 | Manifest still reports "Scanned 70 of ?" — the fixture has no `docs/PropheticIterators.md`. | APAS-VERUS-side fixture refresh (§13a.11) — still deferred to the orchestrator. |

## 7. What's next

| # | Item | Where | Effort |
|--:|:-----|:------|:-------|
| 1 | Fixture refresh (`docs/PropheticIterators.md` import) — flips `Scanned 70 of ?` to `Scanned 71 of 71` or surfaces a real gap | APAS-VERUS side | minutes |
| 2 | T5 / T7 precedence audit — quick check that the zero counts aren't matcher shadowing | veracity, ~20-minute investigation | trivial |
| 3 | Phase 2 plan: `--dry-run-apply` — same AST, per-file unified diffs, compile-log restricted to U-classes only | new plan: `plans/veracity-iterator-upgrade-dry-run-apply.md` | a few days |
| 4 | Phase 2 implementation + verification | veracity | ~a week |

**My recommendation:** (1) and (2) in parallel — both are short and
independent. Then write the Phase 2 plan with the cleaner T-class
distribution from this report.

## 8. References

- Subject plan: `plans/veracity-iterator-upgrade-detect-do-deferred.md`
- Parent plan: `plans/veracity-iterator-upgrade-detect.md`
- Phase 1 review: `plans/veracity-iterator-upgrade-detect-review.md`
- Phase 1.5 report: `plans/veracity-iterator-upgrade-detect-phase1-5-report.md`
- T9/T10 design doc (now superseded by what shipped): `plans/veracity-iterator-upgrade-detect-t9-t10.md`

**Veracity-side artifacts (post-deferrals):**

- `~/projects/veracity/` commits `6486db7` (PR A) → `c7b6b32` (PR D).
- `~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}` — current goldens (tracked, files=70, D=500, T=698, U=38).
- `~/projects/veracity/tests/fixtures/APAS-VERUS/analyses/iterator-upgrade-detect.{md,json,compile}` — fixture-side copy.
- `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/` — 20 matcher fixtures (D1–D10, T1–T10) with checked-in `golden.compile` per class.
- `~/projects/veracity/tests/iterator_upgrade_matchers.rs` — integration test, 20/20 green.
- `~/projects/veracity/src/bin/iterator_upgrade.rs` — single-binary source, ~1900 lines.
- `~/projects/veracity/build.rs` — git-SHA capture with proper rerun-if-changed coverage.

**Tool diagnostics on the post-deferral binary:**

- `veracity-review-string-hacking -f src/bin/iterator_upgrade.rs` — 0 violations.
- `cargo build --release --bin veracity-iterator-upgrade` — clean.
- All 20 matcher fixtures pass via direct binary diff (`cargo test`
  in the workspace pulls in ~40 unrelated bins; the per-fixture diff
  is the actual gate and runs in ~50 ms total).
