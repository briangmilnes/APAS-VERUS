# Agent 2 — Round 23 Report

## Mission

Strengthen helper function specs in Chap40's 3 augmented BST files and prove
the 14 `external_body` holes.

## Results

| # | Chap | File | Holes Before | Holes After | Proven | Technique |
|---|------|------|:------------:|:-----------:|:------:|-----------
| 1 | 40 | BSTSizeStEph.rs | 4 | 3 | 1 | Content ensures + cmp bridge |
| 2 | 40 | BSTKeyValueStEph.rs | 5 | 5 | 0 | Blocked: BST ordering invariant |
| 3 | 40 | BSTReducedStEph.rs | 5 | 5 | 0 | Blocked: BST ordering invariant |
| | | **Total** | **14** | **13** | **1** | |

## Verification

- validate: 3977 verified, 0 errors
- RTT: 2600 passed
- PTT: 147 passed

## Proven Hole: Chap40 BSTSizeStEph::insert

Removed `external_body` from `insert()` in `src/Chap40/BSTSizeStEph.rs`.

**Proof strategy**: Bottom-up content tracking through the helper chain.

### Step 1: Strengthen make_node ensures

Added `Lnk::spec_content_link(&node) == Lnk::spec_content_link(&left).union(Lnk::spec_content_link(&right)).insert(key)` to the trait declaration. Verified directly from construction.

### Step 2: Prove rotation content preservation

Added `Lnk::spec_content_link(link) == Lnk::spec_content_link(old(link))` to
`rotate_left` and `rotate_right` trait ensures.

**Key technique**: Pre-move content capture pattern. Rotation moves a node via
`*link = Some(y)`. After the move, Verus can't track content through the moved
value. Solution: capture ghost content variables and assert equality BEFORE the
move, letting Verus connect pre-move state to post-move ensures.

```rust
let ghost y_left_new_content = Lnk::spec_content_link(&y.left);
assert(y_left_new_content =~= x_left_content.union(y_left_content).insert(x_key));
let ghost pre_move_content = y_left_new_content.union(y_right_content).insert(y_key);
assert(pre_move_content =~= old_content);
*link = Some(y);
```

### Step 3: Bridge structural-vs-runtime equality via OrdSpec

Added `T::obeys_cmp_spec()` and `forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b)` as requires to `insert_link` and `insert`.

Restructured `insert_link` body from `if value < node.key { ... } else if value > node.key { ... } else { ... }` to `match value.cmp(&node.key) { Less => ..., Greater => ..., Equal => ... }`. In the Equal arm, Verus knows `value.cmp_spec(&node.key) == Equal`, and the requires gives `value == node.key` (structural). Then `old_content.insert(value) == old_content` since value is already in the set.

### Step 4: Remove external_body from insert

Top-level `insert` directly delegates to `insert_link`. All four postconditions
(content, wf, size bounds) follow from `insert_link`'s strengthened ensures.

## Spec Strengthening Summary (BSTSizeStEph)

| Function | Ensures Added |
|----------|--------------|
| make_node | Content: `spec_content_link == left.union(right).insert(key)` |
| rotate_left | Content preservation: `spec_content_link(link) == spec_content_link(old(link))` |
| rotate_right | Content preservation: same |
| insert_link | Content: `spec_content_link(link) == spec_content_link(old(link)).insert(value)` |
| find_link | Soundness: `found is Some ==> spec_content_link(link).contains(*found.unwrap())`, `found is Some ==> *found.unwrap() == *target` |

Requires added to insert_link, find_link, insert, find, contains:
- `T::obeys_cmp_spec()`
- `forall |a: T, b: T| a.cmp_spec(&b) == std::cmp::Ordering::Equal ==> (a == b)`

## Blocker Analysis

### 1. BST Ordering Invariant (all 3 files, all remaining holes)

The single root cause blocking all 13 remaining holes is the absence of a BST
ordering spec. Currently `spec_bstsizesteph_wf` only tracks size well-formedness.
There is no spec stating that all left-subtree keys are less than node.key and all
right-subtree keys are greater.

**Why it matters for each hole type:**

- **find/contains** (all 3 files): The `<==>` postcondition requires completeness —
  if an element is in the set/map, find WILL return it. This requires proving the BST
  search visits the correct subtree, which requires ordering.

- **insert** (BSTKeyValueStEph, BSTReducedStEph): These use `Map<K,V>` with
  `union_prefer_right` for `spec_content_link`. Unlike `Set<T>` where union is
  commutative, `union_prefer_right` is NOT commutative — it gives different results
  depending on which map is "preferred." Rotation changes the union order
  (before: `a.union(b.union(c).insert(y)).insert(x)`, after:
  `a.union(b).insert(x).union(c).insert(y)`). These are only equal when keys don't
  overlap between subtrees — exactly what BST ordering guarantees.

- **delete** (all 3 files): Uses collect→filter→rebuild chain. Proving
  `self@ == old(self)@.remove(key)` requires content specs on the entire chain.

**Fix**: Define `spec_ordered_link` using `cmp_spec` for the ordering predicate.
Add it to wf specs. Prove preservation through insert and rotation. This single
addition would unblock find/contains across all 3 files and insert in the Map files.

### 2. Set-based content tracking works; Map-based does not (without ordering)

For BSTSizeStEph (`Set<T>`), set union is commutative and idempotent, so rotation
trivially preserves set content. The insert proof needed only the eq bridge (solved
with `cmp_spec`). For the Map files, rotation genuinely changes `union_prefer_right`
output when keys overlap — which they don't in a valid BST, but proving this requires
the ordering invariant from blocker #1.

### 3. TotalOrder already solves the eq bridge for Map files

BSTKeyValueStEph uses `TotalOrder::cmp` which directly ensures `Ordering::Equal ==> self == other` (structural equality). No additional `obeys_cmp_spec` machinery needed.
BSTReducedStEph uses plain `<`/`>` operators and would need either TotalOrder or the
OrdSpec approach used for BSTSizeStEph.
