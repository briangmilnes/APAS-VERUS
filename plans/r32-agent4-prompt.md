R32: Prove holes in Chap45 and Chap47.

TASK 1 — Chap45: Close the chapter (2 holes).

(a) src/Chap45/BinaryHeapPQ.rs — `extract_all_sorted` assume (line ~1032).
    Assumes result is sorted. The function repeatedly calls extract_min
    in a loop, building a result sequence. Prove sortedness:
    - Loop invariant: result so far is sorted, last element <= heap min
    - Each extract_min returns the minimum element
    - Appending min to sorted prefix preserves sortedness
    The heap's extract_min already has ensures about returning the
    minimum. Connect that to the loop invariant.

(b) src/Chap45/BalancedTreePQ.rs — `insert` external_body.
    Tree-based priority queue insertion. Read the function body to
    understand the insertion logic (likely binary search for position,
    then rebuild). Prove the result maintains the PQ invariant and
    contains the new element.

TASK 2 — Chap47: Prove lookup assumes and insert operations.

(a) src/Chap47/DoubleHashFlatHashTableStEph.rs — 2 lookup assumes.
    Line ~129: Prove second_hash determinism (s == step).
    Line ~171: Prove wrapping arithmetic (slot == (h + attempt*step) % m).
    These are modular arithmetic properties. Search vstd for
    wrapping/modular arithmetic lemmas. The key insight: the probe
    sequence is deterministic given the hash and step values.

(b) src/Chap47/QuadProbFlatHashTableStEph.rs — 1 lookup assume.
    Line ~131: Prove slot == (h + attempt*attempt) % m.
    Same pattern as double hash but with quadratic probing.

(c) src/Chap47/LinProbFlatHashTableStEph.rs — `insert` external_body.
    Linear probe insertion. Prove: finds empty slot, inserts pair,
    maintains hash table invariant. Linear probing is the simplest
    probe pattern — good starting point.

(d) src/Chap47/DoubleHashFlatHashTableStEph.rs — `insert` external_body.
    Double hash probe insertion. Same pattern as linear but with
    double hash step.

(e) src/Chap47/QuadProbFlatHashTableStEph.rs — `insert` external_body.
    Quadratic probe insertion. Same pattern with quadratic step.

Skip all `resize` operations (5 files) — they rebuild the entire
table and are lower priority than insert/lookup correctness.
Skip ParaHashTableStEph.rs external_body (opaque Fn closure boundary).

Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
See plans/orchestrator-r32-hole-reduction.md for context.
