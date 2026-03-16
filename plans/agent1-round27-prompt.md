# Agent 1 — Round 27: Add Real Specs to Chap37 MtEph Helpers (DO THE WORK)

## R26 Feedback

You defined `spec_is_bst_link` in BSTSplayMtEph.rs and BSTRBMtEph.rs. Good. But then
you did not USE it — you skipped the main assignment. The prompt said to add
requires/ensures to the helper functions. You said "cascades to trait impl callers" as
your excuse. That is wrong:

1. The helper functions (`splay`, `bst_insert`, `find_link`, etc.) are **free functions**,
   not trait methods. Adding requires to them does NOT cascade to trait callers.
2. The trait methods (`insert`, `find`, etc.) call the helpers inside `external_body`
   wrappers. External bodies accept any spec — adding requires to helpers does not break them.
3. Even if something DID cascade, the fix is to strengthen the trait specs too, which is
   the whole point.

This round: add the specs. No excuses. The StEph versions already have these specs proved.
You are copying them — not proving new things.

## Mission

Add real requires/ensures to all BST helper functions in BSTSplayMtEph.rs and BSTRBMtEph.rs.

## Part 1: BSTSplayMtEph.rs — Copy Specs from StEph (Priority 1)

Read `src/Chap37/BSTSplayStEph.rs`. Copy these exact specs into BSTSplayMtEph.rs,
adapting function names where needed (`spec_contains_link` → `link_contains`,
`spec_size_link` → `link_spec_size`).

### Exact specs to copy

**`splay` (StEph line 320):**
```rust
fn splay<T: StTInMtT + Ord + TotalOrder>(root: Box<Node<T>>, target: &T) -> (result: Box<Node<T>>)
    requires spec_is_bst_link(Some(root)),
    ensures
        spec_is_bst_link(Some(result)),
        forall|x: T| link_contains(Some(result), x) <==> link_contains(Some(root), x),
    decreases root,
```

**`bst_insert` (StEph line 1299):**
```rust
fn bst_insert<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, value: T) -> (inserted: bool)
    requires spec_is_bst_link(old(link)),
    ensures
        spec_is_bst_link(link),
        link_contains(link, value),
        forall|x: T| link_contains(old(link), x) ==> link_contains(link, x),
        forall|x: T| link_contains(link, x) ==> (link_contains(old(link), x) || x == value),
    decreases old(link),
```

**`insert_link` (StEph line 1442):**
```rust
fn insert_link<T: StTInMtT + Ord + TotalOrder>(link: &mut Link<T>, value: T) -> (inserted: bool)
    requires spec_is_bst_link(old(link)),
    ensures
        spec_is_bst_link(link),
        link_contains(link, value),
        forall|x: T| link_contains(old(link), x) ==> link_contains(link, x),
        forall|x: T| link_contains(link, x) ==> (link_contains(old(link), x) || x == value),
```

**`find_link` (StEph line 1462):**
```rust
fn find_link<'a, T: StTInMtT + Ord + TotalOrder>(link: &'a Link<T>, target: &T) -> (found: Option<&'a T>)
    requires spec_is_bst_link(link),
    ensures
        found.is_some() <==> link_contains(link, *target),
        found.is_some() ==> *found.unwrap() == *target,
    decreases *link,
```

**`min_link` (StEph line 1501):**
```rust
fn min_link<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (min: Option<&T>)
    requires spec_is_bst_link(link),
    ensures
        link.is_some() ==> min.is_some(),
        min.is_some() ==> link_contains(link, *min.unwrap()),
        min.is_some() ==> forall|x: T| link_contains(link, x) ==> T::le(*min.unwrap(), x),
    decreases *link,
```

