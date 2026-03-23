# Agent 5 — Round 63

You are Agent 5 working in `~/projects/APAS-VERUS-agent5`.
You are a **senior proof engineer** and **algorithms expert** who knows the APAS
textbook structure intimately.

## Baseline

- Main: 4504 verified, 0 errors, 6 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Task: Verify and update the iterative-vs-recursive inventory

Read `plans/iterative-vs-recursive-inventory.md` and
`plans/iterative-vs-recursive-rename-plan.md`. These claim 27 functions across
Chap41 and Chap43 are iterative where the APAS textbook presents them
recursively. Your job is to verify each claim against the actual textbook
presentation and produce an updated, accurate plan.

### What to do

**Phase 1: Read the APAS textbook sections.**

The textbook lives at `~/projects/apas/book.pdf`. Read the relevant sections:

- **Chapter 41** (Ordered Sets / Balanced BSTs) — specifically the pseudocode for
  `insert`, `delete`, `find`, `filter`, `intersection`, `union`, `difference`,
  `from_seq`. Are these presented as recursive tree traversals or iterative loops?

- **Chapter 43** (Ordered Tables and Ordered Sets) — specifically `first`, `last`,
  `previous`, `next`, `rank`, `select` (for both OrderedSet and OrderedTable
  variants). Are these recursive traversals on the backing tree, or are they
  defined as delegations to the backing set with the recursion hidden inside?

- Also check: does APAS distinguish between the Set operations (Chap41) and the
  Table operations (Chap43) in terms of algorithm style? Or does Chap43 just say
  "use the ordered set from Chap41"?

**Phase 2: Read our implementations.**

For each of the 27 claimed mismatches, read the actual function in our codebase:

- `src/Chap41/AVLTreeSetStEph.rs` — 8 functions
- `src/Chap41/AVLTreeSetStPer.rs` — 7 functions
- `src/Chap43/OrderedSetStEph.rs` — 6 functions
- `src/Chap43/OrderedTableStEph.rs` — 6 functions

For each function, note:
1. Is our implementation iterative (while/for loop) or recursive (self-call/descent)?
2. What is the time complexity? O(n) linear scan or O(log n) tree traversal?
3. Does it delegate to a backing store operation that IS recursive?

**Phase 3: Classify each function.**

For each of the 27 functions, assign one of:

- **MISMATCH-RENAME**: Textbook says recursive, ours is iterative. Rename current
  to `fn_name_iter`, write recursive `fn_name` later. This is the Phase 1 rename
  target.
- **MISMATCH-DELEGATION**: Textbook says recursive, ours delegates to a backing
  store. The recursion is in the backing store, not here. The delegation itself is
  fine — no rename needed, but the backing store may need a recursive version.
- **MATCH**: Our implementation matches the textbook (either both recursive or
  both iterative). No action needed.
- **TEXTBOOK-ITERATIVE**: Textbook actually presents this iteratively (the
  inventory was wrong). No action needed.
- **NOT-IN-TEXTBOOK**: Function doesn't appear in APAS (e.g., `from_seq` may be
  our addition). Document but don't rename.

**Phase 4: Check the StPer variants.**

The inventory lists 7 StPer mismatches mirroring the 8 StEph ones (missing
`from_seq`). Confirm:
- Does APAS present persistent variants separately, or just says "same algorithm,
  persistent data structure"?
- Are the StPer implementations structurally identical to StEph?
- If so, they inherit the same classification.

**Phase 5: Check what's missing from the inventory.**

The inventory may be incomplete. Scan for other iterative implementations in
Chap41 and Chap43 that should be recursive. Also check:
- `src/Chap43/OrderedSetStPer.rs` — not in the inventory but should mirror StEph
- `src/Chap43/OrderedTableStPer.rs` — same
- `src/Chap43/AugOrderedTableStEph.rs` and `AugOrderedTableStPer.rs` — augmented
  ordered tables
- Any Mt (multi-threaded) variants that inherit the mismatch

### Output

Write the updated plan to `plans/iterative-vs-recursive-inventory-v2.md` with:

1. **Updated mismatch table** with the classification column added.
2. **Rename plan** — only MISMATCH-RENAME functions get renamed. List exactly
   which functions in which files.
3. **Delegation analysis** — for MISMATCH-DELEGATION, identify which backing
   store functions need recursive versions.
4. **StPer/Mt variant table** — which variants inherit which classification.
5. **Priority ordering** — which renames/recursive rewrites give the most value
   (training data quality, algorithmic correctness, complexity match).

### Naming convention

- Textbook says recursive → `fn_name` is recursive (default), `fn_name_iter` is
  the current iterative version being renamed.
- Textbook says iterative → `fn_name` stays iterative, no rename needed.
- If we later add a recursive implementation, it goes under the default name and
  the current iterative impl has already been renamed to `fn_name_iter`.

### Reference experiment

Read `src/experiments/trait_rec_vs_iter.rs` for the verified pattern: one type,
one trait, both `sum` (recursive, textbook default) and `sum_iter` (iterative
alternative) verified against the same spec.

### Do NOT modify any Chap41 or Chap43 source files.

This round is research and planning only. Write the updated inventory. Do not
rename functions, do not add implementations. The actual rename work will be
assigned in a future round based on your analysis.

## Validation

No code changes expected, but if you modify any files, run `scripts/validate.sh`.
Write your analysis to `plans/iterative-vs-recursive-inventory-v2.md`.
Push to `agent5/ready`.
