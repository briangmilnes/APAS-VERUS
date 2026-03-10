//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: generic struct behind Arc<RwLock> with Views on inner and outer.
//!
//! Inner<T>: struct holding a Vec<T> with View<V = Seq<T>>, fully verified.
//! Outer<T>: struct wrapping Inner<T> in Arc<RwLock>, View requires external_body.
//! Question: can specs flow through the lock without external_body on methods?

// Table of Contents
// 1. module
// 4. type definitions
// 5. view impls
// 8. traits
// 9. impls
// 13. derive impls outside verus!

// 1. module

pub mod arc_rwlock_trivial {

    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct Inner<T: Send + Sync> {
        pub data: Vec<T>,
    }

    pub struct ArcRwLockTrivialInv;

    impl<T: Send + Sync> RwLockPredicate<Inner<T>> for ArcRwLockTrivialInv {
        open spec fn inv(self, v: Inner<T>) -> bool {
            v.data@.len() < usize::MAX
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct Outer<T: Send + Sync> {
        inner: Arc<RwLock<Inner<T>, ArcRwLockTrivialInv>>,
    }

    // 5. view impls

    impl<T: Send + Sync> View for Inner<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> {
            self.data@
        }
    }

    // Outer cannot access locked data in spec mode.
    impl<T: Send + Sync> Outer<T> {
        #[verifier::external_body]
        pub open spec fn spec_seq_view(&self) -> Seq<T> {
            Seq::empty()
        }
    }

    impl<T: Send + Sync> View for Outer<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.spec_seq_view() }
    }

    // 8. traits

    pub trait InnerTrait<T: Send + Sync>: Sized + View<V = Seq<T>> {
        fn new() -> (s: Self)
            ensures s@.len() == 0;

        fn append(&mut self, val: T)
            requires old(self)@.len() < usize::MAX,
            ensures self@ == old(self)@.push(val);

        fn length(&self) -> (len: usize)
            ensures len == self@.len();
    }

    pub trait OuterTrait<T: Send + Sync>: Sized + View<V = Seq<T>> {
        fn new() -> (s: Self)
            ensures s@.len() == 0;

        fn append(&self, val: T)
            ensures self@.contains(val);

        fn length(&self) -> (len: usize)
            ensures len == self@.len();
    }

    // 9. impls

    impl<T: Send + Sync> InnerTrait<T> for Inner<T> {
        fn new() -> (s: Self) {
            Inner { data: Vec::new() }
        }

        fn append(&mut self, val: T) {
            self.data.push(val);
        }

        fn length(&self) -> (len: usize) {
            self.data.len()
        }
    }

    impl<T: Send + Sync> OuterTrait<T> for Outer<T> {
        #[verifier::external_body]
        fn new() -> (s: Self)
            ensures s@.len() == 0
        {
            Outer {
                inner: new_arc_rwlock::<Inner<T>, ArcRwLockTrivialInv>(
                    Inner::new(), Ghost(ArcRwLockTrivialInv)),
            }
        }

        #[verifier::external_body]
        fn append(&self, val: T)
            ensures self@.contains(val)
        {
            let (mut inner, write_handle) = self.inner.acquire_write();
            if inner.length() + 1 < usize::MAX {
                inner.append(val);
            }
            write_handle.release_write(inner);
        }

        #[verifier::external_body]
        fn length(&self) -> (len: usize)
            ensures len == self@.len()
        {
            let handle = self.inner.acquire_read();
            let len = handle.borrow().length();
            handle.release_read();
            len
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: Send + Sync + std::fmt::Debug> std::fmt::Debug for Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Inner").field("len", &self.data.len()).finish()
        }
    }

    impl<T: Send + Sync> std::fmt::Debug for Outer<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Outer").finish()
        }
    }

    impl std::fmt::Debug for ArcRwLockTrivialInv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArcRwLockTrivialInv")
        }
    }
}
