# OrderedBST Refactor: Eliminating the Set-to-Map Bridge

## Problem

OrderedTableStEph has 3,809 proof lines — the largest module in the codebase.
Most of that is bridge proof: converting between ParamBST's `Set<(K::V, V::V)>`
view and OrderedTable's `Map<K::V, V::V>` view. The same bridge is duplicated
in StPer and MtEph variants.

The root cause is architectural. ParamBST (Chap38) views as `Set<T::V>`. When
`T = Pair<K,V>`, that gives `Set<(K::V, V::V)>` — a set of key-value tuples.
OrderedTable (Chap43) needs `Map<K::V, V::V>`. Every OrderedTable operation
must re-prove that BST properties (sorted order, structural invariants) imply
Map properties (key uniqueness, containment, union/split correspondence).

This is the wrong place for those proofs. APAS presents ordered tables as
BST-backed maps. The BST operations ARE the map operations. The proofs
should live with the BST, not with the table.

## Why ParamBST's Set View Is Correct

One might ask: if APAS BSTs implement tables, shouldn't ParamBST view as
Map directly? No. ParamBST is `ParamBST<T: Ord>` — a BST of elements
ordered by the elements themselves. When `T` is a plain key type (no value),
the BST is a set: `find(x)` searches for `x` by comparing `x` against nodes.
OrderedSet uses it this way. `Set<T::V>` is the correct view at this level.

The Map abstraction only arises when `T = Pair<K,V>` and comparison uses
only the key. ParamBST doesn't know that — it just sees `T: Ord` and
compares. The two uses are:

| Consumer | T | Correct View |
|----------|---|-------------|
| OrderedSet (Chap43) | `K` | `Set<K::V>` — ParamBST provides this |
| OrderedTable (Chap43) | `Pair<K,V>` | `Map<K::V, V::V>` — needs bridge |

Rust/Verus does not support switching a View associated type based on the
type parameter. `impl View for ParamBST<T>` has one `type V`, period. So
ParamBST stays as `Set<T::V>` (correct for the general case), and the
Map bridge belongs in a wrapper.

That wrapper is OrderedBST — the subject of this refactor.

## Current Architecture

```
ParamBST<Pair<K,V>>     ──View──>  Set<(K::V, V::V)>
        │
        │  OrderedTable wraps directly
        │  3,266 lines of bridge proof per variant:
        │    - 15 bridge lemmas (key_unique, pair_set_to_map, etc.)
        │    - bst_split_by_key: 410 lines
        │    - bst_next_by_key: 305 lines
        │    - bst_prev_by_key: 286 lines
        │    - bst_select_by_rank: 325 lines
        │    - bst_rank_by_key: 279 lines
        │    - union: 255 lines
        │
OrderedTableStEph<K,V>  ──View──>  Map<K::V, V::V>
```

Total across variants: ~9,000 proof lines for the same bridge, three times.

## Proposed Architecture

Insert `OrderedBST<K,V>` between ParamBST and OrderedTable:

```
ParamBST<Pair<K,V>>     ──View──>  Set<(K::V, V::V)>
        │
        │  OrderedBST wraps (new module, ~300 lines)
        │  Bridge proofs live here, ONCE
        │
OrderedBST<K,V>         ──View──>  Map<K::V, V::V>
        │
        │  OrderedTable wraps (thin, ~200 lines)
        │  Zero bridge proof — Map ensures propagate from OrderedBST
        │
OrderedTableStEph<K,V>  ──View──>  Map<K::V, V::V>
```

## OrderedBST Design

### Struct and View

```rust
pub struct OrderedBST<K, V> {
    pub inner: ParamBST<Pair<K, V>>,
}

impl<K: View, V: View> View for OrderedBST<K, V> {
    type V = Map<K::V, V::V>;

    open spec fn view(&self) -> Map<K::V, V::V> {
        spec_pair_set_to_map(self.inner@)
    }
}
```

The `spec_pair_set_to_map` conversion is defined once. The View bridges
`Set<(K::V, V::V)>` to `Map<K::V, V::V>` at a single point.

### Well-Formedness

```rust
pub open spec fn spec_orderedbst_wf(&self) -> bool {
    self.inner.spec_bstparasteph_wf()
    && spec_key_unique_pairs_set(self.inner@)
}
```

Key uniqueness is a structural invariant that follows from BST ordering when
the comparison function uses only the key (first element of Pair). The wf
predicate bundles it with the BST's own wf.

