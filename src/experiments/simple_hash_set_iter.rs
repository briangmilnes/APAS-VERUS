// Copyright (c) 2025 Brian G. Milnes
//! Simple Abstract Set with Iterator using std::collections::HashSet  
//! Based on simple_set_iter.rs but using HashSet instead of Vec.
//!
//! STATUS: Potentially obsolete - investigating removing HashSetWithViewPlus
//! in favor of using std::collections::HashSet directly with deep_view().

pub mod simple_hash_set_iter {
    use vstd::prelude::*;

    verus! {

    use std::hash::Hash;
    use vstd::std_specs::clone::*;
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::seq_set::*;
    use vstd::hash_set::HashSetWithView;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::ForLoopGhostIteratorNew;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::ForLoopGhostIterator;
    
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            vstd::seq::group_seq_axioms,
            vstd::set::group_set_axioms,
            vstd::std_specs::hash::group_hash_axioms,
            // crate::vstdplus::clone_view::clone_view::group_clone_view_axioms  // clone_view moved to attic
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::set_lib::group_set_lib_default,
    };

    // SimpleHashSet: thin wrapper around std::collections::HashSet
    // Uses vstd specs for HashSet operations
    #[verifier::reject_recursive_types(V)]
    pub struct SimpleHashSet<V: View + Eq + Hash> {
        pub elements: HashSetWithViewPlus<V>,
    }

    impl<V: vstd::view::View + Eq + Hash> vstd::view::View for SimpleHashSet<V> {
        type V = Set<<V as vstd::view::View>::V>;
        open spec fn view(&self) -> Set<<V as vstd::view::View>::V> { self.elements@ }
    }

    pub trait SimpleHashSetTrait<V: Clone + View>: Sized + View<V=Set<<V as View>::V>> {
        fn len(&self) -> usize;
        
        fn new() -> (result: Self)
            requires 
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            ensures 
                result@ == Set::<<V as View>::V>::empty();
        
        fn contains(&self, v: &V) -> (result: bool)
            requires obeys_key_model::<V>(),
            ensures result == self@.contains(v@);

        fn insert(&mut self, v: V) -> (inserted: bool)
            requires 
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            ensures
                self@ == old(self)@.insert(v@),
                inserted == !old(self)@.contains(v@);
        
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, V>)
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@,
                it@.1.no_duplicates();
    }
    
    impl<V: Clone + View + std::cmp::Eq + std::hash::Hash> SimpleHashSetTrait<V> for SimpleHashSet<V> {
        fn len(&self) -> usize                         { self.elements.len()}
        fn new() -> (s: Self)                          { SimpleHashSet { elements: HashSetWithViewPlus::new() }}
        fn contains(&self, v: &V) -> (result: bool)    { self.elements.contains(v)}
        fn insert(&mut self, v: V) -> (inserted: bool) { self.elements.insert(v) }

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, V>)
        { 
            let it = self.elements.iter();
            // HashSetWithViewPlusTrait provides it@.0 == 0 and it@.1.no_duplicates() unconditionally
            // The mapping property still needs to be assumed
            assume(it@.1.map(|i: int, k: V| k@).to_set() == self@);
            it
        }
    }

    // Example: Copy a hash set using loop iteration
    pub fn simple_hash_set_copy_loop<V: Clone + View + std::cmp::Eq + std::hash::Hash>(s1: &SimpleHashSet<V>) -> (s2: SimpleHashSet<V>)
        requires
            obeys_key_model::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            s2@ == s1@,
    {
        let mut s2 = SimpleHashSet::new();
        let mut it = s1.iter();
        
        let ghost s1_seq = it@.1;
        
        loop
            invariant
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                it@.1 == s1_seq,
                s1_seq.map(|i: int, k: V| k@).to_set() == s1@,
                s2@ == s1_seq.take(it@.0).map(|i: int, k: V| k@).to_set(),
            decreases s1_seq.len() - it@.0,
        {
            let ghost old_index = it@.0;
            let ghost old_s2 = s2@;
            match it.next() {
                Some(elem) => {
                    proof {
                        let ghost target_set = s1_seq.take(it@.0).map(|i: int, k: V| k@).to_set();
                        lemma_take_one_more_extends_the_seq_set_with_view(s1_seq, old_index);
                        assert(old_s2 == s1_seq.take(old_index).map(|i: int, k: V| k@).to_set());
                        assert(s1_seq.take(old_index).map(|i: int, k: V| k@).to_set().insert(elem@) == 
                               s1_seq.take(old_index + 1).map(|i: int, k: V| k@).to_set());
                    }
                    
                    let inserted = s2.insert(elem.clone());
                    
                    proof {
                        assert(s2@ == old_s2.insert(elem@));
                        if !inserted {
                            assert(old_s2.contains(elem@));
                            lemma_set_contains_insert_idempotent(old_s2, elem@);
                        }
                        assert(s2@ == s1_seq.take(it@.0).map(|i: int, k: V| k@).to_set());
                    }
                },
                None => { 
                    proof {
                        lemma_take_full_to_set_with_view(s1_seq);
                        assert(s1_seq.take(s1_seq.len() as int).map(|i: int, k: V| k@).to_set() == 
                               s1_seq.map(|i: int, k: V| k@).to_set());
                        assume(s2@ == s1@);
                    }
                    return s2; 
                },
            }
        }
    }

