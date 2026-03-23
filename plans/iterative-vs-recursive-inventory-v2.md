# Iterative vs Recursive Implementation Inventory v2

Agent 5, Round 63. Verified against APAS textbook flashcards (Ch37-44)
and actual source code.

## Summary of Changes from v1

1. **from_seq reclassified**: MATCH, not mismatch. The textbook sequential
   variant (Ch41 Ex 41.3) is `Seq.iterate Set.insert empty a` — iterate
   insert, which is exactly what we do. The O(n^2) runtime is a consequence
   of our O(n) insert, not a from_seq algorithm mismatch.
2. **AVLTreeSetStPer from_seq**: Added. Was missing from v1. Same MATCH
   classification as StEph.
3. **Ordering ops expanded**: split, get_range, split_rank/split_rank_key
   added — all iterative O(n) where textbook is O(log n) recursive. Were
   missing from v1.
4. **OrderedSetStPer, OrderedTableStPer**: Added. Were entirely missing
   from v1. Both have direct iterative implementations mirroring their
   StEph counterparts.
5. **OrderedTableStEph/StPer base ops**: find, insert, delete identified
   as iterative mismatches (O(n) linear scan, not delegation).
6. **select/select_key reclassified**: MATCH-DIFFERENT-ALGORITHM. Our
   implementation uses O(log n) index access via AVL nth — same complexity
   as textbook's recursive descent on size-augmented BST, different mechanism.
7. **join/join_key**: MISMATCH-DELEGATION to union, which is itself iterative.

## Textbook Analysis

### Chapter 41: Sets ADT

Ch41 defines the Sets ADT (DT 41.1) abstractly. The **implementation** uses
a tree-based BST (Ch38 Parametric BSTs). All algorithms are **recursive**:

| Operation | Textbook Alg | Style | Work | Span |
|-----------|-------------|-------|------|------|
| find | Ch37 Alg 37.4 | recursive tree descent | O(log n) | O(log n) |
| insert | Ch38 Alg 38.5 | split + joinM (recursive) | O(log n) | O(log n) |
| delete | Ch38 Alg 38.5 | split + joinPair (recursive) | O(log n) | O(log n) |
| filter | Ch38 Alg 38.9 | recursive parallel | O(n) | O(log n) |
| intersection | Ch38 Alg 38.7 | recursive parallel | O(m log(n/m)) | O(log n) |
| union | Ch38 Alg 38.6 | recursive parallel | O(m log(n/m)) | O(log n) |
| difference | Ch38 Alg 38.8 | recursive parallel | O(m log(n/m)) | O(log n) |
| fromSeq (seq) | Ch41 Ex 41.3 | iterate insert (sequential) | O(n log n) | O(n log n) |
| fromSeq (par) | Ch41 Ex 41.3 | reduce union (parallel) | O(n log n) | O(log^2 n) |

### Chapter 43: Ordered Sets / Ordered Tables

Ch43 extends Sets/Tables with ordering operations. All are **recursive**
on the tree structure, O(log n):

| Operation | Textbook Ref | Style | Work |
|-----------|-------------|-------|------|
| first | implicit (leftmost descent) | recursive | O(log n) |
| last | implicit (rightmost descent) | recursive | O(log n) |
| previous | implicit (predecessor) | recursive | O(log n) |
| next | implicit (successor) | recursive | O(log n) |
| rank | Ch40 Alg 40.1 | recursive on size-augmented BST | O(log n) |
| select | Ch40 Alg 40.1 | recursive on size-augmented BST | O(log n) |
| split | Ch38 Alg 38.3 | recursive | O(log n) |
| join | Ch38 Alg 38.4 | recursive | O(log n) |
| getRange | two splits | recursive | O(log n) |
| splitRank | one split by rank | recursive | O(log n) |

### Chapter 42: Tables ADT

Table operations mirror set operations but carry key-value pairs. Insert,
delete, find have the same tree-based recursive algorithms.

### Does APAS distinguish St/Per?

