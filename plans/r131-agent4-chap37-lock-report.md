# R131 Agent 4 — Chap37/39 Lock-Boundary Accept Assessment

## 1. Per-File Assume Inventory

### 1a. BSTPlainMtEph (Chap37, 817 lines)

**Struct**: `BSTPlainMtEph<T>` — fields: `root: RwLock<BalBinTree<T>, BSTPlainMtEphInv<T>>`, `ghost_root: Ghost<BalBinTree<T>>`

**Pattern**: `ghost_root` (BalBinTree ghost shadow)

All assumes are bare `assume()`, not `accept()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 649 | `assume(self.ghost_root@ == tree)` | Writer | `insert` |
| 2 | 671 | `assume(self.ghost_root@ == tree)` | Writer | `delete` |
| 3 | 684 | `assume(found == self@.tree_contains(*target))` | Reader | `contains` |
| 4 | 695 | `assume(n as nat == self@.spec_size())` | Reader | `size` |
| 5 | 705 | `assume(b == (self@ is Leaf))` | Predicate | `is_empty` |
| 6 | 716 | `assume(h as nat == self@.spec_height())` | Reader | `height` |
| 7 | 726 | `assume(found.is_some() == self@.tree_contains(*target))` | Reader | `find` |
| 8 | 727 | `assume(found.is_some() ==> found.unwrap() == *target)` | Reader | `find` |
| 9 | 738 | `assume(self@.spec_size() == 0 ==> min.is_none())` | Reader | `minimum` |
| 10 | 739 | `assume(self@.spec_size() > 0 ==> min.is_some())` | Reader | `minimum` |
| 11 | 740 | `assume(min.is_some() ==> self@.tree_contains(min.unwrap()))` | Reader | `minimum` |
| 12 | 751 | `assume(self@.spec_size() == 0 ==> max.is_none())` | Reader | `maximum` |
| 13 | 752 | `assume(self@.spec_size() > 0 ==> max.is_some())` | Reader | `maximum` |
| 14 | 753 | `assume(max.is_some() ==> self@.tree_contains(max.unwrap()))` | Reader | `maximum` |

**14 assumes** across 10 operations (2 writer, 1 predicate, 11 reader).

---

### 1b. BSTAVLMtEph (Chap37, 1073 lines)

**Struct**: `BSTAVLMtEph<T>` — fields: `root: RwLock<BalBinTree<T>, BSTAVLMtEphInv<T>>`, `ghost_root: Ghost<BalBinTree<T>>`

**Pattern**: `ghost_root` (BalBinTree ghost shadow). No delete operation.

All assumes are bare `assume()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 925 | `assume(self.ghost_root@ == tree)` | Writer | `insert` |
| 2 | 950 | `assume(found == self@.tree_contains(*target))` | Reader | `contains` |
| 3 | 961 | `assume(n as nat == self@.spec_size())` | Reader | `size` |
| 4 | 971 | `assume(b == (self@ is Leaf))` | Predicate | `is_empty` |
| 5 | 982 | `assume(h as nat == self@.spec_height())` | Reader | `height` |
| 6 | 992 | `assume(found.is_some() == self@.tree_contains(*target))` | Reader | `find` |
| 7 | 993 | `assume(found.is_some() ==> found.unwrap() == *target)` | Reader | `find` |

