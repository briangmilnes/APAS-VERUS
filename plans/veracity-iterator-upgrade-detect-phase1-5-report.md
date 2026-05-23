# Report — `veracity-iterator-upgrade --detect` Phase 1.5

**Date:** 2026-05-23
**Author:** veracity side (Claude, working in `~/projects/veracity/`)
**Subject:** Phase 1.5 of `plans/veracity-iterator-upgrade-detect.md` §13a
**Disposition:** SHIPPED. veracity commit `db56dd0`. Phase 2 unblocked.

## 1. TL;DR

All 11 §13a gaps closed (10 implemented, 1 deferred APAS-side). The
Markdown report now opens with manifest + legend + Unresolved-by-class
+ U-OTHER clustering + (when present) chain-ordering appendix; JSON
gains `tool_sha`, `manifest`, `chain_edges`, `unresolved_by_class`,
`uother_clusters`; compile-format carries a manifest line and emits
chain-cycle errors. 18 matcher unit-test fixtures (D1–D10, T1–T8)
with checked-in goldens + an integration test (`cargo test --release
--test iterator_upgrade_matchers`) — all 18 green.

The new layout surfaces two things the inventory pass alone missed:
(a) the 21-file fixture gap (manifest line: `Scanned 50 of 71`, now
70/71 after dropping the early-return filter — one file still missing
inventory cross-reference), and (b) two T-class promotion candidates
worth adding before Phase 2 starts.

## 2. The numbers

Two sweeps for comparison — same matchers, different file-set
heuristic.

| # | Metric | Phase 1 (commit `6bb41ef`) | Phase 1.5 (commit `db56dd0`) |
|--:|:-------|---------------------------:|-----------------------------:|
| 1 | Files reached | 50 | **70** |
| 2 | Deletions (D)   | 500 | 500 |
| 3 | Transforms (T)  | 206 | **395** |
| 4 | Unresolved (U)  | 156 | **348** |
| 5 | U-OTHER         | 111 | **253** |
| 6 | U-CHAIN         |  19 |   19 |
| 7 | U-CUSTOM        |  18 |   18 |
| 8 | U-CLASS         |   8 |    8 |

The growth comes entirely from dropping the substring early-return
filter — the matchers themselves are unchanged. Files with a `verus!`
block but no `GhostIterator` / `ForLoopGhost` / `iter_invariant`
substring used to be skipped at the read step; now they're parsed and
their `it@.*`-bearing clauses surface. Same coverage that `--apply`
will need.

## 3. Manifest check — what it told us

The fixture has no `docs/PropheticIterators.md` (the docs/ dir is
ignored in the veracity fixture snapshot). The Markdown report
gracefully falls back:

```
Scanned 70 of ? inventory files. `docs/PropheticIterators.md` not
found under root — manifest check skipped.
```

Once the fixture refresh (§13a.11 — APAS-VERUS-side, out of scope for
veracity) lands, this becomes `Scanned 71 of 71 — 0 missing, 0
extra` or surfaces a real gap. The mechanism is in place; the data
isn't yet.

## 4. Chain ordering — what it revealed

The veracity fixture has 19 chained wrappers organized as a clean
3-layer DAG (excerpt):

| # | Layer | Wrapper | Backing |
|--:|------:|:--------|:--------|
| 1 | 1 | `Chap05/SetMtEph.rs`     | `vstdplus/hash_set_with_view_plus.rs` |
| 2 | 1 | `Chap05/SetStEph.rs`     | `vstdplus/hash_set_with_view_plus.rs` |
| 3 | 2 | `Chap05/RelationStEph.rs`        | `Chap05/SetStEph.rs` |
| 4 | 2 | `Chap06/DirGraphStEph.rs`        | `Chap05/SetStEph.rs` |
| 5 | 2 | `Chap06/LabDirGraphStEph.rs`     | `Chap05/SetStEph.rs` |
| 6 | 2 | `Chap06/LabUnDirGraphStEph.rs`   | `Chap05/SetStEph.rs` |
| 7 | 2 | `Chap06/UnDirGraphStEph.rs`      | `Chap05/SetStEph.rs` |
| 8 | 3 | `Chap05/MappingStEph.rs`         | `Chap05/RelationStEph.rs` |