No. APAS presents algorithms in a functional style that is inherently
persistent. Our StEph (ephemeral) and StPer (persistent) variants implement
the same algorithms with different ownership semantics. The textbook makes
no distinction — same algorithm, same cost spec for both.

## Root Cause Analysis

The fundamental issue: **AVLTreeSetStEph/StPer wraps AVLTreeSeqStEph/StPer,
which is an INDEX-ORDERED AVL tree, not a BST by value.** The backing store
supports O(log n) access by index (nth) but has no concept of value-based
search or traversal. All value-based operations must either:
- Linear scan the sequence: O(n)
- Binary search by comparing elements: O(log^2 n) (log n comparisons, each
  nth is O(log n))

Neither approach matches the textbook's O(log n) recursive BST descent.

To get textbook-matching recursive implementations, we need either:
1. A proper BST implementation (BSTTreapStEph from Ch39 has this for the
   parametric BST interface)
2. Exposing the tree structure of AVLTreeSeqStEph for value-based traversal
3. Building new tree-based set/table types that ARE BSTs by value

## Updated Classification Table

### Legend

| Classification | Meaning |
|---------------|---------|
| MISMATCH-RENAME | Textbook recursive, ours iterative. Rename to `_iter`. |
| MISMATCH-DELEGATION | Delegates to a backing store function that is itself a mismatch. No rename needed here — fix the backing store. |
| MATCH | Our implementation matches textbook (both recursive, or textbook is iterative). |
| MATCH-DIFF-ALG | Same O(log n) complexity, different algorithm. Low priority. |

### Chap41 — AVLTreeSetStEph.rs (7 MISMATCH-RENAME)

| # | Chap | File | Function | Classification | Our Complexity | Textbook | Notes |
|---|------|------|----------|---------------|---------------|----------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | find | MISMATCH-RENAME | O(n log n) | O(log n) rec | while loop linear scan, each nth O(log n) |
| 2 | 41 | AVLTreeSetStEph.rs | insert | MISMATCH-RENAME | O(n) | O(log n) rec | binary search + rebuild from vec |
| 3 | 41 | AVLTreeSetStEph.rs | delete | MISMATCH-RENAME | O(n) | O(log n) rec | filter elements into result_vec |
| 4 | 41 | AVLTreeSetStEph.rs | filter | MISMATCH-RENAME | O(n) seq | O(n) par | while loop; textbook is recursive parallel |
| 5 | 41 | AVLTreeSetStEph.rs | intersection | MISMATCH-RENAME | O(n^2) | O(m log(n/m)) rec par | while loop + find per element |
| 6 | 41 | AVLTreeSetStEph.rs | union | MISMATCH-RENAME | O(n+m) | O(m log(n/m)) rec par | two consecutive while loops |
| 7 | 41 | AVLTreeSetStEph.rs | difference | MISMATCH-RENAME | O(n^2) | O(m log(n/m)) rec par | while loop + find per element |

### Chap41 — AVLTreeSetStPer.rs (7 MISMATCH-RENAME)

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|---------------|-------|
| 8 | 41 | AVLTreeSetStPer.rs | find | MISMATCH-RENAME | same pattern as StEph |
| 9 | 41 | AVLTreeSetStPer.rs | insert | MISMATCH-RENAME | same pattern as StEph |
| 10 | 41 | AVLTreeSetStPer.rs | delete | MISMATCH-RENAME | same pattern as StEph |
| 11 | 41 | AVLTreeSetStPer.rs | filter | MISMATCH-RENAME | same pattern as StEph |
| 12 | 41 | AVLTreeSetStPer.rs | intersection | MISMATCH-RENAME | same pattern as StEph |
| 13 | 41 | AVLTreeSetStPer.rs | union | MISMATCH-RENAME | same pattern as StEph |
| 14 | 41 | AVLTreeSetStPer.rs | difference | MISMATCH-RENAME | same pattern as StEph |

### Chap43 — OrderedSetStEph.rs (7 MISMATCH-RENAME + 1 MATCH-DIFF-ALG)

