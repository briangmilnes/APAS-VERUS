<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Acceptable Clone and PartialEq Assumes

Proof hole check run: full project (`veracity-review-proof-holes -d src/`).

These assumes are **accepted** — no proposed work. Per project rules:
- **PartialEq assume**: `assume(r == (self@ == other@))` — required; Verus cannot resolve eq_spec through trait machinery (partialeq-eq-pattern).
- **Clone assume**: `assume(cloned@ == self@)` — same Verus limitation for generic Clone.
- **assume_eq_clone_workaround**: Verus cannot prove generic Clone/Eq produce correct view equality; accepted (propose-new-work: do not table).

## Summary

| Category | File Count | Assume Count |
|----------|:----------:|:------------:|
| assume_eq_clone_workaround | 28 | 52 |
| PartialEq assume (explicit) | 28 | ~35 |

## Where We Can Accept Clone Assumes

| # | Chapter | File | Clone | PartialEq | Notes |
|---|---------|------|:-----:|:---------:|-------|
| 1 | 05 | MappingStEph.rs | — | eq | HashSetWithView |
| 2 | 05 | SetMtEph.rs | — | eq | HashSetWithView |
| 3 | 05 | SetStEph.rs | — | eq | HashSetWithView |
| 4 | 17 | MathSeq.rs | ✓ | ✓ | Vec\<T\> |
| 5 | 18 | ArraySeq.rs | — | eq | Vec\<T\> |
| 6 | 18 | ArraySeqMtEph.rs | — | eq | Vec\<T\> |
| 7 | 18 | ArraySeqMtPer.rs | — | eq | Vec\<T\> |
| 8 | 18 | ArraySeqStEph.rs | — | eq | Vec\<T\> |
| 9 | 18 | ArraySeqStPer.rs | — | eq | Vec\<T\> |
| 10 | 18 | LinkedListStEph.rs | ✓ | ✓ | Vec\<T\> |
| 11 | 18 | LinkedListStPer.rs | ✓ | ✓ | Vec\<T\> |
| 12 | 19 | ArraySeqMtEph.rs | — | eq | Vec\<T\> |
| 13 | 19 | ArraySeqStEph.rs | — | eq | Vec\<T\> |
| 14 | 19 | ArraySeqStPer.rs | — | eq | Vec\<T\> |
| 15 | 23 | BalBinTreeStEph.rs | ✓ | ✓ | recursive tree |
| 16 | 23 | PrimTreeSeqStPer.rs | ✓ | ✓ | seq + tree |
| 17 | 37 | AVLTreeSeq.rs | — | eq | compare_trees |
| 18 | 37 | AVLTreeSeqMtPer.rs | ✓ | — | Arc\<Node\> |
| 19 | 37 | AVLTreeSeqStEph.rs | ✓ | — | Box\<Node\> |
| 20 | 37 | AVLTreeSeqStPer.rs | ✓ | — | Arc\<Node\> |
| 21 | 39 | BSTTreapMtEph.rs | ✓ | ✓ | treap |
| 22 | 41 | AVLTreeSetMtPer.rs | ✓ | — | AVLTreeSeqMtPer |
| 23 | 41 | AVLTreeSetStEph.rs | ✓ | — | AVLTreeSeqStEph |
| 24 | 41 | AVLTreeSetStPer.rs | ✓ | — | AVLTreeSeqStPer |
| 25 | 41 | ArraySetStEph.rs | ✓ | — | ArraySeqStEph |
| 26 | 42 | TableStPer.rs | ✓ | — | HashMap-based |
| 27 | 45 | BalancedTreePQ.rs | ✓ | — | PQ |
| 28 | 45 | BinaryHeapPQ.rs | ✓ | ✓ | PQ |
| 29 | 45 | HeapsortExample.rs | ✓ | ✓ | demo |
| 30 | 45 | LeftistHeapPQ.rs | ✓ | ✓ | PQ |
| 31 | 45 | SortedListPQ.rs | ✓ | ✓ | PQ |
| 32 | 45 | UnsortedListPQ.rs | ✓ | ✓ | PQ |
| 33 | 47 | ChainedHashTable.rs | ✓ | — | HashMap-based |
| 34 | 47 | StructChainedHashTable.rs | ✓ | ✓ | Vec chains |
| 35 | 66 | BoruvkaMtEph.rs | ✓ | — | LabeledEdge |
| 36 | 66 | BoruvkaStEph.rs | ✓ | — | LabeledEdge |

## Excluded (experiments)

| File | Reason |
|------|--------|
| experiments/accept.rs | Not in main lib |

## Pattern Reference

**Clone (accepted):**
```rust
fn clone(&self) -> (cloned: Self)
    ensures cloned@ == self@
{
    let cloned = ...;
    proof { assume(cloned@ == self@); }
    cloned
}
```

**PartialEq (accepted):**
```rust
fn eq(&self, other: &Self) -> (r: bool)
    ensures r == (self@ == other@)
{
    let r = self.inner == other.inner;
    proof { assume(r == (self@ == other@)); }
    r
}
```

## Rules

- **Do not** table proposed work on assume_eq_clone_workaround (propose-new-work).
- **Do not** replace these assumes with external_body (partialeq-eq-pattern).
- **Do** keep these assumes when they follow the pattern above.
