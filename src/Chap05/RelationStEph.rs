//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation built on SetStEph<Pair<A,B>>.

pub mod RelationStEph {
    use vstd::prelude::*;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::set_with_view::SetWithView::SetWithView;
    use crate::Types::Types::*;

    verus! {

#[cfg(verus_keep_ghost)]
broadcast use crate::Types::Types::group_pair_axioms;

#[verifier::ext_equal]
#[verifier::reject_recursive_types(T)]
#[verifier::reject_recursive_types(U)]
pub struct RelationStEph<T: StT + Hash, U: StT + Hash> {
    pub pairs: SetStEph<Pair<T, U>>,
}

impl<T: StT + Hash, U: StT + Hash> View for RelationStEph<T, U> {
    type V = Set<(<T as View>::V, <U as View>::V)>;

    open spec fn view(&self) -> Self::V {
        self.pairs@
    }
}

pub trait RelationStEphTrait<T: StT + Hash, U: StT + Hash>: Sized + View<V = Set<(T::V, U::V)>> {
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> Self
        requires vstd::std_specs::hash::obeys_key_model::<crate::Types::Types::Pair<T, U>>();

    /// APAS: Work Θ(1), Span Θ(1)
    fn size(&self) -> (result: N)
        ensures result == self.spec_view().len();

    /// APAS: Work Θ(1), Span Θ(1)
    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.spec_view().contains((t@, u@));

    /// APAS: Work Θ(1), Span Θ(1)
    fn insert(&mut self, t: T, u: U)
        ensures self.spec_view() == old(self).spec_view().insert((t@, u@));

    fn FromSet(pairs: SetStEph<Pair<T, U>>) -> (result: Self)
        ensures result@ == pairs@;

    fn FromVec(v: Vec<Pair<T, U>>) -> Self
        requires vstd::std_specs::hash::obeys_key_model::<crate::Types::Types::Pair<T, U>>();

    fn domain(&self) -> (result: SetStEph<T>)
        requires vstd::std_specs::hash::obeys_key_model::<T>()
        ensures forall |t: T::V| result@.contains(t) <==> exists |u: U::V| self@.contains((t, u));

    fn range(&self) -> (result: SetStEph<U>)
        requires vstd::std_specs::hash::obeys_key_model::<U>()
        ensures forall |u: U::V| result@.contains(u) <==> exists |t: T::V| self@.contains((t, u));

    fn iter(&self) -> crate::Types::Types::PairIter<'_, T, U>;

    open spec fn spec_view(&self) -> Set<(<T as View>::V, <U as View>::V)> {
        self@
    }
}

impl<T: StT + Hash, U: StT + Hash> RelationStEphTrait<T, U> for RelationStEph<T, U> {
    fn empty() -> (result: Self)
        ensures result.spec_view() == Set::<(<T as View>::V, <U as View>::V)>::empty()
    {
        RelationStEph {
            pairs: SetStEph::empty(),
        }
    }

    fn size(&self) -> (result: N)
        ensures result == self.spec_view().len()
    {
        self.pairs.size()
    }

    #[verifier::external_body]
    fn mem(&self, t: &T, u: &U) -> (result: B)
        ensures result == self.spec_view().contains((t@, u@))
    {
        // TODO hole-5: Need axiom that clone() preserves view for StT types
        // Issue: vstd has strictly_cloned() via call_ensures(T::clone, (&a,), b)
        // but can't use it directly here. Need to teach vstdplus about this.
        let pair = Pair(t.clone(), u.clone());
        self.pairs.mem(&pair)
    }

    fn insert(&mut self, t: T, u: U)
        ensures self.spec_view() == old(self).spec_view().insert((t@, u@))
    {
        self.pairs.insert(Pair(t, u));
    }

    fn FromSet(pairs: SetStEph<Pair<T, U>>) -> (result: Self)
        ensures result@ == pairs@
    {
        RelationStEph { pairs }
    }

    #[verifier::external_body]
    fn FromVec(v: Vec<Pair<T, U>>) -> Self {
        RelationStEph {
            pairs: SetStEphTrait::FromVec(v),
        }
    }

    // TODO: Remove external_body once Verus supports ForLoopGhostIterator for newtype wrappers
    // The issue: PairIter wraps hash_set::Iter<Pair<T,U>>, but Verus doesn't recognize the
    // ForLoopGhostIteratorNew impl on newtypes in for loops (orphan rule workaround limitation)
    #[verifier::external_body]
    fn domain(&self) -> (result: SetStEph<T>)
        ensures forall |t: T::V| result@.contains(t) <==> exists |u: U::V| self@.contains((t, u))
    {
        let mut out = SetStEph::<T>::empty();
        for pair in self.iter() {
            out.insert(pair.0.clone());
        }
        out
    }

    // TODO: Remove external_body once Verus supports ForLoopGhostIterator for newtype wrappers
    #[verifier::external_body]
    fn range(&self) -> (result: SetStEph<U>)
        ensures forall |u: U::V| result@.contains(u) <==> exists |t: T::V| self@.contains((t, u))
    {
        let mut out = SetStEph::<U>::empty();
        for pair in self.iter() {
            out.insert(pair.1.clone());
        }
        out
    }

    fn iter(&self) -> crate::Types::Types::PairIter<'_, T, U> {
        crate::Types::Types::PairIter(self.pairs.iter())
    }
}

    } // verus!

    // Pedagogical runtime trait implementations
    use std::fmt::{Debug, Display, Formatter, Result};

    impl<T: StT + Hash, U: StT + Hash> Clone for RelationStEph<T, U> {
        fn clone(&self) -> Self {
            RelationStEph {
                pairs: self.pairs.clone(),
            }
        }
    }

    impl<T: StT + Hash, U: StT + Hash> PartialEq for RelationStEph<T, U> {
        fn eq(&self, other: &Self) -> bool {
            self.pairs == other.pairs
        }
    }

    impl<T: StT + Hash, U: StT + Hash> Eq for RelationStEph<T, U> {}

    impl<T: StT + Hash, U: StT + Hash> Debug for RelationStEph<T, U> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_set().entries(self.pairs.iter()).finish()
        }
    }

    impl<T: StT + Hash, U: StT + Hash> Display for RelationStEph<T, U> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            let mut first = true;
            for pair in self.pairs.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{pair}")?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::empty()
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::FromVec(
                vec![ $( $crate::Types::Types::Pair($a, $b) ),* ]
            )
        }};
    }
}