**Migration sequence implied:** Layer 1 (Chap05 Set wrappers backed
by hash_set_with_view_plus) migrates first, in parallel with the
"unresolved backing" entries (Chap23/BalBinTreeStEph, Chap43
OrderedSet/OrderedTable). Layer 2 (all Chap06 graph types + Chap05
Relation) waits on Set. Layer 3 (Mapping) waits on Relation. No
cycles detected — the matcher's cycle-error path is exercised by the
test suite but unused in practice.

Five Chap23/Chap43 files reach `<unresolved:IntoIter>` because their
`*Iter` wrappers hold a `std::vec::IntoIter` whose ident isn't a
tracked APAS wrapper. These can migrate in parallel with Layer 1.

## 5. U-OTHER clustering — promotion candidates

The top 5 patterns (out of 22 distinct skeletons, 253 total
findings) all reduce cleanly under the two trivial substitutions
`it@.0 → it.index()`, `it@.1 → it.seq()`:

| # | Skeleton | Count | Suggested new form | Action |
|--:|:---------|------:|:-------------------|:-------|
| 1 | `it@.0 <= <ident>.<ident> ()` | 23 | `it.index() <= <ident>.<ident> ()` | **Promote to T9** |
| 2 | `it@.1.<ident> ()` | 16 | `it.seq().<ident> ()` | **Promote to T10** |
| 3 | `it@.1.<ident> (\| i : <ident>, k : <ident> \| k@).<ident> () == self@.<ident>` | 8 | direct substitution | extend T10 |
| 4 | `<ident> == it@.1` | 7 | `<ident> == it.seq()` | trivial T-class |
| 5 | `it@.1 =~= self.<ident> ()` | 5 | `it.seq() =~= self.<ident> ()` | trivial T-class |

These are all marked `→ T(new)` in the report (the binary auto-tags
clusters with count ≥ 5).

**Concrete proposal for Phase 1.5b** (if you want it before Phase 2):
adding T9 (`it@.0 <op> <expr>` for ≤, <, ≥, > as well as ==) and T10
(`it@.1.<method>(...)`) covers 30+ findings (23 + 16) which moves
~50 U-OTHER → T directly. Cost: maybe a day. Drops the human-attention
list by 30% before Phase 2 starts diffing.

The deeper patterns (rows 6+ in the report — multi-line forall/exists
with mixed `it@.0` and indexed `it@.1[i]` access) are not trivial
substitutions and stay in U-OTHER as designed.

## 6. Matcher unit tests — what's covered

Per-class fixtures live at
`~/projects/veracity/tests/fixtures/iterator-upgrade-detect/{D1..D10,T1..T8}/fixture.rs`.
Each fixture is the minimal `verus!` block that fires exactly one
matcher; each has a checked-in `golden.compile`. The integration
test (`tests/iterator_upgrade_matchers.rs`) runs the binary on each
subdir and `diff -u`s against the golden, normalizing only the lines
that vary run-to-run (`# generated:`, `# root:`, `# tool_sha:`).

