# Iterative vs Recursive: Execution Plan

Supersedes the old rename plan. Inventory: `iterative-vs-recursive-inventory-v2.md`.
Standard: `src/standards/iterative_vs_recursive_standard.rs` (standard #23).

## Problem

52 functions across 6 files are iterative where APAS says recursive. All are O(n) or
worse where the textbook is O(log n). Root cause: AVLTreeSetStEph/StPer wraps
AVLTreeSeqStEph (index-ordered AVL), not a BST by value. OrderedTableStEph/StPer wraps
AVLTreeSeqStEph directly for key-value pairs.

## Architecture Decision

**BSTTreapStEph (Ch39) is the target backing store.** It has verified recursive split,
join, expose, insert, delete, find, union, intersection, difference, filter — the full
Ch38 parametric BST interface. Changing the backing store is a prerequisite for O(log n)
recursive implementations.

**Tension: _iter preservation.** Renaming current iterative bodies to `_iter` is clean
and mechanical. But when the backing store changes from AVLTreeSeqStEph to BSTTreapStEph,
those _iter bodies (which call AVLTreeSeqStEph operations) break. Options:

1. **Rename first, rewrite _iter later.** Phase 1 renames are safe and establish the
   naming convention. When the backing store changes in Phase 2, the _iter bodies get
   rewritten to iterate over BSTTreap's structure. Two touches per function.
2. **Rename + backing store in one pass.** Rename to _iter, change backing store, rewrite
   _iter body, write recursive body — all at once. One touch per function, higher risk.
3. **Skip _iter for now.** Change backing store, write recursive defaults. Old iterative
   code is deleted. _iter variants can be written later on BSTTreap if desired.

**Decision: Option 1.** Phase 1 renames are low-risk and give us a clean validated
baseline with the naming convention established in traits. Phase 2 changes the backing
store and rewrites/adds bodies. Two touches is fine — the first touch is mechanical.

## Phases

### Tier 0 — Phase 1 Renames (1 agent, 1 round)

Mechanical. No new proofs. No backing store change. For each of the 52 functions:
1. Add `fn foo_iter(...)` to the trait with the same spec as `fn foo(...)`.
2. Rename the impl body to `fn foo_iter(...)`.
3. The impl's `fn foo(...)` becomes: `self.foo_iter(x)`.
4. Callers unchanged — they use the trait method.
5. Validate after each file.

| # | Chap | File | Count | Functions |
|---|------|------|-------|-----------|
| 1 | 41 | AVLTreeSetStEph.rs | 7 | find, insert, delete, filter, intersection, union, difference |
| 2 | 41 | AVLTreeSetStPer.rs | 7 | find, insert, delete, filter, intersection, union, difference |
| 3 | 43 | OrderedSetStEph.rs | 8 | first, last, previous, next, rank, split, get_range, split_rank |
| 4 | 43 | OrderedSetStPer.rs | 8 | first, last, previous, next, rank, split, get_range, split_rank |
| 5 | 43 | OrderedTableStEph.rs | 11 | find, insert, delete, first_key, last_key, previous_key, next_key, rank_key, split_key, get_key_range, split_rank_key |
| 6 | 43 | OrderedTableStPer.rs | 11 | find, insert, delete, first_key, last_key, previous_key, next_key, rank_key, split_key, get_key_range, split_rank_key |
| | | **Total** | **52** | |

NOT renamed (per inventory v2):
- `from_seq` — MATCH (textbook is iterate insert)
- `select` / `select_key` — MATCH-DIFF-ALG (O(log n) via nth)
- `join` / `join_key` — delegation to union, inherits fix
- `to_seq` — not recursive in textbook

Order: file 1 first (root cause), then 2-6 in any order. Each file is independent.

### Tier 0b — Backing Store Rewire (1 agent, 1-2 rounds)

Change the backing store from AVLTreeSeqStEph to BSTTreapStEph. This is architectural
work that touches 4 files (2 Chap41 + 2 Chap43 table files):

**Chap41 AVLTreeSetStEph/StPer:**
- Field: `pub elements: AVLTreeSeqStEphS<T>` → `pub tree: BSTTreapStEph<T>`
- View: derive Set from BSTTreap's in-order traversal (already `spec_in_order`)
- wf: derive from `spec_bsttreapsteph_wf`
- Rewrite all _iter bodies to use BSTTreap operations
- Sortedness: BSTTreap guarantees sorted in-order traversal (BST property)

**Chap43 OrderedTableStEph/StPer:**
- Field: `pub base_seq: AVLTreeSeqStEphS<Pair<K, V>>` → needs BSTTreap or similar
- BSTTreapStEph is parameterized on T, needs Pair<K, V> with Ord on K
- Rewrite all _iter bodies

**Chap43 OrderedSetStEph/StPer:**
- Wraps AVLTreeSetStEph — if AVLTreeSet's TRAIT is stable, these need no change
  for the backing store rewire. The base_set delegation still works.
- Ordering ops (first, last, etc.) may need access to the tree structure.

**StPer concern:** BSTTreapStEph uses `Box<Node<T>>` (no structural sharing).
StPer semantics need persistent operations (return new value, don't mutate). Options:
- Clone the treap on each persistent operation (O(n) per op — worse than now)
- Use Arc<Node<T>> for structural sharing in a persistent BSTTreap variant
- Defer StPer to later; focus on StEph first

**Recommendation:** Tier 0b focuses on StEph files only (3 files: AVLTreeSetStEph,
OrderedTableStEph, OrderedSetStEph ordering ops). StPer is a follow-up.

### Tiers 1-3 — Recursive Implementations (3 agents, parallel, 2-3 rounds)

After Tier 0b lands on main, write recursive implementations under the default names.
Each agent works on different files. No cross-agent dependency.

| Tier | Agent | Files | Functions | What |
|------|-------|-------|-----------|------|
| 1 | A | AVLTreeSetStEph.rs | 7 | Recursive find/insert/delete/filter/intersection/union/difference via BSTTreap split/join/expose |
| 2 | B | OrderedSetStEph.rs | 8 | Recursive first (leftmost), last (rightmost), prev/next (predecessor/successor), rank (size-augmented descent), split, get_range, split_rank |
| 3 | C | OrderedTableStEph.rs | 11 | Same patterns as Tier 2 but on key-value pairs |

StPer mirrors follow after StEph is proven.

### Dependency Graph

```
Tier 0  (Phase 1 renames, 52 fns)
   │
   ▼
Tier 0b (backing store rewire, StEph only)
   │
   ├──▶ Tier 1 (AVLTreeSetStEph, 7 rec)
   ├──▶ Tier 2 (OrderedSetStEph, 8 rec)
   └──▶ Tier 3 (OrderedTableStEph, 11 rec)
            │
            ▼
        StPer mirrors (14 + 8 + 11 = 33 more)
```

## Totals

| Phase | Functions | New code? | Risk |
|-------|-----------|-----------|------|
| Tier 0 (renames) | 52 | No — mechanical rename + delegation | Low |
| Tier 0b (backing store) | 0 new, ~30 rewritten | Yes — field types, View, wf, _iter bodies | Medium |
| Tiers 1-3 (recursive StEph) | 26 new | Yes — new recursive algorithms + proofs | High |
| StPer mirrors | 33 new + ~30 rewritten | Yes | Medium |
| **Grand total** | 52 renamed + 26 recursive + 33 StPer | | |

## Files NOT Touched

- AVLTreeSetMtEph/MtPer — delegation to St, inherits fix
- OrderedSetMtEph — delegation to StEph, inherits fix
- OrderedTableMtEph/MtPer — delegation to St, inherits fix
- AugOrderedTableStEph/StPer/MtEph — delegation to OrderedTable, inherits fix
- AVLTreeSeqStEph/StPer — old backing store, kept as-is for other users
