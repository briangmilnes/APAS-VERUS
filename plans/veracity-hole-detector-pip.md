# Veracity Proof Hole Detector: Performance Improvement Plan

## Current state: 421 total holes reported, 367 "real actionable"

The real number (with proper exclusions) is 158 holes, 134 real actionable.
The detector is inflating counts 3x and has missing features.

## Bug 1: Section 4 is empty

Section 4 "Proof Targets" with subsections 4.1–4.6 appears in the Table of
Contents but has NO CONTENT in the output. The output skips from section 3
(Summary) to section 5 (Started/Ended). These targeting sections are the
most useful part of the report for planning proof work:

- 4.1 Worst directories (by holes)
- 4.2 Next target files (clean deps, by holes)
- 4.3 Next target directories
- 4.4 Not verusified
- 4.5 Not verusified (clean deps only)
- 4.6 Chapter by chapter proof targeting

Either generate the content or remove the TOC entries. Don't advertise
sections that don't exist.

## Bug 2: Default exclusions not applied

When run as `veracity-review-proof-holes src/`, the tool scans everything
under `src/` including directories that should always be excluded:

| Directory | Holes found | Should count? |
|---|---|---|
| experiments/ | 496 | NO — these are test experiments, not algorithmic code |
| vstdplus/ | 117 | NO — utility library, different project scope |
| standards/ | 17 | NO — reference documentation files |
| **Real code** | **197** | **YES** |

The tool MUST default-exclude `experiments/`, `vstdplus/`, and `standards/`
when scanning APAS-VERUS. These are not proof targets. The `-e` flag exists
but shouldn't be required for the standard directories.

**Fix:** Hard-code default excludes for known non-target directories, or read
them from a config file (e.g., `.veracity-excludes`). The `-e` flag should be
for additional excludes beyond the defaults.

**Validation check:** If total holes > 200, something is wrong. The real
APAS-VERUS algorithmic code has ~160 holes. If you're seeing 400+, you're
scanning excluded directories.

## Bug 3: EQ_CLONE_ASSUME double-counted

The same 7 eq/clone assume instances appear in TWO places:
1. **Warnings:** `7 × assume_eq_clone_workaround`
2. **Structural FPs:** `7 × EQ_CLONE_ASSUME`

Then the math says "61 structural FPs detected" but subtracts only 54 from
holes (421 - 54 = 367). The 7 EQ_CLONE_ASSUME are not subtracted because
they were already counted as warnings, not holes.

This is confusing. Pick ONE bucket:
- If EQ_CLONE_ASSUME is a structural FP → count it there, subtract from
  holes, don't also count as a warning.
- If it's a warning → don't also list it as a structural FP.

Currently it's in both, which makes the "61 detected" number misleading
because only 54 actually reduce the hole count.

## Bug 4: Verus API patterns counted as holes

These are not proof obligations — they are Verus language mechanisms:

| Item | Count | What it is |
|---|---|---|
| `external_trait_specification` | 1 | Verus pattern for wrapping existing Rust traits |
| `external_trait_extension` | 1 | Verus pattern for extending trait specs |
| `Tracked::assume_new()` | 1 | Verus API for creating ghost-tracked values |
| `assume(false)` in experiments | 1 | Test file, not real code |

These should either be auto-classified as structural FPs or excluded entirely.
They are Verus framework plumbing, not missing proofs.

## Bug 5: Incomplete RWLOCK_GHOST detection

The detector finds 24 RWLOCK_GHOST structural FPs. But there are additional
RwLock ghost-boundary assumes that aren't detected:

```
Chap41/AVLTreeSetMtEph.rs:245 — assume(count == self@.len())     ← NOT detected
Chap43/OrderedSetMtEph.rs:241 — assume(count == self@.len())     ← NOT detected
Chap43/OrderedSetMtEph.rs:269 — assume(found == self@.contains(x@))  ← NOT detected
Chap43/OrderedSetMtEph.rs:455 — assume(range.spec_orderedsetsteph_wf())  ← NOT detected
Chap43/OrderedSetMtEph.rs:430 — assume(left.spec_orderedsetsteph_wf())   ← detected ✓
```