### Operations

Each operation delegates to `self.inner` (ParamBST), then proves the Map-level
postcondition from ParamBST's Set-level ensures. The bridge proof is short
because it only needs to connect two well-understood abstractions.

#### find

```rust
fn find(&self, key: &K) -> (found: Option<&Pair<K, V>>)
    requires self.spec_orderedbst_wf(),
    ensures
        found.is_some() <==> self@.contains_key(key@),
        found.is_some() ==> self@[key@] == found.unwrap().snd@;
```

ParamBST's find ensures `found.is_some() <==> self.inner@.contains(key_pair@)`.
Bridge: `inner@.contains(pair@) <==> self@.contains_key(pair.fst@)` — one
invocation of `lemma_pair_in_set_map_contains`.

#### split

```rust
fn split(&self, key: &K) -> (parts: (Self, Option<V>, Self))
    requires self.spec_orderedbst_wf(),
    ensures
        parts.0.spec_orderedbst_wf(),
        parts.2.spec_orderedbst_wf(),
        forall|k: K::V| parts.0@.contains_key(k) <==>
            (self@.contains_key(k) && TotalOrder::le(k, key@) && k != key@),
        forall|k: K::V| parts.2@.contains_key(k) <==>
            (self@.contains_key(k) && TotalOrder::le(key@, k) && k != key@),
        parts.1.is_some() <==> self@.contains_key(key@);
```

ParamBST's split gives `left@ ∪ right@ ∪ {pivot} == self@` in Set terms.
Bridge: key_unique preserved through subset (one lemma call each), then
pair_set_to_map distributes over the partition.

#### union

```rust
fn union(&self, other: &Self) -> (combined: Self)
    requires self.spec_orderedbst_wf(), other.spec_orderedbst_wf(),
    ensures
        combined.spec_orderedbst_wf(),
        combined@.dom() =~= self@.dom().union(other@.dom());
```

ParamBST ensures `combined.inner@ == self.inner@.union(other.inner@)`.
Bridge: pair_set_to_map distributes over set union when keys are unique.

#### next / prev (successor / predecessor)

```rust
fn next(&self, key: &K) -> (succ: Option<Pair<K, V>>)
    requires self.spec_orderedbst_wf(),
    ensures
        succ.is_some() ==> self@.contains_key(succ.unwrap().fst@),
        succ.is_some() ==> TotalOrder::le(key@, succ.unwrap().fst@)
            && succ.unwrap().fst@ != key@,
        succ.is_some() ==> forall|k: K::V|
            self@.contains_key(k) && TotalOrder::le(key@, k) && k != key@
            ==> #[trigger] TotalOrder::le(succ.unwrap().fst@, k);
```

These are the hardest operations. Currently 305 lines each in OrderedTable.
The proof difficulty is real (see below), but the bridge from Set to Map
adds ~100 unnecessary lines on top of the core ordering proof. In OrderedBST,
the ordering proof stays (~200 lines) but the bridge shrinks to ~10 lines.

#### rank / select

```rust
fn rank(&self, key: &K) -> (r: usize)
    requires self.spec_orderedbst_wf(),
    ensures r == self@.dom().filter(|k: K::V| TotalOrder::le(k, key@) && k != key@).len();

fn select(&self, rank: usize) -> (entry: Option<Pair<K, V>>)
    requires self.spec_orderedbst_wf(), rank < self@.dom().len(),
    ensures entry.is_some() ==> self@.contains_key(entry.unwrap().fst@);
```

## Where Bridge Lemmas Live

The 15 bridge lemmas currently in OrderedTableStEph move to OrderedBST's
module. They are:

| Lemma | Purpose | Lines |
|-------|---------|-------|
| `lemma_pair_set_to_map_dom_finite` | Map dom is finite when set is finite | 20 |
| `lemma_pair_set_to_map_len` | Map len == set len when keys unique | 36 |
| `lemma_pair_in_set_map_contains` | Pair in set ↔ key in map | 18 |
| `lemma_map_contains_pair_in_set` | Key in map → pair in set | 8 |
| `lemma_key_unique_insert` | Key uniqueness preserved by insert | 26 |
| `lemma_key_unique_remove` | Key uniqueness preserved by remove | 8 |
| `lemma_key_unique_subset` | Key uniqueness preserved by subset | 10 |
| `lemma_key_unique_empty` | Empty set is key-unique | 4 |
| `lemma_key_unique_disjoint_union` | Key uniqueness for disjoint union | 51 |
| `lemma_set_to_map_union_root` | Map union with BST root insertion | 103 |
| `lemma_view_gen_subset` | View generation subset | 17 |
| `lemma_view_gen_insert` | View generation insert | 20 |
| `lemma_view_gen_union` | View generation union | 17 |
| `lemma_cmp_equal_congruent` | Comparison congruence for Pair | 15 |
| `lemma_sorted_keys_pairwise_distinct` | Sorted keys → distinct keys | 38 |

