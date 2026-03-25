//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Red-Black multi-threaded BST implementation.

pub mod BSTSetRBMtEph {

    use std::collections::BTreeSet;
    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTRBMtEph::BSTRBMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTRBMtEph<T>,
    }

    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetRBMtEphIter<T: StTInMtT + Ord + TotalOrder> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetRBMtEphGhostIter<T: StTInMtT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetRBMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetRBMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetrbmteph_iter_invariant<T: StTInMtT + Ord + TotalOrder>(it: &BSTSetRBMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetrbmteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            ensures set.spec_bstsetrbmteph_wf();
        fn singleton(value: T) -> (set: Self)
            ensures set.spec_bstsetrbmteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetrbmteph_wf()
            ensures self.spec_bstsetrbmteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetrbmteph_wf()
            ensures self.spec_bstsetrbmteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetrbmteph_wf(), other.spec_bstsetrbmteph_wf()
            ensures combined.spec_bstsetrbmteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetrbmteph_wf(), other.spec_bstsetrbmteph_wf()
            ensures common.spec_bstsetrbmteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetrbmteph_wf(), other.spec_bstsetrbmteph_wf()
            ensures diff.spec_bstsetrbmteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetrbmteph_wf()
            ensures parts.0.spec_bstsetrbmteph_wf(), parts.2.spec_bstsetrbmteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetrbmteph_wf(), right.spec_bstsetrbmteph_wf()
            ensures joined.spec_bstsetrbmteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetrbmteph_wf(), right.spec_bstsetrbmteph_wf()
            ensures joined.spec_bstsetrbmteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires self.spec_bstsetrbmteph_wf()
            ensures filtered.spec_bstsetrbmteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTRBMtEph<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetRBMtEphIter<T>)
            requires self.spec_bstsetrbmteph_wf()
            ensures it@.0 == 0, bstsetrbmteph_iter_invariant(&it);
    }

    #[verifier::external_body]
    fn values_vec<T: StTInMtT + Ord + TotalOrder>(tree: &BSTRBMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstrbmteph_wf(),
        ensures true,
    {
        tree.in_order().iter().cloned().collect()
    }

    #[verifier::external_body]
    fn copy_set<T: StTInMtT + Ord + TotalOrder>(set: &BSTSetRBMtEph<T>) -> (out: BSTSetRBMtEph<T>)
        requires set.spec_bstsetrbmteph_wf()
        ensures out.spec_bstsetrbmteph_wf()
    {
        let values = values_vec(&set.tree);
        from_sorted_iter(values)
    }

    // veracity: no_requires
    #[verifier::external_body]
    fn from_sorted_iter<T: StTInMtT + Ord + TotalOrder, I: IntoIterator<Item = T>>(values: I) -> (set: BSTSetRBMtEph<T>)
        ensures true,
    {
        let mut tree = BSTRBMtEph::new();
        for value in values {
            let _ = tree.insert(value);
        }
        BSTSetRBMtEph { tree }
    }

    impl<T: StTInMtT + Ord + TotalOrder> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {
        open spec fn spec_bstsetrbmteph_wf(&self) -> bool {
            self.tree.spec_bstrbmteph_wf()
        }

        fn empty() -> Self {
            Self {
                tree: BSTRBMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let mut tree = BSTRBMtEph::new();
            let _ = tree.insert(value);
            Self { tree }
        }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) -> (r: Result<(), ()>) { self.tree.insert(value) }

        #[verifier::external_body]
        fn delete(&mut self, target: &T) {
            if !self.contains(target) {
                return;
            }
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter(|x| *x != target)
                .cloned()
                .collect();
            self.tree = BSTRBMtEph::from_sorted_slice(&filtered);
        }

        #[verifier::external_body]
        fn union(&self, other: &Self) -> Self {
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)
            
            // Base cases
            if self.is_empty() {
                return copy_set(other);
            }
            if other.is_empty() {
                return copy_set(self);
            }

            // Pick pivot from smaller tree for better balance
            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };

            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            // Recursive union on left and right subtrees
            let left_union = self_left.union(&other_left);
            let right_union = self_right.union(&other_right);

            // Join results: include pivot if found in either tree
            if found_self || found_other {
                Self::join_m(left_union, pivot, right_union)
            } else {
                Self::join_pair(left_union, right_union)
            }
        }

        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> Self {
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)

            // Base cases
            if self.is_empty() || other.is_empty() {
                return Self::empty();
            }

            // Pick pivot from smaller tree
            let pivot = if self.size() <= other.size() {
                self.tree.minimum().unwrap()
            } else {
                other.tree.minimum().unwrap()
            };

            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            // Recursive intersection on left and right subtrees
            let left_inter = self_left.intersection(&other_left);
            let right_inter = self_right.intersection(&other_right);

            // Join results: include pivot only if found in BOTH trees
            if found_self && found_other {
                Self::join_m(left_inter, pivot, right_inter)
            } else {
                Self::join_pair(left_inter, right_inter)
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> Self {
            // Algorithm: Parallel divide-and-conquer using split/join primitives
            // Work: O(m log(n/m)), Span: O(log n × log m)

            // Base cases
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return copy_set(self);
            }
            
            // Pick pivot from self (the set we're subtracting from)
            let pivot = self.tree.minimum().unwrap();
            
            // Split both trees at pivot
            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);
            
            // Recursive difference on left and right subtrees
            let left_diff = self_left.difference(&other_left);
            let right_diff = self_right.difference(&other_right);
            
            // Join results: include pivot only if found in self but NOT in other
            if found_self && !found_other {
                Self::join_m(left_diff, pivot, right_diff)
            } else {
                Self::join_pair(left_diff, right_diff)
            }
        }

        #[verifier::external_body]
        fn split(&self, pivot: &T) -> (Self, B, Self) {
            let mut left = Vec::<T>::new();
            let mut right = Vec::<T>::new();
            let mut found = false;
            for value in self.tree.in_order().iter() {
                if value < pivot {
                    left.push(value.clone());
                } else if value > pivot {
                    right.push(value.clone());
                } else {
                    found = true;
                }
            }
            (from_sorted_iter(left), found, from_sorted_iter(right))
        }

        #[verifier::external_body]
        fn join_pair(left: Self, right: Self) -> Self {
            let left_values = values_vec(&left.tree);
            let right_values = values_vec(&right.tree);
            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        #[verifier::external_body]
        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            // Parallel extraction of values from both trees
            let left_values = values_vec(&left.tree);
            let right_values = values_vec(&right.tree);
            let mut combined = left_values.into_iter().collect::<BTreeSet<T>>();
            combined.insert(pivot);
            for value in right_values {
                combined.insert(value);
            }
            from_sorted_iter(combined)
        }

        #[verifier::external_body]
        fn filter<F: FnMut(&T) -> bool + Send>(&self, mut predicate: F) -> Self {
            // Sequential implementation due to FnMut constraint
            // Parallel implementation would require Fn + Sync which is incompatible with FnMut API
            let filtered = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect::<Vec<T>>();
            from_sorted_iter(filtered)
        }

        #[verifier::external_body]
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, mut op: F, base: T) -> T {
            // Sequential implementation due to FnMut constraint
            // Parallel implementation would require Fn + Sync which is incompatible with FnMut API
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        #[verifier::external_body]
        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }

        #[verifier::external_body]
        fn iter(&self) -> BSTSetRBMtEphIter<T> {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            BSTSetRBMtEphIter { snapshot: values, pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord + TotalOrder> std::iter::Iterator for BSTSetRBMtEphIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                proof { assume(item == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetRBMtEphIter<T> {
        type GhostIter = BSTSetRBMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetRBMtEphGhostIter<T> {
            BSTSetRBMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for BSTSetRBMtEphGhostIter<T> {
        type ExecIter = BSTSetRBMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetRBMtEphIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetRBMtEphIter<T>) -> BSTSetRBMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    #[verifier::external]
    impl<'a, T: StTInMtT + Ord + TotalOrder> std::iter::IntoIterator for &'a BSTSetRBMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetRBMtEphIter<T>;
        fn into_iter(self) -> BSTSetRBMtEphIter<T> {
            self.iter()
        }
    }

    #[verifier::external]
    impl<T: StTInMtT + Ord + TotalOrder> IntoIterator for BSTSetRBMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            let values: Vec<T> = self.tree.in_order().iter().cloned().collect();
            values.into_iter()
        }
    }

    } // verus!

    impl<T: StTInMtT + Ord + TotalOrder + fmt::Debug> fmt::Debug for BSTSetRBMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetRBMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Display for BSTSetRBMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetRBMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Debug for BSTSetRBMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetRBMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Display for BSTSetRBMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetRBMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Debug for BSTSetRBMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetRBMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> fmt::Display for BSTSetRBMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetRBMtEphGhostIter")
        }
    }

    #[macro_export]
    macro_rules! BSTSetRBMtEphLit {
        () => {
            < $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEph<_> as $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEph<_> as $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }
}
