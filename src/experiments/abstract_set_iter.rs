//! Abstract Set with Set-based Iterator View (visited, current, remaining)
//! Based on simple_set_iter but with a set-based ghost model instead of sequence-based.

pub mod abstract_set_iter {
    use vstd::prelude::*;

    verus! {

    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::vec::vec::*;
    
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            vstd::seq::group_seq_axioms,
            vstd::set::group_set_axioms,
            crate::vstdplus::clone_view::clone_view::group_clone_view_axioms,
            crate::vstdplus::feq::feq::group_feq_axioms
    };


    // AbstractSet backed by Vec (no duplicates maintained by insert).
    #[verifier::reject_recursive_types(V)]
    pub struct AbstractSet<V> {pub elements: Vec<V>, }

    impl<V> View for AbstractSet<V> {
        type V = Set<V>;
        open spec fn view(&self) -> Set<V> { self.elements@.to_set() }
    }

    // Set invariant: backing vec has no duplicates and bounded length
    pub open spec fn set_inv<V>(s: &AbstractSet<V>) -> bool {
        s.elements@.no_duplicates() && s.elements@.len() < usize::MAX
    }

    // V::V = V means the view of an element is itself (like u32, i64, etc.)
    pub trait AbstractSetTrait<V: ClonePlus + View<V=V> + Eq>: Sized + View<V=Set<V>> {
        spec fn wf(&self) -> bool;
        
        fn len(&self) -> usize;
        
        fn new() -> (result: Self)
            ensures result@ == Set::<V>::empty(), result.wf();
        
        fn mem(&self, v: &V) -> (result: bool)
            requires obeys_feq_full::<V>()
            ensures result == self@.contains(*v);

        fn insert(&mut self, v: V) -> (inserted: bool)
            requires 
                obeys_feq_full::<V>(), 
                old(self).wf(),
                old(self)@.len() < (usize::MAX - 1) as int,  // Room for one more
            ensures
                self.wf(),
                old(self)@.contains(v) ==> {
                    &&& !inserted
                    &&& self@ == old(self)@
                },
                !old(self)@.contains(v) ==> {
                    &&& inserted
                    &&& self@ == old(self)@.insert(v)
                };
        
        fn iter(&self) -> (it: AbstractSetIter<V>)
            requires self.wf()
            ensures
                it@.visited == Set::<V>::empty(),
                it@.current.is_none(),
                it@.remaining == self@,
                it.pos == 0,
                it.vec@.to_set() == self@,
                it.vec@.no_duplicates(),
                it.vec@.len() < usize::MAX;
    }
    
    impl<V: ClonePlus + Eq + View<V=V>> AbstractSetTrait<V> for AbstractSet<V> {
        open spec fn wf(&self) -> bool {
            self.elements@.no_duplicates() && self.elements@.len() < usize::MAX
        }
        
        fn len(&self) -> usize { self.elements.len() }
        
        fn new() -> (s: Self) { AbstractSet { elements: Vec::new() } }
        
        fn mem(&self, v: &V) -> (result: bool)
        {
            let ghost elements_seq = self.elements@;
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    self.elements@ == elements_seq,
                    0 <= i <= self.elements.len(),
                    obeys_feq_full::<V>(),
                    forall |j: int| 0 <= j < i ==> self.elements@[j] != *v,
                decreases self.elements.len() - i,
            {
                let eq = feq(&self.elements[i], v);
                if eq {
                    return true;
                }
                i = i + 1;
            }
            false
        }
        
        fn insert(&mut self, v: V) -> (inserted: bool) {
            if self.mem(&v) { return false; }
            let ghost old_seq = self.elements@;
            self.elements.push(v);
            proof {
                assert(!old_seq.contains(v));
                assert(self.elements@ =~= old_seq.push(v));
                assert(self.elements@.no_duplicates());
                // From wf(): old_seq.no_duplicates()
                // Use vstd lemma: no_duplicates ==> seq.len() == seq.to_set().len()
                old_seq.unique_seq_to_set();
                // old(self)@ == old_seq.to_set(), so old(self)@.len() == old_seq.len()
                // Precondition: old(self)@.len() < usize::MAX - 1
                // Therefore: old_seq.len() < usize::MAX - 1
                assert(old_seq.len() < (usize::MAX - 1) as int);
                assert(self.elements@.len() == old_seq.len() + 1);
                assert(self.elements@.len() < (usize::MAX) as int);
            }
            true
        }

