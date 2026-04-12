# Lecture Quantitatives — Data Plan

Just the numbers. Prose is the user's job.

## 1. Scale

| # | Metric | Source |
|---|---|---|
| 1 | Total LOC by category (spec / proof / exec) | `scripts/all-fn-impls-by-chap.sh` (per-chap), aggregated |
| 2 | LOC per chapter (spec / proof / exec / total) | same, per-chapter table |
| 3 | File count by chapter | `find src/ChapNN -name '*.rs' | wc -l` |
| 4 | Module count (top-level chapters) | grep `pub mod Chap` in `src/lib.rs` |
| 5 | Total source files (verified scope) | exclude experiments, examples |
| 6 | Round count to date (R196 and counting) | `git log --grep='^R[0-9]'` |
| 7 | Wall-clock days from R1 to current | git log first/last commit dates |
| 8 | Number of agents used | constant (6); peak concurrent |
| 9 | Standards documents count | `ls src/standards/*.rs | wc -l` (24) |
| 10 | vstdplus modules + LOC | `ls src/vstdplus/*.rs`, wc on each |

## 2. Coverage

| # | Metric | Source |
|---|---|---|
| 11 | Total `verified` count | last `scripts/validate.sh` log "verification results" line |
| 12 | Verified by chapter | `scripts/validate.sh isolate ChapNN` per chapter (or parse full log) |
| 13 | Total proof holes | `scripts/all-holes-by-chap.sh` summary line |
| 14 | Holes per chapter | `analyses/chapter-cleanliness-status.log` |
| 15 | Clean-vs-holed chapter count | same |
| 16 | APAS textbook coverage: chapters fully done / partial / skipped | manual classification against APAS table of contents |
| 17 | St / StPer / Mt / MtEph file count | `find src -name '*StEph.rs' \| wc -l`; same for StPer, MtEph |
| 18 | Mt-only algorithms (no St counterpart) | grep + audit |
| 19 | Iterator standard adoption: collections fully implementing 10 components | grep for `IteratorPlus` impls; manual cross-check |
| 20 | Spec strength taxonomy (sample 5 representative chapters) | `veracity-review-module-fn-impls`, classify strong/partial/weak/none |

## 3. Trust Base

| # | Metric | Source |
|---|---|---|
| 21 | Total `external_body` count | `grep -rn 'external_body' src/` |
| 22 | external_body broken out by kind: thread-spawn, eq/clone bridge, iterator, algorithmic | manual classification from grep results |
| 23 | Total `assume(` count | `grep -rn 'assume(' src/` |
| 24 | assume by kind: eq/clone body, thread-join unreachable, iterator invariant, algorithmic | manual classification |
| 25 | Total `accept(` count | `grep -rn 'accept(' src/` |
| 26 | Total `admit(` count (should be ~0) | `grep -rn 'admit()' src/` |
| 27 | Closures: `join(` call sites | `grep -rn '\bjoin(' src/` |
| 28 | Broadcast group declarations | `grep -rn 'broadcast group' src/` |
| 29 | Broadcast group `use` sites | `grep -rn 'broadcast use' src/` |

## 4. Cost

| # | Metric | Source |
|---|---|---|
| 30 | Full-validate wall time (latest) | last `logs/validate.*.log` "Elapsed" |
| 31 | Full-validate peak rust_verify RSS, peak z3 RSS | same log "Sampled Memory Usage" |
| 32 | Validate time/RSS plot over rounds | parse all `logs/validate.*.log`, extract Elapsed + RSS, plot vs date |
| 33 | Per-chapter isolate-validate time | run `scripts/validate.sh isolate ChapNN` for each, capture "Elapsed" |
| 34 | Per-chapter peak z3 RSS | same logs |
| 35 | Hot-spot rlimit overrides: file:fn, rlimit value | `grep -rn 'verifier::rlimit' src/` |
| 36 | Total rlimit overrides count | same, wc -l |
| 37 | RTT count (test functions) | `scripts/rtt.sh` log, count "test result" or `cargo nextest --list` |
| 38 | RTT total wall time | last `logs/rtt.*.log` |
| 39 | PTT count | `find rust_verify_test/tests -name '*.rs' \| xargs grep -c '#\[test\]' \| awk -F: '{s+=$2}END{print s}'` |
| 40 | PTT total wall time | last `logs/ptt.*.log` |
| 41 | Total Z3 instantiations (latest validate, summed) | parse profile logs if available, else skip |