| # | Chap | File | Function | Classification | Our Complexity | Textbook | Notes |
|---|------|------|----------|---------------|---------------|----------|-------|
| 15 | 43 | OrderedSetStEph.rs | first | MISMATCH-RENAME | O(n) | O(log n) rec | while loop linear scan for minimum |
| 16 | 43 | OrderedSetStEph.rs | last | MISMATCH-RENAME | O(n) | O(log n) rec | while loop linear scan for maximum |
| 17 | 43 | OrderedSetStEph.rs | previous | MISMATCH-RENAME | O(n) | O(log n) rec | while loop scan for predecessor |
| 18 | 43 | OrderedSetStEph.rs | next | MISMATCH-RENAME | O(n) | O(log n) rec | while loop scan for successor |
| 19 | 43 | OrderedSetStEph.rs | rank | MISMATCH-RENAME | O(n) | O(log n) rec | linear count |
| 20 | 43 | OrderedSetStEph.rs | split | MISMATCH-RENAME | O(n) | O(log n) rec | iterative partition all elements |
| 21 | 43 | OrderedSetStEph.rs | get_range | MISMATCH-RENAME | O(n) | O(log n) rec | iterative scan |
| 22 | 43 | OrderedSetStEph.rs | split_rank | MISMATCH-RENAME | O(n) | O(log n) rec | iterative partition by index |
| 23 | 43 | OrderedSetStEph.rs | select | MATCH-DIFF-ALG | O(log n) | O(log n) rec | nth(i) index access, not BST descent |

### Chap43 — OrderedSetStPer.rs (8 MISMATCH-RENAME + 1 MATCH-DIFF-ALG)

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|---------------|-------|
| 24 | 43 | OrderedSetStPer.rs | first | MISMATCH-RENAME | same pattern as StEph |
| 25 | 43 | OrderedSetStPer.rs | last | MISMATCH-RENAME | same pattern as StEph |
| 26 | 43 | OrderedSetStPer.rs | previous | MISMATCH-RENAME | same pattern as StEph |
| 27 | 43 | OrderedSetStPer.rs | next | MISMATCH-RENAME | same pattern as StEph |
| 28 | 43 | OrderedSetStPer.rs | rank | MISMATCH-RENAME | same pattern as StEph |
| 29 | 43 | OrderedSetStPer.rs | split | MISMATCH-RENAME | same pattern as StEph |
| 30 | 43 | OrderedSetStPer.rs | get_range | MISMATCH-RENAME | same pattern as StEph |
| 31 | 43 | OrderedSetStPer.rs | split_rank | MISMATCH-RENAME | same pattern as StEph |
| 32 | 43 | OrderedSetStPer.rs | select | MATCH-DIFF-ALG | O(log n) via nth |

### Chap43 — OrderedTableStEph.rs (11 MISMATCH-RENAME + 1 MATCH-DIFF-ALG)

| # | Chap | File | Function | Classification | Our Complexity | Notes |
|---|------|------|----------|---------------|---------------|-------|
| 33 | 43 | OrderedTableStEph.rs | find | MISMATCH-RENAME | O(n) | linear scan for key match |
| 34 | 43 | OrderedTableStEph.rs | insert | MISMATCH-RENAME | O(n) | rebuild from vec |
| 35 | 43 | OrderedTableStEph.rs | delete | MISMATCH-RENAME | O(n) | filter rebuild |
| 36 | 43 | OrderedTableStEph.rs | first_key | MISMATCH-RENAME | O(n) | linear scan for min key |
| 37 | 43 | OrderedTableStEph.rs | last_key | MISMATCH-RENAME | O(n) | linear scan for max key |
| 38 | 43 | OrderedTableStEph.rs | previous_key | MISMATCH-RENAME | O(n) | linear scan for predecessor |
| 39 | 43 | OrderedTableStEph.rs | next_key | MISMATCH-RENAME | O(n) | linear scan for successor |
| 40 | 43 | OrderedTableStEph.rs | rank_key | MISMATCH-RENAME | O(n) | linear count |
| 41 | 43 | OrderedTableStEph.rs | split_key | MISMATCH-RENAME | O(n) | iterative partition |
| 42 | 43 | OrderedTableStEph.rs | get_key_range | MISMATCH-RENAME | O(n) | iterative scan |
| 43 | 43 | OrderedTableStEph.rs | split_rank_key | MISMATCH-RENAME | O(n) | iterative partition by rank |
| 44 | 43 | OrderedTableStEph.rs | select_key | MATCH-DIFF-ALG | O(log n) | nth-based index access |

