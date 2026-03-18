# R38 Agent 4: Chap47 Chained Hash Tables + BSTParaStEph

## Baseline
- Main at `485299d3`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4332 verified, 204 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.
**DO NOT convert assume() to accept().** Leave assumes as assumes.

Read CLAUDE.md and `src/standards/partial_eq_eq_clone_standard.rs` before starting.

## Assignment

You are Agent 4 for R38. Your scope is **Chap47 chained hash tables + Chap38 BSTParaStEph**.

## Task 1: Prove eq/clone bridge assumes in LinkedListChainedHashTableStEph.rs (12 holes)

File: `src/Chap47/LinkedListChainedHashTableStEph.rs`

All 12 holes are eq bridge or clone bridge assumes:
- Lines 55-56: clone bridges (chain_to_vec)
- Line 190: eq bridge (insert)
- Line 283: eq bridge (lookup)
- Line 286: clone bridge (lookup)
- Line 344: eq bridge (delete)
- Lines 357-358: clone bridges (delete)
- Lines 469-470: clone bridges (resize helper)
- Lines 566-567: clone bridges (resize)

**Eq bridge pattern:** After `k == key` (PartialEq::eq), the result equals the spec
because PartialEq::eq ensures `r == (self@ == other@)`. Assert that
`(k@ == key@) == (bucket_seq@[i].0 == key)` using the View equivalence.

**Clone bridge pattern:** After `k = entry.clone()`, the clone's View equals the
original's View because Clone::clone ensures `cloned@ == self@`.

Read `src/standards/partial_eq_eq_clone_standard.rs` for the ensures patterns.

## Task 2: Prove eq/clone bridge assumes in VecChainedHashTableStEph.rs (12 holes)

File: `src/Chap47/VecChainedHashTableStEph.rs`

Identical pattern to LinkedList version:
- Lines 53-54: clone bridges
- Line 191: eq bridge
- Line 285: eq bridge
- Line 288: clone bridge
- Line 345: eq bridge
- Lines 358-359: clone bridges
- Lines 472-473: clone bridges
- Lines 568-569: clone bridges

## Task 3: Prove expose assume in BSTParaStEph.rs (1 hole)

File: `src/Chap38/BSTParaStEph.rs`

One assume in the `expose` function — clone/view bridge for the BST node type.
Read the function, understand the clone ensures, and prove the bridge.

## Strategy

Start with LinkedListChainedHashTableStEph.rs — crack all 12 eq/clone bridges.
The pattern will be identical for VecChainedHashTableStEph.rs (copy-paste proofs).
Then do BSTParaStEph.rs (1 hole, different pattern).

## Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent4-r38-report.md`.