**7 assumes** across 7 operations (1 writer, 1 predicate, 5 reader). `minimum` and `maximum` have NO assumes (they don't bridge to ghost).

---

### 1c. BSTBBAlphaMtEph (Chap37, 817 lines)

**Struct**: `BSTBBAlphaMtEph<T>` — fields: `root: RwLock<BalBinTree<T>, BSTBBAlphaMtEphInv<T>>`, `ghost_root: Ghost<BalBinTree<T>>`

**Pattern**: `ghost_root` (BalBinTree ghost shadow). Identical structure to BSTPlainMtEph.

All assumes are bare `assume()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 649 | `assume(self.ghost_root@ == tree)` | Writer | `insert` |
| 2 | 671 | `assume(self.ghost_root@ == tree)` | Writer | `delete` |
| 3 | 684 | `assume(found == self@.tree_contains(*target))` | Reader | `contains` |
| 4 | 695 | `assume(n as nat == self@.spec_size())` | Reader | `size` |
| 5 | 705 | `assume(b == (self@ is Leaf))` | Predicate | `is_empty` |
| 6 | 716 | `assume(h as nat == self@.spec_height())` | Reader | `height` |
| 7 | 726 | `assume(found.is_some() == self@.tree_contains(*target))` | Reader | `find` |
| 8 | 727 | `assume(found.is_some() ==> found.unwrap() == *target)` | Reader | `find` |
| 9 | 738 | `assume(self@.spec_size() == 0 ==> min.is_none())` | Reader | `minimum` |
| 10 | 739 | `assume(self@.spec_size() > 0 ==> min.is_some())` | Reader | `minimum` |
| 11 | 740 | `assume(min.is_some() ==> self@.tree_contains(min.unwrap()))` | Reader | `minimum` |
| 12 | 751 | `assume(self@.spec_size() == 0 ==> max.is_none())` | Reader | `maximum` |
| 13 | 752 | `assume(self@.spec_size() > 0 ==> max.is_some())` | Reader | `maximum` |
| 14 | 753 | `assume(max.is_some() ==> self@.tree_contains(max.unwrap()))` | Reader | `maximum` |

**14 assumes** across 10 operations (2 writer, 1 predicate, 11 reader).

---

### 1d. BSTRBMtEph (Chap37, 1380 lines)

**Struct**: `BSTRBMtEph<T>` — fields: `root: RwLock<Link<T>, BSTRBMtEphInv>`, `ghost_root: Ghost<Link<T>>`

**Pattern**: `ghost_root` (Link ghost shadow). View maps through `link_to_bbt()`.

All assumes are bare `assume()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 1122 | `assume(spec_is_bst_link(ghost_link))` | Constructor | `from_sorted_slice` |
| 2 | 1133 | `assume(self.ghost_root@ == current)` | Writer | `insert` |
| 3 | 1188 | `assume(found == self@.tree_contains(*target))` | Reader | `contains` |
| 4 | 1199 | `assume(n as nat == self@.spec_size())` | Reader | `size` |
| 5 | 1208 | `assume(b == (self@.spec_size() == 0))` | Predicate | `is_empty` |
| 6 | 1223 | `assume(h as nat == self@.spec_height())` | Reader | `height` |
| 7 | 1234 | `assume(found.is_some() == self@.tree_contains(*target))` | Reader | `find` |
| 8 | 1235 | `assume(found.is_some() ==> found.unwrap() == *target)` | Reader | `find` |

**8 assumes** across 7 operations (1 constructor, 1 writer, 1 predicate, 5 reader). Note: the `from_sorted_slice` assume (#1) is NOT a lock-boundary assume — it's a proof gap where `build_balanced` doesn't prove BST ordering.

---

### 1e. BSTSplayMtEph (Chap37, 2140 lines)

**Struct**: `BSTSplayMtEph<T>` — fields: `root: RwLock<Link<T>, BSTSplayMtEphInv>`, `ghost_root: Ghost<Link<T>>`

**Pattern**: `ghost_root` (Link ghost shadow). View is the Link directly.

All assumes are bare `assume()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 1537 | `assume(c == *link)` | Clone helper | `clone_link` |
| 2 | 1927 | `assume(self.ghost_root@ == current)` | Writer | `insert` |
| 3 | 1932 | `assume(link_node_count(current) <= usize::MAX as nat)` | Writer | `insert` |
| 4 | 1933 | `assume(link_spec_size(current) <= link_spec_size(old(self)@) + 1)` | Writer | `insert` |
| 5 | 1949 | `assume(found == link_contains(self@, *target))` | Reader | `contains` |
| 6 | 1958 | `assume(n as nat == link_spec_size(self@))` | Reader | `size` |
| 7 | 1967 | `assume(b == (self@ is None))` | Predicate | `is_empty` |
| 8 | 1980 | `assume(h as nat == link_height(self@))` | Reader | `height` |
| 9 | 1989 | `assume(found.is_some() <==> link_contains(self@, *target))` | Reader | `find` |
| 10 | 1990 | `assume(found.is_some() ==> found.unwrap() == *target)` | Reader | `find` |
| 11 | 2000 | `assume(link_spec_size(self@) > 0 ==> min.is_some())` | Reader | `minimum` |
| 12 | 2001 | `assume(min.is_some() ==> link_contains(self@, min.unwrap()))` | Reader | `minimum` |
| 13 | 2002 | `assume(min.is_some() ==> forall\|x\| ... T::le(min.unwrap(), x))` | Reader | `minimum` |
| 14 | 2013 | `assume(link_spec_size(self@) > 0 ==> max.is_some())` | Reader | `maximum` |
| 15 | 2014 | `assume(max.is_some() ==> link_contains(self@, max.unwrap()))` | Reader | `maximum` |
| 16 | 2015 | `assume(max.is_some() ==> forall\|x\| ... T::le(x, max.unwrap()))` | Reader | `maximum` |
| 17 | 2079 | `assume(r == *self)` | Clone | `Node::clone` |

**17 assumes** across 10 operations + 2 clone helpers. Note: assumes #1, #3, #4, #17 are NOT lock-boundary assumes — #1/#17 are clone proof gaps, #3/#4 are insert postcondition gaps.

---

### 1f. BSTTreapMtEph (Chap39, 1566 lines)

**Struct**: `BSTTreapMtEph<T>` — fields: `locked_root: RwLock<Link<T>, BSTTreapMtEphInv>`, `ghost_locked_root: Ghost<Set<T::V>>`

**Pattern**: `ghost_locked_root` (Set ghost shadow, NOT tree-shaped). View is `Set<T::V>`, not the tree structure. This is a fundamentally different design from the Chap37 files — the ghost tracks a set abstraction, not the concrete tree.

All assumes are bare `assume()`.

| # | Line | Assume text | Category | Used by |
|---|------|-------------|----------|---------|
| 1 | 562 | `assume(c == *link)` | Clone helper | `clone_link` |
| 2 | 575 | `assume(c == *link)` | Clone helper | `clone_link` |
| 3 | 1358 | `assume(result.is_some() <==> self@.contains(target@))` | Reader | `find` |
| 4 | 1359 | `assume(result.is_some() ==> result.unwrap()@ == target@)` | Reader | `find` |
| 5 | 1376 | `assume(result as nat == self@.len())` | Reader | `size` |
| 6 | 1401 | `assume(result.is_some() ==> self@.contains(result.unwrap()@))` | Reader | `minimum` |
| 7 | 1411 | `assume(result.is_some() ==> self@.contains(result.unwrap()@))` | Reader | `maximum` |
| 8 | 1423 | `assume(ordered@.len() == self@.len())` | Reader | `in_order` |
| 9 | 1435 | `assume(preordered@.len() == self@.len())` | Reader | `pre_order` |
| 10 | 1452 | `assume(Lnk::spec_link_size_wf(&self.left))` | Clone | `Node::clone` |
| 11 | 1453 | `assume(Lnk::spec_link_size_wf(&self.right))` | Clone | `Node::clone` |
| 12 | 1462 | `assume(cloned == *self)` | Clone | `Node::clone` |
| 13 | 1475 | `assume(spec_bsttreapmteph_link_wf(&inner_clone))` | Clone | `BSTTreapMtEph::clone` |
| 14 | 1476 | `assume(self.ghost_locked_root@.finite())` | Clone | `BSTTreapMtEph::clone` |
| 15 | 1482 | `assume(cloned@ == self@)` | Clone | `BSTTreapMtEph::clone` |

**15 assumes** across 7 operations + 3 clone functions.

Notable: `insert` and `delete` have **NO assumes** — they update `ghost_locked_root` optimistically without reading the locked value. This is a different (weaker) approach: the ghost tracks the *intended* set, not the actual tree structure.

---

## 2. Summary

| # | Chap | File | Lines | Assumes | Lock-boundary | Non-lock | Ghost field | Pattern |
|---|------|------|-------|---------|---------------|----------|-------------|---------|
| 1 | 37 | BSTPlainMtEph.rs | 817 | 14 | 14 | 0 | `ghost_root: Ghost<BalBinTree<T>>` | ghost_root |
| 2 | 37 | BSTAVLMtEph.rs | 1073 | 7 | 7 | 0 | `ghost_root: Ghost<BalBinTree<T>>` | ghost_root |
| 3 | 37 | BSTBBAlphaMtEph.rs | 817 | 14 | 14 | 0 | `ghost_root: Ghost<BalBinTree<T>>` | ghost_root |
| 4 | 37 | BSTRBMtEph.rs | 1380 | 8 | 7 | 1 | `ghost_root: Ghost<Link<T>>` | ghost_root |
| 5 | 37 | BSTSplayMtEph.rs | 2140 | 17 | 12 | 5 | `ghost_root: Ghost<Link<T>>` | ghost_root |
| 6 | 39 | BSTTreapMtEph.rs | 1566 | 15 | 9 | 6 | `ghost_locked_root: Ghost<Set<T::V>>` | ghost_locked_root |
| | | **Total** | 7793 | **75** | **63** | **12** | | |

Non-lock assumes breakdown:
- RB #1: `build_balanced` doesn't prove BST ordering (constructor proof gap).
- Splay #1, #17: clone proof gaps (clone_link, Node::clone).
- Splay #3, #4: insert postcondition proof gaps (size/count bounds after splay).
- Treap #1, #2, #10, #11, #12, #13, #14, #15: clone proof gaps (6 assumes in clone functions).

---

## 3. BSTPlainMtEph TSM Migration Sketch

### Current Architecture (14 assumes)

```
BSTPlainMtEph<T> {
    root: RwLock<BalBinTree<T>, BSTPlainMtEphInv<T>>,
    ghost_root: Ghost<BalBinTree<T>>,    // ghost shadow — stale between operations
}
```

Writer ops: acquire_write → `assume(ghost == inner)` → mutate → update ghost → release.
Reader ops: acquire_read → compute → `assume(result == f(ghost))` → release.

### TSM Approach (from bst_plain_mt_tsm experiment)

The TSM experiment eliminates assumes by putting a ghost token INSIDE the lock alongside
the data. The RwLockPredicate ties the token to the concrete state. After acquire, the
predicate PROVES equality — no assume needed.

**What changes on the struct:**

```rust
// State machine (outside verus!)
tokenized_state_machine!(BSTPlainSM {
    fields {
        #[sharding(variable)]
        pub size: nat,          // tracks tree size
    }
    // transitions for insert (size+1), delete (size-1), noop
});

// Lock interior: tree + ghost token
pub struct BSTPlainLockInterior<T: TotalOrder> {
    pub tree: BalBinTree<T>,
    pub ghost_token: Tracked<BSTPlainSM::size>,
}

// Predicate: token.size == tree.spec_size() && tree.tree_is_bst()
pub struct BSTPlainMtEphInv<T> { ... }
impl RwLockPredicate<BSTPlainLockInterior<T>> for BSTPlainMtEphInv<T> {
    open spec fn inv(self, interior: BSTPlainLockInterior<T>) -> bool {
        interior.tree.tree_is_bst()
        && interior.tree.spec_size() <= usize::MAX
        && interior.tree.spec_height() <= usize::MAX
        && interior.ghost_token@.value() == interior.tree.spec_size()
        && interior.ghost_token@.instance_id() == self.instance.id()
    }
}

// Outer struct: lock + instance (no ghost shadow!)
pub struct BSTPlainMtEph<T: TotalOrder> {
    pub lock: RwLock<BSTPlainLockInterior<T>, BSTPlainMtEphInv<T>>,
    pub inst: Tracked<BSTPlainSM::Instance>,
}
```

**Operation pattern:**

Writers: acquire_write → predicate proves tree state → mutate tree → step TSM transition → release. Zero assumes.

Readers: acquire_read → borrow interior → predicate proves tree state → compute → release. Zero assumes.

**Which operations need transitions vs just reads:**
- `insert`: transition `tr_insert` (size += 1) or `tr_noop` (duplicate)
- `delete`: transition `tr_delete` (size -= 1) or `tr_noop` (missing key)
- `contains`, `size`, `is_empty`, `height`, `find`, `minimum`, `maximum`, `in_order`, `pre_order`: read only, no transition

**View problem:**

The TSM approach gives zero assumes but **no View on the outer struct**. The experiment
confirms this: `SetMtTsm` has no `impl View`. Specs only appear on return values
(`ensures n == ...`) not on self (`self@ == ...`).

The current BSTPlainMtEph has `View<V = BalBinTree<T>>` which lets callers write
`self@.tree_contains(x)` in their specs. BSTSetPlainMtEph wraps it and uses
`spec_bstplainmteph_wf()` — it does NOT reference `self@` of the inner BSTPlainMtEph
in its own specs. So the View is used only in BSTPlainMtEph's own trait specs (ensures
clauses that say `self@.tree_contains(value)` etc).

**Can we keep View with TSM?** Only if we have a ghost field outside the lock that tracks
the tree. But then we need to prove ghost == inner on acquire — exactly the assume we're
eliminating. The PCell experiment (Approach B) confirms this: View requires either (a)
single ownership (&mut self for writes) or (b) an assume bridge. APAS Mt modules use
&mut self for writes, so Approach B WOULD work — but then insert/delete take &mut self
instead of &self, which is already the current signature.

**Wait — current signature already uses &mut self for writes.** The current `insert(&mut
self, value: T)` takes `&mut self`. So the PCell Approach B could work: with &mut self
on writes, the external ghost_count is updated atomically with the lock release, and
no other thread can observe the intermediate state. This means View + zero assumes IS
achievable for the current API.

But PCell Approach B only proved a simple `nat` view (element count). Proving View = 
`BalBinTree<T>` (the full tree structure) would require the PointsTo to track the tree
value, and the predicate to tie `ghost_view == perm.value()`. On acquire_write, the
predicate proves the internal ghost matches the PCell value. Since we have &mut self,
we're the only writer, so the external ghost (set at our last release) equals the internal
ghost (proved by predicate). This chain: external_ghost == internal_ghost == perm.value()
== actual_tree eliminates the assume.

**Estimated lines of change for BSTPlainMtEph:**
- Add TSM definition: ~40 lines (or PCell setup: ~30 lines)
- Rewrite lock interior struct: ~15 lines
- Rewrite predicate: ~10 lines
- Rewrite outer struct: ~10 lines
- Rewrite each of 10 operations: ~5-10 lines each = ~75 lines
- Remove/rewrite type_invariant: ~10 lines
- Total: ~160-180 lines changed (out of 817)

---

## 4. Assessment: TSM Migration vs Wait for Verus

### Option A: TSM migration now

**Pros:**
- Eliminates 63 lock-boundary assumes across 6 files.
- Pattern is proven (experiment verifies with zero assumes).
- Mechanical transformation — same structure for all 6 files.

**Cons:**
- ~180 lines changed per file × 6 files = ~1080 lines of churn.
- Loses View on the outer struct (TSM approach) or requires PCell+&mut self (already have &mut self, so PCell may work).
- The 12 non-lock assumes remain (clone proof gaps, constructor gaps).
- TSM adds complexity: state machine definition, token passing, instance cloning.
- Every Mt module in the project would eventually need this — 6 BST files here, but there are more Mt modules elsewhere.

### Option B: PCell approach (preferred over TSM for this use case)

**Pros:**
- Zero assumes AND View retained (since writes are &mut self).
- Simpler than TSM — no state machine definition, just PCell + PointsTo.
- PCell is a lighter mechanism (no transition definitions, no inductive proofs).
- More natural for single-writer patterns (which is what APAS Mt uses).

**Cons:**
- Same churn as TSM (~180 lines per file).
- PCell experiment only proved `nat` view, not full `BalBinTree<T>` view. The full-tree PCell approach needs validation.
- Still doesn't fix the 12 non-lock assumes.

### Option C: Wait for Verus `make-ghost-send-sync`

The `make-ghost-send-sync` branch would make `Ghost<T>` implement `Send + Sync`
regardless of `T`. This doesn't directly eliminate the lock-boundary assumes — the
core problem is that Ghost fields can't see inside the lock, not that Ghost isn't Send.
The assumes exist because `RwLockPredicate` is frozen at construction and can't track
value changes. `make-ghost-send-sync` doesn't change this.

### Recommendation

**Do the PCell migration, but as a staged experiment first.** 

1. **Phase 1**: Write a `bst_plain_mt_pcell_full.rs` experiment that uses PCell with
   `View<V = BalBinTree<u64>>` (not just `nat`). Confirm it verifies with zero lock-
   boundary assumes AND retains View.

2. **Phase 2**: If Phase 1 succeeds, migrate BSTPlainMtEph first (simplest, most callers).
   Validate that BSTSetPlainMtEph still works.

3. **Phase 3**: Migrate the other 5 files. BSTBBAlpha is identical to BSTPlain. AVL is
   similar but insert-only. RB and Splay use `Link<T>` instead of `BalBinTree<T>` but
   the pattern is the same. Treap is different (Set view) and may benefit from a 
   different approach.

**Don't wait for `make-ghost-send-sync`** — it doesn't solve this problem. The lock-
boundary assume is about value tracking, not Send/Sync bounds.

The 12 non-lock assumes (clone gaps, constructor gaps) should be addressed separately —
they're proof obligations, not architectural issues.