### Chap43 — OrderedTableStPer.rs (11 MISMATCH-RENAME + 1 MATCH-DIFF-ALG)

| # | Chap | File | Function | Classification | Notes |
|---|------|------|----------|---------------|-------|
| 45 | 43 | OrderedTableStPer.rs | find | MISMATCH-RENAME | same pattern as StEph |
| 46 | 43 | OrderedTableStPer.rs | insert | MISMATCH-RENAME | same pattern as StEph |
| 47 | 43 | OrderedTableStPer.rs | delete | MISMATCH-RENAME | same pattern as StEph |
| 48 | 43 | OrderedTableStPer.rs | first_key | MISMATCH-RENAME | same pattern as StEph |
| 49 | 43 | OrderedTableStPer.rs | last_key | MISMATCH-RENAME | same pattern as StEph |
| 50 | 43 | OrderedTableStPer.rs | previous_key | MISMATCH-RENAME | same pattern as StEph |
| 51 | 43 | OrderedTableStPer.rs | next_key | MISMATCH-RENAME | same pattern as StEph |
| 52 | 43 | OrderedTableStPer.rs | rank_key | MISMATCH-RENAME | same pattern as StEph |
| 53 | 43 | OrderedTableStPer.rs | split_key | MISMATCH-RENAME | same pattern as StEph |
| 54 | 43 | OrderedTableStPer.rs | get_key_range | MISMATCH-RENAME | same pattern as StEph |
| 55 | 43 | OrderedTableStPer.rs | split_rank_key | MISMATCH-RENAME | same pattern as StEph |
| 56 | 43 | OrderedTableStPer.rs | select_key | MATCH-DIFF-ALG | O(log n) via nth |

### Reclassified from v1 (no longer mismatches)

| # | Chap | File | Function | v1 | v2 | Reason |
|---|------|------|----------|----|----|--------|
| - | 41 | AVLTreeSetStEph.rs | from_seq | MISMATCH | MATCH | Textbook sequential = iterate insert (Ch41 Ex 41.3) |
| - | 41 | AVLTreeSetStPer.rs | from_seq | (missing) | MATCH | Same as StEph |
| - | 43 | OrderedSetStEph.rs | select | MISMATCH | MATCH-DIFF-ALG | O(log n) via nth, matches textbook complexity |
| - | 43 | OrderedTableStEph.rs | select_key | MISMATCH | MATCH-DIFF-ALG | O(log n) via nth, matches textbook complexity |

## Delegation Analysis (MISMATCH-DELEGATION)

These files delegate to backing stores whose operations are themselves
iterative mismatches. No rename needed at these levels — the fix propagates
when the backing store is fixed.

### OrderedSetStEph/StPer base operations → AVLTreeSetStEph/StPer

The following OrderedSet operations are one-line delegations to AVLTreeSetStEph/StPer.
The mismatch lives in AVLTreeSetStEph/StPer, not here.

| Delegating File | Function | Delegates To |
|----------------|----------|-------------|
| OrderedSetStEph.rs | find | base_set.find() |
| OrderedSetStEph.rs | insert | base_set.insert() |
| OrderedSetStEph.rs | delete | base_set.delete() |
| OrderedSetStEph.rs | filter | base_set.filter() |
| OrderedSetStEph.rs | intersection | base_set.intersection() |
| OrderedSetStEph.rs | union | base_set.union() |
| OrderedSetStEph.rs | difference | base_set.difference() |
| OrderedSetStPer.rs | (same 7) | base_set.{same}() |