The detector catches `split` and `split_rank` (lines 430/431/489/490) but
misses `size`, `find`, and `get_range` in the same file. All are the same
pattern: acquire_read → call StEph → release_read → assume result matches
ghost view.

**Detection heuristic should be:** any `assume(...)` inside a function that
(a) is in a file ending in `Mt*.rs`, and (b) appears after a method call on
an RwLock-wrapped inner type, and (c) bridges the result to `self@` or a
wf predicate. At minimum, look for assumes in Mt files that reference
`self@` or `spec_*_wf()`.

## Bug 6: Warning-only files shown as errors

Files with ONLY `fn_missing_*` warnings (no actual holes) are shown with the
`❌` error icon. For example, `SetMtEph.rs` and `SetStEph.rs` show `❌` but
their only findings are fn_missing_wf_requires/ensures warnings.

A warning is not a hole. Use a different icon:
- `✓` — clean (no holes, no warnings)
- `⚠` — warnings only (fn_missing_*, requires_true)
- `❌` — actual holes (assume, external_body, admit)

This would let users quickly scan for real problems vs spec hygiene issues.

## Enhancement 1: Distinguish assume categories

The 105 `assume()` holes are not all the same. Useful subcategories:

| Category | Pattern | Actionability |
|---|---|---|
| RwLock ghost bridge | `assume(inner@ =~= self@)` in Mt files | Structural — can't prove without Verus changes |
| Algorithmic assume | `assume(sorted)`, `assume(len < MAX)` | High — these are real proof targets |
| Closure requires | `assume(f.requires(...))` | Medium — need closure spec propagation |
| Clone/eq bridge | `assume(cloned@ == self@)` | Structural — standard workaround |

Right now they're all lumped together. The consumer (me, orchestrating agents)
needs to know which assumes are proof targets vs structural boundaries.

## Enhancement 2: Per-chapter summary in Section 3

Add a per-chapter holes table to the summary:

```
Per-Chapter Holes (excluding accepted):
  Chap37:  2    Chap41: 23    Chap45:  1
  Chap38: 10    Chap43: 98    Chap47: 10
  Chap39: 10    Chap57:  4    Chap59:  1
```

This is the single most useful view for proof orchestration. The
chapter-cleanliness-status.sh script produces this, but it should also
be in the hole detector output since the data is already computed.

## Enhancement 3: Accepted-hole provenance

The "Accepted" section shows 293 `accept()` but doesn't break down by
chapter or file. When I'm orchestrating agents, I need to know which
chapters have accepted holes vs which are fully proved. Add:

```
Accepted by Chapter:
  Chap02: 10    Chap39:  6    Chap43: 30
  ...
```

## Priority order

| # | Item | Severity | Effort |
|---|---|---|---|
| 1 | Section 4 empty | High — advertised feature doesn't exist | Medium |
| 2 | Default exclusions | High — inflates all numbers 3x | Low |
| 3 | EQ_CLONE double-count | Medium — confusing math | Low |
| 4 | Warning-only icon | Medium — visual noise | Low |
| 5 | Verus API as holes | Low — 4 items total | Low |
| 6 | Incomplete RWLOCK_GHOST | Low — affects SFP accuracy | Medium |
| 7 | Assume subcategories | Enhancement — useful for orchestration | Medium |
| 8 | Per-chapter summary | Enhancement — useful for orchestration | Low |
| 9 | Accepted provenance | Enhancement — useful for orchestration | Low |

## Validation after fixes

Run against APAS-VERUS `src/` with default exclusions. Expected:
- Holes Found: ~158 (not 421)
- Warnings: ~40 (not 360)
- Accepted: ~328 (not 359)
- Structural FPs: ~24 (not 61)
- Real Actionable: ~134 (not 367)
- Section 4 should have actual content
- No `❌` icons on warning-only files
- EQ_CLONE_ASSUME in exactly one bucket

If Holes Found > 200, the exclusions are wrong. If Structural FPs detected
!= Structural FPs subtracted, the math is wrong.