        fn iter(&self) -> (it: AbstractSetIter<V>)
        { 
            let cloned = self.elements.clone_plus();
            proof {
                // clone_plus ensures cloned(self.elements, cloned)
                // Use lemma to get view equality
                lemma_vec_clone_preserves_view(&self.elements, cloned);
            }
            AbstractSetIter { vec: cloned, pos: 0 }
        }
    }

    // Iterator: exec state is still vec + position, but ghost view is set-based
    #[verifier::reject_recursive_types(V)]
    pub struct AbstractSetIter<V> {
        pub vec: Vec<V>,   // Exec: backing vector (linearized set)
        pub pos: usize,    // Exec: current position
    }

    // Ghost view: visited, current, remaining sets
    #[verifier::reject_recursive_types(V)]
    pub struct AbstractSetIterView<V> {
        pub visited: Set<V>,
        pub current: Option<V>,
        pub remaining: Set<V>,
    }

    // Iterator view is (visited, current, remaining) sets
    // At pos=0: visited={}, current=None, remaining=all
    // At pos=k (0<k<=len): visited=take(k-1), current=Some(vec[k-1]), remaining=skip(k)
    // At pos>len: visited=all, current=None, remaining={}
    impl<V> View for AbstractSetIter<V> {
        type V = AbstractSetIterView<V>;
        
        open spec fn view(&self) -> AbstractSetIterView<V> {
            let seq = self.vec@;
            if self.pos == 0 {
                AbstractSetIterView {
                    visited: Set::empty(),
                    current: None,
                    remaining: seq.to_set(),
                }
            } else if self.pos as int <= seq.len() {
                AbstractSetIterView {
                    visited: seq.take(self.pos as int - 1).to_set(),
                    current: Some(seq[self.pos as int - 1]),
                    remaining: seq.skip(self.pos as int).to_set(),
                }
            } else {
                // pos > len: exhausted
                AbstractSetIterView {
                    visited: seq.to_set(),
                    current: None,
                    remaining: Set::empty(),
                }
            }
        }
    }

    // Helper: convert Option<V> to Set<V>
    pub open spec fn option_to_set<V>(opt: Option<V>) -> Set<V> {
        match opt {
            None => Set::empty(),
            Some(v) => Set::empty().insert(v),
        }
    }

    // The iterator invariant
    pub open spec fn iter_invariant<V>(it: &AbstractSetIter<V>, original: Set<V>) -> bool {
        let seq = it.vec@;
        &&& it.pos <= seq.len()
        &&& seq.to_set() == original
        &&& seq.no_duplicates()
        &&& it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == original
        &&& it@.visited.disjoint(it@.remaining)
        &&& it@.current.is_some() ==> !it@.visited.contains(it@.current.unwrap())
        &&& it@.current.is_some() ==> !it@.remaining.contains(it@.current.unwrap())
    }