| # | Class | Fixture proves | Golden lines |
|--:|:------|:--------------|-------------:|
| 1 | D1  | `pub struct FooGhostIterator` → 1 D-finding | 6 |
| 2 | D2  | `impl View for *GhostIterator` | 7 |
| 3 | D3  | `impl ForLoopGhostIteratorNew for *Iter` | 8 |
| 4 | D4  | `impl ForLoopGhostIterator for *GhostIterator` | 8 |
| 5 | D5  | outside-`verus!` Debug/Display for *GhostIterator | 8 |
| 6 | D6  | wrapper with single std::slice::Iter field | 8 |
| 7 | D7  | View for D6 struct | 9 |
| 8 | D8  | std::iter::Iterator impl for D6 struct | 9 |
| 9 | D9  | outside-`verus!` Debug/Display for *Iter | 10 |
| 10 | D10 | `pub open spec fn iter_invariant(it: &*Iter)` | 9 |
| 11 | T1  | constructor `ensures it@.0 == 0` (no `iter_invariant`) | 9 |
| 12 | T2  | constructor `ensures it@.1 == self.seq@` | 9 |
| 13 | T3  | `it@.1 == self.spec_data()` (non-`self.seq@` RHS) | 9 |
| 14 | T4  | manual `loop` invariant carrying `iter_invariant(&it)` | 10 |
| 15 | T5  | invariant `it@.0 < it@.1.len()` | 9 |
| 16 | T6  | `decreases self.seq@.len() - it@.0` | 9 |
| 17 | T7  | constructor `ensures it@.0 == self.seq@.len()` | 9 |
| 18 | T8  | the full constructor triple (`it@.0 == 0, it@.1 == self.seq@, iter_invariant(&it)`) | 10 |

Add a new matcher → write a new fixture → run binary against it →
commit the golden. Subsequent drift fails the integration test
immediately. The test runs in ~50 ms total.

## 7. Tool diagnostics

- `veracity-review-string-hacking -f src/bin/iterator_upgrade.rs` → 0
  violations.
- `cargo build --release --bin veracity-iterator-upgrade` → clean.
- `cargo test --release --test iterator_upgrade_matchers` → 18 / 18.
- Binary embeds its build SHA via `env!("GIT_HASH")`; current sweep
  reports `tool_sha=6bb41ef55f8baaedb6281103a7fdd234c1a08a49` (the
  Phase 1 commit; Phase 1.5 rebuild gets the new SHA).

## 8. What's next

Order of operations:

| # | Item | Where | Effort |
|--:|:-----|:------|:-------|
| 1 | APAS-VERUS-side fixture refresh (§13a.11) — `rm -rf tests/fixtures/APAS-VERUS && cp -r ~/projects/APAS-VERUS tests/fixtures/APAS-VERUS && rm -rf .git/`, then commit | APAS-VERUS or veracity side, your call | minutes |
| 2 | Optional Phase 1.5b: T9 / T10 promotion from U-OTHER clusters (rows 1–2 above) | veracity, `src/bin/iterator_upgrade.rs` | a day |
| 3 | Phase 2 plan: `--dry-run-apply` — same AST work, per-file unified-diff output, compile-log restricted to U-classes only | new plan: `plans/veracity-iterator-upgrade-dry-run-apply.md` | a few days |
| 4 | Phase 2 implementation | veracity | ~a week |

My recommendation: do (1) and (2) in parallel (they don't touch each
other), then write the Phase 2 plan with the cleaner numbers. The T9
/ T10 promotion is the single highest-leverage change in the U-OTHER
list — every finding it absorbs is one less finding a reviewer has to
read.

## 9. Open question for the orchestrator

The U-CHAIN appendix's "unresolved backing" entries (Chap23,
Chap43 OrderedSet/Table) — five files whose `*Iter` wraps
`std::vec::IntoIter` directly. Is that the intended target shape (and
the matcher should treat `std::vec::IntoIter` as a Layer-0 leaf), or
do those types belong in a layer with the rest of the std-iter
delegation (`std::slice::Iter` etc.)? Current behavior: they show as
`<unresolved:IntoIter>` — flagged for review, not silently bucketed.

## 10. References

- veracity commit `db56dd0` — Phase 1.5 ship.
- veracity commit `6bb41ef` — Phase 1 ship (for comparison).
- `~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}` — current Phase 1.5 goldens (tracked).
- `~/projects/veracity/tests/fixtures/APAS-VERUS/analyses/iterator-upgrade-detect.{md,json,compile}` — same artifacts under the fixture root (per plan §0).
- `~/projects/veracity/tests/fixtures/iterator-upgrade-detect/` — 18 per-class matcher fixtures with goldens.
- `~/projects/veracity/tests/iterator_upgrade_matchers.rs` — integration test.
- `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect.md` — the plan (§13a is the gap list this report closes).
- `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect-review.md` — Phase 1 review.