## 5. Holes Over Time

| # | Metric | Source |
|---|---|---|
| 42 | Hole count per round (R1 → R196) | git log of `analyses/chapter-cleanliness-status.log`; `git show <commit>:analyses/chapter-cleanliness-status.log` for each round-end commit |
| 43 | Daily-proof-table data (already maintained) | `~/.claude/projects/.../memory/MEMORY.md` daily proof table section + git history |
| 44 | Holes-closed per round | derived from #42 (delta) |
| 45 | Holes-closed per agent (cumulative) | parse `plans/agent{N}-round{R}-report.md` files |
| 46 | Cumulative agent-rounds | count of files matching `plans/agent[1-6]-round*-report.md` |

## 6. Veracity Minimize Productivity

| # | Metric | Source |
|---|---|---|
| 47 | Total minimize runs to date | `ls analyses/veracity-minimize-*.log \| wc -l` |
| 48 | Asserts removed (cumulative) | parse all `analyses/veracity-minimize-proofs.*.log`, sum "removed" counts |
| 49 | Proof blocks removed (cumulative) | same |
| 50 | Lemmas removed | parse `analyses/veracity-minimize-lib.*.log` |
| 51 | CPU saved (cumulative, from delta hints) | parse "cpu: -Ns" lines, sum negatives |
| 52 | Memory saved (cumulative) | parse "mem: -NMB" lines, sum negatives |
| 53 | Minimize runs that found 0 removals | count logs where "removed: 0" |
| 54 | Per-chapter minimize impact (top 10 chapters by CPU saved) | aggregate #51 per chapter |
| 55 | Total minimizer wall time invested | sum "complete in Xm Ys" lines across all logs |

## 7. Eq / Clone / PartialEq

| # | Metric | Source |
|---|---|---|
| 56 | `impl Clone for` count | `grep -rn 'impl.*Clone for' src/` |
| 57 | `impl PartialEq for` count | `grep -rn 'impl.*PartialEq for' src/` |
| 58 | `impl Eq for` count | `grep -rn 'impl.*Eq for' src/` |
| 59 | `PartialEqSpecImpl` count | `grep -rn 'PartialEqSpecImpl' src/` |
| 60 | `obeys_eq_spec` count | `grep -rn 'obeys_eq_spec' src/` |
| 61 | `ClonePreservesView` count | `grep -rn 'ClonePreservesView' src/` |
| 62 | DeepView impls | `grep -rn 'impl.*DeepView' src/` |

## 8. Where We Optimized (commit/round narrative — qualitative but structured)

| # | Item | Source |
|---|---|---|
| 63 | Round-by-round: which optimizations landed in which round | parse `plans/agent*-round*-report.md` for "wf decomposed", "opaque", "matching loop", "rlimit reduced" keywords |
| 64 | List of named patterns invented (opaque vs closed, conjunction flakiness, broadcast cross-fire, closure clone workaround, ghost capture before call) | from MEMORY.md feedback entries + report grep |
| 65 | Z3 hot-spots fixed (per-round profile peak deltas) | parse `logs/profile/SUMMARY-*.txt` over time |

## Output deliverables

1. `analyses/lecture-data/scale.md` — sections 1.
2. `analyses/lecture-data/coverage.md` — sections 2.
3. `analyses/lecture-data/trust-base.md` — sections 3.
4. `analyses/lecture-data/cost.md` — sections 4.
5. `analyses/lecture-data/holes-over-time.{md,csv}` + `holes-plot.png` — section 5.
6. `analyses/lecture-data/minimize-productivity.md` — section 6.
7. `analyses/lecture-data/eq-clone.md` — section 7.
8. `analyses/lecture-data/optimization-history.md` — section 8.
9. `analyses/lecture-data/validate-time-rss.{csv,png}` — section 4 plots.

## Effort estimate

- Sections 1, 3, 7: pure grep, < 30 min each.
- Section 2: medium (manual APAS textbook classification + spec strength sampling), 2–3 hours.
- Section 4: easy except #41 (skip if profile data sparse).
- Section 5: needs git-log-walk script + matplotlib (or gnuplot). 1–2 hours.
- Section 6: parse minimize logs with awk/python (ASK first per project rules). 1 hour.
- Section 8: read 196 round reports + extract. Most expensive. 3–4 hours.
