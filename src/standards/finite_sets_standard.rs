// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Finite Sets Standard: finiteness is a property of the type, not of wf.
//!
//! At verus 0.2026.09.13, `vstd::set::Set<A>` is finite by construction:
//! `Set::finite()` is `open spec fn finite(self) -> bool { true }` and is
//! `#[deprecated]` (vstd/set.rs:227). The possibly-infinite set type is
//! `vstd::iset::ISet<A>`. Every APAS view that is a `Set<A>`, or a
//! `Map<K, V>` whose `dom()` is a `Set<K>`, is therefore finite without
//! saying so.
//!
//! Rules:
//!
//! 1. Never write `.finite()` on a `Set` or on a `Map::dom()`. It is a
//!    deprecation warning and carries no information; a wf conjunct
//!    `self@.finite()` is `true` in disguise, which spec_wf_standard.rs
//!    forbids. A Vec-only wrapper's wf is `true`; a set-viewing type's wf
//!    states its real invariants only (no duplicates, ordering, key
//!    validity). vstd removed `axiom_set_*_finite`, `lemma_set_subset_finite`,
//!    `lemma_set_union_finite_iff` and `seq_to_set_is_finite`; a proof
//!    statement that called one of them established a tautology and is
//!    deleted, not replaced.
//!
//! 2. `Set::new(f)` returns `Option<Set<A>>` (vstd/set.rs:133): a
//!    comprehension over a bare predicate is a `Set` only if the predicate's
//!    extension is finite, which vstd cannot decide from `f` alone. A spec fn
//!    typed `Set<A>` cannot return `Set::new(f)`, and a `Set<A>` cannot be
//!    compared to `Set::new(f)` with `==`.
//!
//! 3. Preferred: comprehend inside a finite set with `filter`. `s.filter(f)`
//!    is a `Set<A>` (no `Option`) whenever `s` is a `Set<A>`, and
//!    `lemma_set_filter` (in `group_set_lemmas`) gives
//!    `s.filter(f).contains(a) <==> s.contains(a) && f(a)`. Every APAS
//!    comprehension `{x in A | p(x)}` names an enclosing finite set `A`, which
//!    is the collection's own view, so write it as `self@.filter(p)`.
//!
//! 4. When the predicate is not written over an enclosing set, take the
//!    `lemma_set_new_some` route: (a) show `ISet::new(f).finite()`, for
//!    instance with `lemma_iset_finite_if_subset_of_seq` (vstd/iset.rs) or
//!    with `lemma_iset_subset_finite` (vstd/iset_lib.rs) against
//!    `s.to_iset()`, which `lemma_to_iset_finite` makes finite; (b)
//!    `lemma_set_new_some(f)` then gives `Set::new(f) is Some`; (c)
//!    `lemma_set_new(f, a)` gives `Set::new(f).unwrap().contains(a) == f(a)`.
//!    Steps (b) and (c) are broadcast members of `group_set_lemmas`. Bind
//!    the predicate to a named spec fn so that the spec and the proof name
//!    one closure term, as `spec_lt` does below.
//!
//! References:
//! - vstd/set.rs (Set::new, Set::filter, lemma_set_filter, lemma_set_new,
//!   lemma_set_new_some, lemma_to_iset_finite).
//! - vstd/iset.rs (ISet::finite, lemma_iset_new,
//!   lemma_iset_finite_if_subset_of_seq).
//! - vstd/iset_lib.rs (lemma_iset_subset_finite).
//! - src/standards/spec_wf_standard.rs (wf carries real invariants only).

pub mod finite_sets_standard {

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::iset::lemma_iset_finite_if_subset_of_seq;

