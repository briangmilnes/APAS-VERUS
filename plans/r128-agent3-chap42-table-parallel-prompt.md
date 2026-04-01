# R128 Agent 3 — Parallelize Chap42 TableMtEph operations. AFK.

## Background

`src/Chap42/TableMtEph.rs` has 12 DIFFERS functions — all sequential where APAS
expects parallel span. These are array-backed table operations (not tree-based),
so the D&C pattern from Chap18 applies: split array, join halves, combine.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs`
2. `src/standards/hfscheduler_standard.rs`

## The 12 DIFFERS functions

Run this to see them:
```bash
~/projects/veracity/target/release/veracity-analyze-alg-analysis -c ~/projects/APAS-VERUS 2>&1 | grep 'Chap42.*DIFFERS'
```

Expected: domain, tabulate, map, filter, intersection, union, difference, find,
delete, insert, restrict, subtract.

## Approach

1. Read the file thoroughly. Understand the data structure (array of key-value pairs
   behind a RwLock) and the existing trait + impl.

2. For each function, assess whether D&C parallelism is feasible:
   - **Parallelizable**: Operations that scan the array and can be split (map, filter,
     tabulate, domain — read-only scans). These follow the Chap18 map/reduce pattern.
   - **Probably sequential**: Operations with ordering constraints or nested lookups
     (insert, delete, find — single-element operations that are O(n) due to linear
     scan, not parallelizable computation). Also intersection/union/difference which
     are nested linear scans — could use sorted merge but that's a different algorithm.

3. For parallelizable functions:
   - Add `Clone + Send + Sync + 'static` bounds to closure parameters in trait + impl
   - Rewrite body with D&C: split at mid, join() both halves, combine
   - Use `clone_fn`, `clone_pred` from `crate::vstdplus::clone_plus::clone_plus::*`
   - Update Code review annotations

4. For functions that remain sequential, update the DIFFERS annotation with an
   accurate reason explaining why.

## Reference

Read `src/Chap18/ArraySeqMtEph.rs` for the verified D&C pattern (map_dc, reduce_dc
from R127). Same split/join/append pattern applies to array-backed tables.

## Validation

Run `scripts/validate.sh isolate Chap42` after changes. Fix verification errors.
Then run `scripts/rtt.sh`.

## Rules

- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Preserve all existing RTTs.
- If a function can't be parallelized, leave it sequential with an accurate DIFFERS annotation.

## When done

Commit with `git add -A && git commit` and push.

## Report

Write `plans/agent3-r128-report.md` with:
- Table: # | Function | Parallelized? | Old Span | New Span | Reason if not
- Verification count
