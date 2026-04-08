# R162 Prompt A — Glob Import Minimization Experiment. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER run RTTs.** Skip `scripts/rtt.sh` entirely.
5. **NEVER delete `target/` or any subdirectory.**

## Hypothesis

Replacing `use crate::ChapNN::...::*` glob imports with specific named imports
reduces the number of names (types, traits, spec fns) visible in each module's
scope. This may reduce Z3's quantifier instantiation workload and lower
`rust_verify` peak RSS memory.

## Baseline (already measured on main at afec35fcf)

```
scripts/validate.sh isolate Chap43
verification results:: 2804 verified, 0 errors
Elapsed: 31s
peak rust_verify RSS: 3368MB, peak z3 RSS: 329MB
```

## Your task

Replace glob imports with specific named imports across the Chap18→Chap43
dependency chain, then re-measure. Report whether RSS dropped.

## Scope

Files to process — every file that contains `use crate::ChapNN::...::*` in:

- `src/Chap18/`
- `src/Chap19/`
- `src/Chap37/`
- `src/Chap38/`
- `src/Chap41/`
- `src/Chap42/`
- `src/Chap43/`

Skip `Example*.rs` and `Problem*.rs` files.

## Approach

For each glob import in each file:

1. Note the glob: `use crate::ChapNN::Foo::Foo::*;`
2. Remove the `*` import.
3. Run `scripts/validate.sh isolate Chap43`.
4. Read the error output — Verus will list every unresolved name.
5. Add back only those specific names: `use crate::ChapNN::Foo::Foo::{A, B, C};`
6. Re-validate until clean.
7. Move to the next glob.

Work one file at a time. Validate isolate Chap43 after each file is complete.

**STEP 10 per glob** — if a glob is proving very difficult to resolve (many
nested re-exports, trait aliases, macro names), leave it as `*` and move on.
Note it in the report.

## Validation command

```bash
scripts/validate.sh isolate Chap43
```

Estimated ~31s per run.

## Report

Write to: `plans/r162-glob-import-experiment-report.md`

Include:
- Table of every glob processed: file, original `*`, specific names substituted
  (or "left as *" with reason)
- Baseline vs final: elapsed time, peak rust_verify RSS, peak z3 RSS
- Conclusion: did memory drop? By how much?

Format:

| # | Chap | File | Original glob | Result | Specific names |
|---|------|------|---------------|--------|----------------|
| 1 | 43 | OrderedTableStEph.rs | `OrdKeyMap::*` | replaced | `OrdKeyMap, OrdKeyMapTrait, ...` |

## When done

Do NOT commit. Report results. Stop. The orchestrator will decide whether to
keep the changes based on the memory measurement.
