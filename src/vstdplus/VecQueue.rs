// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 2 — Simple FIFO queue built on Vec.
//!
//! Not optimized - O(n) dequeue. But fully verifiable.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod VecQueue {

    //		Section 2. imports

    use vstd::prelude::*;

verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


    pub struct VecQueue<T> { pub data: Vec<T> }

    //		Section 5. view impls


    impl<T> View for VecQueue<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

    //		Section 8. traits


    pub trait VecQueueTrait<T>: Sized + View<V = Seq<T>> {
        fn new() -> (vq: Self)
            ensures vq@ == Seq::<T>::empty();

        fn len(&self) -> (len: usize)
            ensures len == self@.len();

        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self@.len() == 0);

        fn enqueue(&mut self, val: T)
            ensures self@ == old(self)@.push(val);

        fn dequeue(&mut self) -> (dequeued: Option<T>)
            ensures
                old(self)@.len() == 0 ==> dequeued.is_none() && self@ == old(self)@,
                old(self)@.len() > 0 ==> dequeued == Some(old(self)@[0]) && self@ == old(self)@.subrange(1, old(self)@.len() as int);
    }

    //		Section 9. impls


    impl<T> VecQueueTrait<T> for VecQueue<T> {

        fn new() -> (vq: Self) { VecQueue { data: Vec::new() } }

        fn len(&self) -> (len: usize) { self.data.len() }

        fn is_empty(&self) -> (is_empty: bool) { self.data.len() == 0 }

        fn enqueue(&mut self, val: T) { self.data.push(val); }

        fn dequeue(&mut self) -> (dequeued: Option<T>) {
            if self.data.len() == 0 { None } else { Some(self.data.remove(0)) }
        }
    }

 } // verus!
}
