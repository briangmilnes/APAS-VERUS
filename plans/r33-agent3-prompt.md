R33: Chap45 BinaryHeapPQ sortedness + Chap52 EdgeSetGraph proofs.

TASK 1 — Chap45: Prove extract_all_sorted sortedness (1 assume).

src/Chap45/BinaryHeapPQ.rs — `extract_all_sorted` (line ~1032).

The function builds a sorted sequence by repeatedly extracting
the minimum element from the heap. The assume asserts the result
is sorted. Prove it:

1. Read the extract_all_sorted function body.
2. Read the extract_min ensures clause — it guarantees the
   returned element is the minimum.
3. Add a loop invariant:
   - `result` so far is sorted (spec_sorted or similar)
   - If result is non-empty, its last element <= current heap min
   - The heap's well-formedness is maintained
4. After each extract_min call, the new minimum >= previous minimum
   (because previous minimum was already removed).
5. Appending the new minimum to a sorted prefix where the last
   element <= new minimum preserves sortedness.

Remove the assume once the loop invariant proves sortedness.

TASK 2 — Chap52: Prove EdgeSetGraph operations (5 external_body).

These are graph operations using edge-set representation.

(a) src/Chap52/EdgeSetGraphStEph.rs — `out_neighbors` (1 ext_body).
    Read the function. It should collect neighbors from the edge set.
    Remove external_body and prove the result matches the spec.

(b) src/Chap52/EdgeSetGraphStPer.rs — `out_neighbors` + `delete_vertex`
    (2 ext_body). Same pattern as StEph but persistent.

(c) src/Chap52/EdgeSetGraphMtPer.rs — `out_neighbors` + `delete_vertex`
    (2 ext_body). Mt wrapper around StPer.

Read the StEph version first to understand the algorithm, then
apply the same proof pattern to StPer and MtPer.

Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