Total: ~391 lines. These move to OrderedBST, shared across all variants.

## Impact

### Lines eliminated

| Module | Before | After | Reduction |
|--------|--------|-------|-----------|
| OrderedTableStEph | 3,809 | ~600 | ~3,200 |
| OrderedTableStPer | 2,632 | ~500 | ~2,100 |
| OrderedTableMtEph | (via delegation) | ~300 | ~500 |
| OrderedBST (new) | 0 | ~700 | — |
| **Net** | **~6,400** | **~2,100** | **~4,300** |

### What stays hard

The ordering proofs for next/prev/rank/select are genuinely difficult. They
require BST traversal reasoning with TotalOrder transitivity chains. Those
proofs don't shrink much — they're doing real math. But they move from
OrderedTable (wrong place) to OrderedBST (right place) and are written once
instead of three times.

### What disappears

The `bst_*_by_key` functions in OrderedTable. These exist because OrderedTable
re-implements BST traversal to bridge Set→Map at each step. With OrderedBST,
the traversal stays in ParamBST, the bridge is in OrderedBST's ensures, and
OrderedTable just calls `self.tree.next(key)`.

## Risks

### Pair comparison semantics

ParamBST compares elements with `Ord`. For `Pair<K,V>`, the `Ord` impl must
compare by K only (not lexicographic on (K,V)). Verify that `Pair<K,V>`'s
`Ord` does this. If it compares `(K, V)` lexicographically, two pairs with
the same key but different values would be considered different, breaking the
map abstraction. Fix: ensure Pair's Ord compares only fst.

### Standalone rule

OrderedBST would be shared across StEph, StPer, and MtEph variants. This is
permitted by the standalone rule's exception: "When APAS explicitly presents
one algorithm as building on another." APAS Chapter 43 explicitly builds
ordered tables on balanced BSTs from Chapter 38.

### Proof migration

The bridge proofs don't vanish — they move from OrderedTable to OrderedBST.
The savings come from:
1. Writing them once instead of three times (StEph, StPer, MtEph).
2. Eliminating the `bst_*_by_key` re-implementations (~1,800 lines) that
   exist only because the bridge wasn't at the right abstraction level.

### AugOrderedTable

AugOrderedTable (augmented ordered table) wraps OrderedTable. After this
refactor, AugOrderedTable wraps OrderedTable which wraps OrderedBST which
wraps ParamBST. The delegation chain is one level deeper but each level is
thinner. AugOrderedTable's proof obligations should shrink proportionally.

## Execution Plan

### Phase 1: OrderedBST module

Create `src/Chap43/OrderedBST.rs` (or `src/Chap38/OrderedBST.rs` — location
TBD based on dependency direction).

Contents:
- Struct, View, wf
- Bridge lemmas (moved from OrderedTableStEph)
- OrderedBSTTrait with Map-level ensures
- Impl delegating to ParamBST with bridge proofs

Validate: `scripts/validate.sh isolate Chap43`

### Phase 2: OrderedTableStEph migration

- Change `tree: ParamBST<Pair<K,V>>` to `tree: OrderedBST<K,V>`
- Delete bridge lemmas (now in OrderedBST)
- Delete `bst_*_by_key` functions (OrderedBST handles these)
- Simplify remaining trait methods to delegate to `self.tree`

Validate after each function migration. Track proof line count.

### Phase 3: OrderedTableStPer migration

Same as Phase 2. The standalone rule allows importing OrderedBST since
it's in the same chapter and APAS builds tables on BSTs.

### Phase 4: OrderedTableMtEph migration

Mt variant. May need its own OrderedBSTMtEph if the Mt standalone rule
requires it — but since OrderedBST is pure structural (no locking), it
should be shareable.

### Phase 5: AugOrderedTable simplification

After OrderedTable shrinks, AugOrderedTable's delegation proofs should
simplify. Measure the reduction.
