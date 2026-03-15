//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! TotalOrder Standard: using TotalOrder for spec-level ordering in APAS-VERUS.
//!
//! The `TotalOrder` trait (in `src/vstdplus/total_order.rs`) bridges executable
//! comparison to spec-level ordering. It provides:
//! - `spec fn le(self, other: Self) -> bool` — the spec-level ordering predicate.
//! - `proof fn reflexive`, `transitive`, `antisymmetric`, `total` — the four axioms.
//! - `fn cmp(&self, other: &Self) -> Ordering` — exec comparison with spec ensures.
//!
//! Impls exist for all integer types (u8..u128, i8..i128, usize, isize) and String.
//! Integer impls have empty proof bodies (solver handles `<=` automatically).
//! String uses `assume` (vstd has no ordering spec for String).
//!
//! 1. When to add `T: TotalOrder`.
//!
//!    Any module whose textbook ADT defines ordering-dependent operations needs
//!    `T: TotalOrder` on the trait bound. This includes:
//!    - BSTs (min, max, find, insert, contains) — Chap37, Chap38, Chap39.
//!    - Ordered sets/tables (first, last, previous, next, rank, select) — Chap43.
//!    - Priority queues (find_min, find_max, extract_min) — Chap45.
//!    - Sorted sequences (merge, sort) — Chap03, Chap35, Chap36.
//!
//!    Do NOT add `TotalOrder` to modules that only need equality (sets, tables, hash
//!    tables) unless ordering operations exist.
//!
//! 2. Trait bound syntax.
//!
//!    ```
//!    pub trait OrderedSetStEphTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<T::V>> {
//!    ```
//!
//!    The `Ord` bound provides exec comparison (`<`, `<=`, `cmp`). The `TotalOrder`
//!    bound provides spec-level ordering (`TotalOrder::le`) and proof fns.
//!
//! 3. Extremality ensures patterns.
//!
//!    When a function returns a minimum element, the ensures must assert that the
//!    returned value is `<=` every other element in the collection. These are the
//!    standard patterns for common ADT operations:
//!
//!    Minimum (find_min, first):
//!    ```
//!    fn find_min(&self) -> (min: Option<&T>)
//!        requires self.spec_wf(),
//!        ensures
//!            self.spec_size() == 0 ==> min.is_none(),
//!            self.spec_size() > 0 ==> min.is_some(),
//!            min.is_some() ==> self@.count(*min.unwrap()) > 0,  // multiset view
//!            // -- or for set view --
//!            min.is_some() ==> self@.contains(min.unwrap()@),   // set view
//!            // The minimality clause:
//!            min.is_some() ==> forall|e: T| self@.count(e) > 0
//!                ==> #[trigger] TotalOrder::le(*min.unwrap(), e),  // multiset
//!            // -- or for set view --
//!            min.is_some() ==> forall|x: T::V| self@.contains(x)
//!                ==> #[trigger] TotalOrder::le(min.unwrap()@, x),  // set
//!    ;
//!    ```
//!
//!    Maximum (find_max, last):
//!    ```
//!    fn last(&self) -> (max: Option<T>)
//!        requires self.spec_wf(),
//!        ensures
//!            self@.len() == 0 <==> max is None,
//!            max matches Some(v) ==> self@.contains(v@),
//!            max matches Some(v) ==>
//!                forall|x: T::V| self@.contains(x)
//!                    ==> #[trigger] TotalOrder::le(x, v@),
//!    ;
//!    ```
//!
//!    Predecessor (previous — max{k' in A | k' < k}):
//!    ```
//!    fn previous(&self, k: &T) -> (pred: Option<T>)
//!        requires self.spec_wf(),
//!        ensures
//!            pred matches Some(v) ==> self@.contains(v@),
//!            pred matches Some(v) ==> TotalOrder::le(v@, k@) && v@ != k@,
//!            pred matches Some(v) ==>
//!                forall|x: T::V| self@.contains(x) && TotalOrder::le(x, k@) && x != k@
//!                    ==> #[trigger] TotalOrder::le(x, v@),
//!    ;
//!    ```
//!
//!    Successor (next — min{k' in A | k' > k}):
//!    ```
//!    fn next(&self, k: &T) -> (succ: Option<T>)
//!        requires self.spec_wf(),
//!        ensures
//!            succ matches Some(v) ==> self@.contains(v@),
//!            succ matches Some(v) ==> TotalOrder::le(k@, v@) && v@ != k@,
//!            succ matches Some(v) ==>
//!                forall|x: T::V| self@.contains(x) && TotalOrder::le(k@, x) && x != k@
//!                    ==> #[trigger] TotalOrder::le(v@, x),
//!    ;
//!    ```
//!
//!    Rank (|{k' in A | k' < k}|):
//!    ```
//!    fn rank(&self, k: &T) -> (r: usize)
//!        requires self.spec_wf(),
//!        ensures
//!            r as int == self@.filter(|x: T::V| TotalOrder::le(x, k@) && x != k@).len(),
//!            r <= self@.len(),
//!    ;
//!    ```
//!
//!    Containment (find, contains — bidirectional iff):
//!    ```
//!    fn find(&self, target: &T) -> (found: Option<&T>)
//!        requires self.spec_wf(),
//!        ensures
//!            found.is_some() <==> self@.contains(target@),
//!            found.is_some() ==> *found.unwrap() == *target,
//!    ;
//!    ```
//!
//!    Sorted sequence (in_order):
//!    ```
//!    fn in_order(&self) -> (result: Vec<T>)
//!        requires self.spec_wf(),
//!        ensures
//!            forall|i: int, j: int| 0 <= i < j < result@.len()
//!                ==> #[trigger] TotalOrder::le(result@[i], result@[j]),
//!    ;
//!    ```
//!
//! 4. TotalOrder::le on T vs T::V.
//!
//!    `TotalOrder::le(self, other)` operates on `Self` — the concrete type `T`. The
//!    View type `T::V` is typically the same type for primitives (u64::V == u64), but
//!    may differ for wrapped types.
//!
//!    When the collection's View is `Set<T::V>` (e.g., OrderedSet) and you need to
//!    quantify over elements:
//!    - If `T::V == T` (primitives): `TotalOrder::le(x, y)` works directly with
//!      `forall|x: T::V| self@.contains(x) ==> TotalOrder::le(v@, x)`.
//!    - If `T::V != T` (wrapped types): you need `T::V: TotalOrder` or a spec bridge.
//!
//!    For multiset views (`Multiset<T>`, e.g., PQs): quantify over `T` directly:
//!    `forall|e: T| self@.count(e) > 0 ==> TotalOrder::le(*min, e)`.
//!
//!    For set views (`Set<T::V>`, e.g., OrderedSets): quantify over `T::V`:
//!    `forall|x: T::V| self@.contains(x) ==> TotalOrder::le(v@, x)`.
//!    This requires `T::V: TotalOrder`. If this bound is too restrictive, add
//!    `external_body` with the correct spec and document the type gap.
//!
//! 5. Proof techniques.
//!
//!    a. Inductive minimality lemma.
//!
//!       For tree-based structures, prove root is minimum via structural induction:
//!       ```
//!       proof fn lemma_heap_root_is_min<T: StT + Ord + TotalOrder>(node: &HeapNode<T>)
//!           requires node.spec_is_heap(),
//!           ensures
//!               node.spec_seq().len() > 0 ==>
//!                   forall|i: int| 0 <= i < node.spec_seq().len() ==>
//!                       #[trigger] TotalOrder::le(node.spec_seq()[0], node.spec_seq()[i]),
//!           decreases *node,
//!       ```
//!       Pattern: recurse on subtrees, then for each element `s[i]`:
//!       - If `i == 0`: use `TotalOrder::reflexive`.
//!       - If in left subtree: heap gives `le(root, left_root)`, recursion gives
//!         `le(left_root, s[i])`, then `TotalOrder::transitive(root, left_root, s[i])`.
//!       - If in right subtree: same with right_root.
//!
//!       Reference: `src/Chap45/LeftistHeapPQ.rs` lines 115-173.
//!
//!    b. BST search exclusion.
//!
//!       When searching a BST, use `TotalOrder::antisymmetric` to exclude the wrong
//!       subtree:
//!       ```
//!       Ordering::Less => {
//!           proof {
//!               if inner.right.tree_contains(*target) {
//!                   T::antisymmetric(*target, inner.value);
//!               }
//!           }
//!           contains_node(&inner.left, target)
//!       }
//!       ```
//!       Reference: `src/Chap37/BSTPlainStEph.rs` lines 378-409.
//!
//!    c. Transitivity chains.
//!
//!       For delete-min where the second element becomes the new min:
//!       ```
//!       TotalOrder::transitive(min_val, node_val, x);
//!       ```
//!       This chains: `le(min, node) && le(node, x) ==> le(min, x)`.
//!
//!    d. Totality for case splits.
//!
//!       When you need `a <= b || b <= a` to split cases:
//!       ```
//!       proof { TotalOrder::total(a, b); }
//!       ```
//!
//! 6. Import pattern.
//!
//!    ```
//!    use crate::vstdplus::total_order::total_order::TotalOrder;
//!    ```
//!
//!    No broadcast group needed — TotalOrder methods are called explicitly, not
//!    triggered by broadcast.
//!
//! 7. What NOT to do.
//!
//!    - Do NOT define ad-hoc `spec_leq` or `spec_le_view` spec fns that duplicate
//!      TotalOrder::le. Use TotalOrder::le directly.
//!    - Do NOT assume ordering properties. If you need `le(a, b)`, prove it via
//!      the trait's proof fns (reflexive, transitive, antisymmetric, total).
//!    - Do NOT omit extremality from ensures. `contains(min)` without
//!      `forall|x| contains(x) ==> le(min, x)` is a weak spec. The textbook defines
//!      "minimum" as the least element, not just "some element in the set."
//!    - Do NOT declare TotalOrder "blocked" because the trait doesn't have it yet.
//!      Add the bound. That is the fix.

// No code body — this is a documentation-only standard file.