### AugOrderedTableStEph/StPer → OrderedTableStEph/StPer

All AugOrderedTable operations delegate to base_table. The mismatch is
in OrderedTableStEph/StPer.

| Delegating File | Delegates To | Functions |
|----------------|-------------|-----------|
| AugOrderedTableStEph.rs | base_table.{fn}() | all 11+ ordering ops |
| AugOrderedTableStPer.rs | base_table.{fn}() | all 11+ ordering ops |

### Mt variants → St variants (through RwLock)

All Mt variants acquire a read/write lock and delegate to the corresponding
St implementation.

| Mt File | Delegates To | Functions |
|---------|-------------|-----------|
| AVLTreeSetMtEph.rs | AVLTreeSetStEph.{fn}() | find, insert, delete, etc. |
| AVLTreeSetMtPer.rs | AVLTreeSetStPer.{fn}() | find, insert, delete, etc. |
| OrderedSetMtEph.rs | OrderedSetStEph.{fn}() | first, last, previous, etc. |
| OrderedTableMtEph.rs | OrderedTableStEph.{fn}() | first_key, last_key, etc. |
| OrderedTableMtPer.rs | OrderedTableStPer.{fn}() | first_key, last_key, etc. |
| AugOrderedTableMtEph.rs | AugOrderedTableStEph → OrderedTableStEph | double delegation |

### join/join_key → union

`join` and `join_key` are one-line wrappers around `union`:
```
fn join(&mut self, other: Self) { self.union(&other); }
fn join_key(&mut self, other: Self) { self.union(&other, |v1, _v2| v1.clone()); }
```
When union becomes recursive, join automatically becomes recursive.
No independent rename needed.

## Rename Plan (Phase 1)

Phase 1 renames the 50 MISMATCH-RENAME functions to `_iter`. The trait
keeps the default name and delegates to `_iter`. No new proofs needed.

### By file

| # | Chap | File | Functions to Rename | Count |
|---|------|------|-------------------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | find, insert, delete, filter, intersection, union, difference | 7 |
| 2 | 41 | AVLTreeSetStPer.rs | find, insert, delete, filter, intersection, union, difference | 7 |
| 3 | 43 | OrderedSetStEph.rs | first, last, previous, next, rank, split, get_range, split_rank | 8 |
| 4 | 43 | OrderedSetStPer.rs | first, last, previous, next, rank, split, get_range, split_rank | 8 |
| 5 | 43 | OrderedTableStEph.rs | find, insert, delete, first_key, last_key, previous_key, next_key, rank_key, split_key, get_key_range, split_rank_key | 11 |
| 6 | 43 | OrderedTableStPer.rs | find, insert, delete, first_key, last_key, previous_key, next_key, rank_key, split_key, get_key_range, split_rank_key | 11 |
| **Total** | | | | **52** |

### Rename mechanics

For each function `fn foo(...)`:

1. Rename the function body to `fn foo_iter(...)` (same ensures, same body).
2. The trait method `fn foo(...)` stays with the same signature.
3. The impl's `fn foo(...)` becomes a one-line delegation to `foo_iter(...)`.
4. Callers are unaffected — they call the trait method.
5. Run `scripts/validate.sh` after each file.

Example:
```rust
// In trait:
fn find(&self, x: &T) -> (found: B)
    requires self.spec_avltreesetsteph_wf(),
    ensures found == self@.contains(x@);

// In impl (after rename):
fn find_iter(&self, x: &T) -> (found: B)
    requires self.spec_avltreesetsteph_wf(),
    ensures found == self@.contains(x@),
{ /* existing while loop body */ }

fn find(&self, x: &T) -> (found: B)
{ self.find_iter(x) }
```

### Functions NOT renamed

| Function | Reason |
|----------|--------|
| from_seq | MATCH — textbook sequential variant is iterate insert |
| select / select_key | MATCH-DIFF-ALG — O(log n) via nth, same complexity |
| join / join_key | DELEGATION — wraps union, inherits fix automatically |
| to_seq | Not recursive in textbook (in-order traversal is linear) |

