# Agent 3 — Round 10 Prompt

## Mission

Prove Chap38 parallel BST (36 holes) and Chap39 treap variants (26 holes). Both have
clean or internal deps — these are directly attackable.

## Your Files (no other agent touches these)

**Chap38** (36 holes across 2 files):
- `BSTParaStEph.rs` — parallel BST operations (split, join, union, intersection, difference)
- `BSTParaMtEph.rs` — multi-threaded version

**Chap39** (26 holes across 4 files):
- `BSTTreapMtEph.rs` — 8 holes
- `BSTParaTreapMtEph.rs` — 15 holes
- `BSTSetTreapMtEph.rs` — 3 holes (you proved 10 in Round 9)
- `BSTTreapStEph.rs` — 0 (already clean)

## Priority Order

1. **BSTParaStEph.rs** — 36 holes, clean deps. This is your primary target.
2. **BSTSetTreapMtEph.rs** — only 3 remaining, you have deep context.
3. **BSTTreapMtEph.rs** — 8 holes.
4. **BSTParaTreapMtEph.rs** — 15 holes, likely shares patterns with BSTParaStEph.
5. **BSTParaMtEph.rs** — Mt wrapper of BSTParaStEph.

## Specific Guidance

### BSTParaStEph.rs (36 holes) — ROOT CAUSE FIX

In Round 9 you identified: "Previous agent removed `pub ghost contents: Set<T::V>` from
the RwLock predicate, breaking the link↔set bridge. All 19 holes require restoring this."

**Action**: Restore `pub ghost contents: Set<T::V>` to the RwLock predicate. This is the
structural fix that unlocks proofs for the entire file. Do it first, validate, then prove.

After restoring the ghost field:
1. The RwLock predicate should relate the tree structure to the ghost set.
2. `spec_set_view` can return the ghost set instead of being external_body.
3. insert/delete/find can update/read the ghost set with proves.
4. union/intersection/difference can be proved using set algebra on the ghost sets.

Check what the RwLock predicate looks like in similar Mt files (e.g., Chap41 AVLTreeSetMtEph
or Chap52 AdjSeqGraphMtEph) for the pattern.

### BSTSetTreapMtEph.rs (3 remaining)

You identified in Round 9: "singleton/insert/delete need `old(self)@` semantics through
`&self` interior mutability — architecturally blocked by `ParamTreap::view` being
`external_body` returning `Set::empty()`."

**Try**: Strengthen `ParamTreap::view` spec. It currently returns `Set::empty()` which is
useless. If the ParamTreap has a ghost contents field (or you can add one to its RwLock
predicate), make `view()` return that. This unblocks all 3 remaining holes.

### BSTTreapMtEph.rs (8 holes)

Read the clean `BSTTreapStEph.rs` for algorithmic patterns. The Mt version wraps with
threading. Apply the same external_body spec strengthening technique you used in Round 9.

### BSTParaTreapMtEph.rs (15 holes)

Parallel operations on treaps. Similar to BSTParaStEph but using treap balancing.
If BSTParaStEph is proved first, the parallel treap patterns should follow.

### BSTParaMtEph.rs

Mt wrapper of BSTParaStEph. If StEph is proved, the Mt version needs only thread-boundary
external_body and ghost state propagation.

## Techniques from Round 9

You used these successfully — apply them again:
- **External_body spec strengthening**: Safe because body isn't checked. Add ensures that
  match the algorithmic semantics.
- **Congruence-based clone proofs**: view equality flows through to_set().
- **Subset-based reasoning**: For recursive min/max.

## Rules

- Read `src/standards/using_closures_standard.rs` for closure requires patterns.
- Read `src/standards/rwlock_standard.rs` for RwLock predicate patterns.
- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversions.
- When modifying trait signatures (adding requires/ensures), UPDATE ALL CALLERS.
- Push to `agent3/ready`.
- Write `plans/agent3-round10-report.md`.

## Targets

- Chap38: ≤ 26 holes (-10)
- Chap39: ≤ 18 holes (-8)
