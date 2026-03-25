//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Plain multi-threaded BST implementation.

pub mod BSTSetPlainMtEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
    use crate::Types::Types::*;

    verus! {

    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetPlainMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTPlainMtEph<T>,
    }

    pub type BSTSetPlainMt<T> = BSTSetPlainMtEph<T>;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetPlainMtEphIter<T: StTInMtT + Ord + TotalOrder> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetPlainMtEphGhostIter<T: StTInMtT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetPlainMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetPlainMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetplainmteph_iter_invariant<T: StTInMtT + Ord + TotalOrder>(it: &BSTSetPlainMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub trait BSTSetPlainMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetplainmteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetplainmteph_wf();
        fn singleton(value: T) -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetplainmteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetplainmteph_wf()
            ensures self.spec_bstsetplainmteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetplainmteph_wf()
            ensures self.spec_bstsetplainmteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetplainmteph_wf(), other.spec_bstsetplainmteph_wf()
            ensures combined.spec_bstsetplainmteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetplainmteph_wf(), other.spec_bstsetplainmteph_wf()
            ensures common.spec_bstsetplainmteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetplainmteph_wf(), other.spec_bstsetplainmteph_wf()
            ensures diff.spec_bstsetplainmteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetplainmteph_wf()
            ensures parts.0.spec_bstsetplainmteph_wf(), parts.2.spec_bstsetplainmteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetplainmteph_wf(), right.spec_bstsetplainmteph_wf()
            ensures joined.spec_bstsetplainmteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetplainmteph_wf(), right.spec_bstsetplainmteph_wf()
            ensures joined.spec_bstsetplainmteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires
                self.spec_bstsetplainmteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures filtered.spec_bstsetplainmteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires
                self.spec_bstsetplainmteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTPlainMtEph<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetPlainMtEphIter<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures it@.0 == 0, bstsetplainmteph_iter_invariant(&it);
    }

    fn values_vec<T: StTInMtT + Ord + TotalOrder>(tree: &BSTPlainMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstplainmteph_wf(), obeys_feq_clone::<T>(),
        ensures true,
    {
        let seq = tree.in_order();
        let n = seq.length();
        let mut out = Vec::<T>::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                n as int == seq.spec_len(),
            decreases n - i,
        {
            out.push(seq.nth(i).clone());
            i = i + 1;
        }
        out
    }
    // veracity: no_requires
    fn rebuild_from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (tree: BSTPlainMtEph<T>)
        ensures tree.spec_bstplainmteph_wf(),
    {
        let mut tree = BSTPlainMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstplainmteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        tree
    }

    fn from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (set: BSTSetPlainMtEph<T>)
        requires obeys_feq_clone::<T>(),
        ensures set.spec_bstsetplainmteph_wf(),
    {
        let mut tree = BSTPlainMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstplainmteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        BSTSetPlainMtEph { tree }
    }

    fn copy_set<T: StTInMtT + Ord + TotalOrder>(set: &BSTSetPlainMtEph<T>) -> (out: BSTSetPlainMtEph<T>)
        requires set.spec_bstsetplainmteph_wf()
        ensures out.spec_bstsetplainmteph_wf()
    {
        from_vec(values_vec(&set.tree))
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> BSTSetPlainMtEphTrait<T> for BSTSetPlainMtEph<T> {
        open spec fn spec_bstsetplainmteph_wf(&self) -> bool {
            self.tree.spec_bstplainmteph_wf() && obeys_feq_clone::<T>()
        }

        fn empty() -> Self {
            Self {
                tree: BSTPlainMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let mut tree = BSTPlainMtEph::new();
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

        fn delete(&mut self, target: &T) {
            if !self.contains(target) {
                return;
            }
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut filtered = Vec::<T>::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                decreases n - i,
            {
                if *seq.nth(i) != *target {
                    filtered.push(seq.nth(i).clone());
                }
                i = i + 1;
            }
            self.tree = rebuild_from_vec(filtered);
        }

        #[verifier::exec_allows_no_decreases_clause]
        fn union(&self, other: &Self) -> Self {
            if self.is_empty() {
                return copy_set(other);
            }
            if other.is_empty() {
                return copy_set(self);
            }

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return copy_set(other); }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_left.union(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_right.union(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_union, right_union) = crate::ParaPair!(f1, f2);

            if found_self || found_other {
                Self::join_m(left_union, pivot, right_union)
            } else {
                Self::join_pair(left_union, right_union)
            }
        }

        #[verifier::exec_allows_no_decreases_clause]
        fn intersection(&self, other: &Self) -> Self {
            if self.is_empty() || other.is_empty() {
                return Self::empty();
            }

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return Self::empty(); }
            };

            let (self_left, found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_left.intersection(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_right.intersection(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_inter, right_inter) = crate::ParaPair!(f1, f2);

            if found_self && found_other {
                Self::join_m(left_inter, pivot, right_inter)
            } else {
                Self::join_pair(left_inter, right_inter)
            }
        }

        #[verifier::exec_allows_no_decreases_clause]
        fn difference(&self, other: &Self) -> Self {
            if self.is_empty() {
                return Self::empty();
            }
            if other.is_empty() {
                return copy_set(self);
            }

            let pivot = match self.tree.minimum() {
                Some(v) => v,
                None => { return Self::empty(); }
            };

            let (self_left, _found_self, self_right) = self.split(&pivot);
            let (other_left, found_other, other_right) = other.split(&pivot);

            let f1 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_left.difference(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetplainmteph_wf()
            { self_right.difference(&other_right) };
            use crate::Types::Types::Pair;
            let Pair(left_diff, right_diff) = crate::ParaPair!(f1, f2);

            if found_other {
                Self::join_pair(left_diff, right_diff)
            } else {
                Self::join_m(left_diff, pivot, right_diff)
            }
        }

        fn split(&self, pivot: &T) -> (Self, B, Self) {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut left = Vec::<T>::new();
            let mut right = Vec::<T>::new();
            let mut found = false;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                decreases n - i,
            {
                let value = seq.nth(i);
                match TotalOrder::cmp(value, pivot) {
                    core::cmp::Ordering::Less => { left.push(value.clone()); }
                    core::cmp::Ordering::Greater => { right.push(value.clone()); }
                    core::cmp::Ordering::Equal => { found = true; }
                }
                i = i + 1;
            }
            (from_vec(left), found, from_vec(right))
        }

        fn join_pair(left: Self, right: Self) -> Self {
            let mut combined = values_vec(&left.tree);
            let right_vals = values_vec(&right.tree);
            let mut i: usize = 0;
            while i < right_vals.len()
                invariant i <= right_vals.len(),
                decreases right_vals.len() - i,
            {
                combined.push(right_vals[i].clone());
                i = i + 1;
            }
            from_vec(combined)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined = values_vec(&left.tree);
            combined.push(pivot);
            let right_vals = values_vec(&right.tree);
            let mut i: usize = 0;
            while i < right_vals.len()
                invariant i <= right_vals.len(),
                decreases right_vals.len() - i,
            {
                combined.push(right_vals[i].clone());
                i = i + 1;
            }
            from_vec(combined)
        }

        fn filter<F>(&self, mut predicate: F) -> Self
        where
            F: FnMut(&T) -> bool,
        {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut filtered = Vec::<T>::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                    forall|t: &T| #[trigger] predicate.requires((t,)),
                decreases n - i,
            {
                if predicate(seq.nth(i)) {
                    filtered.push(seq.nth(i).clone());
                }
                i = i + 1;
            }
            from_vec(filtered)
        }

        fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            let seq = self.tree.in_order();
            let n = seq.length();
            let mut acc = base;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == seq.spec_len(),
                    forall|a: T, b: T| #[trigger] op.requires((a, b)),
                decreases n - i,
            {
                acc = op(acc, seq.nth(i).clone());
                i = i + 1;
            }
            acc
        }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTPlainMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetPlainMtEphIter<T> {
            BSTSetPlainMtEphIter { snapshot: values_vec(&self.tree), pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord + TotalOrder> std::iter::Iterator for BSTSetPlainMtEphIter<T> {
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

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetPlainMtEphIter<T> {
        type GhostIter = BSTSetPlainMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetPlainMtEphGhostIter<T> {
            BSTSetPlainMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for BSTSetPlainMtEphGhostIter<T> {
        type ExecIter = BSTSetPlainMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetPlainMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetPlainMtEphIter<T>) -> BSTSetPlainMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a BSTSetPlainMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetPlainMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetPlainMtEphIter<T>)
            requires self.spec_bstsetplainmteph_wf()
            ensures it@.0 == 0, bstsetplainmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> IntoIterator for BSTSetPlainMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_bstsetplainmteph_wf()
        {
            values_vec(&self.tree).into_iter()
        }
    }

    } // verus!

    impl<T: StTInMtT + Ord + TotalOrder + std::fmt::Debug> std::fmt::Debug for BSTSetPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Display for BSTSetPlainMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetPlainMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetPlainMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetPlainMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetPlainMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetPlainMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetPlainMtEphGhostIter")
        }
    }

    #[macro_export]
    macro_rules! BSTSetPlainMtEphLit {
        () => {
            < $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEph<_> as $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEph<_> as $crate::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }
}
