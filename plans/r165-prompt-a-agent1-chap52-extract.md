# R165 Prompt A — Agent 1: Extract AdjTableGraphSpecsAndLemmas from Chap52. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Extract shared spec fns and proof lemmas from Chap52's graph files into a new
`src/Chap52/AdjTableGraphSpecsAndLemmas.rs`. ~68 duplicated proof functions
across 14 files, 10238 lines.

## Files

`src/Chap52/`:

- AdjListGraphStEph.rs, AdjListGraphStPer.rs
- AdjMatrixGraphMtPer.rs, AdjMatrixGraphStEph.rs, AdjMatrixGraphStPer.rs
- AdjSeqGraphMtPer.rs, AdjSeqGraphStEph.rs, AdjSeqGraphStPer.rs
- AdjTableGraphMtPer.rs, AdjTableGraphStEph.rs, AdjTableGraphStPer.rs

## Approach

Follow the pattern established by `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files and all standards.**
2. **Identify shared specs and lemmas.** Look for spec fns and proof lemmas
   duplicated across St/Mt/Per variants. Graph invariant proofs (symmetry,
   edge counts, vertex sets) are prime candidates. Make shared versions
   generic over the type parameters.
3. **Create `AdjTableGraphSpecsAndLemmas.rs`.** Sections 1 (module),
   6 (spec fns), 7 (proof fns) only. No types — types stay in variant files.
4. **Register in lib.rs** as the FIRST entry in Chap52's module block.
5. **Variant files import** via `pub use crate::Chap52::AdjTableGraphSpecsAndLemmas::AdjTableGraphSpecsAndLemmas::*;`
6. **Validate after each file:** `scripts/validate.sh isolate Chap52`

## Report

Write `plans/agent1-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 1: extract AdjTableGraphSpecsAndLemmas (−N lines)"`, then `git push`.
