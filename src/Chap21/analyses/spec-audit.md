# Chap21 Spec Audit — Examples

Audited: 2026-03-15, Agent 4, Round 19.
Prose source: prompts/Chap21.txt (Chapter 21: Examples).

## Summary

Chapter 21 is titled "Examples" and contains 12 files implementing textbook demo and
exercise code (2D/3D points, Cartesian product, isPrime, primes, prime sieve, etc.).
All files are named Algorithm21_*, Exercise21_*, or Problem21_*. Per CLAUDE.md:
"Skip Example files unless explicitly assigned."

These are standalone exercise implementations, not reusable ADT modules. They do not
define traits used by other chapters.

**No ADT specs to audit. No changes needed.**

## File Inventory

| # | File | Type | Description |
|---|------|------|-------------|
| 1 | Algorithm21_1.rs | Algorithm | 2D points (tabulate + flatten) |
| 2 | Algorithm21_2.rs | Algorithm | 3D points |
| 3 | Algorithm21_5.rs | Algorithm | Brute force primes |
| 4 | Algorithm21_6.rs | Algorithm | Prime sieve |
| 5 | Exercise21_5.rs | Exercise | All contiguous subsequences |
| 6 | Exercise21_6.rs | Exercise | Cost analysis (subsequences) |
| 7 | Exercise21_7.rs | Exercise | Comprehension with conditionals |
| 8 | Exercise21_8.rs | Exercise | Primes problem statement |
| 9 | Exercise21_9.rs | Exercise | Composite number proof |
| 10 | Problem21_1.rs | Problem | 2D points problem |
| 11 | Problem21_3.rs | Problem | 3D points problem |
| 12 | Problem21_4.rs | Problem | Cartesian product |

## Note on Task Description

The task description listed "Chap21 (Trees)" with "TreeStEph.rs" but no such file
exists. Chapter 21 in APAS is "Examples", not "Trees". Trees appear in Chapter 6
(Def 6.24-6.25) and Chapter 23 (Tree Sequences).

## Verdict

Skipped per CLAUDE.md example-file policy. No changes.
