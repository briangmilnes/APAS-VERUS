# R48 Agent 4: Chap26 ETSP (4 holes)

## Assignment

Prove the ETSP (Euclidean Traveling Salesman Problem) holes in Chap26. These
involve float distance axioms. You've been on Chap38 for 3 rounds — fresh eyes
on a different problem.

## Baseline

38 holes total. 4419 verified. Your chapter: Chap26 (4 holes).

## REQUIRED READING

1. `src/vstdplus/float.rs` — Float axioms and FloatTotalOrder trait
2. `src/Chap26/ETSPMtEph.rs` — The file with holes
3. `src/standards/using_closures_standard.rs`

## Current Holes

Run `scripts/holes.sh src/Chap26/` to verify.

The holes are in ETSPMtEph.rs. Read the file to identify exact functions and
line numbers. They involve float distance computations for the ETSP approximation
algorithm.

## Strategy

The ETSP algorithm computes Euclidean distances between points. The proofs need:
- Triangle inequality for Euclidean distance
- Non-negativity of distance
- Distance symmetry
- Possibly: distance monotonicity under point insertion/removal

Check what axioms `vstdplus/float.rs` already provides. If the needed axioms
exist, use them. If not, you may need to add float arithmetic axioms to
vstdplus/float.rs (addition monotonicity, multiplication, sqrt properties).

Key question: are the holes in the float arithmetic itself, or in the algorithm
logic that uses distances? Read the verification errors carefully.

If the float axioms are the real blocker and require substantial new axiom
development, document exactly what's needed and move on. Don't spend the
whole round on axiom development.

## What NOT to do
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT modify vstdplus/float.rs without reading it thoroughly first.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap26/`.
Write your report to `plans/agent4-round48-report.md`.
