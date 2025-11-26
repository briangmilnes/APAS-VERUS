//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation built on `SetStEphWithIter<Pair<A,B>>`.
//! This version uses SetStEphWithIter internally.

pub mod RelationStEphWithIter {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap05::SetStEphWithIter::SetStEphWithIter::*;
    use crate::Types::Types::*;

    broadcast use {
        vstd::seq_lib::group_seq_properties, 
        vstd::seq::group_seq_axioms,
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms
    };

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct RelationStEphWithIter<A: StT + Hash, B: StT + Hash> {
        pub pairs: SetStEphWithIter<Pair<A, B>>,
    }

    // Iterator wrapper - delegates to SetStEphIter
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct RelationStEphIter<'a, X: StT + Hash, Y: StT + Hash> {
        pub inner: SetStEphIter<'a, Pair<X, Y>>,
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> View for RelationStEphIter<'a, X, Y> {
        type V = SetStEphIterView<(X::V, Y::V)>;
        open spec fn view(&self) -> SetStEphIterView<(X::V, Y::V)> { self.inner@ }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> RelationStEphIter<'a, X, Y> {
        pub fn next(&mut self) -> (result: Option<&'a Pair<X, Y>>)
            requires
                old(self).inner.inner@.1.no_duplicates(),
                obeys_feq_full::<Pair<X, Y>>(),
            ensures
                self.inner.inner@.1 == old(self).inner.inner@.1,
                self.inner.inner@.1.no_duplicates(),
                ({
                    let old_view = old(self)@;
                    let new_view = self@;
                    match result {
                        None => {
                            &&& old_view.remaining.is_empty()
                            &&& new_view == old_view
                        },
                        Some(element) => {
                            &&& old_view.remaining.contains(element@)
                            &&& new_view.visited == old_view.visited.union(option_to_set(old_view.current))
                            &&& new_view.current == Some(element@)
                            &&& new_view.remaining == old_view.remaining.remove(element@)
                        },
                    }
                })
        {
            self.inner.next()
        }
    }

