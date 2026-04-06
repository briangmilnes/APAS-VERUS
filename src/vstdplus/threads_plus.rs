//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Thread utilities - self-contained copy of vstd::thread with is_finished added.
//! We copy rather than extend because vstd::thread doesn't expose is_finished.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 7c. proof fns/broadcast groups
//	Section 9c. impls
//	Section 14c. derive impls outside verus!

#![allow(unused_imports)]


//		Section 1. module

pub mod threads_plus {

    //		Section 2. imports

    use core::marker;
    use vstd::prelude::*;

verus! 
{

    //		Section 4a. type definitions


    #[verifier::external_body]
    #[verifier::reject_recursive_types(Ret)]
    pub struct JoinHandlePlus<Ret> { handle: std::thread::JoinHandle<Ret> }

    //		Section 9a. impls


    impl<Ret> JoinHandlePlus<Ret> {
        pub uninterp spec fn predicate(&self, ret: Ret) -> bool;

    /// Check if thread finished without blocking. (Addition over vstd::thread.)
        #[verifier::external_body]
        pub fn is_finished(&self) -> (finished: bool) { self.handle.is_finished() }

        #[verifier::external_body]
        pub fn join(self) -> (res: Result<Ret, ()>)
            ensures match res { Result::Ok(r) => self.predicate(r), Result::Err(_) => true }
        {
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || {
                    match self.handle.join() { Ok(v) => Ok(v), Err(_) => Err(()) }
                }));
            match res {
                Ok(res) => res,
                Err(_) => { println!("panic on join"); std::process::abort(); }
            }
        }
    }

    #[verifier::external_body]
    pub fn spawn_plus<F, Ret>(f: F) -> (handle: JoinHandlePlus<Ret>)
    where F: FnOnce() -> Ret + Send + 'static, 
          Ret: Send + 'static requires f.requires(()) 
        ensures forall|ret: Ret| #[trigger] handle.predicate(ret) ==> f.ensures((), ret)
    {
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || {
                JoinHandlePlus { handle: std::thread::spawn(move || f()) }
            }));
        match res {
            Ok(res) => res,
            Err(_) => { println!("panic on spawn_plus"); std::process::abort(); }
        }
    }

    //		Section 4b. type definitions


    #[verifier::external_body]
    pub struct ThreadIdPlus { thread_id: std::thread::ThreadId }

    //		Section 4c. type definitions


    #[cfg(verus_keep_ghost)]
    pub tracked struct IsThreadPlus {}

    #[cfg(not(verus_keep_ghost))]
    pub tracked struct IsThreadPlus { _no_send_sync: core::marker::PhantomData<*const ()> }

    //		Section 7c. proof fns/broadcast groups


    pub axiom fn ghost_thread_id_plus() -> (tracked res: IsThreadPlus);

    //		Section 9c. impls


    #[cfg(verus_keep_ghost)]
    impl !Sync for IsThreadPlus {}

    #[cfg(verus_keep_ghost)]
    impl !Send for IsThreadPlus {}

    impl IsThreadPlus {
        pub uninterp spec fn view(&self) -> ThreadIdPlus;

        pub axiom fn agrees(tracked self, tracked other: IsThreadPlus)
        ensures self@ == other@;
    }

    impl Copy for IsThreadPlus {}

    #[verifier::external_body]
    pub fn thread_id_plus() -> (res: (ThreadIdPlus, Tracked<IsThreadPlus>))
        ensures res.1@@ == res.0
    {
        let id = ThreadIdPlus { thread_id: std::thread::current().id() };
        (id, Tracked::assume_new())
    }
} // verus!

    //		Section 14c. derive impls outside verus!


impl Clone for IsThreadPlus {
    #[cfg(verus_keep_ghost)]
    fn clone(&self) -> Self { IsThreadPlus {} }

    #[cfg(not(verus_keep_ghost))]
    fn clone(&self) -> Self { IsThreadPlus { _no_send_sync: Default::default() } }
}

// ThreadShareablePlus deleted — dead code, was the only source of unsafe impl Send/Sync in vstdplus.

}
