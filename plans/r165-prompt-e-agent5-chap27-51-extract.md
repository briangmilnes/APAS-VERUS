# R165 Prompt E — Agent 5: Extract specs from Chap27+51. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent5`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Extract shared specs and lemmas from two chapters:

- Chap27 (Reduce/Scan contracts): `src/Chap27/ContractSpecsAndLemmas.rs` —
  ~16 duplicated proof fns, 4 files, 1575 lines
- Chap51 (sequences/lists): `src/Chap51/SeqSpecsAndLemmas.rs` —
  ~18 duplicated proof fns, 8 files, 4026 lines

## Files

`src/Chap27/`: ReduceContract{St,Mt}Eph.rs, ScanContract{St,Mt}Eph.rs

`src/Chap51/`: all variant files

## Approach

Two separate extractions, same pattern as `src/Chap42/TableSpecsAndLemmas.rs`:

1. **Read all files and all standards.**
2. **Chap27:** Reduce and Scan contracts share contraction lemmas, associativity
   proofs, identity element proofs. St and Mt variants duplicate these.
3. **Chap51:** Identify shared sequence operation specs and proofs across variants.
4. **Register in lib.rs** as first entries in their chapter modules.
5. **Validate:**
   ```bash
   scripts/validate.sh isolate Chap27
   scripts/validate.sh isolate Chap51
   ```

## Report

Write `plans/agent5-round165-report.md`.

## RCP

`git add -A && git commit -m "R165 Agent 5: extract Contract/Seq SpecsAndLemmas (−N lines)"`, then `git push`.
