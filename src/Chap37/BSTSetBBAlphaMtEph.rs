//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the BB-Alpha multi-threaded BST implementation.

pub mod BSTSetBBAlphaMtEph {

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
    use crate::Types::Types::*;

    verus! {

    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEph<T: StTInMtT + Ord + TotalOrder> {
        pub tree: BSTBBAlphaMtEph<T>,
    }

    pub type BSTSetBBAlphaMt<T> = BSTSetBBAlphaMtEph<T>;

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEphIter<T: StTInMtT + Ord + TotalOrder> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct BSTSetBBAlphaMtEphGhostIter<T: StTInMtT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetBBAlphaMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> View for BSTSetBBAlphaMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn bstsetbbalphamteph_iter_invariant<T: StTInMtT + Ord + TotalOrder>(it: &BSTSetBBAlphaMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    pub trait BSTSetBBAlphaMtEphTrait<T: StTInMtT + Ord + TotalOrder>: Sized {
        spec fn spec_bstsetbbalphamteph_wf(&self) -> bool;

        fn empty() -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetbbalphamteph_wf();
        fn singleton(value: T) -> (set: Self)
            requires obeys_feq_clone::<T>()
            ensures set.spec_bstsetbbalphamteph_wf();
        fn size(&self) -> (n: N)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn is_empty(&self) -> (b: B)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn find(&self, value: &T) -> (found: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn contains(&self, value: &T) -> (found: B)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn minimum(&self) -> (min: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn maximum(&self) -> (max: Option<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn insert(&mut self, value: T) -> (r: Result<(), ()>)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        fn delete(&mut self, target: &T)
            requires old(self).spec_bstsetbbalphamteph_wf()
            ensures self.spec_bstsetbbalphamteph_wf();
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures combined.spec_bstsetbbalphamteph_wf();
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures common.spec_bstsetbbalphamteph_wf();
        fn difference(&self, other: &Self) -> (diff: Self)
            requires self.spec_bstsetbbalphamteph_wf(), other.spec_bstsetbbalphamteph_wf()
            ensures diff.spec_bstsetbbalphamteph_wf();
        fn split(&self, pivot: &T) -> (parts: (Self, B, Self))
            requires self.spec_bstsetbbalphamteph_wf()
            ensures parts.0.spec_bstsetbbalphamteph_wf(), parts.2.spec_bstsetbbalphamteph_wf();
        fn join_pair(left: Self, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        fn join_m(left: Self, pivot: T, right: Self) -> (joined: Self)
            requires left.spec_bstsetbbalphamteph_wf(), right.spec_bstsetbbalphamteph_wf()
            ensures joined.spec_bstsetbbalphamteph_wf();
        fn filter<F: FnMut(&T) -> bool + Send>(&self, predicate: F) -> (filtered: Self)
            requires
                self.spec_bstsetbbalphamteph_wf(),
                forall|t: &T| #[trigger] predicate.requires((t,)),
            ensures filtered.spec_bstsetbbalphamteph_wf();
        fn reduce<F: FnMut(T, T) -> T + Send>(&self, op: F, base: T) -> (reduced: T)
            requires
                self.spec_bstsetbbalphamteph_wf(),
                forall|a: T, b: T| #[trigger] op.requires((a, b)),
            ensures true;
        fn iter_in_order(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn as_tree(&self) -> (tree: &BSTBBAlphaMtEph<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures true;
        fn iter(&self) -> (it: BSTSetBBAlphaMtEphIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures it@.0 == 0, bstsetbbalphamteph_iter_invariant(&it);
    }

    fn values_vec<T: StTInMtT + Ord + TotalOrder>(tree: &BSTBBAlphaMtEph<T>) -> (values: Vec<T>)
        requires tree.spec_bstbbalphamteph_wf(), obeys_feq_clone::<T>(),
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
    fn rebuild_from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (tree: BSTBBAlphaMtEph<T>)
        ensures tree.spec_bstbbalphamteph_wf(),
    {
        let mut tree = BSTBBAlphaMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstbbalphamteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        tree
    }

    fn from_vec<T: StTInMtT + Ord + TotalOrder>(values: Vec<T>) -> (set: BSTSetBBAlphaMtEph<T>)
        requires obeys_feq_clone::<T>(),
        ensures set.spec_bstsetbbalphamteph_wf(),
    {
        let mut tree = BSTBBAlphaMtEph::new();
        let mut i: usize = 0;
        while i < values.len()
            invariant tree.spec_bstbbalphamteph_wf(),
                i <= values.len(),
            decreases values.len() - i,
        {
            let value = values[i].clone();
            let _ = tree.insert(value);
            i = i + 1;
        }
        BSTSetBBAlphaMtEph { tree }
    }

    fn copy_set<T: StTInMtT + Ord + TotalOrder>(set: &BSTSetBBAlphaMtEph<T>) -> (out: BSTSetBBAlphaMtEph<T>)
        requires set.spec_bstsetbbalphamteph_wf()
        ensures out.spec_bstsetbbalphamteph_wf()
    {
        from_vec(values_vec(&set.tree))
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> BSTSetBBAlphaMtEphTrait<T> for BSTSetBBAlphaMtEph<T> {
        open spec fn spec_bstsetbbalphamteph_wf(&self) -> bool {
            self.tree.spec_bstbbalphamteph_wf() && obeys_feq_clone::<T>()
        }

        fn empty() -> Self {
            Self {
                tree: BSTBBAlphaMtEph::new(),
            }
        }

        fn singleton(value: T) -> Self {
            let mut tree = BSTBBAlphaMtEph::new();
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
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.union(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
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
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.intersection(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
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
                ensures r.spec_bstsetbbalphamteph_wf()
            { self_left.difference(&other_left) };
            let f2 = move || -> (r: Self)
                ensures r.spec_bstsetbbalphamteph_wf()
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

        fn as_tree(&self) -> &BSTBBAlphaMtEph<T> { &self.tree }

        fn iter(&self) -> BSTSetBBAlphaMtEphIter<T> {
            BSTSetBBAlphaMtEphIter { snapshot: values_vec(&self.tree), pos: 0 }
        }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord + TotalOrder> std::iter::Iterator for BSTSetBBAlphaMtEphIter<T> {
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

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for BSTSetBBAlphaMtEphIter<T> {
        type GhostIter = BSTSetBBAlphaMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> BSTSetBBAlphaMtEphGhostIter<T> {
            BSTSetBBAlphaMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for BSTSetBBAlphaMtEphGhostIter<T> {
        type ExecIter = BSTSetBBAlphaMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &BSTSetBBAlphaMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &BSTSetBBAlphaMtEphIter<T>) -> BSTSetBBAlphaMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + TotalOrder + 'static> std::iter::IntoIterator for &'a BSTSetBBAlphaMtEph<T> {
        type Item = T;
        type IntoIter = BSTSetBBAlphaMtEphIter<T>;
        fn into_iter(self) -> (it: BSTSetBBAlphaMtEphIter<T>)
            requires self.spec_bstsetbbalphamteph_wf()
            ensures it@.0 == 0, bstsetbbalphamteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> IntoIterator for BSTSetBBAlphaMtEph<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_bstsetbbalphamteph_wf()
        {
            values_vec(&self.tree).into_iter()
        }
    }

    } // verus!

    impl<T: StTInMtT + Ord + TotalOrder + fmt::Debug> fmt::Debug for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEph").field("tree", &self.tree).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder + 'static> fmt::Display for BSTSetBBAlphaMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BSTSetBBAlphaMtEph(size={})", self.size())
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetBBAlphaMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEphIter").field("pos", &self.pos).finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetBBAlphaMtEphIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetBBAlphaMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Debug for BSTSetBBAlphaMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BSTSetBBAlphaMtEphGhostIter").finish()
        }
    }

    impl<T: StTInMtT + Ord + TotalOrder> std::fmt::Display for BSTSetBBAlphaMtEphGhostIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BSTSetBBAlphaMtEphGhostIter")
        }
    }

    #[macro_export]
    macro_rules! BSTSetBBAlphaMtEphLit {
        () => {
            < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph<_> as $crate::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEphTrait<_> >::empty();
            $( let _ = __set.insert($x); )*
            __set
        }};
    }
}