    impl<V: ClonePlus + View<V=V> + Eq> AbstractSetIter<V> {
        pub fn next(&mut self) -> (result: Option<V>)
            requires
                old(self).pos <= old(self).vec@.len(),
                old(self).vec@.no_duplicates(),
                old(self).vec@.len() < usize::MAX,
                obeys_feq_full::<V>(),
            ensures 
                self.vec@ =~= old(self).vec@,
                self.vec@.no_duplicates(),
                self.pos <= self.vec@.len() + 1,
                self.pos == old(self).pos + 1,
                result.is_none() ==> old(self).pos == old(self).vec@.len(),
                result.is_some() ==> old(self).pos < old(self).vec@.len(),
                result.is_some() ==> result.unwrap() == old(self).vec@[old(self).pos as int],
                // Set-based postconditions
                ({
                    let old_view = old(self)@;
                    let new_view = self@;
                    match result {
                        None => {
                            &&& old_view.remaining.is_empty()
                            &&& new_view.visited == old_view.visited.union(option_to_set(old_view.current))
                            &&& new_view.current.is_none()
                            &&& new_view.remaining.is_empty()
                        },
                        Some(element) => {
                            &&& old_view.remaining.contains(element)
                            &&& new_view.visited == old_view.visited.union(option_to_set(old_view.current))
                            &&& new_view.current == Some(element)
                            &&& new_view.remaining == old_view.remaining.remove(element)
                        },
                    }
                })
        {
            if self.pos < self.vec.len() {
                let ghost old_pos = self.pos as int;
                let ghost old_seq = self.vec@;
                let elem = self.vec[self.pos].clone_plus();
                // Use feq to prove elem == old_seq[old_pos]
                // For V: View<V=V>, feq gives elem@ == vec[pos]@ which is elem == vec[pos]
                let eq = feq(&elem, &self.vec[self.pos]);
                // feq ensures: eq == (elem@ == self.vec[self.pos]@)
                // Since V::V = V, this is: eq == (elem == self.vec[self.pos])
                // And self.vec[self.pos] == old_seq[old_pos] (vec unchanged)
                self.pos = self.pos + 1;
                proof {
                    assert(old_seq.skip(old_pos).len() > 0);
                    assert(old_seq.skip(old_pos)[0] == old_seq[old_pos]);
                    assert(old_seq.skip(old_pos).to_set().contains(old_seq[old_pos]));
                    // feq gave us elem@ == old_seq[old_pos]@, and V::V = V, so elem == old_seq[old_pos]
                    assert(eq);  // feq returns true for clone
                    assert(elem == old_seq[old_pos]);
                }
                Some(elem)
            } else {
                self.pos = self.pos + 1;
                proof {
                    let seq = self.vec@;
                    assert(seq.skip((self.pos - 1) as int).len() == 0);
                }
                None
            }
        }
    }

    // Example: Copy a set using loop iteration with set-based invariants
    pub fn abstract_set_copy_loop(s1: &AbstractSet<u32>) -> (s2: AbstractSet<u32>)
        requires s1.wf(), s1@.len() < (usize::MAX - 1) as int
        ensures
            s2@ == s1@,
            s2.wf(),
    {
        let mut s2 = AbstractSet::new();
        let mut it = s1.iter();
        let ghost original = s1@;
        let ghost the_seq = it.vec@;
        
        loop
            invariant
                original == s1@,
                original.len() < (usize::MAX - 1) as int,
                it.vec@ == the_seq,
                the_seq.to_set() == original,
                the_seq.no_duplicates(),
                the_seq.len() < usize::MAX,
                it.pos <= it.vec@.len(),
                s2.wf(),
                s2@.len() < (usize::MAX - 1) as int,
                s2@ == it@.visited.union(option_to_set(it@.current)),
                s2@ <= original,  // s2 is a subset of original
                it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == original,
            decreases it.vec@.len() - it.pos,
        {
            match it.next() {
                Some(elem) => { 
                    let ghost old_s2 = s2@;
                    let _ = s2.insert(elem);
                    proof {
                        // After insert, s2@ is either old_s2 or old_s2.insert(elem)
                        // Either way, s2@ <= original (since elem is from original)
                        assert(s2@ <= original);
                        // Use vstd lemma: subset of finite set has smaller or equal cardinality
                        vstd::set_lib::lemma_len_subset(s2@, original);
                    }
                },
                None => { 
                    proof {
                        // original == s1@ from initialization
                        assert(original =~= s1@);
                        // After None: it@.current is None, it@.remaining is empty
                        // it@.visited absorbed the old current
                        assert(it@.current.is_none());
                        assert(it@.remaining.is_empty());
                        assert(option_to_set::<u32>(it@.current) =~= Set::empty());
                        // From invariant: s2@ == it@.visited.union(option_to_set(it@.current))
                        assert(s2@ =~= it@.visited.union(Set::empty()));
                        assert(s2@ =~= it@.visited);
                        // From invariant: it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == original
                        assert(it@.visited.union(Set::empty()).union(Set::empty()) =~= original);
                        assert(it@.visited =~= original);
                        assert(s2@ =~= original);
                    }
                    return s2; 
                },
            }
        }
    }

    } // verus!
}
