# Agent 3 — Round 18: TotalOrder Minimality for Chap45 PQs + Chap38 in_order

## Mission

Two tasks:

### Task A: Copy LeftistHeapPQ's TotalOrder minimality to BinaryHeapPQ + BalancedTreePQ

LeftistHeapPQ ALREADY has the correct `find_min` ensures with TotalOrder minimality:
```rust
self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
    #[trigger] TotalOrder::le(*min_elem.unwrap(), e);
```

BinaryHeapPQ and BalancedTreePQ do NOT have this. Their find_min only ensures containment.
Add the minimality clause to both.

### Task B: Strengthen in_order/collect_in_order in BSTParaStEph.rs

In_order should ensure the returned sequence is sorted and contains exactly the set elements.

## Required Reading

**Before writing any code**, read `src/standards/total_order_standard.rs`. It defines
the extremality ensures patterns, the inductive minimality lemma technique, and import
pattern. Also read `src/Chap45/LeftistHeapPQ.rs` as the reference implementation.

## Task A: Exact Code to Write

### Reference: LeftistHeapPQ (already done, use as template)

File: `src/Chap45/LeftistHeapPQ.rs`

**find_min trait ensures** (line 278):
```rust
fn find_min(&self) -> (min_elem: Option<&T>)
    requires self.spec_leftistheappq_wf(),
    ensures
        self.spec_size() == 0 ==> min_elem.is_none(),
        self.spec_size() > 0 ==> min_elem.is_some(),
        self.spec_size() > 0 ==> self@.count(*min_elem.unwrap()) > 0,
        self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
            #[trigger] TotalOrder::le(*min_elem.unwrap(), e);
```

**lemma_heap_root_is_min** (line 115): A recursive proof that `spec_seq()[0] <= spec_seq()[i]`
for all `i`, using `TotalOrder::transitive` through subtree roots.

### BinaryHeapPQ — what to change

File: `src/Chap45/BinaryHeapPQ.rs`

1. Add `+ TotalOrder` to the trait bound if not already present.
2. Add to find_min ensures:
   ```rust
   self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
       #[trigger] TotalOrder::le(*min_elem.unwrap(), e),
   ```
3. Write a `lemma_heap_root_is_min` for binary heaps. The BinaryHeapPQ uses an array-based
   heap where children of index `i` are at `2i+1` and `2i+2`. The proof shows:
   - Root at index 0 is `<=` every element.
   - For any index `i > 0`, its parent at `(i-1)/2` satisfies `le(parent, child)`.
   - By induction/transitivity from root to any node: `le(root, node)`.
4. Add `external_body` to find_min impl if the proof is complex.

### BalancedTreePQ — what to change

File: `src/Chap45/BalancedTreePQ.rs`

1. The trait bound already has `T: StT + Ord + TotalOrder` (from R17 Agent 4's work).
2. Add to find_min ensures:
   ```rust
   self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
       #[trigger] TotalOrder::le(*min_elem.unwrap(), e),
   ```
3. Add to find_max ensures:
   ```rust
   self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
       #[trigger] TotalOrder::le(*max_elem.unwrap(), e) == false || *max_elem.unwrap() == e,
   ```
   Or more precisely:
   ```rust
   self.spec_size() > 0 ==> forall|e: T| self@.count(e) > 0 ==>
       #[trigger] TotalOrder::le(e, *max_elem.unwrap()),
   ```
4. Write or adapt `lemma_heap_root_is_min` for the balanced tree structure.
5. Add `external_body` if the proof is complex.

### insert multiset preservation

Also strengthen `insert` in both BinaryHeapPQ and BalancedTreePQ:
```rust
fn insert(&mut self, value: T) -> (inserted: bool)
    ensures
        self.spec_binaryheappq_wf(),
        self@.to_multiset() =~= old(self)@.to_multiset().insert(value),
```

This is the content preservation ensures that Agent 4 identified as missing.

## Task B: BSTParaStEph in_order

File: `src/Chap38/BSTParaStEph.rs`

The `in_order` function returns a `Vec<T>` of the BST elements in sorted order. Currently
it only ensures length. Strengthen to:

```rust
fn in_order(&self) -> (result: Vec<T>)
    requires self.spec_bstparasteph_wf(),
    ensures
        result@.len() == self.spec_size(),
        // Sorted:
        forall|i: int, j: int| 0 <= i < j < result@.len()
            ==> #[trigger] TotalOrder::le(result@[i], result@[j]),
        // Content matches the tree's set:
        forall|x: T| self.spec_root().tree_contains(x) <==>
            result@.contains(x),
;
```

Add `external_body` if the proof is difficult.

## Procedure

1. **Read** `src/Chap45/LeftistHeapPQ.rs` — the template (find_min + lemma_heap_root_is_min).
2. **Read** `src/Chap45/BinaryHeapPQ.rs` — understand the current spec and heap structure.
3. **Read** `src/Chap45/BalancedTreePQ.rs` — understand the balanced tree PQ.
4. **Read** `src/Chap38/BSTParaStEph.rs` — understand in_order.
5. Write the strengthened ensures. Add external_body where needed.
6. `scripts/validate.sh` — 0 errors.

## Important

- LeftistHeapPQ is the reference. Copy the pattern.
- Strong spec + external_body > weak spec. Do not weaken to avoid proof work.
- Do NOT modify LeftistHeapPQ (it's already correct).
- Do NOT modify Chap37/39/41/42/43/47 (other agents' scope).

## Deliverables

- Strengthened find_min/find_max in BinaryHeapPQ and BalancedTreePQ.
- Strengthened insert multiset preservation.
- Strengthened in_order in BSTParaStEph.
- `plans/agent3-round18-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
