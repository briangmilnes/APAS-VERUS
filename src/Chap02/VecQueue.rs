//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 2 — Simple FIFO queue built on Vec.
//!
//! Not optimized - O(n) dequeue. But fully verifiable.

pub mod VecQueue {
    use vstd::prelude::*;

verus! {

    pub struct VecQueue<T> { pub data: Vec<T> }

    impl<T> View for VecQueue<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

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
