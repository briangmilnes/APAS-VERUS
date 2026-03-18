# R37 Agent 1 Update 2: Iterative vs Recursive Inventory

## Context

The APAS textbook by Blelloch presents most algorithms recursively —
divide-and-conquer is the textbook's core paradigm. Some of our Verus
implementations converted recursive algorithms to iterative loops,
either for verification convenience or by design choice.

We want both versions (iterative and recursive) as Verus training data.
Your job: build an inventory of where we diverge from the textbook.

## Task

Produce a table in `plans/iterative-vs-recursive-inventory.md` with
one row per function that is implemented differently from the textbook.

### What to check

Focus on algorithmic files (not Mt wrappers, not Example files):

1. **Chap38**: BSTParaStEph.rs — parallel BST operations. Which are
   recursive in code? Which are iterative? What does the textbook say?

2. **Chap39**: BSTTreapStEph.rs, BSTParaTreapStEph.rs — treap operations.
   insert, find, delete, split, join — recursive in textbook?

3. **Chap41**: AVLTreeSetStEph.rs — AVL operations. insert, delete,
   find, rebalance — which are recursive, which iterative?

4. **Chap43**: OrderedTableStEph.rs, OrderedSetStEph.rs — ordering ops.
   first_key, last_key, previous_key, next_key use linear scans over
   arrays. Does the textbook present these as tree-recursive traversals?

5. **Chap36**: MergeSortStEph.rs, QuickSortStEph.rs — textbook is
   recursive. Are ours?

6. **Chap47**: Hash table operations — probing is iterative in textbook
   too, so these should match. Confirm.

7. **Any other St files** with loop-based implementations of what the
   textbook presents recursively.

### Table format

```markdown
| # | Chap | File | Function | Textbook | Ours | Match? | Notes |
|---|------|------|----------|----------|------|--------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | insert | recursive | iterative | NO | loop over vec |
```

Values for Textbook/Ours columns: `recursive`, `iterative`, `parallel-recursive`.

### How to determine textbook presentation

The textbook algorithms are functional/recursive by default. If a function
uses `while` or `for` loops in our code but the textbook presents it as
a recursive function with `match` on tree structure, that's a mismatch.

Read the function bodies. Look for:
- `while` / `for` loops → iterative
- Self-calls or calls to child traversals → recursive
- `join()` with recursive calls → parallel-recursive

### Do NOT modify any source files

This is a research/inventory task only. Read files, produce the table.
Do not edit any .rs files.

## Deliverables

1. `plans/iterative-vs-recursive-inventory.md` — the table
2. Update `plans/agent1-round37-report.md` with a note about this task
3. Commit, push to `agent1/ready`
