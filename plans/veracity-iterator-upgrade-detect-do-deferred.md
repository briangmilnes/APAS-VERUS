# `veracity-iterator-upgrade --detect` — finish the three deferrals

The Phase 1.5 commit (`db56dd0`, 2026-05-23) shipped the §13a report
layout — wide MD, legend, clustering, Iter column, chain appendix,
matcher unit tests. Good. But three items in the produced report are
either non-functional or visibly half-done. Close them.

Evidence is the live report at
`~/projects/veracity/analyses/iterator-upgrade-detect.md`
(generated 2026-05-23T12:39:46Z, totals `files=70, D=500, T=395, U=348`).

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace
on Rust source. All edits must be token-aware or AST-aware. Parse
ensures/requires blocks with brace/comma/semicolon awareness. A
string-hacking detector will flag and kill tools that corrupt source
syntax.

## Deferral 1 — T-class promotion

The U-OTHER clustering table tags **8 skeletons** with `→ T(new)`
(count ≥ 5). The matcher does not actually match any of them — they
are tagged and left in U-OTHER. The top skeleton alone is 112
findings.

| Cluster row | Skeleton | Count |
|---:|---|---:|
| 1 | `it@.0 <= <ident>.<ident>()` | 112 |
| 2 | `it@.1.<ident>()` | 19 |
| 3 | `it@.0 <= it@.1.<ident>()` | 16 |
| 4 | `<ident> == it@.1` | 8 |
| 5 | `it@.1.<ident>(\| i : <ident>, k : <ident> \| k@).<ident>() == self@.<ident>` | 8 |
| 6 | `<ident>@ == <ident>.<ident>(it@.0 <ident> int)…` | 6 |
| 7 | `<ident>@ == <ident>.<ident>(it@.0 <ident> int)…` | 6 |
| 8 | `it@.1 =~= self.<ident>()` | 5 |

**Required.** Grow T1–T8 (or add T9, T10, … as needed) to cover at
least rows 1–4. The promotion rule is the two trivial substitutions
the plan already specifies:

- `it@.0` (the iterator's index field) → `it.index()`
- `it@.1` (the iterator's full-seq field) → `it.seq()`

Apply them as AST rewrites at the `Expr` level, NOT by string. Use the
same `verus_syn::visit::Visit` walker that already clusters the
skeletons.

**Acceptance.** After promotion:
- The U-OTHER count drops by ≥ (112 + 19 + 16 + 8) = 155 findings.
- The corresponding cluster rows disappear from the U-OTHER
  clustering table (or fall below the count-≥-2 cutoff).
- The new T-class findings appear in the per-file Transforms tables
  with correct `old` / `new` columns.
- The matcher unit-test fixtures from §13a.10 grow a `T9*`, `T10*`, …
  pair (`.rs` + `.golden.compile`) for each new T-class.

The flag `→ T(new)` stays in the clustering table only for clusters
that remain after the new T-classes ship — it should mean "*next*
promotion candidate," not "thing the tool noticed and forgot."

## Deferral 2 — Chain backings unresolved

The chain-ordering appendix has **5 rows with `<unresolved:IntoIter>`**:

```
| 3 | 1 | Chap23/BalBinTreeStEph.rs       | <unresolved:IntoIter> |
| 4 | 1 | Chap43/OrderedSetStEph.rs       | <unresolved:IntoIter> |
| 5 | 1 | Chap43/OrderedSetStPer.rs       | <unresolved:IntoIter> |
| 6 | 1 | Chap43/OrderedTableStEph.rs     | <unresolved:IntoIter> |
| 7 | 1 | Chap43/OrderedTableStPer.rs     | <unresolved:IntoIter> |
```

These are not chains. They are **bare-std-iter delegated** files
whose `*Iter` wrapper holds a `std::vec::IntoIter` field. The D6 rule
classifies a wrapper as chained iff its sole non-PhantomData field is
**another APAS `*Iter`** — `std::vec::IntoIter` is a std type and
should match the D6 std-iter branch, not the chain branch.

**Root cause.** The D6 visitor's recognition of "another APAS `*Iter`"
is too liberal — it accepts any type whose ident ends in `Iter`,
including `IntoIter`. The std-iter whitelist in the plan (§4 D6) is
explicit: `std::slice::Iter`, `std::vec::IntoIter`,
`std::collections::hash_set::Iter`, `std::collections::hash_map::Iter`.

**Required.** Fix the D6 matcher to:
- Recognize the four std-iter types by their full path AND by their
  unqualified ident (`Iter`, `IntoIter`, `hash_set::Iter`,
  `hash_map::Iter`) when imported via `use`.
- Only record a chain edge `(wrapper, backing)` when the backing is
  a non-std iterator ident.
- For std-iter wrappers, treat the file as delegated to the std iter
  (which it already is in the per-file summary) and emit NO chain
  edge.

**Acceptance.** After the fix:
- The chain-ordering appendix has zero `<unresolved:*>` rows.
- The 5 listed files do not appear in the chain appendix at all
  (they are delegated to `std::vec::IntoIter`).
- `U-CHAIN` count in §13a.5 drops by 5 (from 19 to 14).
- Per-file summary still shows the 5 files as `delegated`, unchanged.

## Deferral 3 — `tool_sha` reports the wrong commit

The report's header shows `Tool SHA: 6bb41ef55f…` — that is the
Phase 1 commit, not the Phase 1.5 commit (`db56dd0`) that actually
produced this layout. The build script captured the SHA at first
compile and is not regenerating.

**Required.** The `build.rs` must rerun `git rev-parse HEAD` on every
build, not cache. Two common fixes:

- Add `println!("cargo:rerun-if-changed=.git/HEAD");` and
  `println!("cargo:rerun-if-changed=.git/refs/heads");` so Cargo
  invalidates the build script when HEAD or refs change.
- Or, better, drop the `build.rs` `env::set_var` approach and read
  the SHA at runtime via `git rev-parse HEAD` (only correct if the
  binary is run from the veracity checkout — acceptable per §0 since
  veracity always runs from its own root).

**Acceptance.**

- `cd ~/projects/veracity && git rev-parse HEAD` matches the
  `tool_sha` in `analyses/iterator-upgrade-detect.json` and the
  `Tool SHA:` line in the Markdown report header, for the *current*
  HEAD — i.e., after committing a no-op change and rebuilding, the
  SHA in the next report's header advances.
- The matcher unit-test goldens normalize the `tool_sha` line so the
  diff stays clean across commits (§13a.10 already does this — keep
  it).

## Deferral 4 — unified "Unique transforms" table

The per-file Transforms tables and the U-OTHER pattern clustering
together repeat the same handful of shapes hundreds of times. The
2026-05-23 sweep has `it@.1 == wa_seq` firing 70+ times across the
Chap06 weighted-graph files (T3, same rewrite every time). A reader
scrolling per-file detail sees the same row over and over with no
sense of how many distinct rewrites the codebase actually needs.

The plan now requires a single global **Unique transforms** table
(§13a.12) that dedupes every `it`-bearing rewrite — both T-matched
and U-OTHER — into one view.

**Required.** Emit this table at the top of the MD report, between
the §13a.5 "Unresolved by class" table and the per-file summary.
Format and clustering rule are in §13a.12. Key points:

- Source set: every `transforms[*]` finding (any T-class) PLUS every
  `unresolved[*]` finding whose `code == "U-OTHER"`.
- Columns: `Status` (T-class id, or `U-OTHER`), `Old skeleton`, `New
  skeleton`, `Count`, `Files`.
- Skeleton normalization: same `verus_syn` re-parse + ident/literal
  replacement that §13a.9 already implements.
- Strict `it` filter: drop any skeleton whose tokens do not include
  `it`. APAS convention names every iterator literal `it`; clauses
  without `it` are not iterator-bearing. This is a belt-and-braces
  check at the table-write step — the matcher should already obey
  it (§5 / §6).
- Sort: Count descending, top 50 or all with Count ≥ 2.
- JSON parallel: top-level `unique_rewrites` array.

**The existing §13a.9 standalone "U-OTHER patterns" table is
subsumed.** Drop it once the unified table is in place — otherwise
the report has two near-identical clusters with different scopes,
which is worse than one.

**Acceptance.**

- Sum of `Count` column equals `(T-class findings total) +
  (U-OTHER total)`. Other U-codes (`U-CHAIN`, `U-CUSTOM`, `U-CLASS`)
  are not in this table.
- Every skeleton row literally contains the token `it`.
- On the next sweep against the current fixture, the table has ~50
  rows and the top row has Count ≥ 70 (the `wa_seq` patterns
  collapse).

## Out of scope

- **Fixture refresh** (`docs/PropheticIterators.md` is missing from
  the veracity fixture; manifest check falls back to "N of ?"). That
  is APAS-VERUS-side per §13a.11 and is being scheduled separately.
  Do NOT modify the fixture from the veracity side.
- **Phase 2 (`--dry-run-apply`).** Still gated on this work landing
  clean.

## Single PR or four small ones

Either is acceptable. Recommended sequence if splitting:

1. **PR A.** Fix Deferral 2 (D6 matcher tightening). Smallest change,
   removes a visible report defect, shrinks the U-CHAIN baseline.
2. **PR B.** Deferral 3 (`tool_sha`). Independent, tiny; restores
   trust in report headers.
3. **PR C.** Deferral 4 (unified Unique transforms table). Report
   layout only — does not touch matchers, so it lands cleanly before
   the T-class promotion that will change counts.
4. **PR D.** Deferral 1 (T-class promotion). The substantive matcher
   work. Lands last so the count delta is visible against PR C's
   already-deduplicated baseline.

Each PR must include:
- `cargo test --release --test iterator_upgrade_matchers` green.
- A re-run of `veracity-iterator-upgrade --detect --root
  tests/fixtures/APAS-VERUS --out-dir analyses` with the regenerated
  `analyses/iterator-upgrade-detect.{md,json,compile}` committed —
  that IS the report. Commit message: one line. No prose summary, no
  count deltas, no acceptance-criteria recap. The diff and the
  regenerated report speak for themselves.

## References

- Subject plan: `~/projects/APAS-VERUS/plans/veracity-iterator-upgrade-detect.md`
  (especially §7, §13a, §13a.8, §13a.9)
- Phase 1.5 commit: `~/projects/veracity/` at `db56dd0`
- Live report: `~/projects/veracity/analyses/iterator-upgrade-detect.md`
- Veracity CLAUDE.md (string-hacking ban, fixture-only edits)
