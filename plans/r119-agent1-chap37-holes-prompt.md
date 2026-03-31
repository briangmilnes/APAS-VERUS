# R119 Agent 1 — Fill Chap37 BST MtEph proof holes. AFK. DOT.

Restarted after R118 Chap37 compare work. This round: remove assumes.

## Problem

5 Chap37 MtEph files have RwLock-boundary assumes that veracity now counts
as proof holes (closure-assume detection fix). Each file has 7-10 assumes
bridging inner StEph ensures through the RwLock. Many are fixable.

## Files and assume counts

| # | Chap | File | Assumes | Hole count |
|---|------|------|---------|------------|
| 1 | 37 | BSTPlainMtEph.rs | 10 | 1 |
| 2 | 37 | BSTBBAlphaMtEph.rs | 10 | 1 |
| 3 | 37 | BSTAVLMtEph.rs | 7 | 1 |
| 4 | 37 | BSTRBMtEph.rs | 10 | 1 |
| 5 | 37 | BSTSplayMtEph.rs | 12 | 1 |

## Assume categories (same pattern across all 5 files)

### Category A — ghost_root bridging (1 per file)
```rust
proof { assume(self.ghost_root@ == tree); }
```
After acquiring the RwLock, the inner tree should equal `self.ghost_root@`.
This should follow from the RwLockPredicate invariant. Check if the inv
carries `ghost_root` and whether Verus can see through the lock acquisition
to prove equality.

### Category B — operation result bridging (2-4 per file)
```rust
assume(found == self@.tree_contains(*target));  // contains
assume(n as nat == self@.spec_size());           // size
assume(b == (self@ is Leaf));                    // is_empty
assume(h as nat == self@.spec_height());         // height
```
These bridge the inner StEph function's ensures to the outer MtEph ensures.
The inner call returns with ensures, then the result passes through the lock
boundary. If the outer MtEph ensures matches the inner StEph ensures, these
should be provable by chaining: inner ensures → result → outer ensures.

### Category C — find result (2 per file)
```rust
assume(found.is_some() == self@.tree_contains(*target));
assume(found.is_some() ==> found.unwrap() == *target);
```
Same pattern as B but for find's two-clause ensures.

### Category D — min/max result (BSTPlain/BSTBBAlpha only, 3 per file)
```rust
assume(self@.spec_size() == 0 ==> min.is_none());
assume(self@.spec_size() > 0 ==> min.is_some());
assume(min.is_some() ==> self@.tree_contains(min.unwrap()));
```

### Category E — insert structural (BSTRBMtEph/BSTSplayMtEph, 2-5 per file)
```rust
assume(spec_is_bst_link(ghost_link));           // BSTRB
assume(link_spec_size(new_root) <= old_size + 1); // size bound
assume(link_spec_size(new_root) <= usize::MAX);   // capacity
assume(link_contains(current, value));            // Splay
```
These are structural properties of the insert operation. Harder to remove —
they depend on the inner insert's ensures being strong enough.

### Category F — Splay clone (BSTSplayMtEph only, 2)
```rust
proof { assume(c == *link); }  // clone result
```
Clone bridging — the `clone()` ensures should cover this.

## Strategy

1. **Start with BSTPlainMtEph** — simplest tree, 10 assumes, good template.
2. For each assume, check:
   a. Does the RwLockPredicate inv carry the needed fact?
   b. Does the inner StEph function's ensures provide it?
   c. Can you chain inner ensures → lock release → outer ensures?
3. If yes to any, remove the assume and prove it.
4. If no, document exactly what's missing (e.g., "inv doesn't carry ghost_root",
   "inner ensures too weak").
5. Apply the same fixes to BSTBBAlphaMtEph (identical structure to Plain).
6. Then BSTAVLMtEph, BSTRBMtEph, BSTSplayMtEph.
7. Validate: `scripts/validate.sh isolate Chap37` after each file.

## Read first

- `src/Chap37/BSTPlainMtEph.rs` — full file
- `src/Chap37/BSTPlainStEph.rs` — the inner StEph trait ensures
- The RwLockPredicate struct (BSTPlainMtEphInv or similar)

## Rules

- Do NOT weaken any ensures.
- Do NOT convert assume to accept.
- If an assume can't be removed, leave it as assume and document why.
- Do NOT add new assumes.
- No subagents.

## STEP 30

## Report

Write `plans/agent1-r119-chap37-holes-report.md`. For each file, table of
assumes: line, content, status (removed/structural), reason if structural.