    verus! {

    // 3. broadcast use
    broadcast use {
        vstd::set::group_set_lemmas,
        vstd::iset::group_iset_lemmas,
    };

    // 4. type definitions

    pub struct FiniteCollection {
        pub elements: Vec<u64>,
    }

    // 5. view impls

    impl View for FiniteCollection {
        type V = Set<u64>;
        open spec fn view(&self) -> Set<u64> {
            self.elements@.to_set()
        }
    }

    // 6. spec fns

    /// Rule 3: a comprehension over an enclosing finite set is a `Set`.
    pub open spec fn spec_evens(s: Set<u64>) -> Set<u64> {
        s.filter(|x: u64| x % 2 == 0)
    }

    /// Rule 4: the predicate as one named closure term, shared by the spec
    /// and its proof.
    pub open spec fn spec_lt(bound: u64) -> spec_fn(u64) -> bool {
        |x: u64| x < bound
    }

    /// Rule 4: a comprehension over a bare predicate is an `Option<Set>`.
    pub open spec fn spec_below(bound: u64) -> Option<Set<u64>> {
        Set::new(spec_lt(bound))
    }

    // 7. proof fns

    /// Rule 3 needs no proof steps: `lemma_set_filter` fires by broadcast.
    pub proof fn lemma_evens_contains(s: Set<u64>, x: u64)
        ensures
            spec_evens(s).contains(x) <==> s.contains(x) && x % 2 == 0,
    {
    }

    /// Rule 4: `spec_lt(bound)`'s extension is a subset of the finite sequence
    /// `0, 1, ..., bound - 1`, so `ISet::new(spec_lt(bound))` is finite,
    /// `Set::new` is `Some`, and membership follows from `lemma_set_new`.
    pub proof fn lemma_below_is_some(bound: u64)
        ensures
            spec_below(bound) is Some,
            forall|x: u64| #[trigger] spec_below(bound).unwrap().contains(x) <==> x < bound,
    {
        let witnesses = Seq::new(bound as nat, |i: int| i as u64);
        assert forall|x: u64| #[trigger] ISet::new(spec_lt(bound)).contains(x)
            implies witnesses.contains(x) by {
            assert(witnesses[x as int] == x);
        }
        lemma_iset_finite_if_subset_of_seq(ISet::new(spec_lt(bound)), witnesses);
    }

    // 8. traits

    pub trait FiniteCollectionTrait: Sized + View<V = Set<u64>> {
        spec fn spec_finitecollection_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s.spec_finitecollection_wf(), s@ == Set::<u64>::empty();

        fn insert(&mut self, x: u64)
            requires old(self).spec_finitecollection_wf(),
            ensures self.spec_finitecollection_wf(), self@ == old(self)@.insert(x);

        fn contains(&self, x: u64) -> (found: bool)
            requires self.spec_finitecollection_wf(),
            ensures found == self@.contains(x);

        fn len(&self) -> (count: usize)
            requires self.spec_finitecollection_wf(),
            ensures count == self@.len();
    }

    // 9. impls

    impl FiniteCollectionTrait for FiniteCollection {
        // Rule 1: no `finite()` conjunct. The real invariant of a Vec-backed
        // set is that the Vec holds no duplicates, so `len()` is the set's.
        open spec fn spec_finitecollection_wf(&self) -> bool {
            self.elements@.no_duplicates()
        }

        fn new() -> (s: Self) {
            let s = FiniteCollection { elements: Vec::new() };
            proof {
                s.elements@.to_set_ensures();
                assert(s@ =~= Set::<u64>::empty());
            }
            s
        }

        fn insert(&mut self, x: u64) {
            if !self.contains(x) {
                proof {
                    self.elements@.to_set_ensures();
                    self.elements@.lemma_push_to_set_commute(x);
                }
                self.elements.push(x);
                proof {
                    assert forall|i: int, j: int|
                        0 <= i < self.elements@.len() && 0 <= j < self.elements@.len() && i != j
                        implies #[trigger] self.elements@[i] != #[trigger] self.elements@[j] by {
                        if i == old(self).elements@.len() || j == old(self).elements@.len() {
                            assert(!old(self).elements@.contains(x));
                        }
                    }
                }
            } else {
                proof {
                    assert(self@ =~= old(self)@.insert(x));
                }
            }
        }

        // A linear scan; its postcondition crosses from the Vec's sequence to
        // the set view through `Seq::to_set_ensures`.
        fn contains(&self, x: u64) -> (found: bool) {
            proof {
                self.elements@.to_set_ensures();
            }
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    i <= self.elements@.len(),
                    forall|j: int| 0 <= j < i ==> #[trigger] self.elements@[j] != x,
                decreases self.elements@.len() - i,
            {
                if self.elements[i] == x {
                    return true;
                }
                i = i + 1;
            }
            false
        }

        fn len(&self) -> (count: usize) {
            proof {
                self.elements@.unique_seq_to_set();
            }
            self.elements.len()
        }
    }

    // ANTIPATTERN: `finite()` anywhere.
    //
    // open spec fn spec_badcollection_wf(&self) -> bool {
    //     self@.finite()          // deprecated; always true; a `requires true`
    // }
    // fn size(&self) -> (count: usize)
    //     requires self.spec_badcollection_wf(),
    // {
    //     proof { assert(self@.finite()); }  // a tautology, and a warning
    //     ...
    // }
    //
    // ANTIPATTERN: unwrapping a comprehension over a bare predicate.
    //
    // open spec fn spec_neighbors(&self, v: u64) -> Set<u64> {
    //     Set::new(|w: u64| self.spec_has_arc(v, w)).unwrap()   // Some? unproved
    // }
    // Write it over the enclosing finite set instead:
    //     self.spec_vertices().filter(|w: u64| self.spec_has_arc(v, w))

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for FiniteCollection {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "FiniteCollection(len={})", self.elements.len())
        }
    }
    impl std::fmt::Display for FiniteCollection {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "FiniteCollection(len={})", self.elements.len())
        }
    }
}