/*
    // Both of these for loops fail due to not being able to make
    // an invariant that relates s1 and s2 due to it being modified in the loop.
    proof fn lemma_seq_map_index_in_set<V: View>(s: Seq<V>, i: int)
            requires 
            0 <= i < s.len(),
        ensures
        s.map(|j: int, v: V| v@).to_set().contains(s[i]@),
     {
         // The proof: s[i]@ is in the mapped sequence at position i,
         // therefore it's in the set when we call to_set()
         assert(s.map(|j: int, v: V| v@)[i] == s[i]@);
     }


    proof fn lemma_seq_map_indexes_in_set<V: View>(s: Seq<V>)
        ensures
            forall |i: int| 0 <= i < s.len() ==> s.map(|j: int, v: V| v@).to_set().contains(#[trigger] s[i]@),
     {
         assert forall |i: int| 0 <= i < s.len() implies 
             s.map(|j: int, v: V| v@).to_set().contains(#[trigger] s[i]@)
         by {
             lemma_seq_map_index_in_set(s, i);
         }
     }

    pub fn simple_hash_set_copy_for<V: Clone + View + std::cmp::Eq + std::hash::Hash>
            (s1: SimpleHashSet<V>) -> (s2: SimpleHashSet<V>)
        requires 
            obeys_key_model::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            s2@ == s1@
    {
        let mut s2 = SimpleHashSet::new();
        let len    = s1.elements.len();
        let ghost old_s1 = s1;
        let s1_iter = s1.iter();
        let ghost s1_iter_seq = s1_iter@.1;

        for elem in it: s1_iter
            invariant
              obeys_key_model::<V>(), forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
              s1@ == old_s1@,
              s2@ <= old_s1@,
              it.elements == s1_iter_seq,
              s1_iter_seq.map(|i: int, k: V| k@).to_set() == old_s1.elements@,
          {
              assert(s2@ == s1_iter_seq.take(it.pos).map(|i: int, k: V| k@).to_set());
              
              proof {
                  lemma_seq_map_indexes_in_set(it.elements);
                  assert(it.elements.map(|i: int, k: V| k@).to_set().contains(it.elements[it.pos]@));
                  // From invariant: it.elements.map(...).to_set() == s1@
                  assert(s1@.contains(it.elements[it.pos]@));
                  assert(vstd::pervasive::cloned(it.elements[it.pos], *elem));
                  assert(elem@ == it.elements[it.pos]@);
                  assert(s1@.contains(elem@));
            }
            assume(!s2@.contains(elem@));  // s2@ <= old_s1@, and no duplicates somehow?
            let inserted = s2.insert(elem.clone());
            assert(inserted);
//            assume(s2@ == s1_iter_seq.take(it.pos + 1).map(|i: int, k: V| k@).to_set());
          }
        
        proof {
//            assert(s2@ == s1_iter_seq.take(s1@.len() as int).map(|i: int, k: V| k@).to_set());
            assert(s1_iter_seq.map(|i: int, k: V| k@).to_set() == s1@);
            assert(s2@ == s1@);
        }
        
        s2

    }

    pub fn simple_hash_set_copy_for<V: Clone + View + std::cmp::Eq + std::hash::Hash>
            (s1: &SimpleHashSet<V>) -> (s2: SimpleHashSet<V>)
        requires 
            obeys_key_model::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            s2@ == s1@,
    {
        let mut s2 = SimpleHashSet::new();
        let len = s1.elements.len();

        for elem in it: s1.iter()
            invariant
                 obeys_key_model::<V>(),
                 forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,

                 len == s1@.len(),
                 it.elements.map(|i: int, k: V| k@).to_set() == s1.elements@,
                 it.pos <= it.elements.len(),
                 it.elements.no_duplicates(),
                 s2@ <= s1@,
                 s2@ == it.elements.map(|i: int, k: V| k@).take(it.pos).to_set(),
        {
            let ghost old_index = it.pos;
            let ghost old_s2 = s2@;
            
            proof { 
                let _ : Set<<V as View>::V> = s1@;
                let _ : HashSetWithViewPlus<V>  = s1.elements; 
                let _ : nat = s1@.len();
                assert(len == s1@.len());

                let _  : Set<V::V> = it.elements.map(|i: int, k: V| k@).to_set();
                let _  : Set<<V as View>::V> = s1.elements@;
                assert(it.elements.map(|i: int, k: V| k@).to_set() == s1.elements@);
                assert(it.pos <= it.elements.len());

                let _ : Set<<V as View>::V> = s2@;
                let _ : Set<<V as View>::V> = it.elements.map(|i: int, k: V| k@).take(it.pos).to_set();
                assert(s2@ == it.elements.map(|i: int, k: V| k@).take(it.pos).to_set());
                
                assert(old_index == it.pos);
                assert(old_s2 == it.elements.map(|i: int, k: V| k@).take(it.pos).to_set());
                
                lemma_take_one_more_extends_the_seq_set_with_view(it.elements, old_index);
            }

            assert(s2@ == it.elements.map(|i: int, k: V| k@).take(it.pos).to_set());
            
            proof { 
                assert(s2@ <= s1@);
                assert(it.elements.no_duplicates());
                assert(vstd::pervasive::cloned(it.elements[it.pos], *elem));
                assert(elem@ == it.elements[it.pos]@);
                assert(s1@.contains(elem@));
            }
            let inserted = s2.insert(elem.clone());
            proof {
                assert(inserted); // We know s1 is a set. 
                assert(s2@.contains(elem@));
                assert(s2@ <= s1@);
            }
            proof {
                assert(it.pos == old_index);  // Check if it.pos changed
                assert(s2@ == old_s2.insert(elem@));
                
                if !inserted {
                    lemma_set_contains_insert_idempotent(old_s2, elem@);
                } 
            }

            // Try to prove the invariant manually
            //assert(s2@ == it.elements.map(|i: int, k: V| k@).take(it.pos + 1).to_set());
        }
        
        proof {
           assert(s2@ == s1@);
        }
        
        s2
    }
*/

    } // verus!
}