## StPer / Mt Variant Table

| Variant | Inherits From | Classification | Rename Needed? |
|---------|--------------|---------------|---------------|
| AVLTreeSetStPer | — | MISMATCH-RENAME (7 fns) | YES — own iterative code |
| AVLTreeSetMtEph | AVLTreeSetStEph | DELEGATION | NO — delegates |
| AVLTreeSetMtPer | AVLTreeSetStPer | DELEGATION | NO — delegates |
| OrderedSetStPer | AVLTreeSetStPer | MISMATCH-RENAME (8 ordering) + DELEGATION (7 base) | YES for ordering ops |
| OrderedSetMtEph | OrderedSetStEph | DELEGATION | NO — delegates |
| OrderedTableStPer | — | MISMATCH-RENAME (11 fns) | YES — own iterative code |
| OrderedTableMtEph | OrderedTableStEph | DELEGATION | NO — delegates |
| OrderedTableMtPer | OrderedTableStPer | DELEGATION | NO — delegates |
| AugOrderedTableStEph | OrderedTableStEph | DELEGATION | NO — delegates |
| AugOrderedTableStPer | OrderedTableStPer | DELEGATION | NO — delegates |
| AugOrderedTableMtEph | → StEph → OrdTable | DELEGATION | NO — double delegation |

## Priority Ordering

### Tier 1: Highest impact (wrong asymptotic complexity)

These functions have WORSE complexity than textbook, not just different style.

| # | Chap | File | Function | Our O() | Textbook O() | Impact |
|---|------|------|----------|---------|-------------|--------|
| 1 | 41 | AVLTreeSetStEph.rs | find | n log n | log n | Every find caller is O(n) too slow |
| 2 | 41 | AVLTreeSetStEph.rs | intersection | n^2 | m log(n/m) | Quadratic vs near-linear |
| 3 | 41 | AVLTreeSetStEph.rs | difference | n^2 | m log(n/m) | Quadratic vs near-linear |
| 4 | 41 | AVLTreeSetStEph.rs | insert | n | log n | Cascades to from_seq (n^2 vs n log n) |
| 5 | 41 | AVLTreeSetStEph.rs | delete | n | log n | Linear vs logarithmic |
| 6 | 43 | OrderedTableStEph.rs | find | n | log n | Table lookup should be O(log n) |
| 7 | 43 | OrderedTableStEph.rs | insert | n | log n | Table insert should be O(log n) |
| 8 | 43 | OrderedTableStEph.rs | delete | n | log n | Table delete should be O(log n) |

### Tier 2: High impact (O(n) where textbook is O(log n))

| # | Chap | File | Functions | Count |
|---|------|------|-----------|-------|
| 9 | 43 | OrderedSetStEph.rs | first, last, previous, next, rank | 5 |
| 10 | 43 | OrderedSetStPer.rs | first, last, previous, next, rank | 5 |
| 11 | 43 | OrderedTableStEph.rs | first_key through rank_key | 5 |
| 12 | 43 | OrderedTableStPer.rs | first_key through rank_key | 5 |
| 13 | 43 | all 4 files | split*, get*range, split_rank* | 12 |

### Tier 3: Style mismatch (same O(n) work, different structure)

| # | Chap | File | Functions | Count | Notes |
|---|------|------|-----------|-------|-------|
| 14 | 41 | AVLTreeSetStEph.rs | filter, union | 2 | O(n) matches textbook work; span differs |
| 15 | 41 | AVLTreeSetStPer.rs | filter, union | 2 | same |

### Execution order recommendation

1. **Rename Chap41 AVLTreeSetStEph (7 functions)** — root cause file.
   All Chap43 OrderedSet operations delegate base ops here.
2. **Rename Chap41 AVLTreeSetStPer (7 functions)** — persistent mirror.
3. **Rename Chap43 OrderedSetStEph ordering ops (8 functions)** — direct
   iterative implementations of ordering extensions.
