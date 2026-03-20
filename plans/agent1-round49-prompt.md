# R49 Agent1 Prompt: Prove select rank property (Chap41+43)

## Context

R48 applied the capacity-bounds standard to Chap41/43 insert operations. The net
hole count is unchanged at 38 because the 4 original Chap43 holes are all
irreducible with current infrastructure. Two of those holes are the `select`
function in OrderedSetStEph.rs and OrderedSetStPer.rs — both have the same
assume about filter cardinality.

## Assignment: Prove select by adding sortedness infrastructure

The `select` function (Chap43 OrderedSetStEph.rs:1134, OrderedSetStPer.rs:1031)
assumes:
```rust
assume(self@.filter(|x: T::V| exists|t: T|
    #[trigger] TotalOrder::le(t, result) && t@ == x && t@ != result@
).len() == i as int);
```

This says: the i-th element in sorted order has exactly i predecessors. It's true
because the AVL tree's inorder sequence is sorted and has no duplicates.

### Step 1: Add sortedness to AVLTreeSetStEph wf (Chap41)

Currently `spec_avltreesetsteph_wf` is:
```rust
self.elements.spec_avltreeseqsteph_wf() && self.elements@.no_duplicates() && self@.finite()
```

It should also include sortedness. The spec already exists:
- `spec_seq_sorted<T: TotalOrder>(s: Seq<T>)` at Chap41/AVLTreeSetStEph.rs:80
- `spec_elements_sorted(&self)` at Chap41/AVLTreeSetStEph.rs:1168

But sortedness is on `AVLTreeSetStEphTotalOrderTrait` which requires `T: TotalOrder`.
The base wf is on `AVLTreeSetStEphTrait` which only requires `T: StT + Ord`.

**Options:**
A. Add `spec_elements_sorted()` to the base wf predicate (requires T: TotalOrder
   bound on the base trait — breaking change)
B. Add a separate `spec_avltreesetsteph_sorted_wf` that includes sortedness
   (used only by functions that need it, like select)
C. Add a proof lemma `lemma_wf_implies_sorted` that proves sortedness from the
   BST property embedded in the tree structure

Option C is cleanest: it doesn't change existing wf predicates. But it requires
proving that the tree's BST invariant implies inorder sortedness. Check if the
`spec_avltreeseqsteph_wf` captures BST ordering or just structural properties.

### Step 2: Write lemma_sorted_rank_property

Given a sorted no-duplicate sequence, prove the i-th element has exactly i
predecessors:
```rust
proof fn lemma_sorted_rank_property<T: TotalOrder>(
    seq: Seq<T>,
    i: int,
)
    requires
        spec_seq_sorted(seq),
        seq.no_duplicates(),  // or the T::V version
        0 <= i < seq.len(),
    ensures
        seq.to_set().filter(|x: T::V| exists|t: T|
            #[trigger] TotalOrder::le(t, seq[i]) && t@ == x && t@ != seq[i]@
        ).len() == i,
```

Key proof steps:
1. Sorted + no_duplicates implies strict ordering: j < i ==> seq[j] < seq[i]
2. Elements at indices 0..i are exactly those satisfying the filter predicate
3. The set filter has cardinality i (bijection with indices 0..i)

### Step 3: Apply to OrderedSetStEph and OrderedSetStPer select

Replace the `assume` with a call to the sortedness lemma and the rank lemma.

### Existing infrastructure (from R48 exploration)

- `spec_seq_sorted` defined at Chap41/AVLTreeSetStEph.rs:80
- `lemma_push_sorted` at Chap41/AVLTreeSetStEph.rs:129 (appending preserves sort)
- `lemma_subseq_sorted` at Chap41/AVLTreeSetStEph.rs:161 (subrange preserves sort)
- `lemma_inorder_values_maps_to_views` at Chap41/AVLTreeSetStEph.rs:107
- `spec_inorder_values` maps the tree to Seq<T>
- `spec_inorder` maps the tree to Seq<T::V>
- vstd `sorted_by` at `vstd::relations` matches `spec_seq_sorted` pattern

### Target: -2 holes

If successful: 38 → 36 holes. Both select functions in Chap43 become clean.

### Stretch goal: union capacity requires

The union assume in AVLTreeSetStEph.rs (2nd loop) needs
`self@.len() + other@.len() < usize::MAX` on the union trait. This cascades to:
- Chap41 AVLTreeSetMtEph union
- Chap43 OrderedSetStEph union/join
- Chap43 OrderedSetMtEph union/join
- Chap53 GraphSearchStEph (calls union on visited set)

Check if the cascade is manageable. If so: -1 more hole (37 total).

### Files to modify

| # | Chap | File | What |
|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | Add sortedness lemma or wf extension |
| 2 | 43 | OrderedSetStEph.rs | Replace select assume with proof |
| 3 | 43 | OrderedSetStPer.rs | Replace select assume with proof |
| 4 | 41 | AVLTreeSetStEph.rs | (stretch) Add capacity to union trait |
| 5 | 41 | AVLTreeSetMtEph.rs | (stretch) Propagate union capacity |
| 6 | 43 | OrderedSetStEph.rs | (stretch) Propagate union capacity |

### Validation

Run `scripts/validate.sh` after each major change. Target: 4419+ verified, 0 errors.
Run `scripts/rtt.sh` and `scripts/ptt.sh` before committing.
Write report to `plans/agent1-round49-report.md`.
