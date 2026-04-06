# Veracity Rule [17] Fix: Suppress False Positives on Non-Collection Structs

## Context

You are working in `~/projects/veracity`. There may be another agent on
`~/projects/veracity-agent1` — do not touch that worktree.

## Problem

Rule [17] fires on structs that are not iterable collections. It currently flags
any struct that has a Vec or Map field as a "collection" that needs iterators.
But many structs with Vec/Map fields are NOT collections:

### False positive category 1: DP result/memoization structs

These store 2D tables of optimal costs, not iterable sequences. Iterating them
makes no sense.

Types: `MatrixChainStEphS`, `MatrixChainStPerS`, `MatrixChainMtEphS`,
`MatrixChainMtPerS`, `OBSTStEphS`, `OBSTStPerS`, `OBSTMtEphS`, `OBSTMtPerS`,
`OBSTStEphV`, `OBSTStPerV`, `MinEditDistStEphS`, `MinEditDistStPerS`,
`SubsetSumStEphS`, `SubsetSumStPerS`.

All in Chap49/Chap50. ~40 warnings.

### False positive category 2: Return/result structs

Small structs returned from functions, not collections.

Types: `SearchResult`, `PQMinResult`, `KeyProb`, generic `T`.

~15 warnings.

## Fix

Add heuristics to skip [17] for structs that are clearly not collections:

1. **Structs whose name ends with `Result`, `Info`, `Prob`, or `V` (when V means
   "value/view type")**: suppress. These are return types or view types.

2. **Structs in Chap49/Chap50 whose name contains `Eph` or `Per` and whose fields
   include `Vec<Vec<...>>` (2D tables)**: suppress. These are DP memoization structs.

3. **Better approach**: Only flag a struct as a "collection" if it has a trait with
   methods like `insert`, `push`, `add`, `remove`, `delete`, `contains`, `find`,
   `len`, `size`, `is_empty` — i.e., collection-like API. A struct with a Vec field
   but no collection API is not a collection.

Option 3 is the most robust. A struct is a collection if its trait has >= 2 of:
`insert`/`push`/`add`, `len`/`size`/`is_empty`, `contains`/`find`/`get`.
Otherwise it's just a struct that happens to have a Vec.

## Expected impact

~55 false positives eliminated. Remaining ~109 are real (missing iterators or
IntoIterator outside verus!).

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. A string-hacking detector
will flag and kill tools that corrupt source syntax.
