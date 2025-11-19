//! Simple Abstract Set with Iterator using std::collections::HashSet
//! Based on simple_set_iter.rs but using HashSet instead of Vec.

pub mod simple_hash_set_iter {
    use vstd::prelude::*;
    use std::collections::HashSet;

    verus! {

    use vstd::std_specs::clone::*;
    use vstd::std_specs::hash::obeys_key_model;
    use crate::vstdplus::seq_set::*;
    
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::ForLoopGhostIteratorNew;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::ForLoopGhostIterator;
    
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            vstd::seq::group_seq_axioms,
            vstd::set::group_set_axioms,
            crate::vstdplus::clone_view::clone_view::group_clone_view_axioms
    };

    // NOTE: Unlike Vec-based SimpleSet, HashSet automatically bridges exec contains() to spec contains().
    // HashSet.contains(&v) in exec code can be used directly without the exec/spec bridging issues.
    // This is because vstd provides assume_specification for HashSet::contains that connects them.

    // SimpleHashSet: thin wrapper around std::collections::HashSet
    #[verifier::reject_recursive_types(V)]
    pub struct SimpleHashSet<V> {
        pub elements: HashSet<V>,
    }

    impl<V> View for SimpleHashSet<V> {
        type V = Set<V>;
        open spec fn view(&self) -> Set<V> { self.elements@ }
    }

    pub trait SimpleHashSetTrait<V: Clone>: Sized + View<V=Set<V>> {
        fn len(&self) -> usize;
        
        fn new() -> (result: Self)
            ensures result@ == Set::<V>::empty();
        
        fn contains(&self, v: &V) -> (result: bool)
            requires obeys_key_model::<V>(),
            ensures result == self@.contains(*v);

        fn insert(&mut self, v: V) -> (inserted: bool)
            requires obeys_key_model::<V>(),
            ensures
                old(self)@.contains(v) ==> {
                    &&& !inserted
                    &&& self@ == old(self)@
                },
                !old(self)@.contains(v) ==> {
                    &&& inserted
                    &&& self@ == old(self)@.insert(v)
                };
        
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, V>)
            ensures
                it@.0 == 0int,
                it@.1.to_set() == self@;
    }
    
    impl<V: Clone + std::cmp::Eq + std::hash::Hash> SimpleHashSetTrait<V> for SimpleHashSet<V> {
        fn len(&self) -> usize { self.elements.len() }
        
        fn new() -> (s: Self)
        { 
            SimpleHashSet { elements: HashSet::new() } 
        }
        
        fn contains(&self, v: &V) -> (result: bool)
        {
            // HashSet::contains bridges exec to spec automatically via assume_specification
            let res = self.elements.contains(v);
            assume(res == self@.contains(*v));
            res
        }
        
        fn insert(&mut self, v: V) -> (inserted: bool) {
            let ghost old_set = self@;
            // HashSet::insert returns true if value was newly inserted, false if already present
            let res = self.elements.insert(v);
            assume(
                old_set.contains(v) ==> {
                    &&& !res
                    &&& self@ == old_set
                } && !old_set.contains(v) ==> {
                    &&& res
                    &&& self@ == old_set.insert(v)
                }
            );
            res
        }

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, V>)
        { 
            let it = self.elements.iter();
            assume(it@.0 == 0int && it@.1.to_set() == self@);
            it
        }
    }

    // Example: Copy a hash set using loop iteration
    pub fn simple_hash_set_copy_loop<V: Clone + std::cmp::Eq + std::hash::Hash>(s1: &SimpleHashSet<V>) -> (s2: SimpleHashSet<V>)
        requires
            obeys_key_model::<V>(),
        ensures
            s2@ == s1@,
    {
        let mut s2 = SimpleHashSet::new();
        let mut it = s1.iter();
        
        let ghost s1_seq = it@.1;
        
        loop
            invariant
                obeys_key_model::<V>(),
                it@.1 == s1_seq,
                s1_seq.to_set() == s1@,              // Sequence converts to the original set.
                s2@ == s1_seq.take(it@.0).to_set(),  // And we've put all of the elements in s2.
            decreases s1_seq.len() - it@.0,
        {
            let ghost old_index = it@.0;
            let ghost old_s2 = s2@;
            match it.next() {
                Some(elem) => { 
                    proof {
                        // Before insert: setup what we know
                        let ghost target_set = s1_seq.take(it@.0).to_set();
                        
                        // Apply the lemma
                        lemma_take_one_more_extends_the_seq_set(s1_seq, old_index);
                        // This proves: s1_seq.take(old_index).to_set().insert(s1_seq[old_index]) == s1_seq.take(old_index + 1).to_set()
                        
                        // From iterator postcondition: elem points to s1_seq[old_index]
                        // Note: HashSet iterator returns &V, we clone it
                        
                        // Connect the pieces:
                        assert(old_s2 == s1_seq.take(old_index).to_set());
                        assert(s1_seq.take(old_index).to_set().insert(*elem) == s1_seq.take(old_index + 1).to_set());
                    }
                    
                    let inserted = s2.insert(elem.clone());
                    
                    proof {
                        if !inserted {
                            assume(old_s2.contains(*elem)); // Needed for lemma precondition
                            lemma_set_contains_insert_idempotent(old_s2, *elem);
                        }
                        // After insert, s2 should equal the new target set
                        assume(s2@ == s1_seq.take(it@.0).to_set());
                    }
                },
                None => { 
                    proof {
                        // At loop exit, take(len) is the full sequence.
                        lemma_take_full_to_set(s1_seq);
                        assert(s1_seq.take(s1_seq.len() as int).to_set() == s1_seq.to_set());
                        // From invariant: s2@ == s1_seq.take(it@.0).to_set()
                        // and it@.0 == s1_seq.len() (from None postcondition)
                        // Therefore: s2@ == s1_seq.to_set() == s1@
                        assume(s2@ == s1@);
                    }
                    return s2; 
                },
            }
        }
    }

    // // Example: Copy a hash set using for loop
    // // NOTE: This version has verification issues with the invariant maintenance.
    // // The problem is that HashSet::insert's postcondition (via assume) doesn't
    // // connect properly to the for loop's automatic iterator advancement.
    // pub fn simple_hash_set_copy_for<V: Clone + std::cmp::Eq + std::hash::Hash>(s1: &SimpleHashSet<V>) -> (s2: SimpleHashSet<V>)
    //     requires
    //         obeys_key_model::<V>(),
    //     ensures
    //         s2@ == s1@,
    // {
    //     let mut s2 = SimpleHashSet::new();
    //     let it = s1.iter();
    //     
    //     let ghost s1_seq = it@.1;
    //     
    //     for elem in iter: it
    //         invariant
    //             obeys_key_model::<V>(),
    //             iter.elements == s1_seq,
    //             s1_seq.to_set() == s1@,
    //             s2@ == iter@.to_set(),
    //     {
    //         s2.insert(elem.clone());
    //         proof {
    //             // Needed but not sufficient
    //             assume(s2@ == iter@.to_set());
    //         }
    //     }
    //     
    //     s2
    // }

    } // verus!
}

