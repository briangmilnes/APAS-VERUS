# R18 Gaps — For Follow-Up

## Agent 2 (Chap43 TotalOrder) — Incomplete

### Done
- TotalOrder bound added to all 6 St files + Chap52 callers.
- Extremality ensures for first/last/previous/next — correct, all external_body.
- T-quantifier pattern used (no T::V: TotalOrder needed).

### Not Done

1. **rank/select**: Agent claimed `Set::filter` needs `T::V: TotalOrder`. Wrong —
   use the same T-quantifier pattern, or write the correct ensures and add
   external_body. The spec is: `r == |{x in A | x < k}|`.

2. **Mt files**: Did not propagate ordering ensures to Mt/MtPer wrappers. These are
   pure delegation — copy St ensures verbatim. No proof work needed.

### Why external_body is correct here
The Chap43 implementations use AVL trees (Chap37). Proving first = min requires
access to the AVL sortedness invariant. That invariant is internal to Chap37 impls.
The external_body is a legitimate proof placeholder, not an excuse.
