# R169 Prompt A — Agent 1: Repair UnionFindStEph proofs for current dependencies. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`.** That's what we're removing, not adding.

## Background

`src/Chap65/UnionFindStEph.rs` was fully proven at R130 (April 1, 2026) with
zero holes and zero external_body. Between then and now, vstdplus changed
(TotalOrder restructuring, bridge lemma moves) and the proofs no longer
verify. The code is structurally correct — it just needs proof repair for the
current dependency state.

KruskalStEph.rs and PrimStEph.rs are from the same era. Prim was separately
proven and verifies. Kruskal depends on UnionFind, so fix UnionFind first.

## The 5 errors

Run `scripts/validate.sh isolate Chap65` to see them. From our last run:

1. **Postcondition not satisfied** at line 286 (also 273, 247)
2. **rlimit exceeded** at line 740
3. **rlimit exceeded** at line 830
4. **rlimit exceeded** at line 1163
5. **rlimit exceeded** at line 1813

The postcondition failure is likely a missing lemma call or changed ensures
from a dependency. The rlimit failures are Z3 matching loops on the
13-quantifier wf predicate — they need either fuel bumps, trigger fixes,
or intermediate assert hints to guide Z3.

## Approach

1. **Read all standards.** Read `src/Chap65/UnionFindStEph.rs` end to end.
2. **Run `scripts/validate.sh isolate Chap65`** and read the full output.
3. **Fix the postcondition failure first** — it's likely the root cause
   that cascades into the rlimit failures.
4. **For rlimit failures**: try `#[verifier::rlimit(N)]` bumps first
   (double the current limit). If that doesn't work, add intermediate
   `assert` hints to break the proof into smaller steps for Z3.
5. **Profile if stuck**: `scripts/validate.sh isolate Chap65 --profile`
   then `ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat`.
6. **After UnionFind verifies**, run `scripts/validate.sh isolate Chap65`
   again to confirm Kruskal also passes (it depends on UnionFind).

## What changed in dependencies

Between R130 (April 1) and now, these vstdplus changes happened:
- TotalOrderBridge merged into TotalOrder (trait methods with where Self: Ord)
- Bridge lemma assumes moved to TotalOrder default bodies
- ThreadShareablePlus deleted

These may have changed which axioms Z3 sees in scope, affecting trigger
selection and quantifier instantiation in UnionFind's wf predicate.

## No punting

Do not add `external_body`. Do not add `assume`. Do not add `admit`.
Do not add `accept`. The proofs worked 8 days ago. They need repair,
not replacement.

## Validation

```bash
scripts/validate.sh isolate Chap65
```

Target: 0 errors across all 3 files.

## Report

Write `plans/agent1-round169-report.md`.

## RCP

`git add -A && git commit -m "R169 Agent 1: repair UnionFindStEph proofs for current deps (−5 errors)"`, then `git push`.
