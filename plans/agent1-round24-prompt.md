# Agent 1 — Round 24: Chap37 Remaining + Chap45 Priority Queues

## Mission

Finish Chap37 remaining holes (whatever R23 left), then move to Chap45 priority queues
(5 holes across 3 non-Example files). Chap37 is the root blocker — every hole you close
there unblocks downstream chapters.

## Part 1: Chap37 Remaining Holes

Check what R23 accomplished on BSTSplayStEph.rs. The R23 target was:
- `spec_bstsplaysteph_wf`: strengthen from `{ true }` to real BST invariant
- 6 `external_body` holes: insert, find, minimum, maximum, in_order, pre_order

Whatever didn't get proved in R23, continue here. Also remaining:

| # | Chap | File | Holes | Notes |
|---|------|------|:-----:|-------|
| 1 | 37 | AVLTreeSeq.rs | 1 | Iterator `next` — may be structurally irreducible |
| 2 | 37 | AVLTreeSeqMtPer.rs | 2 | `build_balanced_from_slice`, `subseq_copy` — thread boundary |
| 3 | 37 | BSTSplayStEph.rs | 7 | Whatever R23 left |
| 4 | 37 | BSTSplayMtEph.rs | 1 | Mt wrapper |
| 5 | 37 | BSTRBMtEph.rs | 2 | Mt wrapper |

Priority: BSTSplayStEph first (most holes, most impact), then the AVLTreeSeq/Mt holes
if tractable.

## Part 2: Chap45 Priority Queues (5 holes)

Chap45 depends on Chap37::AVLTreeSeqStPer. If that's still holed, you may not be able
to prove BalancedTreePQ. But BinaryHeapPQ and LeftistHeapPQ may be independently provable.

| # | Chap | File | Holes | Functions |
|---|------|------|:-----:|-----------|
| 1 | 45 | BalancedTreePQ.rs | 2 | `insert` (ext_body), 1 `external` |
| 2 | 45 | BinaryHeapPQ.rs | 2 | `find_min` (ext_body), 1 `assume(sorted)` |
| 3 | 45 | LeftistHeapPQ.rs | 0 | Listed as holed but may just be fn_missing warnings |

### BinaryHeapPQ::find_min (trivial)

```rust
#[verifier::external_body]
fn find_min(&self) -> (min_elem: Option<&T>) {
    if self.elements.length() == 0 { None }
    else { Some(self.elements.nth(0)) }
}
```

This returns the root of a min-heap. Should be provable if the spec says the root is
the minimum element (heap property in wf).

### BinaryHeapPQ assume(sorted)

```rust
assume(Self::spec_sorted(result.seq@));
```

This assume is on the `in_order` function. Proving sorted output from a heap requires
the heap extraction loop to maintain sortedness. May be tractable.

### BalancedTreePQ::insert

Uses AVLTreeSeq as backing — depends on Chap37 AVLTreeSeq being clean enough.

## Approach

1. Read your R23 report and check what you accomplished on BSTSplay.
2. Continue from where you left off on Chap37.
3. Run `scripts/holes.sh src/Chap37/` to get current state.
4. When Chap37 progress stalls, switch to Chap45.
5. Start with BinaryHeapPQ::find_min (likely trivial).

## Important

- You MAY strengthen wf predicates and add requires — that's the goal.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Skip Example files (Example45_2.rs, HeapsortExample.rs).
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- Proven holes in Chap37 and Chap45 source files.
- `plans/agent1-round24-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