    pub trait RelationStEphWithIterTrait<X: StT + Hash, Y: StT + Hash> : 
        View<V = Set<(<X as View>::V, <Y as View>::V)>> + Sized {

        fn empty() -> (empty: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures empty@ == Set::<(<X as View>::V, <Y as View>::V)>::empty();

        fn FromSet(pairs: SetStEphWithIter<Pair<X, Y>>) -> (relation: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures relation@ == pairs@;

        fn FromVec(v: Vec<Pair<X, Y>>) -> (relation: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures relation@ == v@.map(|i: int, p: Pair<X, Y>| p@).to_set();

        fn size(&self) -> N;

        fn domain(&self) -> (domain: SetStEphWithIter<X>)
            requires valid_key_type_Pair::<X, Y>()
            ensures domain@ == Set::<X::V>::new(|x: X::V| exists |y: Y::V| self@.contains((x, y)));

        fn range(&self) -> (range: SetStEphWithIter<Y>)
            requires valid_key_type_Pair::<X, Y>()
            ensures range@ == Set::<Y::V>::new(|y: Y::V| exists |x: X::V| self@.contains((x, y)));

        fn mem(&self, a: &X, b: &Y) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>()
            ensures contains == self@.contains((a@, b@));

        fn relates(&self, p: &Pair<X, Y>) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>()
            ensures contains == self@.contains(p@);

        fn iter<'a>(&'a self) -> (it: RelationStEphIter<'a, X, Y>)
            requires valid_key_type_Pair::<X, Y>()
            ensures
                it@.visited == Set::<<Pair<X, Y> as View>::V>::empty(),
                it@.current.is_none(),
                it@.remaining == self@,
                it.inner.inner@.1.no_duplicates();
    }

    impl<A: StT + Hash, B: StT + Hash> View for RelationStEphWithIter<A, B> {
        type V = Set<(<A as View>::V, <B as View>::V)>;
        open spec fn view(&self) -> Self::V { self.pairs@ }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for RelationStEphWithIter<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { RelationStEphWithIter { pairs: self.pairs.clone() } }
    }

    impl<X: StT + Hash, Y: StT + Hash> 
        RelationStEphWithIterTrait<X, Y> for RelationStEphWithIter<X, Y> {

        fn empty() -> RelationStEphWithIter<X, Y> { 
            RelationStEphWithIter { pairs: SetStEphWithIter::empty() }
        }

        fn FromSet(pairs: SetStEphWithIter<Pair<X, Y>>) -> RelationStEphWithIter<X, Y> { 
            RelationStEphWithIter { pairs } 
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> RelationStEphWithIter<X, Y> {
            RelationStEphWithIter { pairs: SetStEphWithIter::FromVec(v) }
        }

        fn size(&self) -> N { self.pairs.size() }

        fn domain(&self) -> SetStEphWithIter<X> {
            let mut out = SetStEphWithIter::<X>::empty();
            let mut it = self.iter();
            let ghost pairs_view = self@;
            let ghost the_seq = it.inner.inner@.1;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    it.inner.inner@.1 == the_seq,
                    the_seq.no_duplicates(),
                    the_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == pairs_view,
                    out@ == Set::<X::V>::new(|x: X::V| 
                        exists |y: Y::V| it@.visited.union(option_to_set(it@.current)).contains((x, y))),
                decreases it@.remaining.len(),
            {
                let ghost old_visited = it@.visited;
                let ghost old_current = it@.current;
                let ghost old_processed = old_visited.union(option_to_set(old_current));
                let ghost old_out = out@;
                
                match it.next() {
                    Some(pair) => {
                        let Pair(a, _b) = pair;
                        proof {
                            // After next(): new_visited = old_visited ∪ {old_current}, new_current = Some(pair@)
                            let new_processed = it@.visited.union(option_to_set(it@.current));
                            assert(new_processed =~= old_processed.insert(pair@));
                        }
                        let a_clone = a.clone_plus();
                        let _ = out.insert(a_clone);
                        proof {
                            // After insert: out@ = old_out@ ∪ {a@}
                            // Need: out@ == Set::new(|x| exists |y| new_processed.contains((x, y)))
                            let new_processed = it@.visited.union(option_to_set(it@.current));
                            
                            // old_out@ == Set::new(|x| exists |y| old_processed.contains((x, y)))
                            // new_processed == old_processed ∪ {pair@}
                            // pair@ == (a@, _b@)
                            
                            // Show the two sets are equal
                            assert forall |x: X::V| out@.contains(x) <==> 
                                (exists |y: Y::V| new_processed.contains((x, y))) by {
                                if out@.contains(x) {
                                    if x == a@ {
                                        // pair@ is in new_processed
                                        assert(new_processed.contains(pair@));
                                    } else {
                                        // x was in old_out, so exists y in old_processed
                                        let y = choose |y: Y::V| old_processed.contains((x, y));
                                        assert(new_processed.contains((x, y)));
                                    }
                                }
                                if exists |y: Y::V| new_processed.contains((x, y)) {
                                    let y = choose |y: Y::V| new_processed.contains((x, y));
                                    if (x, y) == pair@ {
                                        assert(x == a@);
                                        assert(out@.contains(a@));
                                    } else {
                                        // (x, y) was in old_processed
                                        assert(old_processed.contains((x, y)));
                                        assert(old_out.contains(x));
                                        assert(out@.contains(x));
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        proof {
                            assert(it@.remaining.is_empty());
                            let processed = it@.visited.union(option_to_set(it@.current));
                            assert(processed =~= pairs_view);
                            assert forall |x: X::V| out@.contains(x) implies 
                                (exists |y: Y::V| self@.contains((x, y))) by {
                                if out@.contains(x) {
                                    let y = choose |y: Y::V| processed.contains((x, y));
                                    assert(pairs_view.contains((x, y)));
                                }
                            }
                            assert forall |x: X::V| (exists |y: Y::V| self@.contains((x, y))) implies 
                                out@.contains(x) by {
                                if exists |y: Y::V| self@.contains((x, y)) {
                                    let y = choose |y: Y::V| self@.contains((x, y));
                                    assert(processed.contains((x, y)));
                                }
                            }
                        }
                        return out;
                    }
                }
            }
        }

        fn range(&self) -> SetStEphWithIter<Y> {
            let mut out = SetStEphWithIter::<Y>::empty();
            let mut it = self.iter();
            let ghost pairs_view = self@;
            let ghost the_seq = it.inner.inner@.1;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    it.inner.inner@.1 == the_seq,
                    the_seq.no_duplicates(),
                    the_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == pairs_view,
                    out@ == Set::<Y::V>::new(|y: Y::V| 
                        exists |x: X::V| it@.visited.union(option_to_set(it@.current)).contains((x, y))),
                decreases it@.remaining.len(),
            {
                let ghost old_visited = it@.visited;
                let ghost old_current = it@.current;
                let ghost old_processed = old_visited.union(option_to_set(old_current));
                let ghost old_out = out@;
                
                match it.next() {
                    Some(pair) => {
                        let Pair(_a, b) = pair;
                        proof {
                            let new_processed = it@.visited.union(option_to_set(it@.current));
                            assert(new_processed =~= old_processed.insert(pair@));
                        }
                        let b_clone = b.clone_plus();
                        let _ = out.insert(b_clone);
                        proof {
                            let new_processed = it@.visited.union(option_to_set(it@.current));
                            
                            assert forall |y: Y::V| out@.contains(y) <==> 
                                (exists |x: X::V| new_processed.contains((x, y))) by {
                                if out@.contains(y) {
                                    if y == b@ {
                                        assert(new_processed.contains(pair@));
                                    } else {
                                        let x = choose |x: X::V| old_processed.contains((x, y));
                                        assert(new_processed.contains((x, y)));
                                    }
                                }
                                if exists |x: X::V| new_processed.contains((x, y)) {
                                    let x = choose |x: X::V| new_processed.contains((x, y));
                                    if (x, y) == pair@ {
                                        assert(y == b@);
                                        assert(out@.contains(b@));
                                    } else {
                                        assert(old_processed.contains((x, y)));
                                        assert(old_out.contains(y));
                                        assert(out@.contains(y));
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        proof {
                            assert(it@.remaining.is_empty());
                            let processed = it@.visited.union(option_to_set(it@.current));
                            assert(processed =~= pairs_view);
                            assert forall |y: Y::V| out@.contains(y) implies 
                                (exists |x: X::V| self@.contains((x, y))) by {
                                if out@.contains(y) {
                                    let x = choose |x: X::V| processed.contains((x, y));
                                    assert(pairs_view.contains((x, y)));
                                }
                            }
                            assert forall |y: Y::V| (exists |x: X::V| self@.contains((x, y))) implies 
                                out@.contains(y) by {
                                if exists |x: X::V| self@.contains((x, y)) {
                                    let x = choose |x: X::V| self@.contains((x, y));
                                    assert(processed.contains((x, y)));
                                }
                            }
                        }
                        return out;
                    }
                }
            }
        }

        fn mem(&self, a: &X, b: &Y) -> B {
            let a_clone = a.clone_plus();
            let b_clone = b.clone_plus();
            self.pairs.mem(&Pair(a_clone, b_clone))
        }

        fn relates(&self, p: &Pair<X, Y>) -> B {
            self.mem(&p.0, &p.1)
        }

        fn iter(&self) -> RelationStEphIter<'_, X, Y> {
            RelationStEphIter { inner: self.pairs.iter() }
        }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for RelationStEphWithIter<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.pairs.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for RelationStEphWithIter<A, B> {}

  } // verus!

    impl<A: StT + Hash, B: StT + Hash> PartialEq for RelationStEphWithIter<A, B> {
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for RelationStEphWithIter<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for RelationStEphWithIter<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) }
    }
}