**`max_link` (StEph line 1553):**
```rust
fn max_link<T: StTInMtT + Ord + TotalOrder>(link: &Link<T>) -> (max: Option<&T>)
    requires spec_is_bst_link(link),
    ensures
        link.is_some() ==> max.is_some(),
        max.is_some() ==> link_contains(link, *max.unwrap()),
        max.is_some() ==> forall|x: T| link_contains(link, x) ==> T::le(x, *max.unwrap()),
    decreases *link,
```

### What happens when you add these specs

- `splay`, `bst_insert`, `find_link`, `min_link`, `max_link` are **proved** functions (no
  external_body). Adding stronger specs means Verus must verify the ensures hold. If a
  function can't prove the new ensures, you need to add proof annotations inside the body
  (assert steps, reveal_with_fuel, etc.) — look at how StEph does it.
- `insert_link` calls `bst_insert` then `splay`. If both have strong specs, insert_link
  should verify.
- The key proof technique in StEph is `reveal_with_fuel(spec_is_bst_link, 4)` and
  `reveal_with_fuel(spec_contains_link, 4)` at the top of function bodies.

### Note on MtEph naming

MtEph uses `link_contains` where StEph uses `spec_contains_link`. Use the MtEph names.
Similarly `link_spec_size` not `spec_size_link`. The `spec_is_bst_link` name is the same
in both (you already defined it in R26).

The `&` reference pattern may differ. StEph uses `spec_is_bst_link(&Some(root))` while
MtEph may need `spec_is_bst_link(Some(root))` (by value). Check how MtEph's existing
`spec_is_bst_link` takes its argument and match that.

## Part 2: BSTRBMtEph.rs — Copy Specs (Priority 2)

Same treatment. Read `src/Chap37/BSTRBStEph.rs` for RB-specific specs. The RB tree uses
`tree_is_bst()` on a `BalBinTree` type — you'll need to translate to the MtEph
`spec_is_bst_link` on `Link<T>`.

Key RB functions needing specs:
- `rotate_left`: requires BST ordering, ensures BST ordering + content preservation
- `rotate_right`: same
- `flip_colors`: no ordering change, ensures size/content preservation
- `fix_up`: requires BST ordering, ensures BST ordering
- `insert_link`: requires BST ordering, ensures BST ordering + containment
- `find_link`: requires BST ordering, ensures correctness
- `min_link`, `max_link`: same as Splay versions

## Part 3: Strengthen `spec_bstsplaymteph_wf` and `spec_bstrbmteph_wf` (Priority 3)

Currently these wf predicates only assert `link_spec_size(self@) <= usize::MAX`. That's
a size bound, not a BST invariant. Add `spec_is_bst_link(self@)` to the wf:

```rust
open spec fn spec_bstsplaymteph_wf(&self) -> bool {
    link_spec_size(self@) <= usize::MAX
    && spec_is_bst_link(self@)
}
```

This is safe because all trait methods are `external_body` — they accept any spec. The
strengthened wf means callers know the tree is a valid BST. When trait methods are later
proved, they'll need to maintain BST ordering, which is correct.

## Important

- **Read BSTSplayStEph.rs lines 320-1600** before touching BSTSplayMtEph.rs.
- You are COPYING specs, then adding proof annotations to bodies that need them.
- The MtEph helper functions already have bodies — they're proved functions (not
  external_body). You need to make those bodies verify with the stronger specs.
- If a body fails to verify with the new ensures, add proof annotations (asserts,
  reveal_with_fuel). Look at how StEph does it.
- Do NOT weaken the StEph spec to make it fit. If the MtEph body can't prove the full
  StEph spec, flag which ensures clause fails and why.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT skip functions because "it cascades." That was wrong last round.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- Real requires/ensures on all 6 Splay helpers (splay, bst_insert, insert_link, find_link,
  min_link, max_link)
- Real requires/ensures on all RB helpers (rotate_left, rotate_right, flip_colors, fix_up,
  insert_link, find_link, min_link, max_link)
- `spec_bstsplaymteph_wf` and `spec_bstrbmteph_wf` strengthened to include BST ordering
- `plans/agent1-round27-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
