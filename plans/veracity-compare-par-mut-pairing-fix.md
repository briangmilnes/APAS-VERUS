# Veracity compare-par-mut: fix variant pairing order

## Problem

The tool currently groups all variants of a base name (e.g., OrderedTable)
and compares every variant against whichever has the strongest specs (usually
StPer). This generates massive false-positive warning counts because it
compares across persistence boundaries (StPer→MtEph, StPer→StEph) where
legitimate differences exist.

Example: Chap43 OrderedTable has 4 variants. The tool compares MtEph, StEph,
and MtPer all against StPer, generating 203 warnings for one module family.

## Correct pairing order

Specs flow in a specific direction through the variant lattice:

```
StPer → StEph → MtEph
  |                ↑
  +--→ MtPer ------+
```

The comparison chain should be:

1. **StPer → StEph**: Persistent sequential is the strongest spec. Compare
   StEph against StPer. Warnings here mean StEph is missing specs that StPer
   has (and StEph should have the same or equivalent).

2. **StPer → MtPer** (if MtPer exists): Compare MtPer against StPer. Same
   persistence model, different threading. MtPer should match StPer's specs.
   Most modules don't have MtPer so this step is often skipped.

3. **StEph → MtEph**: Compare MtEph against StEph. Same ephemeral model,
   different threading. MtEph should match StEph's specs. This is the most
   common pairing.

4. **MtPer → MtEph** (if both exist): Compare MtEph against MtPer. Rarely
   both exist, but when they do, check consistency.

Each comparison is a directed pair: reference → current. Warnings say "reference
has X that current doesn't."

## What this fixes

- StEph no longer compared against StPer for `_iter` and `_wf` functions that
  only exist in StPer. Those are legitimate StPer-only functions.
- MtEph compared against StEph (its actual source), not StPer (two hops away).
- Warning counts drop dramatically because each variant is only compared against
  its immediate predecessor in the lattice, not the global strongest.

## What stays the same

- Phase 1 file grouping by base name still works.
- Phase 4 per-function clause comparison logic stays the same.
- The `--chapter`, `--exclude`, `--phase4-only` flags are unaffected.

## Expected variants per group

| Variants present | Comparisons |
|-----------------|-------------|
| StEph only | Nothing to compare |
| StEph + MtEph | StEph → MtEph |
| StPer + StEph | StPer → StEph |
| StPer + StEph + MtEph | StPer → StEph, StEph → MtEph |
| StPer + MtPer + StEph + MtEph | StPer → StEph, StPer → MtPer, StEph → MtEph, MtPer → MtEph |
| StEph + MtEph + MtPer | StEph → MtEph, MtPer → MtEph |
| MtEph only | Nothing to compare |

If only MtEph exists (e.g., HFScheduler), no comparison. If StPer exists
without StEph (shouldn't happen but defensive), skip.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
