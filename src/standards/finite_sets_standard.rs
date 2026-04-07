//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Finite Sets Standard: finiteness flows through wf, not assumes.
//!
//! Rule: if a data structure views as Set<A>, Seq<A>, or Map<K,V>, then
//! self@.finite() (or self@.dom().finite() for maps) MUST be part of
//! spec_X_wf. No exceptions. No alternatives. No "ensure it per-method."
//!
//! Why: every APAS data structure is backed by a finite representation (Vec,
//! Box-linked tree, HashMap). There is no infinite APAS collection. Finiteness
//! is a structural invariant of the type, not a per-call property to be proved
//! or assumed separately. It belongs in wf alongside other structural facts.
//!
//! What this buys you:
//! - Every method that requires wf gets self@.finite() for free.
//! - Every method that ensures wf gives callers finite() for free.
//! - vstd broadcast lemmas fire automatically: insert, remove, union,
//!   intersect, difference all preserve finiteness when the input is finite.
//!   With finite() in wf, the broadcasts trigger without manual proof steps.
//! - No redundant `ensures self@.finite()` on every method. The wf ensures
//!   already carries it.
//! - Mt wrappers get finiteness through the RwLockPredicate inv, which
//!   requires the inner value's wf (which includes finite). No lock-boundary
//!   assume needed for finiteness — it comes from the predicate.
//!
//! Correct example (SetStEph):
//!   open spec fn spec_setsteph_wf(&self) -> bool {
//!       self@.finite() && valid_key_type::<V>()
//!   }
//!
//! Correct example (Map-backed table):
//!   open spec fn spec_tablesteph_wf(&self) -> bool {
//!       self@.dom().finite() && ...
//!   }
//!
//! ANTIPATTERN 1: finite() missing from wf, assumed in every body.
//!
//!   open spec fn spec_bad_wf(&self) -> bool { /* no finite */ }
//!   fn size(&self) -> (count: usize)
//!       requires self.spec_bad_wf(),
//!   {
//!       proof { assume(self@.finite()); }  // WRONG: 1 hole per method
//!       ...
//!   }
//!   Twenty methods = twenty holes, all saying the same thing that should
//!   have been said once in wf.
//!
//! ANTIPATTERN 2: finite() ensured per-method but not in wf.
//!
//!   fn insert(&mut self, x: T)
//!       requires old(self).spec_X_wf(),
//!       ensures self@.finite(), self.spec_X_wf();
//!
//!   This works but is verbose and fragile. If one method forgets the
//!   `ensures self@.finite()`, downstream breaks. Put it in wf once.
//!
//! ANTIPATTERN 3: Seq-viewing types that skip finite().
//!
//!   A Seq<T> produced from a Vec is always finite. But Verus does not
//!   know Vec::view().len() < infinity unless you tell it. If your type
//!   views as Seq<T> and you call .to_set() or .len(), you need
//!   self@.len() < usize::MAX or equivalent in wf. For Seq views, the
//!   natural analog is that the sequence length is bounded — which Vec
//!   guarantees but the spec type does not.
//!
//! vstd finiteness broadcast lemmas (fire automatically when input is finite):
//!   - axiom_set_empty_finite:      Set::empty().finite()
//!   - axiom_set_insert_finite:     s.finite() ==> s.insert(a).finite()
//!   - axiom_set_remove_finite:     s.finite() ==> s.remove(a).finite()
//!   - axiom_set_union_finite:      s1.finite() && s2.finite() ==> s1.union(s2).finite()
//!   - axiom_set_intersect_finite:  s1.finite() ==> s1.intersect(s2).finite()
//!   - axiom_set_difference_finite: s1.finite() ==> s1.difference(s2).finite()
//!   - lemma_set_subset_finite:     s.finite() && sub.subset_of(s) ==> sub.finite()
//!
//! Once wf gives you self@.finite(), these fire for free. This is the payoff.
//!
//! References:
//! - src/Chap05/SetStEph.rs (correct: finite in wf).
//! - src/standards/spec_wf_standard.rs (wf contract: requires wf, ensures wf).
//! - vstd/set.rs (axiom_set_*_finite broadcast lemmas).
//! - vstd/set_lib.rs (lemma_set_subset_finite, lemma_set_union_finite_iff).

pub mod finite_sets_standard {

    use vstd::prelude::*;

    verus! {

    // CORRECT: finite() in wf.

    pub struct FiniteCollection {
        pub elements: Vec<u64>,
    }

    impl View for FiniteCollection {
        type V = Set<u64>;
        open spec fn view(&self) -> Set<u64> {
            self.elements@.to_set()
        }
    }

    pub trait FiniteCollectionTrait: Sized + View<V = Set<u64>> {
        spec fn spec_finitecollection_wf(&self) -> bool;

        fn new() -> (s: Self)
            ensures s.spec_finitecollection_wf(), s@ == Set::<u64>::empty();

        fn insert(&mut self, x: u64)
            requires old(self).spec_finitecollection_wf(),
            ensures self.spec_finitecollection_wf();

        fn len(&self) -> (count: usize)
            requires self.spec_finitecollection_wf();
        // No `ensures self@.finite()` needed — callers get it from wf.
    }

    impl FiniteCollectionTrait for FiniteCollection {
        // finite() is part of wf. Said once, flows everywhere.
        open spec fn spec_finitecollection_wf(&self) -> bool {
            self@.finite()
        }

        fn new() -> (s: Self) {
            let s = FiniteCollection { elements: Vec::new() };
            assert(s@.finite());
            s
        }

        fn insert(&mut self, x: u64) {
            self.elements.push(x);
            // seq_to_set_is_finite broadcast fires: elements@.to_set().finite().
            // That is self@.finite(), which is wf. No assume needed.
        }

        fn len(&self) -> (count: usize) {
            // wf gives us self@.finite() for free.
            self.elements.len()
        }
    }

    // ANTIPATTERN: assume(self@.finite()) scattered across method bodies.
    //
    // struct BadCollection { elements: Vec<u64> }
    // open spec fn spec_badcollection_wf(&self) -> bool {
    //     true  // no finite!
    // }
    // fn size(&self) -> (count: usize)
    //     requires self.spec_badcollection_wf(),
    // {
    //     proof { assume(self@.finite()); }  // hole
    //     ...
    // }
    // fn insert(&mut self, x: u64) {
    //     proof { assume(self@.finite()); }  // another hole, same fact
    //     ...
    // }
    // fn union(&self, other: &Self) -> (combined: Self) {
    //     proof { assume(self@.finite()); }  // yet another
    //     proof { assume(other@.finite()); } // and another
    //     ...
    // }
    //
    // Four holes for one structural fact. Put it in wf, get zero holes.

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
