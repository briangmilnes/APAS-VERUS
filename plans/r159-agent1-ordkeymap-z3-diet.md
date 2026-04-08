# R159 Agent 1 — OrdKeyMap Z3 Diet: Cut Quantifier Instantiations. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/OrdKeyMap.rs` — your file.
Read this prompt fully before starting — there are 4 tasks in priority order.

Report file: `plans/r159-agent1-ordkeymap-z3-diet-report.md`

## Problem

Full-crate profiling shows OrdKeyMap is responsible for 3.37M quantifier
instantiations — the #1 consumer in the codebase. Full validation now takes
160s and uses 22GB RSS (dangerously close to 32GB limit).

The top controllable contributors:

```
872,029  lemma_reveal_view_injective (called 7 times in OrdKeyMap)
113,305  spec_key_unique_pairs_set (3 instances across methods)
 70,530  axiom_set_contains_len (from broadcast group_set_axioms)
 55,395  OrdSpec.cmp_spec (from Pair comparisons)
```

## Task 1: Reduce lemma_reveal_view_injective calls (highest impact)

`lemma_reveal_view_injective::<K>()` is called 7 times across OrdKeyMap
methods. Each call reveals a `forall` quantifier about view injectivity
that Z3 then instantiates across every K-typed term in scope.

**Current locations** (lines approximate):
```
1166, 1536, 1864, 2175, 2466, 4303, 4347
```

**Fix**: For each call site, check if the proof still verifies WITHOUT the
call. Many may have been added as "just in case" assertions by AI agents.
Comment out one at a time, validate with `scripts/validate.sh isolate Chap41`.
If it passes, leave it commented. If it fails, restore it.

For calls that ARE needed: check if a weaker assertion achieves the same
proof goal. `lemma_reveal_view_injective` reveals the full injectivity
quantifier. Sometimes a specific `assert(x@ == y@ ==> x == y)` for the
exact terms needed is sufficient and doesn't dump the universal into Z3.

**Target**: Reduce from 7 calls to 2-3.

## Task 2: Try group_set_axioms_early instead of group_set_axioms

The current broadcast is `vstd::set::group_set_axioms` which contains 27
set axioms. `vstd::set::group_set_axioms_early` is a smaller subset.

**Fix**: Change the broadcast from:
```rust
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
};
```
to:
```rust
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms_early,
};
```

Validate. If methods fail, they need the full group — add
`broadcast use vstd::set::group_set_axioms;` locally in those specific
methods' proof blocks instead of module-wide.

## Task 3: Try removing group_feq_axioms broadcast

The feq broadcast group includes 6 axioms (clone implies eq, view eq, etc.)
that fire on every type in scope. OrdKeyMap might not need all of them at
module scope.

**Fix**: Remove `group_feq_axioms` from the module-level broadcast. Add it
back in specific proof blocks of methods that need it.

Validate after removal. Track which methods need it restored.

## Task 4: Measure the improvement

After each task, run the profile to measure the reduction:

```bash
scripts/profile.sh isolate Chap41
ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs head -30
```

Report the instantiation count before/after for:
- lemma_reveal_view_injective
- spec_key_unique_pairs_set
- axiom_set_contains_len
- Total for Chap41__OrdKeyMap module

Also run full validation and report time + RSS:
```bash
scripts/validate.sh
```

## Validation

Use `scripts/validate.sh isolate Chap41` for iterative testing (40s each).
Full `scripts/validate.sh` before pushing to check RSS/time.
Then `scripts/rtt.sh`.

## Rules

- Do NOT modify any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken any ensures.
- Do NOT remove broadcast groups without testing each method.
- Comment out before deleting — restore if validation fails.
- All existing RTTs must pass.

## When done

RCP. Report: instantiation counts before/after, validation time before/after,
RSS before/after, which lemma_reveal calls were removed, which broadcast
changes stuck.