4. **Rename Chap43 OrderedSetStPer ordering ops (8 functions)** — mirror.
5. **Rename Chap43 OrderedTableStEph (11 functions)** — independent file
   with its own iterative implementations.
6. **Rename Chap43 OrderedTableStPer (11 functions)** — mirror.

Each file can be renamed independently. Files 1-2 can be done in parallel
with 3-6. Validate after each file.

## Phase 2: Recursive Implementations (Future)

After Phase 1 renames, write recursive implementations under the default
names. The trait delegates to the recursive version. Callers who want
iterative can call `_iter` explicitly.

### Prerequisite: BST backing store

The current AVLTreeSetStEph/StPer wraps AVLTreeSeqStEph/StPer (index-ordered,
not a BST by value). Recursive tree-based implementations require either:

- **Option A**: Build on BSTTreapStEph (Ch39), which IS a proper BST with
  recursive split/join/expose. Ch38's parametric BST algorithms (insert,
  delete, union, intersection, difference, filter) are already implemented
  recursively there.
- **Option B**: Add value-based traversal methods to AVLTreeSeqStEph.
- **Option C**: New dedicated BST-by-value type for Ch41.

Option A is the path of least resistance — BSTTreapStEph already has verified
recursive implementations of most Ch38 algorithms.

### What the recursive implementations look like

Per the experiment at `src/experiments/trait_rec_vs_iter.rs`:
- Same trait, same spec
- Recursive impl = default name (e.g., `fn find(...)`)
- Iterative impl = `_iter` suffix (e.g., `fn find_iter(...)`)
- Both verified against the same spec function

## Completeness Check

### Files scanned

| File | In v1? | In v2? | Status |
|------|--------|--------|--------|
| src/Chap41/AVLTreeSetStEph.rs | YES (8) | YES (7) | from_seq reclassified as MATCH |
| src/Chap41/AVLTreeSetStPer.rs | YES (7) | YES (7) | + from_seq confirmed MATCH |
| src/Chap43/OrderedSetStEph.rs | YES (6) | YES (8+1) | + split, get_range, split_rank; select reclassified |
| src/Chap43/OrderedSetStPer.rs | NO | YES (8+1) | NEW — mirrors StEph |
| src/Chap43/OrderedTableStEph.rs | YES (6) | YES (11+1) | + find, insert, delete, split_key, get_key_range, split_rank_key; select_key reclassified |
| src/Chap43/OrderedTableStPer.rs | NO | YES (11+1) | NEW — mirrors StEph |
| src/Chap43/AugOrderedTableStEph.rs | NO | YES (delegation) | All ops delegate to OrderedTableStEph |
| src/Chap43/AugOrderedTableStPer.rs | NO | YES (delegation) | All ops delegate to OrderedTableStPer |
| src/Chap43/AugOrderedTableMtEph.rs | NO | YES (delegation) | Delegates to AugOrderedTableStEph |
| src/Chap41/AVLTreeSetMtEph.rs | NO | YES (delegation) | Delegates to AVLTreeSetStEph |
| src/Chap41/AVLTreeSetMtPer.rs | NO | YES (delegation) | Delegates to AVLTreeSetStPer |
| src/Chap43/OrderedSetMtEph.rs | NO | YES (delegation) | Delegates to OrderedSetStEph |
| src/Chap43/OrderedTableMtEph.rs | NO | YES (delegation) | Delegates to OrderedTableStEph |
| src/Chap43/OrderedTableMtPer.rs | NO | YES (delegation) | Delegates to OrderedTableStPer |

### Totals

| Category | v1 Count | v2 Count |
|----------|---------|---------|
| MISMATCH-RENAME | 27 | 50 |
| MATCH (reclassified) | 0 | 2 (from_seq × 2) |
| MATCH-DIFF-ALG | 0 | 4 (select × 4 files) |
| MISMATCH-DELEGATION files | 0 | 8 files |
| Files with direct iterative code | 4 | 6 |
