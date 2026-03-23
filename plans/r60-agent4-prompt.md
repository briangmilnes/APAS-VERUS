# Agent 4 — Round 60

You are Agent 4 working in `~/projects/APAS-VERUS-agent4`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap26 ETSPMtEph.rs — 1 hole (external_body on point_distance)

Agent 1 assessed this as structural (float arithmetic boundary). The spec
function `spec_point_distance` is `uninterp`. However, investigate whether
we can close this without full float axioms:

1. Read `src/Chap26/ETSPMtEph.rs` and `ETSPStEph.rs`.
2. Read `src/vstdplus/float.rs` for existing float axioms.
3. The function computes Euclidean distance: `sqrt((x1-x2)^2 + (y1-y2)^2)`.
4. If the spec only needs the result to be non-negative (`d >= 0.0`),
   that's provable without arithmetic axioms (sqrt returns non-negative).
5. If the spec needs the exact formula, that requires float arithmetic
   axioms we don't have — report as structural.

If you can prove even a partial spec (non-negativity, symmetry), that's
progress. If not, confirm it's structural and move on.

### Target 2: Validate trigger warnings — full codebase sweep

Run `scripts/validate.sh` and check for any "automatically chose triggers"
notes or trigger warnings. For each:

1. Read the quantifier that triggered the warning.
2. Add explicit `#[trigger]` annotations per the trigger rules in CLAUDE.md.
3. Re-validate to confirm the warning is gone.

The current validate output shows warnings in Chap47 and Chap62 areas.
Clean them all.

### Target 3: Verify Chap47 QuadProbFlatHashTableStEph.rs trigger warnings

Agent 1's R59 work left 2 trigger warnings in this file (lines ~467).
Fix the `assert forall` quantifiers with explicit triggers. The file now
has 9 clean proof functions and 0 holes — make the trigger warnings match
that cleanliness.

### Target 4: Run 3x stability validation

After all changes, run `scripts/validate.sh` three times. All three must
show 0 errors. If any run shows errors, diagnose and fix the flake before
pushing.

## Validation

Run `scripts/validate.sh` (3x), `scripts/rtt.sh`, `scripts/ptt.sh`
sequentially. Write report to `plans/agent4-round60-report.md`.
Push to `agent4/ready`.
